const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const Connection = @import("../../network/Connection.zig");
const EchoInfo = @import("../../fs/EchoInfo.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const gameplay_tags = @import("gameplay_tags.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
const PlayerRoleComponent = @import("../component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerGuideComponent = @import("../component/player/PlayerGuideComponent.zig");
const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const CalabashHelper = @import("calabash.zig");
const Range = @import("../../data/tables/entity_components/Range.zig");
const Scene = @import("../Scene.zig");

pub const EvalContext = struct {
    assets: ?*const Assets = null,
    basic_comp: *const PlayerBasicComponent,
    echo_comp: *const PlayerEchoComponent,
    inventory_comp: *const PlayerInventoryComponent,
    map_comp: *const PlayerMapComponent,
    quest_comp: *const PlayerQuestComponent,
    role_comp: ?*const PlayerRoleComponent = null,
    weapon_comp: ?*const PlayerWeaponComponent = null,
    guide_comp: ?*const PlayerGuideComponent = null,
    progress_comp: ?*const PlayerProgressComponent = null,
    scene: ?*const Scene = null,
    owner_quest_id: i32 = 0,
    target_role_id: i32 = 0,
    self_entity_config_id: i32 = 0,
};

pub fn pushFuncOpenNotify(
    conn: *Connection,
    alloc: mem.Alloc,
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    echo_comp: *const PlayerEchoComponent,
    inventory_comp: *const PlayerInventoryComponent,
    map_comp: *const PlayerMapComponent,
    quest_comp: *const PlayerQuestComponent,
    role_comp: ?*const PlayerRoleComponent,
    io: std.Io,
) !void {
    const eval_ctx: EvalContext = .{
        .assets = assets,
        .basic_comp = basic_comp,
        .echo_comp = echo_comp,
        .inventory_comp = inventory_comp,
        .map_comp = map_comp,
        .quest_comp = quest_comp,
        .role_comp = role_comp,
    };

    var notify: pb.FuncOpenNotify = .default;
    for (assets.tables.function_condition.items) |config| {
        const is_open = isFunctionConfigOpen(eval_ctx, config);
        const flag: i32 = if (is_open) 2 else 0;
        try notify.Func.append(alloc.arena, .{
            .Id = config.FunctionId,
            .Flag = flag,
        });
    }

    try conn.push(alloc.gpa, io, notify);
}

fn isFunctionConfigOpen(ctx: EvalContext, config: Assets.DataTables.FunctionCondition) bool {
    return config.OpenConditionId == 0 or
        config.FunctionId == 10026010 or
        ctx.quest_comp.info.hasUnlockedSystem(config.FunctionId) or
        evalConditionGroup(ctx, config.OpenConditionId);
}

pub fn isFunctionOpen(ctx: EvalContext, function_id: i32) bool {
    if (function_id == 10026010 or ctx.quest_comp.info.hasUnlockedSystem(function_id)) return true;
    const assets = ctx.assets orelse return false;
    const config = assets.tables.function_condition.getDataById(function_id) orelse return false;
    return isFunctionConfigOpen(ctx, config);
}

pub fn evalConditionGroup(ctx: EvalContext, group_id: i32) bool {
    if (group_id <= 0) return true;
    const a = ctx.assets orelse return false;
    const group = a.tables.condition_group.getDataById(group_id) orelse return false;
    if (group.GroupId.len == 0) return true;

    // Relation 0 = AND (all must pass); Relation 1 = OR (at least one passes)
    const is_or = group.Relation == 1;
    if (is_or) {
        for (group.GroupId) |cond_id| {
            if (evalConditionId(ctx, cond_id)) return true;
        }
        return false;
    }
    for (group.GroupId) |cond_id| {
        if (!evalConditionId(ctx, cond_id)) return false;
    }
    return true;
}

pub fn evalConditionOrGroup(ctx: EvalContext, id: i32) bool {
    if (id <= 0) return true;
    const a = ctx.assets orelse return false;
    if (a.tables.condition_group.getDataById(id) != null) {
        return evalConditionGroup(ctx, id);
    }
    if (a.tables.condition.getDataById(id) != null) {
        return evalConditionId(ctx, id);
    }
    return false;
}

pub fn evalConditionId(ctx: EvalContext, cond_id: i32) bool {
    const a = ctx.assets orelse return false;
    const cond = a.tables.condition.getDataById(cond_id) orelse return false;
    return evalConditionRecord(ctx, cond);
}

pub fn evalConditionRecord(ctx: EvalContext, cond: Assets.DataTables.Condition) bool {
    const lp: std.json.ObjectMap = if (cond.LimitParams) |v| switch (v) {
        .object => |obj| obj,
        else => .empty,
    } else .empty;

    if (std.mem.eql(u8, cond.Type, "CheckLevel")) {
        const req = getParamInt(lp, "Level") orelse 1;
        return ctx.basic_comp.info.attributes.level >= req;
    } else if (std.mem.eql(u8, cond.Type, "CheckOriginWorldLevel")) {
        const req = getParamInt(lp, "Level") orelse 0;
        return ctx.basic_comp.info.attributes.origin_world_level >= req;
    } else if (std.mem.eql(u8, cond.Type, "CheckCurWorldLevel")) {
        const req = getParamInt(lp, "Level") orelse 0;
        return ctx.basic_comp.info.attributes.cur_world_level >= req;
    } else if (std.mem.eql(u8, cond.Type, "CalabashLevel")) {
        const req = getParamInt(lp, "Level") orelse 1;
        const a = ctx.assets orelse return false;
        return currentCalabashLevel(a, ctx.basic_comp, ctx.echo_comp) >= req;
    } else if (std.mem.eql(u8, cond.Type, "NewQuestState")) {
        const qid = getParamInt(lp, "NewQuestId") orelse return false;
        const st = getParamInt(lp, "State") orelse 3;
        return questStateValue(ctx, qid) == st;
    } else if (std.mem.eql(u8, cond.Type, "NewQuestStepState") or std.mem.eql(u8, cond.Type, "QuestNodeStatus")) {
        const qid = getParamInt(lp, "NewQuestId") orelse return false;
        const nid = getParamInt(lp, "QuestNodeId") orelse return false;
        const st = getParamInt(lp, "State") orelse 3;
        return evalQuestNodeStatus(ctx, qid, nid, st);
    } else if (std.mem.eql(u8, cond.Type, "CheckTeleportStatus")) {
        const tid = getParamInt(lp, "TeleportId") orelse return false;
        const want = (getParamInt(lp, "State") orelse 1) != 0;
        const unlocked = containsInt(ctx.map_comp.info.unlocked_teleporters, tid);
        return unlocked == want;
    } else if (std.mem.eql(u8, cond.Type, "PassInst")) {
        const inst_id = getParamInt(lp, "InstanceId") orelse return false;
        return containsInt(ctx.quest_comp.info.completed_dungeons, inst_id);
    } else if (std.mem.eql(u8, cond.Type, "LevelPlayComplete")) {
        const raw = getParamStr(lp, "LevelPlayId") orelse return false;
        var count: i32 = 0;
        var it = std.mem.splitScalar(u8, raw, '-');
        while (it.next()) |tok| {
            const lpid = std.fmt.parseInt(i32, tok, 10) catch continue;
            if (containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) count += 1;
        }
        return count >= @max(cond.NeedNum, 1);
    } else if (std.mem.eql(u8, cond.Type, "LevelPlayCompleteNew")) {
        const raw = getParamStr(lp, "LevelPlayId") orelse return false;
        var it = std.mem.tokenizeScalar(u8, raw, '-');
        while (it.next()) |tok| {
            const lpid = std.fmt.parseInt(i32, tok, 10) catch continue;
            if (containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) return true;
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "ItemCheck") or std.mem.eql(u8, cond.Type, "GetItem") or std.mem.eql(u8, cond.Type, "GetNewItem") or std.mem.eql(u8, cond.Type, "CostItem")) {
        const item_id = getParamInt(lp, "ItemId") orelse getParamInt(lp, "ItemID") orelse return false;
        return itemCount(ctx, item_id) >= @max(cond.NeedNum, 1);
    } else if (std.mem.eql(u8, cond.Type, "CheckItemWithOperator")) {
        const item_id = getParamInt(lp, "ItemId") orelse getParamInt(lp, "ItemID") orelse return false;
        const req_count = getParamInt(lp, "Count") orelse @max(cond.NeedNum, 1);
        const op = getParamStr(lp, "Op") orelse "Ge";
        return compareInts(itemCount(ctx, item_id), req_count, op);
    } else if (std.mem.eql(u8, cond.Type, "FunctionUnlock")) {
        const fid = getParamInt(lp, "FunctionId") orelse return false;
        return ctx.quest_comp.info.hasUnlockedSystem(fid);
    } else if (std.mem.eql(u8, cond.Type, "CheckInstanceState")) {
        const inst_id = getParamInt(lp, "InstanceId") orelse return false;
        const want_done = (getParamInt(lp, "State") orelse 1) != 0;
        return containsInt(ctx.quest_comp.info.completed_dungeons, inst_id) == want_done;
    } else if (std.mem.eql(u8, cond.Type, "TriggerTeleport")) {
        const tid = getParamInt(lp, "TeleportId") orelse return false;
        return containsInt(ctx.map_comp.info.unlocked_teleporters, tid);
    } else if (std.mem.eql(u8, cond.Type, "VarDefineInt")) {
        const vd_name = getParamStr(lp, "VDName") orelse return false;
        const expected = getParamInt(lp, "VDValueInt") orelse 0;
        return ctx.quest_comp.info.getVarInt(vd_name) == expected;
    } else if (std.mem.eql(u8, cond.Type, "CheckClientQuestNodeStatus") or std.mem.eql(u8, cond.Type, "CheckClientQuest")) {
        const qid = getParamInt(lp, "任务Id") orelse return false;
        const nid_opt = getParamInt(lp, "步骤Id");
        const st = getParamInt(lp, "状态") orelse 3;
        if (nid_opt) |nid| {
            return evalQuestNodeStatus(ctx, qid, nid, st);
        }
        return evalQuestState(ctx, qid, st);
    } else if (std.mem.eql(u8, cond.Type, "EntityStateById")) {
        const cid = getParamInt(lp, "EntityConfigId") orelse return false;
        const expected_tag = getParamInt(lp, "Tag") orelse return false;
        const state_str = ctx.quest_comp.info.getEntityState(cid) orelse return false;
        const a = ctx.assets orelse return false;
        const cur_tag = gameplay_tags.idFromName(&a.tables.gameplay_tag_remap, state_str) catch return false;
        return cur_tag == expected_tag;
    } else if (std.mem.eql(u8, cond.Type, "CheckActivityState")) {
        const aid = getParamInt(lp, "ActivityId") orelse return false;
        const want = getParamInt(lp, "ActivityState") orelse 2;
        const op = getParamStr(lp, "Compare") orelse "Eq";
        const cur: i32 = if (isActivityOpen(ctx, aid)) 2 else 0;
        return compareInts(cur, want, op);
    } else if (std.mem.eql(u8, cond.Type, "ReadMail")) {
        const mid = getParamInt(lp, "MailId") orelse return false;
        return containsInt(ctx.quest_comp.info.progress.read_mails, mid);
    } else if (std.mem.eql(u8, cond.Type, "ReadPhoneMessage")) {
        const pmid = getParamInt(lp, "PhoneMessageId") orelse return false;
        return containsInt(ctx.quest_comp.info.progress.read_phone_messages, pmid);
    } else if (std.mem.eql(u8, cond.Type, "ReceiveTelecom")) {
        const tid = getParamInt(lp, "TelecomId") orelse return false;
        return containsInt(ctx.quest_comp.info.progress.received_telecoms, tid);
    } else if (std.mem.eql(u8, cond.Type, "FinishBvbChallenge")) {
        const bid = getParamInt(lp, "BvbChallengeId") orelse return false;
        return containsInt(ctx.quest_comp.info.progress.completed_bvb, bid);
    } else if (std.mem.eql(u8, cond.Type, "CheckPlayerGender")) {
        const gender_str = getParamStr(lp, "Gender") orelse return true;
        if (std.mem.eql(u8, gender_str, "男")) return ctx.basic_comp.info.attributes.sex == 1;
        if (std.mem.eql(u8, gender_str, "女")) return ctx.basic_comp.info.attributes.sex == 0;
        return true;
    } else if (std.mem.eql(u8, cond.Type, "RoleBreach")) {
        const req = getParamInt(lp, "Breach") orelse 0;
        return evalRoleThreshold(ctx, lp, .breach, req);
    } else if (std.mem.eql(u8, cond.Type, "RoleLevel") or std.mem.eql(u8, cond.Type, "CheckRoleLevel")) {
        const req = getParamInt(lp, "Level") orelse 1;
        return evalRoleThreshold(ctx, lp, .level, req);
    } else if (std.mem.eql(u8, cond.Type, "RoleFavorLevel")) {
        const req = getParamInt(lp, "Level") orelse 1;
        return evalRoleThreshold(ctx, lp, .favor, req);
    } else if (std.mem.eql(u8, cond.Type, "RoleResonance")) {
        const req = getParamInt(lp, "GroupIndex") orelse 1;
        return evalRoleThreshold(ctx, lp, .resonance, req);
    } else if (std.mem.eql(u8, cond.Type, "CheckRoleOwned")) {
        const role_id = getParamInt(lp, "RoleId") orelse return false;
        const rc = ctx.role_comp orelse return false;
        return rc.role_map.contains(role_id);
    } else if (std.mem.eql(u8, cond.Type, "PassLordGymDifficulty")) {
        const gym_id = getParamInt(lp, "Id") orelse return false;
        const pc = ctx.progress_comp orelse return false;
        return pc.info.hasPassedLordGym(gym_id);
    } else if (std.mem.eql(u8, cond.Type, "DailyQuestGroupCompleteTimes")) {
        const gid = getParamInt(lp, "DQGroupId") orelse return false;
        const need = getParamInt(lp, "DQTimes") orelse cond.NeedNum;
        const pc = ctx.progress_comp orelse return false;
        return ProgressInfo.getIdCount(pc.info.counters.daily_quest_group_counts, gid) >= need;
    } else if (std.mem.eql(u8, cond.Type, "CheckDailyCounter")) {
        const ctype = getParamInt(lp, "CounterType") orelse return false;
        const extra = getParamInt(lp, "ExtraParam") orelse 0;
        const pc = ctx.progress_comp orelse return false;
        return pc.info.getDailyCounter(ctype, extra) >= cond.NeedNum;
    } else if (std.mem.eql(u8, cond.Type, "LevelPlayNodeStatus")) {
        const lpid = getParamInt(lp, "LevelPlayId") orelse return false;
        const want = getParamInt(lp, "State") orelse 3;
        const done = containsInt(ctx.quest_comp.info.completed_level_plays, lpid);
        return if (want == 3) done else !done;
    } else if (std.mem.eql(u8, cond.Type, "Record")) {
        const rid = getParamInt(lp, "RecordId") orelse return false;
        const pc = ctx.progress_comp orelse return false;
        return ProgressInfo.containsId(pc.info.counters.records, rid);
    } else if (std.mem.eql(u8, cond.Type, "FinishRogueInst")) {
        const iid = getParamInt(lp, "InstanceId") orelse return false;
        const pc = ctx.progress_comp orelse return false;
        return ProgressInfo.containsId(pc.info.challenge.finished_rogue_insts, iid);
    } else if (std.mem.eql(u8, cond.Type, "NewTowerTotalScore")) {
        const season = getParamInt(lp, "SeasonId") orelse 1;
        const diff = getParamInt(lp, "Difficulty") orelse -1;
        const pc = ctx.progress_comp orelse return false;
        return pc.info.getNewTowerScore(season, diff) >= cond.NeedNum;
    } else if (std.mem.eql(u8, cond.Type, "CheckGuideStatus")) {
        const gid = getParamInt(lp, "GuideGroupId") orelse return false;
        const target_status = getParamInt(lp, "Status") orelse 2;
        const finished = if (ctx.guide_comp) |gc| gc.info.hasFinished(gid) else false;
        const cur_status: i32 = if (finished) 2 else 0;
        const ope_opt: ?[]const u8 = if (cond.LimitParamsOpe) |op_val| switch (op_val) {
            .object => |m| getParamStr(m, "Status"),
            else => null,
        } else null;
        return compareInts(cur_status, target_status, ope_opt orelse "Eq");
    } else if (std.mem.eql(u8, cond.Type, "CheckLevelOp")) {
        const req = getParamInt(lp, "Level") orelse 1;
        const op = getParamStr(lp, "Op") orelse getOpeStr(cond.LimitParamsOpe, "Level") orelse "Ge";
        return compareInts(ctx.basic_comp.info.attributes.level, req, op);
    } else if (std.mem.eql(u8, cond.Type, "CheckExplorationLevel")) {
        const req = getParamInt(lp, "Id") orelse getParamInt(lp, "Level") orelse 1;
        return ctx.basic_comp.info.attributes.level >= req;
    } else if (std.mem.eql(u8, cond.Type, "CheckFuncOpen")) {
        const fid = getParamInt(lp, "Id") orelse getParamInt(lp, "FunctionId") orelse return false;
        return ctx.quest_comp.info.hasUnlockedSystem(fid);
    } else if (std.mem.eql(u8, cond.Type, "ReduceWorldLevel")) {
        return ctx.basic_comp.info.attributes.cur_world_level < ctx.basic_comp.info.attributes.origin_world_level;
    } else if (std.mem.eql(u8, cond.Type, "VarDefineString")) {
        const vd_name = getParamStr(lp, "VDName") orelse return false;
        const expected = getParamStr(lp, "VDValueString") orelse "";
        const actual = ctx.quest_comp.info.getVarStr(vd_name);
        if (actual.len > 0) return std.mem.eql(u8, actual, expected);
        if (ctx.progress_comp) |pc| {
            var kbuf: [128]u8 = undefined;
            if (std.fmt.bufPrint(&kbuf, "var:{s}", .{vd_name})) |k| {
                return std.mem.eql(u8, pc.info.getKeyStr(k), expected);
            } else |_| {}
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "CheckDungeonId")) {
        const did = getParamInt(lp, "Id") orelse return false;
        return containsInt(ctx.quest_comp.info.completed_dungeons, did);
    } else if (std.mem.eql(u8, cond.Type, "CheckInstPlayNodeState")) {
        const iid = getParamInt(lp, "InstanceId") orelse return false;
        return containsInt(ctx.quest_comp.info.completed_dungeons, iid);
    } else if (std.mem.eql(u8, cond.Type, "CheckLevelPlayNodeState")) {
        const lpid = getParamInt(lp, "LevelPlayId") orelse return false;
        const want = getParamInt(lp, "State") orelse 3;
        const done = containsInt(ctx.quest_comp.info.completed_level_plays, lpid);
        return if (want == 3) done else !done;
    } else if (std.mem.eql(u8, cond.Type, "LevelPlaysDoneCount") or std.mem.eql(u8, cond.Type, "ExploreLevelPlayComplete")) {
        const raw = getParamStr(lp, "LevelPlayId") orelse "";
        if (raw.len == 0) {
            return @as(i32, @intCast(ctx.quest_comp.info.completed_level_plays.len)) >= @max(cond.NeedNum, 1);
        }
        var count: i32 = 0;
        var it = std.mem.tokenizeScalar(u8, raw, '-');
        while (it.next()) |tok| {
            const lpid = std.fmt.parseInt(i32, tok, 10) catch continue;
            if (containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) count += 1;
        }
        return count >= @max(cond.NeedNum, 1);
    } else if (std.mem.eql(u8, cond.Type, "LevelPlayTypeCompleteWithMulti")) {
        return @as(i32, @intCast(ctx.quest_comp.info.completed_level_plays.len)) >= @max(cond.NeedNum, 1);
    } else if (std.mem.eql(u8, cond.Type, "PassLordGymDifficultyV2") or std.mem.eql(u8, cond.Type, "PassLordGymDifficultyWithLevel")) {
        const gid = getParamInt(lp, "Id") orelse 0;
        const pc = ctx.progress_comp orelse return false;
        if (gid > 0) return pc.info.hasPassedLordGym(gid);
        return pc.info.challenge.passed_lord_gyms.len > 0;
    } else if (std.mem.eql(u8, cond.Type, "FinishRogueInstDiffRole") or std.mem.eql(u8, cond.Type, "FinishRogueInstPopularEntrie")) {
        const pc = ctx.progress_comp orelse return false;
        return @as(i32, @intCast(pc.info.challenge.finished_rogue_insts.len)) >= @max(cond.NeedNum, 1);
    } else if (std.mem.eql(u8, cond.Type, "FirstGetRole")) {
        const rc = ctx.role_comp orelse return false;
        return @as(i32, @intCast(rc.role_map.count())) >= @max(cond.NeedNum, 1);
    } else if (std.mem.eql(u8, cond.Type, "RoleLevelAdd")) {
        const req = getParamInt(lp, "Level") orelse 1;
        return evalRoleThreshold(ctx, lp, .level, req);
    } else if (std.mem.eql(u8, cond.Type, "FirstRoleBreakthrough")) {
        const req = getParamInt(lp, "Level") orelse 1;
        return evalRoleThreshold(ctx, lp, .breach, req);
    } else if (std.mem.eql(u8, cond.Type, "SkillLevelAdd")) {
        const req = getParamInt(lp, "Level") orelse 1;
        const rc = ctx.role_comp orelse return false;
        var it = rc.role_map.iterator();
        while (it.next()) |kv| {
            for (kv.value_ptr.skills) |sk| {
                if (sk.@"1" >= req) return true;
            }
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "RoleSkillAllNodeMax")) {
        const rc = ctx.role_comp orelse return false;
        var it = rc.role_map.iterator();
        while (it.next()) |kv| {
            if (kv.value_ptr.skill_node_state.len > 0) {
                var all_active = true;
                for (kv.value_ptr.skill_node_state) |st| {
                    if (!st.active) {
                        all_active = false;
                        break;
                    }
                }
                if (all_active) return true;
            }
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "WeaponLevelAdd") or std.mem.eql(u8, cond.Type, "WeaponBreakthrough")) {
        const wc = ctx.weapon_comp orelse return false;
        const is_breach = std.mem.eql(u8, cond.Type, "WeaponBreakthrough");
        const req = (if (is_breach) getParamInt(lp, "BreachLevel") else getParamInt(lp, "Level")) orelse 1;
        var it = wc.weapon_map.iterator();
        while (it.next()) |kv| {
            const val = if (is_breach) kv.value_ptr.breach else kv.value_ptr.level;
            if (val >= req) return true;
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "CapturePhantom") or std.mem.eql(u8, cond.Type, "NewCapturePhantom") or std.mem.eql(u8, cond.Type, "PhantomGetCounter") or std.mem.eql(u8, cond.Type, "StrengthenPhantomMax") or std.mem.eql(u8, cond.Type, "ActivatePhantomFettles") or std.mem.eql(u8, cond.Type, "PhantomIdentify")) {
        const need = @max(cond.NeedNum, getParamInt(lp, "Count") orelse 1);
        const total: i32 = @intCast(ctx.echo_comp.echo_map.count());
        return total >= need;
    } else if (std.mem.eql(u8, cond.Type, "EntityStateByBpTypeOnline")) {
        const eid = getParamInt(lp, "EntityConfigId") orelse return false;
        const expected_tag = getParamInt(lp, "Tag") orelse return false;
        const state_str = ctx.quest_comp.info.getEntityState(eid) orelse return false;
        const a = ctx.assets orelse return false;
        const cur_tag = gameplay_tags.idFromName(&a.tables.gameplay_tag_remap, state_str) catch return false;
        return cur_tag == expected_tag;
    } else if (std.mem.eql(u8, cond.Type, "CostItemBagType")) {
        const pc = ctx.progress_comp orelse return false;
        return pc.info.getKeyInt("cond_cnt:CostItemBagType") >= cond.NeedNum;
    } else if (std.mem.eql(u8, cond.Type, "CheckActivityOpen")) {
        const aid = getParamInt(lp, "ActivityId") orelse return false;
        const want_open = (getParamInt(lp, "IsOpen") orelse 1) != 0;
        return isActivityOpen(ctx, aid) == want_open;
    } else if (std.mem.eql(u8, cond.Type, "SomeQuestComplete") or std.mem.eql(u8, cond.Type, "SomeTypeQuestComplete")) {
        return @as(i32, @intCast(ctx.quest_comp.info.finished_quests.len)) >= @max(cond.NeedNum, 1);
    } else if (std.mem.eql(u8, cond.Type, "RecallQuestState")) {
        const qid = getParamInt(lp, "RecallQuestId") orelse return false;
        const st = getParamInt(lp, "State") orelse 3;
        return evalQuestState(ctx, qid, st);
    } else if (std.mem.eql(u8, cond.Type, "SceneGamePlayStatus")) {
        const qid = getParamInt(lp, "NewQuestId") orelse return false;
        const nid = getParamInt(lp, "QuestNodeId") orelse return false;
        const st = getParamInt(lp, "State") orelse 3;
        return evalQuestNodeStatus(ctx, qid, nid, st);
    } else if (std.mem.eql(u8, cond.Type, "CheckExploreProgress") or std.mem.eql(u8, cond.Type, "CheckMulExploreProgress")) {
        if (ctx.progress_comp) |pc| {
            return pc.info.getKeyInt("cond_cnt:CheckExploreProgress") >= cond.NeedNum or
                pc.info.countClaimsPrefix("explore_") >= cond.NeedNum;
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "CheckDungeonFinished") or std.mem.eql(u8, cond.Type, "NewCheckInstanceState")) {
        if (getParamInt(lp, "InstanceId")) |inst_id| {
            if (inst_id > 0) {
                const want_finish = (getParamInt(lp, "Finish") orelse 1) != 0;
                const finished = containsInt(ctx.quest_comp.info.completed_dungeons, inst_id);
                return if (want_finish) finished else !finished;
            }
        }
        if (ctx.progress_comp) |pc| {
            return pc.info.getKeyInt("cond_cnt:CheckDungeonFinished") >= cond.NeedNum;
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "CheckAccountSettingOpen")) {
        if (ctx.progress_comp) |pc| {
            const sid = getParamInt(lp, "Id") orelse 0;
            var buf: [64]u8 = undefined;
            if (sid > 0) {
                if (std.fmt.bufPrint(&buf, "acct_setting:{d}", .{sid})) |k| {
                    if (pc.info.getKeyInt(k) != 0) return true;
                } else |_| {}
            }
            return pc.info.getKeyInt("cond_cnt:CheckAccountSettingOpen") >= @max(cond.NeedNum, 1);
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "BabelTowerLevelFinishStar")) {
        if (ctx.progress_comp) |pc| {
            const lid = getParamInt(lp, "Id") orelse 0;
            var buf: [64]u8 = undefined;
            if (lid > 0) {
                if (std.fmt.bufPrint(&buf, "babel:star:{d}", .{lid})) |k| {
                    if (pc.info.getKeyInt(k) >= cond.NeedNum) return true;
                } else |_| {}
            }
            return pc.info.getKeyInt("babel:star") >= cond.NeedNum;
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "RoleTowerPassStage")) {
        if (ctx.progress_comp) |pc| {
            const sid = getParamInt(lp, "Id") orelse 0;
            var buf: [64]u8 = undefined;
            if (sid > 0) {
                if (std.fmt.bufPrint(&buf, "role_tower:stage:{d}", .{sid})) |k| {
                    if (pc.info.hasClaim(k)) return true;
                } else |_| {}
            }
            return pc.info.getKeyInt("role_tower:stage") >= @max(sid, cond.NeedNum);
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "AbyssLittleRoleLevel")) {
        if (ctx.progress_comp) |pc| {
            const req = @max(getParamInt(lp, "Level") orelse 1, cond.NeedNum);
            return pc.info.getKeyInt("abyss:lrole_max_lv") >= req;
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "BossPilingLevelBossHpNum")) {
        if (ctx.progress_comp) |pc| {
            const lid = getParamInt(lp, "Id") orelse 0;
            var buf: [64]u8 = undefined;
            if (lid > 0) {
                if (std.fmt.bufPrint(&buf, "bp:hp:{d}", .{lid})) |k| {
                    if (pc.info.getKeyInt(k) >= cond.NeedNum) return true;
                } else |_| {}
            }
            return pc.info.getKeyInt("bp:max_hp") >= cond.NeedNum;
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "TeamParkOurLevelCheck")) {
        if (ctx.progress_comp) |pc| {
            const lid = getParamInt(lp, "Id") orelse 0;
            var buf: [64]u8 = undefined;
            if (lid > 0) {
                if (std.fmt.bufPrint(&buf, "parkour:score:{d}", .{lid})) |k| {
                    if (pc.info.getKeyInt(k) >= cond.NeedNum) return true;
                } else |_| {}
            }
            return pc.info.getKeyInt("parkour:best") >= cond.NeedNum;
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "CiacconaChapterResult")) {
        if (ctx.progress_comp) |pc| {
            const res_id = getParamInt(lp, "ChapterResult") orelse 0;
            var buf: [64]u8 = undefined;
            if (res_id > 0) {
                if (std.fmt.bufPrint(&buf, "ciaccona:res:{d}", .{res_id})) |k| {
                    if (pc.info.hasClaim(k)) return true;
                } else |_| {}
                if (pc.info.getKeyInt("ciaccona:res") == res_id) return true;
            }
            return pc.info.getKeyInt("cond_cnt:CiacconaChapterResult") >= @max(cond.NeedNum, 1);
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "KurotatoFinishLevel")) {
        const lv_id = getParamInt(lp, "Level") orelse 0;
        if (lv_id > 0 and containsInt(ctx.quest_comp.info.completed_dungeons, lv_id)) return true;
        if (ctx.progress_comp) |pc| {
            var buf: [64]u8 = undefined;
            if (lv_id > 0) {
                if (std.fmt.bufPrint(&buf, "kurotato:done:{d}", .{lv_id})) |k| {
                    if (pc.info.hasClaim(k)) return true;
                } else |_| {}
            }
            return pc.info.getKeyInt("kurotato:finish_cnt") >= @max(cond.NeedNum, 1);
        }
        return false;
    } else if (std.mem.eql(u8, cond.Type, "KurotatoUnlockInfo")) {
        if (ctx.progress_comp) |pc| {
            const info_id = getParamInt(lp, "Id") orelse 0;
            if (cond.NeedNum <= 1 and info_id > 0) {
                var buf: [64]u8 = undefined;
                if (std.fmt.bufPrint(&buf, "kurotato:info:{d}", .{info_id})) |k| {
                    if (pc.info.hasClaim(k)) return true;
                } else |_| {}
            }
            return @max(pc.info.getKeyInt("kurotato:unlock_cnt"), pc.info.countClaimsPrefix("kurotato:info:")) >= @max(cond.NeedNum, 1);
        }
        return false;
    } else if (server_cond_specs.get(cond.Type)) |spec| {
        return evalServerSpecCondition(ctx, cond, lp, spec);
    }

    return false;
}

const ServerCondSpec = struct {
    prog_key: []const u8,
    claim_prefix: []const u8 = "",
    param_id_key: []const u8 = "",
};

fn getOpeStr(ope_opt: ?std.json.Value, field: []const u8) ?[]const u8 {
    const ov = ope_opt orelse return null;
    if (ov != .object) return null;
    return getParamStr(ov.object, field);
}

fn evalServerSpecCondition(
    ctx: EvalContext,
    cond: anytype,
    lp: std.json.ObjectMap,
    spec: ServerCondSpec,
) bool {
    const pc = ctx.progress_comp orelse return false;
    const target: i32 = @max(
        cond.NeedNum,
        getParamInt(lp, "Count") orelse getParamInt(lp, "Num") orelse getParamInt(lp, "Number") orelse getParamInt(lp, "Level") orelse 1,
    );

    if (spec.param_id_key.len > 0) {
        if (getParamInt(lp, spec.param_id_key)) |pid| {
            if (pid > 0) {
                var kbuf: [128]u8 = undefined;
                if (spec.claim_prefix.len > 0) {
                    if (std.fmt.bufPrint(&kbuf, "{s}{d}", .{ spec.claim_prefix, pid })) |k| {
                        if (pc.info.hasClaim(k)) return true;
                    } else |_| {}
                }
                if (std.fmt.bufPrint(&kbuf, "{s}:{d}", .{ spec.prog_key, pid })) |k| {
                    const v: i32 = @intCast(std.math.clamp(pc.info.getKeyInt(k), 0, std.math.maxInt(i32)));
                    if (v >= target) return true;
                } else |_| {}
                return false;
            }
        }
    }

    if (spec.claim_prefix.len > 0) {
        const claims = pc.info.countClaimsPrefix(spec.claim_prefix);
        if (claims >= target) return true;
    }

    var gbuf: [128]u8 = undefined;
    const direct_val: i32 = @intCast(std.math.clamp(pc.info.getKeyInt(spec.prog_key), 0, std.math.maxInt(i32)));
    const cond_val: i32 = if (std.fmt.bufPrint(&gbuf, "cond_cnt:{s}", .{cond.Type})) |ck|
        @intCast(std.math.clamp(pc.info.getKeyInt(ck), 0, std.math.maxInt(i32)))
    else |_|
        0;
    return @max(direct_val, cond_val) >= target;
}

const server_cond_specs = std.StaticStringMap(ServerCondSpec).initComptime(&[_]struct { []const u8, ServerCondSpec }{
    .{ "AbyssPass", .{ .prog_key = "abyss:settle_inst", .claim_prefix = "abyss:pass:", .param_id_key = "Id" } },
    .{ "AddFriends", .{ .prog_key = "social:friends_added" } },
    .{ "AtmosphereLevelUpdate", .{ .prog_key = "diy_atm", .claim_prefix = "diy_atm:" } },
    .{ "BabelTowerLevelFinish", .{ .prog_key = "babel:finish", .claim_prefix = "babel:done:", .param_id_key = "Id" } },
    .{ "BabelTowerLevelStar", .{ .prog_key = "babel:star", .claim_prefix = "babel_rw:" } },
    .{ "BossLinkInstHistoryMaxScore", .{ .prog_key = "rogue:boss_link_score", .param_id_key = "InstanceId" } },
    .{ "BossRushScore", .{ .prog_key = "bossrush:score", .claim_prefix = "bossrush:" } },
    .{ "BrochureCollectNum", .{ .prog_key = "brochure:cnt", .claim_prefix = "brochure:" } },
    .{ "CdkGiftCdkReceived", .{ .prog_key = "cdk:received" } },
    .{ "CheckDailyLevelPlayAndInstCounter", .{ .prog_key = "daily:lp_inst_cnt" } },
    .{ "CheckGachaTenTimesItemCount", .{ .prog_key = "gacha:ten_times" } },
    .{ "CheckGachaTypeConsecutive", .{ .prog_key = "gacha:consecutive" } },
    .{ "CheckGachaTypeItemQuality", .{ .prog_key = "gacha:quality_cnt" } },
    .{ "CheckMoonBuilding", .{ .prog_key = "trackmoon:bld", .claim_prefix = "tm_ent:" } },
    .{ "CheckMoonBuildingUp", .{ .prog_key = "trackmoon:bld_lv", .claim_prefix = "tm_ent:" } },
    .{ "CheckMoonEntrust", .{ .prog_key = "trackmoon:entrust", .claim_prefix = "tm_ent:" } },
    .{ "CheckMoonRoleAttribute", .{ .prog_key = "trackmoon:role_attr" } },
    .{ "CheckMoonRoleLikeLevel", .{ .prog_key = "trackmoon:role_like" } },
    .{ "CheckOnTreasureBoxOpen", .{ .prog_key = "explore:treasure_opened" } },
    .{ "CheckPeopleLikeCount", .{ .prog_key = "bld:like" } },
    .{ "CheckPopularityLevel", .{ .prog_key = "bld:pop_lv" } },
    .{ "CheckReviveByBuff", .{ .prog_key = "combat:revive_buff" } },
    .{ "CheckShop", .{ .prog_key = "shop:buy", .param_id_key = "ShopItemId" } },
    .{ "CheckTetrisFinish", .{ .prog_key = "tetris:best", .param_id_key = "Id" } },
    .{ "CiacconaActivityResult", .{ .prog_key = "ciaccona:res" } },
    .{ "CompareActorVar", .{ .prog_key = "actor_var:done" } },
    .{ "CookingLevelUp", .{ .prog_key = "cook:level" } },
    .{ "CorniceMeetingFinish", .{ .prog_key = "cornice:finish", .claim_prefix = "cornice_rw:" } },
    .{ "CreateAdvice", .{ .prog_key = "advice:created" } },
    .{ "CrossSceneSyncKillMonster", .{ .prog_key = "simple_combat:kills" } },
    .{ "DailyAdventureCheck", .{ .prog_key = "dadv:check", .claim_prefix = "dadv_" } },
    .{ "DangoBrawlComposeQuality", .{ .prog_key = "db_uid" } },
    .{ "DangoBrawlPassAllBaseFilled", .{ .prog_key = "db_fight_cnt" } },
    .{ "DangoBrawlPassChallenge", .{ .prog_key = "db_fight_cnt", .claim_prefix = "db_task:", .param_id_key = "Id" } },
    .{ "DangoBrawlPassItemCountLimit", .{ .prog_key = "db_uid" } },
    .{ "DangoBrawlPassWithQualityItems", .{ .prog_key = "db_uid" } },
    .{ "DangoBrawlRegularWinCount", .{ .prog_key = "db_round" } },
    .{ "DangoMonopolyBoardCompleted", .{ .prog_key = "dmono_board", .claim_prefix = "dmono_board_rw:", .param_id_key = "BoardId" } },
    .{ "Detection", .{ .prog_key = "explore:detection" } },
    .{ "DetectionSkillUseCount", .{ .prog_key = "explore:detection_skill" } },
    .{ "DrawCardCompleteTimes", .{ .prog_key = "gacha:total_pulls" } },
    .{ "DropCatchActivityScore", .{ .prog_key = "dc:score", .param_id_key = "Id" } },
    .{ "EncircleChallengeComplete", .{ .prog_key = "encircle:done", .claim_prefix = "encircle:done:", .param_id_key = "Id" } },
    .{ "EndlessTetrisScore", .{ .prog_key = "tetris:best" } },
    .{ "EntityDie", .{ .prog_key = "simple_combat:kills" } },
    .{ "EntityDieByTemplate", .{ .prog_key = "simple_combat:kills" } },
    .{ "EntityDieByType", .{ .prog_key = "simple_combat:kills" } },
    .{ "EntityInteractById", .{ .prog_key = "explore:interact" } },
    .{ "EntityInteractByType", .{ .prog_key = "explore:interact" } },
    .{ "ExploreProgressCompleted", .{ .prog_key = "explore:prog_done", .claim_prefix = "explore_prog:" } },
    .{ "ExploreProgressCounter", .{ .prog_key = "explore:prog_cnt" } },
    .{ "ExploreProgressStepRewardTime", .{ .prog_key = "explore:step_rw", .claim_prefix = "explore_prog:" } },
    .{ "FarmGoldLevelScore", .{ .prog_key = "fg:score", .claim_prefix = "fg_lp:", .param_id_key = "Id" } },
    .{ "FinishInst", .{ .prog_key = "inst:finish_cnt" } },
    .{ "FinishFlipper", .{ .prog_key = "flipper:fin" } },
    .{ "FinishKuroSimpleCombatKillMonster", .{ .prog_key = "simple_combat:kills" } },
    .{ "FinishKurotato", .{ .prog_key = "kurotato:wave" } },
    .{ "FinishRoleTower", .{ .prog_key = "role_tower:stage" } },
    .{ "FinishRollBlock", .{ .prog_key = "rb_blk:fin" } },
    .{ "FinishSurvivorsRouge", .{ .prog_key = "surv:wave" } },
    .{ "FinishTetris", .{ .prog_key = "tetris:best" } },
    .{ "FinishTrapDefense", .{ .prog_key = "trap:pass_wave" } },
    .{ "NewHandInItems", .{ .prog_key = "handin_pm:cnt", .claim_prefix = "handin_pm:", .param_id_key = "EntityId" } },
    .{ "ParallaxAlign", .{ .prog_key = "parallax:done" } },
    .{ "PlaybackShowTimeline", .{ .prog_key = "timeline:done" } },
    .{ "TakePicturesWithTimeScale", .{ .prog_key = "explore:photo_cnt" } },
    .{ "WalkingPattern", .{ .prog_key = "walk_pat:done" } },
    .{ "FishingEntrust", .{ .prog_key = "fish:entrust_done", .claim_prefix = "fish_ent:" } },
    .{ "FishingFish", .{ .prog_key = "fish:catch_cnt" } },
    .{ "FishingIllustrated", .{ .prog_key = "fish:ill_cnt", .claim_prefix = "fish_ill:" } },
    .{ "FishingQuickSell", .{ .prog_key = "fish:sell_cnt" } },
    .{ "FishingReputationLevel", .{ .prog_key = "fish:rep_lv" } },
    .{ "FishingSkillUse", .{ .prog_key = "fish:skill_use", .param_id_key = "Id" } },
    .{ "FishingTargetEntrustCount", .{ .prog_key = "fish:target_entrust", .param_id_key = "Id" } },
    .{ "FishingTechLevel", .{ .prog_key = "fish:tech_lv", .param_id_key = "Id" } },
    .{ "FlagChallengePassLevel", .{ .prog_key = "flag:pass_lv", .claim_prefix = "flag_rw:", .param_id_key = "Id" } },
    .{ "FlagChallengePerLevel", .{ .prog_key = "flag:per_lv", .claim_prefix = "flag_rw:" } },
    .{ "FloroRanchCounter", .{ .prog_key = "fr:day", .claim_prefix = "fr_task:" } },
    .{ "FoodProcess", .{ .prog_key = "cook:process_cnt" } },
    .{ "GachaRuleGroupFinishCount", .{ .prog_key = "gacha:rule_finish" } },
    .{ "GiantTotalStableStage", .{ .prog_key = "giant:stage", .claim_prefix = "giant_sv:" } },
    .{ "GivebackScoreReach", .{ .prog_key = "giveback:score" } },
    .{ "GolemCrackFinish", .{ .prog_key = "golem:finish", .param_id_key = "Id" } },
    .{ "HarvestScore", .{ .prog_key = "harv:score", .claim_prefix = "harv_lv:" } },
    .{ "HonamiStoryActiveTalent", .{ .prog_key = "honami:talent_cnt", .claim_prefix = "honami:talent:", .param_id_key = "Id" } },
    .{ "HonamiStoryCheckTowerPass", .{ .prog_key = "honami:tower_floor" } },
    .{ "HonamiStoryItemCollection", .{ .prog_key = "honami:col_cnt", .claim_prefix = "honami_col:" } },
    .{ "HonamiStoryItemSold", .{ .prog_key = "honami:sold_val" } },
    .{ "HonamiStoryMascotUnlock", .{ .prog_key = "honami:mascot_cnt", .claim_prefix = "honami_mascot:" } },
    .{ "HonamiStorySafeLeave", .{ .prog_key = "honami:safe_leave" } },
    .{ "HonamiStorySettleValue", .{ .prog_key = "honami:settle_val" } },
    .{ "HonamiStoryTotalRevenue", .{ .prog_key = "honami:total_rev" } },
    .{ "HonamiStoryWareHouseNoFull", .{ .prog_key = "honami:wh_ok" } },
    .{ "InfrLevelUp", .{ .prog_key = "infr:lv", .claim_prefix = "infr_lv:" } },
    .{ "InfrRoadComplete", .{ .prog_key = "infr:road", .claim_prefix = "infr_road:", .param_id_key = "InfrRoadId" } },
    .{ "InfrV2LevelUp", .{ .prog_key = "infr2:lv", .claim_prefix = "infr2_lv:" } },
    .{ "InstACE", .{ .prog_key = "inst:ace_cnt" } },
    .{ "KillWithBossRushBuff", .{ .prog_key = "bossrush:buff_kill", .param_id_key = "Id" } },
    .{ "KurotatoRoleFinishWave", .{ .prog_key = "kurotato:wave" } },
    .{ "LongShanRwdCount", .{ .prog_key = "ls:rwd_cnt", .claim_prefix = "longshan_" } },
    .{ "MapTravelLevel", .{ .prog_key = "map_travel:lv" } },
    .{ "MatchThreePass", .{ .prog_key = "m3:pass", .param_id_key = "Id" } },
    .{ "MotorFightBestScore", .{ .prog_key = "motor_fight:best_score", .param_id_key = "Id" } },
    .{ "MotorFightLevelKillMonster", .{ .prog_key = "motor_fight:kills", .param_id_key = "Id" } },
    .{ "MotorFightPassLevel", .{ .prog_key = "motor_fight:pass", .param_id_key = "Id" } },
    .{ "MotorGainLiveness", .{ .prog_key = "motor:liveness" } },
    .{ "MotorGainTp", .{ .prog_key = "motor:tp" } },
    .{ "MotorLevelUp", .{ .prog_key = "motor:lv" } },
    .{ "MowTowerLevelScore", .{ .prog_key = "mow:score", .claim_prefix = "mow_rw:", .param_id_key = "Id" } },
    .{ "NewTowerAllTeamScore", .{ .prog_key = "new_tower:all_score" } },
    .{ "NewTowerCycleHistoryScore", .{ .prog_key = "new_tower:cycle_score" } },
    .{ "NewTowerDamage", .{ .prog_key = "new_tower:damage" } },
    .{ "NewTowerDamageInSeason", .{ .prog_key = "new_tower:season_damage" } },
    .{ "NewTowerGetScoreNoFiveStarRole", .{ .prog_key = "new_tower:no5_score" } },
    .{ "NewTowerGetScoreNoInjury", .{ .prog_key = "new_tower:noinj_score" } },
    .{ "NewTowerGetScoreWithRole", .{ .prog_key = "new_tower:role_score" } },
    .{ "NewTowerKillBoss", .{ .prog_key = "new_tower:kill_boss" } },
    .{ "NewTowerKillBossInSeason", .{ .prog_key = "new_tower:season_boss" } },
    .{ "NewTowerKillWave", .{ .prog_key = "new_tower:kill_wave" } },
    .{ "NewTowerKillWaveInSeason", .{ .prog_key = "new_tower:season_wave" } },
    .{ "NewTowerLimitCondPass", .{ .prog_key = "new_tower:limit_pass" } },
    .{ "NewTowerLimitCondPassWave", .{ .prog_key = "new_tower:limit_wave" } },
    .{ "NewTowerRecord", .{ .prog_key = "new_tower:record" } },
    .{ "NewTowerSeasonAwardFinish", .{ .prog_key = "new_tower:season_rw", .claim_prefix = "nt_rw:" } },
    .{ "NewTowerTeamChallengingFinish", .{ .prog_key = "new_tower:team_finish", .claim_prefix = "nt_role:", .param_id_key = "RoleId" } },
    .{ "OnDeathCalculate", .{ .prog_key = "combat:death_calc" } },
    .{ "OnEnterOrExitBattle", .{ .prog_key = "combat:battle_toggle" } },
    .{ "OnHonamiStoryLifeSupportChange", .{ .prog_key = "honami:ls_lv" } },
    .{ "OnPickUpHonamiStoryItem", .{ .prog_key = "honami:pickup" } },
    .{ "OnPlayerUseSkill", .{ .prog_key = "combat:skill_use" } },
    .{ "OnSceneItemDurabilityEmpty", .{ .prog_key = "explore:durability_empty" } },
    .{ "OnSkillEnable", .{ .prog_key = "combat:skill_enable" } },
    .{ "OnlineKillBoss", .{ .prog_key = "coop:kill_boss" } },
    .{ "OnlineMotorJoin", .{ .prog_key = "motor:online_join", .claim_prefix = "online_motor:" } },
    .{ "OnlineWithFriend", .{ .prog_key = "coop:with_friend" } },
    .{ "OpenTreasureBox", .{ .prog_key = "explore:chest_cnt" } },
    .{ "OpenTreasureBoxNew", .{ .prog_key = "explore:chest_cnt" } },
    .{ "ParkourActivityComplete", .{ .prog_key = "parkour:done", .claim_prefix = "parkour:" } },
    .{ "PassSlashAndTowerScore", .{ .prog_key = "slash_tower:score", .claim_prefix = "slash_rw:" } },
    .{ "PassTowerDefence", .{ .prog_key = "td:pass", .claim_prefix = "td_inst:", .param_id_key = "Id" } },
    .{ "PassTowerStarCount", .{ .prog_key = "tower:total_stars" } },
    .{ "PhantomBattleCardLookoutUp", .{ .prog_key = "pb_card_cnt", .claim_prefix = "pb_card:" } },
    .{ "PhantomBattleCollectBadgeNum", .{ .prog_key = "pb_badge_cnt", .claim_prefix = "pb_badge:" } },
    .{ "PhantomBattleCollectCardNum", .{ .prog_key = "pb_card_cnt", .claim_prefix = "pb_card:" } },
    .{ "PhantomBattleCompleteChallenge", .{ .prog_key = "pb:challenge", .claim_prefix = "pb:challenge:", .param_id_key = "ChallengeId" } },
    .{ "PhantomBattleIntegrationMaxNum", .{ .prog_key = "pb:score" } },
    .{ "PhantomBattleLv", .{ .prog_key = "pb:lv" } },
    .{ "PhantomBattleSkillConditionGroupCardTypeCount", .{ .prog_key = "pb:cond_type_cnt" } },
    .{ "PhantomBattleSkillConditionGroupCostCount", .{ .prog_key = "pb:cond_cost_cnt" } },
    .{ "PhantomBattleSkillConditionGroupElementCount", .{ .prog_key = "pb:cond_elem_cnt" } },
    .{ "Photograph", .{ .prog_key = "explore:photo_cnt" } },
    .{ "PinballCowScore", .{ .prog_key = "pinball:score" } },
    .{ "PinballInGameAction", .{ .prog_key = "pinball:action_cnt" } },
    .{ "PinballLevelPass", .{ .prog_key = "pinball:pass", .claim_prefix = "pinball_task:" } },
    .{ "PinballLevelStar", .{ .prog_key = "pinball:star" } },
    .{ "PinballLimitTaskFinish", .{ .prog_key = "pinball:task_cnt", .claim_prefix = "pinball_task:" } },
    .{ "PinballRoleLevel", .{ .prog_key = "pinball:role_lv" } },
    .{ "PinballWeaponCal", .{ .prog_key = "pinball:wp_cal" } },
    .{ "RacingBetsTotalOddsFund", .{ .prog_key = "rb:odds_fund" } },
    .{ "RacingBetsTotalOddsTimes", .{ .prog_key = "rb:odds_times" } },
    .{ "RealmBetweenLevel", .{ .prog_key = "realm:lv" } },
    .{ "RecallGetItem", .{ .prog_key = "recall:item_cnt" } },
    .{ "RhythmDifficultyRankCount", .{ .prog_key = "rhythm:rank_cnt" } },
    .{ "RhythmRankReach", .{ .prog_key = "rhythm:best", .param_id_key = "Id" } },
    .{ "RhythmRoleComplete", .{ .prog_key = "rhythm:role_done" } },
    .{ "RideSharingInvite", .{ .prog_key = "vehicle:share_invite" } },
    .{ "RiskHarvestScore", .{ .prog_key = "rh:score", .claim_prefix = "rh_score:" } },
    .{ "RoadBookLevel", .{ .prog_key = "roadbook:lv" } },
    .{ "RogueBlackFlowerReceive", .{ .prog_key = "rogue:bf_cnt", .claim_prefix = "rogue:bf:" } },
    .{ "RogueFinishAllTimeLimitTask", .{ .prog_key = "rogue:limit_task_cnt", .claim_prefix = "rogue:season_rw:" } },
    .{ "RogueLinkCount", .{ .prog_key = "rogue:link_cnt" } },
    .{ "RoguePassRoom", .{ .prog_key = "rogue:layer" } },
    .{ "RogueResCostMood", .{ .prog_key = "rogue_res:mood_cost" } },
    .{ "RogueResFinishDiffEnd", .{ .prog_key = "rogue_res:end_cnt", .claim_prefix = "rogue_res:end:" } },
    .{ "RogueResFinishEvent", .{ .prog_key = "rogue_res:evt", .claim_prefix = "rogue_res:evt:", .param_id_key = "Id" } },
    .{ "RogueResFinishEventStep", .{ .prog_key = "rogue_res:evt_step", .claim_prefix = "rogue_res:evt_step:", .param_id_key = "Id" } },
    .{ "RogueResPassInst", .{ .prog_key = "rogue_res:pass", .param_id_key = "Id" } },
    .{ "RogueResPassInstWithBond", .{ .prog_key = "rogue_res:bond_pass" } },
    .{ "RogueResRoleRecruit", .{ .prog_key = "rogue_res:recruit" } },
    .{ "RogueResTriggerLinkWithBond", .{ .prog_key = "rogue_res:bond_link" } },
    .{ "RogueRoleConcerto", .{ .prog_key = "rogue:concerto" } },
    .{ "RogueTalentTreeMax", .{ .prog_key = "rogue:talent_max" } },
    .{ "RogueTowerTrialFinishLevel", .{ .prog_key = "rogue_trial:inst", .param_id_key = "Id" } },
    .{ "RogueUseDiffPhantom", .{ .prog_key = "rogue:diff_phantom" } },
    .{ "RogueUseDiffRolePassBossLink", .{ .prog_key = "rogue:diff_role_boss" } },
    .{ "RoleTowerUseDiffRole", .{ .prog_key = "role_tower:diff_role" } },
    .{ "RougeWeeklyScore", .{ .prog_key = "rogue_weekly:score" } },
    .{ "RoverRogueBlessingGet", .{ .prog_key = "rover_rogue:bless_cnt", .claim_prefix = "rover_rogue:gain:" } },
    .{ "RoverRogueBlessingTypeGet", .{ .prog_key = "rover_rogue:bless_type_cnt" } },
    .{ "RoverRogueBossPass", .{ .prog_key = "rover_rogue:boss_pass" } },
    .{ "RoverRogueCondition", .{ .prog_key = "rover_rogue:cond", .param_id_key = "InsId" } },
    .{ "RoverRogueElitePass", .{ .prog_key = "rover_rogue:elite_pass" } },
    .{ "RoverRogueForgeGet", .{ .prog_key = "rover_rogue:forge_cnt" } },
    .{ "RoverRogueItemGet", .{ .prog_key = "rover_rogue:item_cnt" } },
    .{ "RoverRogueLootGet", .{ .prog_key = "rover_rogue:loot_cnt", .claim_prefix = "rover_rogue:loot:" } },
    .{ "RoverRogueReviveTimes", .{ .prog_key = "rover_rogue:revive_cnt" } },
    .{ "RoverRogueStageClearCheckHp", .{ .prog_key = "rover_rogue:hp_clear" } },
    .{ "RoverRogueTalentUnlock", .{ .prog_key = "rover_rogue:talent", .claim_prefix = "rover_rogue:talent:", .param_id_key = "TalentId" } },
    .{ "ScratchCardAllReward", .{ .prog_key = "scratch:all_rw" } },
    .{ "SlashTowerEndlessMaxScoreCount", .{ .prog_key = "slash_tower:endless_cnt" } },
    .{ "SlashTowerFullSeasonReward", .{ .prog_key = "slash_tower:full_rw", .claim_prefix = "slash_rw:" } },
    .{ "SomeMonsterKill", .{ .prog_key = "simple_combat:kills" } },
    .{ "SpringSignInviteNum", .{ .prog_key = "ssign:draw_idx" } },
    .{ "SurvivorsLevelFinish", .{ .prog_key = "surv:wave", .param_id_key = "Level" } },
    .{ "SurvivorsUnlockItem", .{ .prog_key = "surv:item_cnt" } },
    .{ "SurvivorsUnlockWeapon", .{ .prog_key = "surv:wp_cnt" } },
    .{ "SurvivorsWeaponEvolve", .{ .prog_key = "surv:wp_evo" } },
    .{ "Synthesis", .{ .prog_key = "craft:synthesis_cnt" } },
    .{ "SynthesisLevelUp", .{ .prog_key = "craft:synthesis_lv" } },
    .{ "SynthesisOrCookingCounter", .{ .prog_key = "craft:total_cnt" } },
    .{ "TakeChallenge", .{ .prog_key = "challenge:take", .param_id_key = "Id" } },
    .{ "TakeLordGymChallenge", .{ .prog_key = "challenge:lord_gym" } },
    .{ "TakeSceneItem", .{ .prog_key = "explore:pickup_cnt" } },
    .{ "TotalFinishInstTimesWithMulti", .{ .prog_key = "inst:multi_cnt" } },
    .{ "TowerDefenceLevelScore", .{ .prog_key = "td:level_score", .param_id_key = "Id" } },
    .{ "TowerDefenceScore", .{ .prog_key = "td:total_score", .claim_prefix = "td_score:" } },
    .{ "TowerLimitedPass", .{ .prog_key = "tower:limited_pass" } },
    .{ "TowerNoHarmPass", .{ .prog_key = "tower:no_harm_pass" } },
    .{ "TowerPassMaxDifficultCount", .{ .prog_key = "tower:max_diff_cnt" } },
    .{ "TowerPerfectPass", .{ .prog_key = "tower:perfect_cnt" } },
    .{ "TrapDefenseAuxiliaryUnlockCount", .{ .prog_key = "trap:aux_cnt" } },
    .{ "TrapDefenseBuildingUnlock", .{ .prog_key = "trap:bld_unlock" } },
    .{ "TrapDefenseBuildingUnlockCount", .{ .prog_key = "trap:bld_cnt" } },
    .{ "TrapDefenseChallengeStar", .{ .prog_key = "trap:star", .param_id_key = "ChallengeId" } },
    .{ "TrapDefenseConsecutiveKill", .{ .prog_key = "trap:streak_kill" } },
    .{ "TrapDefenseEnvironmentalKill", .{ .prog_key = "trap:env_kill" } },
    .{ "TrapDefenseGainWaveBuff", .{ .prog_key = "trap:wave_buff" } },
    .{ "TrapDefensePassFullStar", .{ .prog_key = "trap:full_star", .param_id_key = "ChallengeId" } },
    .{ "TrapDefensePassWave", .{ .prog_key = "trap:pass_wave", .param_id_key = "ChallengeId" } },
    .{ "TrapDefenseTotalBuildingBranch", .{ .prog_key = "trap:bld_branch_cnt" } },
    .{ "TrapDefenseTotalStar", .{ .prog_key = "trap:total_star" } },
    .{ "TrapDefenseUnlockTech", .{ .prog_key = "trap:tech", .claim_prefix = "trap:tech:", .param_id_key = "Id" } },
    .{ "TrapDefenseUnlockTechCount", .{ .prog_key = "trap:tech_cnt", .claim_prefix = "trap:tech:" } },
    .{ "TriggerInterative", .{ .prog_key = "explore:interactive" } },
    .{ "TurnTableBigRewardCount", .{ .prog_key = "turntable:spin" } },
    .{ "UseSkill", .{ .prog_key = "combat:skill_use", .param_id_key = "SkillId" } },
    .{ "WeekInstanceCount", .{ .prog_key = "inst:week_cnt" } },
    .{ "WeeklyFrameTotalScore", .{ .prog_key = "adv:weekly_score" } },
    .{ "WuWuKujiRewardCount", .{ .prog_key = "wuwu:spin", .claim_prefix = "wuwu_" } },
    .{ "WuWuWeekCheckInDay", .{ .prog_key = "wuwu_week:checkin" } },
});

const RoleThresholdField = enum { breach, level, favor, resonance };

fn evalRoleThreshold(ctx: EvalContext, lp: std.json.ObjectMap, comptime field: RoleThresholdField, req: i32) bool {
    const param_role = getParamInt(lp, "RoleId") orelse 0;
    const role_id = if (param_role != 0) param_role else ctx.target_role_id;
    const rc = ctx.role_comp orelse return false;
    if (role_id == 0) {
        var it = rc.role_map.iterator();
        while (it.next()) |kv| {
            if (roleMeetsThreshold(kv.value_ptr.*, field, req)) return true;
        }
        return false;
    }
    const role = rc.role_map.get(role_id) orelse return false;
    return roleMeetsThreshold(role, field, req);
}

fn roleMeetsThreshold(role: anytype, comptime field: RoleThresholdField, req: i32) bool {
    return switch (field) {
        .breach => role.breakthrough >= req,
        .level => role.level >= req,
        .favor => role.favor >= req,
        .resonance => blk: {
            var active_reson: i32 = 0;
            for (role.reson) |r| {
                if (r.open) active_reson += 1;
            }
            break :blk @max(role.resonant_chain_group_index, active_reson) >= req;
        },
    };
}

pub fn evalQuestProvideConditions(ctx: EvalContext, provide_val: ?std.json.Value) bool {
    const pv = provide_val orelse return true;
    if (pv != .object) return true;
    const conds_val = pv.object.get("Conditions") orelse return true;
    if (conds_val != .array or conds_val.array.items.len == 0) return true;

    const is_or = (jsonInt(pv.object.get("Type")) orelse 0) == 1;

    for (conds_val.array.items) |cond_item| {
        const passed = evalSingleProvideCondition(ctx, cond_item);
        if (is_or and passed) return true;
        if (!is_or and !passed) return false;
    }
    return !is_or;
}

fn evalSystemStateProvideCondition(ctx: EvalContext, obj: std.json.ObjectMap) bool {
    const cfg_val = obj.get("Config") orelse return false;
    if (cfg_val != .object) return false;
    const cfg = cfg_val.object;
    const st = if (cfg.get("Type")) |tv| (if (tv == .string) tv.string else "") else "";
    const pc = ctx.progress_comp orelse return false;

    if (std.mem.eql(u8, st, "TrackMoonBuilding")) {
        const bid = jsonInt(cfg.get("BuildingId")) orelse return false;
        const want_built = if (cfg.get("IsBuilt")) |bv| (bv == .bool and bv.bool) else true;
        var buf1: [48]u8 = undefined;
        var buf2: [48]u8 = undefined;
        const k1 = std.fmt.bufPrint(&buf1, "trackmoon:bld:{d}", .{bid}) catch return false;
        const k2 = std.fmt.bufPrint(&buf2, "track_moon:bld:{d}", .{bid}) catch return false;
        const is_built = pc.info.getKeyInt(k1) > 0 or pc.info.getKeyInt(k2) > 0;
        return is_built == want_built;
    }
    if (std.mem.eql(u8, st, "RoadBuildState")) {
        const rid = jsonInt(cfg.get("RoadId")) orelse return false;
        const want_built = if (cfg.get("IsBuilt")) |bv| (bv == .bool and bv.bool) else true;
        var buf1: [48]u8 = undefined;
        var buf2: [48]u8 = undefined;
        const k1 = std.fmt.bufPrint(&buf1, "infr:road:{d}", .{rid}) catch return false;
        const k2 = std.fmt.bufPrint(&buf2, "road_build:{d}", .{rid}) catch return false;
        const is_built = pc.info.hasClaim(k1) or pc.info.getKeyInt(k1) > 0 or pc.info.hasClaim(k2) or pc.info.getKeyInt(k2) > 0;
        return is_built == want_built;
    }
    if (std.mem.eql(u8, st, "TrackMoonPopularity")) {
        const req = jsonInt(cfg.get("Popularity")) orelse return false;
        const cmp = if (cfg.get("Compare")) |cv| (if (cv == .string) cv.string else "Ge") else "Ge";
        const raw = @max(pc.info.getKeyInt("track_moon:pop"), pc.info.getKeyInt("trackmoon:popular"));
        const cur: i32 = @intCast(std.math.clamp(raw, std.math.minInt(i32), std.math.maxInt(i32)));
        return compareInts(cur, req, cmp);
    }
    if (std.mem.eql(u8, st, "InfrActivityExp")) {
        const req = jsonInt(cfg.get("Exp")) orelse return false;
        const cmp = if (cfg.get("Compare")) |cv| (if (cv == .string) cv.string else "Ge") else "Ge";
        const cur: i32 = @intCast(std.math.clamp(pc.info.getKeyInt("infr:exp"), std.math.minInt(i32), std.math.maxInt(i32)));
        return compareInts(cur, req, cmp);
    }
    if (std.mem.eql(u8, st, "InfrastructureStage")) {
        const req = jsonInt(cfg.get("InfrastructureStage")) orelse return false;
        return pc.info.getKeyInt("infr:stage") >= req;
    }
    if (std.mem.eql(u8, st, "SpringFestivalAtmosphereLevel")) {
        const req = jsonInt(cfg.get("AtmosphereLevel")) orelse return false;
        return @max(pc.info.getKeyInt("spring:atmosphere"), pc.info.getKeyInt("spring_manor:atmosphere")) >= req;
    }
    if (std.mem.eql(u8, st, "VillageInfrLevel")) {
        const req = jsonInt(cfg.get("Level")) orelse return false;
        return @max(pc.info.getKeyInt("village:level"), pc.info.getKeyInt("infr2:lvl")) >= req;
    }
    if (std.mem.eql(u8, st, "VillageInfrTreeState")) {
        const tid = jsonInt(cfg.get("Id")) orelse return false;
        const want_built = if (cfg.get("IsBuilt")) |bv| (bv == .bool and bv.bool) else true;
        var buf1: [48]u8 = undefined;
        var buf2: [48]u8 = undefined;
        const k1 = std.fmt.bufPrint(&buf1, "infr2:tree:{d}", .{tid}) catch return false;
        const k2 = std.fmt.bufPrint(&buf2, "village:tree:{d}", .{tid}) catch return false;
        const is_built = pc.info.hasClaim(k1) or pc.info.getKeyInt(k1) > 0 or pc.info.getKeyInt(k2) > 0;
        return is_built == want_built;
    }
    if (std.mem.eql(u8, st, "DropCatchGameplayCompleteLevel")) {
        const lid = jsonInt(cfg.get("LevelId")) orelse return false;
        var buf1: [48]u8 = undefined;
        var buf2: [48]u8 = undefined;
        const k1 = std.fmt.bufPrint(&buf1, "drop_catch:done:{d}", .{lid}) catch return false;
        const k2 = std.fmt.bufPrint(&buf2, "dc:score:{d}", .{lid}) catch return false;
        return pc.info.hasClaim(k1) or pc.info.getKeyInt(k1) > 0 or pc.info.getKeyInt(k2) > 0;
    }
    if (std.mem.eql(u8, st, "RhythmSpaceshipReachLevelRank")) {
        const lid = jsonInt(cfg.get("RhythmSpaceshipLevelId")) orelse return false;
        const req_rank = jsonInt(cfg.get("Rank")) orelse 1;
        var buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&buf, "rhythm:rank:{d}", .{lid}) catch return false;
        const cur_rank = pc.info.getKeyInt(key);
        // RhythmShipRank enum: Undefined=0, SSS=1, SS=2, S=3, A=4, B=5 (lower positive is better)
        return cur_rank > 0 and cur_rank <= req_rank;
    }
    if (std.mem.eql(u8, st, "SpringFestivalGuessJokerFirstComplete")) {
        const lid = jsonInt(cfg.get("LevelId")) orelse return false;
        var buf1: [48]u8 = undefined;
        var buf2: [48]u8 = undefined;
        const k1 = std.fmt.bufPrint(&buf1, "guess_joker:done:{d}", .{lid}) catch return false;
        const k2 = std.fmt.bufPrint(&buf2, "joker_rw:{d}", .{lid}) catch return false;
        return pc.info.hasClaim(k1) or pc.info.getKeyInt(k1) > 0 or pc.info.hasClaim(k2);
    }
    if (std.mem.eql(u8, st, "DangoBrawlChallengeComplete")) {
        const cid = jsonInt(cfg.get("ChallengeId")) orelse return false;
        var buf1: [48]u8 = undefined;
        var buf2: [48]u8 = undefined;
        const k1 = std.fmt.bufPrint(&buf1, "dango:challenge:{d}", .{cid}) catch return false;
        const k2 = std.fmt.bufPrint(&buf2, "db_task:{d}", .{cid}) catch return false;
        return pc.info.hasClaim(k1) or pc.info.getKeyInt(k1) > 0 or pc.info.hasClaim(k2);
    }
    if (std.mem.eql(u8, st, "CollectionShopFull")) {
        const shop_type = if (cfg.get("ShopType")) |stv| (if (stv == .string) stv.string else "") else "";
        if (shop_type.len == 0) return false;
        var buf: [64]u8 = undefined;
        const key = std.fmt.bufPrint(&buf, "coll_shop:full:{s}", .{shop_type}) catch return false;
        return pc.info.hasClaim(key) or pc.info.getKeyInt(key) > 0;
    }
    if (std.mem.eql(u8, st, "EncircleLevelComplete")) {
        const eid = jsonInt(cfg.get("EncircleLevelId")) orelse return false;
        const want_done = if (cfg.get("IsCompleted")) |bv| (bv == .bool and bv.bool) else true;
        var buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&buf, "encircle:done:{d}", .{eid}) catch return false;
        const is_done = pc.info.hasClaim(key) or pc.info.getKeyInt(key) > 0;
        return is_done == want_done;
    }
    return false;
}

fn evalSingleProvideCondition(ctx: EvalContext, cond_item: std.json.Value) bool {
    if (cond_item != .object) return true;
    const obj = cond_item.object;
    const t_val = obj.get("Type") orelse return true;
    switch (t_val) {
        .integer => return evalInlineConditionBlock(ctx, cond_item),
        .string => |ctype| {
            if (std.mem.eql(u8, ctype, "PreQuest")) {
                const pre_id = jsonInt(obj.get("PreQuest")) orelse return false;
                return ctx.quest_comp.info.hasFinishedQuest(pre_id);
            } else if (std.mem.eql(u8, ctype, "PreChildQuest")) {
                const sub = obj.get("PreChildQuest") orelse return false;
                if (sub != .object) return false;
                const qid = jsonInt(sub.object.get("QuestId")) orelse return false;
                const cid = jsonInt(sub.object.get("ChildQuestId")) orelse return false;
                return ctx.quest_comp.info.hasFinishedChildQuest(qid, cid);
            } else if (std.mem.eql(u8, ctype, "ExploreLevel")) {
                const req = jsonInt(obj.get("ExploreLevel")) orelse 1;
                return ctx.basic_comp.info.attributes.level >= req;
            } else if (std.mem.eql(u8, ctype, "PreLevelPlay")) {
                const lpid = jsonInt(obj.get("PreLevelPlay")) orelse return false;
                return containsInt(ctx.quest_comp.info.completed_level_plays, lpid);
            } else if (std.mem.eql(u8, ctype, "PreDungeon")) {
                const sub = obj.get("Dungeon") orelse return false;
                if (sub != .object) return false;
                const did = jsonInt(sub.object.get("DungeonId")) orelse return false;
                return containsInt(ctx.quest_comp.info.completed_dungeons, did);
            } else if (std.mem.eql(u8, ctype, "Item")) {
                const items_val = obj.get("Items") orelse return false;
                if (items_val != .array) return false;
                for (items_val.array.items) |it| {
                    if (it != .object) continue;
                    const iid = jsonInt(it.object.get("ItemId")) orelse continue;
                    const cnt = jsonInt(it.object.get("Count")) orelse 1;
                    if (itemCount(ctx, iid) < cnt) return false;
                }
                return true;
            } else if (std.mem.eql(u8, ctype, "Gender")) {
                const gval = obj.get("Gender") orelse return true;
                if (gval == .string) {
                    if (std.mem.eql(u8, gval.string, "男")) return ctx.basic_comp.info.attributes.sex == 1;
                    if (std.mem.eql(u8, gval.string, "女")) return ctx.basic_comp.info.attributes.sex == 0;
                }
                return true;
            } else if (std.mem.eql(u8, ctype, "SystemState")) {
                return evalSystemStateProvideCondition(ctx, obj);
            } else if (std.mem.eql(u8, ctype, "OpenDate")) {
                return true;
            }
            return evalSingleInlineCondition(ctx, cond_item);
        },
        else => return true,
    }
}

pub fn evalInlineConditionBlock(ctx: EvalContext, block_val: std.json.Value) bool {
    if (block_val != .object) return true;
    const conds_val = block_val.object.get("Conditions") orelse return true;
    if (conds_val != .array or conds_val.array.items.len == 0) return true;

    // Type 0 = AND, Type 1 = OR
    const is_or = (jsonInt(block_val.object.get("Type")) orelse 0) == 1;
    for (conds_val.array.items) |c| {
        const passed = evalSingleInlineCondition(ctx, c);
        if (is_or and passed) return true;
        if (!is_or and !passed) return false;
    }
    return !is_or;
}

/// Resolves persisted entity state first, falling back to the entity/template configured default state.
fn resolveCurrentEntityState(ctx: EvalContext, eid: i32) []const u8 {
    if (ctx.quest_comp.info.getEntityState(eid)) |saved| return saved;
    if (ctx.assets) |assets| {
        const inst_id: i32 = if (ctx.scene) |s| s.instance_id else 8;
        if (assets.findLevelEntity(inst_id, eid)) |lcfg| {
            var comps = lcfg.ComponentsData;
            if (assets.findTemplateConfig(lcfg.BlueprintType)) |tpl| {
                tpl.ComponentsData.mergeInto(&comps);
            }
            if (comps.EntityStateComponent) |esc| {
                if (esc.State) |st| {
                    if (st.len > 0) return st;
                }
            }
        }
    }
    return "关卡.Common.状态.常态";
}

fn evalEntityStateCompare(ctx: EvalContext, eid: i32, obj: std.json.ObjectMap) bool {
    const want_state = if (obj.get("State")) |v| switch (v) {
        .string => |s| s,
        else => return false,
    } else return false;
    const cmp = if (obj.get("Compare")) |v| switch (v) {
        .string => |s| s,
        else => "Eq",
    } else "Eq";
    const cur = resolveCurrentEntityState(ctx, eid);
    const eq = std.mem.eql(u8, cur, want_state);
    return if (std.mem.eql(u8, cmp, "Ne") or std.mem.eql(u8, cmp, "NotEq")) !eq else eq;
}

pub fn evalSingleInlineCondition(ctx: EvalContext, c_val: std.json.Value) bool {
    if (c_val != .object) return true;
    const obj = c_val.object;
    const t_val = obj.get("Type") orelse return true;
    const ct = switch (t_val) {
        .integer => return evalInlineConditionBlock(ctx, c_val),
        .string => |s| s,
        else => return true,
    };

    if (std.mem.eql(u8, ct, "CompareVar")) {
        return evalCompareVarCondition(ctx, obj);
    } else if (std.mem.eql(u8, ct, "CheckChildQuestFinished")) {
        const qid = jsonInt(obj.get("QuestId")) orelse return false;
        const cid = jsonInt(obj.get("ChildQuestId")) orelse return false;
        return ctx.quest_comp.info.hasFinishedChildQuest(qid, cid);
    } else if (std.mem.eql(u8, ct, "CheckQuestFinished")) {
        const qid = jsonInt(obj.get("QuestId")) orelse return false;
        return ctx.quest_comp.info.hasFinishedQuest(qid);
    } else if (std.mem.eql(u8, ct, "CheckItems")) {
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Ge",
        } else "Ge";
        const items = obj.get("Items") orelse return false;
        if (items != .array or items.array.items.len == 0) return true;
        for (items.array.items) |it| {
            if (it != .object) continue;
            const iid = jsonInt(it.object.get("ItemId")) orelse continue;
            const cnt = jsonInt(it.object.get("Count")) orelse 1;
            if (!compareInts(itemCount(ctx, iid), cnt, cmp)) return false;
        }
        return true;
    } else if (std.mem.eql(u8, ct, "CompareEntityState")) {
        const eid = jsonInt(obj.get("EntityId")) orelse return false;
        return evalEntityStateCompare(ctx, eid, obj);
    } else if (std.mem.eql(u8, ct, "CompareEntitySelfState")) {
        if (ctx.self_entity_config_id <= 0) return false;
        return evalEntityStateCompare(ctx, ctx.self_entity_config_id, obj);
    } else if (std.mem.eql(u8, ct, "CheckLevelPlay")) {
        if (jsonInt(obj.get("LevelPlayId"))) |lpid| {
            return containsInt(ctx.quest_comp.info.completed_level_plays, lpid);
        }
        if (obj.get("LevelPlayIds")) |ids_val| {
            if (ids_val == .array) {
                const req = @max(1, jsonInt(obj.get("CompleteNumber")) orelse 1);
                var done: i32 = 0;
                for (ids_val.array.items) |v| {
                    const lpid = jsonInt(v) orelse continue;
                    if (containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) done += 1;
                }
                return done >= req;
            }
        }
        return false;
    } else if (std.mem.eql(u8, ct, "CheckDungeon")) {
        const did = jsonInt(obj.get("DungeonId")) orelse return false;
        return containsInt(ctx.quest_comp.info.completed_dungeons, did);
    } else if (std.mem.eql(u8, ct, "CheckActivityState")) {
        const aid = jsonInt(obj.get("ActivityId")) orelse return false;
        const want = jsonInt(obj.get("ActivityState")) orelse 2;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Eq",
        } else "Eq";
        const cur: i32 = if (isActivityOpen(ctx, aid)) 2 else 0;
        return compareInts(cur, want, cmp);
    } else if (std.mem.eql(u8, ct, "CheckPlayerGender")) {
        const g = if (obj.get("Gender")) |v| switch (v) {
            .string => |s| s,
            else => return true,
        } else return true;
        if (std.mem.eql(u8, g, "男")) return ctx.basic_comp.info.attributes.sex == 1;
        if (std.mem.eql(u8, g, "女")) return ctx.basic_comp.info.attributes.sex == 0;
        return true;
    } else if (std.mem.eql(u8, ct, "CheckPlayerMoraleLevelRange")) {
        const min_level = jsonInt(obj.get("MinMoraleLevel")) orelse return false;
        const max_level = jsonInt(obj.get("MaxMoraleLevel")) orelse return false;
        if (min_level > max_level) return false;
        // Decompiled MoraleBattleModel initializes MoraleLevel to 1 and the
        // condition reads GetMoraleLevel() (not the temporary level). A
        // persisted server value can override this baseline once the morale
        // gameplay state is available to the server.
        const saved_level: i64 = if (ctx.progress_comp) |progress|
            progress.info.getKeyInt("morale:level")
        else
            0;
        const current_level: i32 = if (saved_level > 0)
            @intCast(@min(saved_level, std.math.maxInt(i32)))
        else
            1;
        return current_level >= min_level and current_level <= max_level;
    } else if (std.mem.eql(u8, ct, "CheckCurrentRole")) {
        const role_id = jsonInt(obj.get("RoleId")) orelse return false;
        const scene = ctx.scene orelse return false;
        const formation_index = std.math.cast(usize, scene.formation_info.cur_formation) orelse return false;
        if (formation_index >= scene.formation_info.formations.len) return false;
        const formation = scene.formation_info.formations[formation_index];
        const current_role_id = formation.cur_role;
        if (current_role_id <= 0 or role_id <= 0) return false;
        var current_role_exists = false;
        for (formation.roles) |maybe_role| {
            const role = maybe_role orelse continue;
            if (role.role_id == current_role_id) {
                current_role_exists = true;
                break;
            }
        }
        if (!current_role_exists) return false;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Eq",
        } else "Eq";
        return compareInts(current_role_id, role_id, cmp);
    } else if (std.mem.eql(u8, ct, "CompareFishingPrestigeLevel")) {
        const required = jsonInt(obj.get("PrestigeLevel")) orelse return false;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Ge",
        } else "Ge";
        // Client FishingModel derives this level from FishingSailingExpItemId
        // (CommonParam 35) and the configured FishingReputation thresholds.
        const assets = ctx.assets orelse return false;
        return compareInts(fishingPrestigeLevel(assets, itemCount(ctx, 35)), required, cmp);
    } else if (std.mem.eql(u8, ct, "CheckDangoCultivationProgress")) {
        const check_type = obj.get("CheckType") orelse return false;
        if (check_type != .object or !jsonHasType(check_type.object, "CountDangoOverTargetLevel")) return false;
        const target_level = jsonInt(check_type.object.get("TargetLevel")) orelse return false;
        const target_number = jsonInt(check_type.object.get("TargetNumber")) orelse return false;
        if (target_level < 0 or target_number < 0) return false;
        const progress = ctx.progress_comp orelse return false;
        const assets = ctx.assets orelse return false;
        return countDangoAboveLevel(assets, progress.info, target_level) >= target_number;
    } else if (std.mem.eql(u8, ct, "CompareAlertValue")) {
        const area_id = jsonInt(obj.get("AreaId")) orelse return false;
        const compare_type = obj.get("CompareType") orelse return false;
        const compare_value = obj.get("CompareValue") orelse return false;
        if (compare_type != .string or compare_value != .object) return false;
        const value_type = jsonInt(compare_value.object.get("Type")) orelse return false;
        // Alert values are the alert-state enum: 0 = clear, 1 = alert.
        if (value_type != 0 and value_type != 1) return false;
        const progress = ctx.progress_comp orelse return false;
        var key_buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&key_buf, "alert:area:{d}", .{area_id}) catch return false;
        return compareInts(@intCast(@intFromBool(progress.info.getKeyInt(key) != 0)), value_type, compare_type.string);
    } else if (std.mem.eql(u8, ct, "CheckTeleControlState")) {
        const config_id = jsonInt(obj.get("EntityId")) orelse return false;
        const wanted = obj.get("State") orelse return false;
        if (wanted != .string) return false;
        const scene = ctx.scene orelse return false;
        _ = scene.findEntityNetIdByConfigId(config_id) orelse return false;
        const progress = ctx.progress_comp orelse return false;
        var key_buf: [64]u8 = undefined;
        const key = std.fmt.bufPrint(&key_buf, "item_bb:{d}:0", .{config_id}) catch return false;
        const state = progress.info.getKeyInt(key);
        if (std.mem.eql(u8, wanted.string, "Hold")) return state == 4;
        if (std.mem.eql(u8, wanted.string, "LetGo")) return state == 11;
        if (std.mem.eql(u8, wanted.string, "Throwing")) return state == 6 or state == 7 or state == 8 or state == 9;
        return false;
    } else if (std.mem.eql(u8, ct, "ListenEntityThroughPortal")) {
        const portal_id = jsonInt(obj.get("PortalEntityId")) orelse return false;
        const target = obj.get("CheckTarget") orelse return false;
        if (target != .object or !jsonHasType(target.object, "Player")) return false;
        const scene = ctx.scene orelse return false;
        return scene.last_passed_portal_id == portal_id;
    } else if (std.mem.eql(u8, ct, "CheckIsTargetHoldElementalBox")) {
        const target = obj.get("Target") orelse return false;
        if (target != .object or !jsonHasType(target.object, "Player")) return false;
        const want_holding = switch (obj.get("IsHolding") orelse return false) {
            .bool => |b| b,
            else => return false,
        };
        const scene = ctx.scene orelse return false;
        const progress = ctx.progress_comp orelse return false;
        const assets = ctx.assets orelse return false;
        const wanted_box = jsonInt(obj.get("ElementalBoxId"));
        const wanted_type = getParamStr(obj, "TargetElementalType");
        var is_holding_matching_box = false;
        const slice = scene.entities.slice();
        for (slice.items(.config)) |config| {
            var key_buf: [64]u8 = undefined;
            const key = std.fmt.bufPrint(&key_buf, "item_bb:{d}:0", .{config.config_id}) catch continue;
            if (progress.info.getKeyInt(key) != 4) continue; // BeHolding
            const box = assets.tables.elemental_box_define.getDataById(config.config_id) orelse continue;
            if (wanted_box) |box_id| if (box_id != config.config_id) continue;
            if (wanted_type) |elemental_type| if (!std.mem.eql(u8, box.ElementalType, elemental_type)) continue;
            is_holding_matching_box = true;
            break;
        }
        return is_holding_matching_box == want_holding;
    } else if (std.mem.eql(u8, ct, "CheckGiantStableValue")) {
        const option = obj.get("StableValueOption") orelse return false;
        if (option != .object or !jsonHasType(option.object, "Region")) return false;
        const eco_type = getParamStr(option.object, "EcoType") orelse return false;
        const eco_id: i32 = if (std.mem.eql(u8, eco_type, "Fire"))
            0
        else if (std.mem.eql(u8, eco_type, "Water"))
            1
        else if (std.mem.eql(u8, eco_type, "Wood"))
            2
        else
            return false;
        const range = option.object.get("Range") orelse return false;
        if (range != .object) return false;
        const min_obj = range.object.get("MinValue") orelse return false;
        const max_obj = range.object.get("MaxValue") orelse return false;
        if (min_obj != .object or max_obj != .object) return false;
        const min_value = jsonInt(min_obj.object.get("Value")) orelse return false;
        const max_value = jsonInt(max_obj.object.get("Value")) orelse return false;
        const min_boundary = getParamStr(min_obj.object, "Boundary") orelse return false;
        const max_boundary = getParamStr(max_obj.object, "Boundary") orelse return false;
        const progress = ctx.progress_comp orelse return false;
        var key_buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&key_buf, "giant:stable:{d}", .{eco_id}) catch return false;
        const stable: i32 = @intCast(std.math.clamp(progress.info.getKeyInt(key), 0, std.math.maxInt(i32)));
        const min_ok = if (std.mem.eql(u8, min_boundary, "Closed")) stable >= min_value else stable > min_value;
        const max_ok = if (std.mem.eql(u8, max_boundary, "Closed")) stable <= max_value else stable < max_value;
        return min_ok and max_ok;
    } else if (std.mem.eql(u8, ct, "CheckFormationRoleInfo")) {
        return evalFormationRoleInfo(ctx, obj);
    } else if (std.mem.eql(u8, ct, "CheckEntityGravityDirection")) {
        return evalEntityGravityDirection(ctx, obj);
    } else if (std.mem.eql(u8, ct, "CheckEntityLocked")) {
        const scene = ctx.scene orelse return false;
        const entities_val = obj.get("Entities") orelse return false;
        if (entities_val != .array or entities_val.array.items.len == 0) return false;
        const want_locked = if (obj.get("IsLocked")) |v| switch (v) {
            .bool => |b| b,
            else => return false,
        } else return false;
        const assets = ctx.assets orelse return false;
        const locked_tag_id = gameplay_tags.idFromName(
            &assets.tables.gameplay_tag_remap,
            "关卡.Common.属性.锁定",
        ) catch return false;
        const slice = scene.entities.slice();
        const configs = slice.items(.config);
        const tags = slice.items(.tag);
        for (entities_val.array.items) |entity_val| {
            const entity_id = jsonInt(entity_val) orelse return false;
            var found = false;
            var is_locked = false;
            for (configs, tags) |config, tag_opt| {
                if (config.config_id != entity_id) continue;
                found = true;
                if (tag_opt) |*tag| {
                    is_locked = tag.hasTag(&assets.tables.gameplay_tag_parent, locked_tag_id);
                }
                break;
            }
            if (!found or is_locked != want_locked) return false;
        }
        return true;
    } else if (std.mem.eql(u8, ct, "CompareTimePeriod")) {
        const period = if (obj.get("TimePeriod")) |v| switch (v) {
            .string => |s| s,
            else => return false,
        } else return false;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Eq",
        } else "Eq";
        const hour: i32 = if (ctx.quest_comp.info.env.time_locked)
            ctx.quest_comp.info.env.time_lock_hour
        else if (ctx.scene) |scene|
            scene.instance.map_time.hour
        else
            ctx.quest_comp.info.env.time_lock_hour;
        const minute: i32 = if (ctx.quest_comp.info.env.time_locked)
            ctx.quest_comp.info.env.time_lock_minute
        else if (ctx.scene) |scene|
            scene.instance.map_time.minute
        else
            ctx.quest_comp.info.env.time_lock_minute;
        const total_minute = hour * 60 + minute;
        const is_day = total_minute >= 360 and total_minute < 1080;
        const matches = if (std.mem.eql(u8, period, "DayTime"))
            is_day
        else if (std.mem.eql(u8, period, "Night"))
            !is_day
        else
            return false;
        if (std.mem.eql(u8, cmp, "Eq")) return matches;
        if (std.mem.eql(u8, cmp, "Ne")) return !matches;
        return false;
    } else if (std.mem.eql(u8, ct, "CheckConditionGroup")) {
        if (obj.get("Condition")) |sub| return evalInlineConditionBlock(ctx, sub);
        if (jsonInt(obj.get("GroupId"))) |gid| return evalConditionGroup(ctx, gid);
        return true;
    } else if (std.mem.eql(u8, ct, "ReadMail")) {
        const mid = jsonInt(obj.get("MailId")) orelse return false;
        return containsInt(ctx.quest_comp.info.progress.read_mails, mid);
    } else if (std.mem.eql(u8, ct, "ReadPhoneMessage")) {
        const pmid = jsonInt(obj.get("PhoneMessageId")) orelse return false;
        return containsInt(ctx.quest_comp.info.progress.read_phone_messages, pmid);
    } else if (std.mem.eql(u8, ct, "ReceiveTelecom")) {
        const tid = jsonInt(obj.get("TelecomId")) orelse return false;
        return containsInt(ctx.quest_comp.info.progress.received_telecoms, tid);
    } else if (std.mem.eql(u8, ct, "FinishBvbChallenge")) {
        const bid = jsonInt(obj.get("BvbChallengeId")) orelse return false;
        return containsInt(ctx.quest_comp.info.progress.completed_bvb, bid);
    } else if (std.mem.eql(u8, ct, "CompareQuestState") or std.mem.eql(u8, ct, "CheckQuestState")) {
        const qid = jsonInt(obj.get("QuestId")) orelse return false;
        const want_st = jsonInt(obj.get("State")) orelse 3;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Eq",
        } else "Eq";
        const cur_st = questStateValue(ctx, qid);
        return compareInts(cur_st, want_st, cmp);
    } else if (std.mem.eql(u8, ct, "CheckNodeStatus")) {
        const nid = jsonInt(obj.get("NodeId")) orelse return false;
        const want_st = jsonInt(obj.get("Status")) orelse 4;
        const qid = jsonInt(obj.get("QuestId")) orelse if (ctx.owner_quest_id > 0) ctx.owner_quest_id else ctx.quest_comp.info.tracked_quest_id;
        return evalInlineNodeStatus(ctx, qid, nid, want_st);
    } else if (std.mem.eql(u8, ct, "CheckIsTrackingCurrentQuest")) {
        if (jsonInt(obj.get("QuestId"))) |qid| {
            return ctx.quest_comp.info.tracked_quest_id == qid;
        }
        return ctx.quest_comp.info.tracked_quest_id > 0;
    } else if (std.mem.eql(u8, ct, "CheckIsTrackingQuest")) {
        const qid = jsonInt(obj.get("QuestId")) orelse return false;
        const want_track = if (obj.get("IsTracking")) |v| (if (v == .bool) v.bool else true) else true;
        return (ctx.quest_comp.info.tracked_quest_id == qid) == want_track;
    } else if (std.mem.eql(u8, ct, "Directly")) {
        const eid = jsonInt(obj.get("EntityId")) orelse return false;
        const want_state = if (obj.get("State")) |v| switch (v) {
            .string => |s| s,
            else => return false,
        } else return false;
        const cur = ctx.quest_comp.info.getEntityState(eid) orelse "关卡.Common.状态.常态";
        return std.mem.eql(u8, cur, want_state);
    } else if (std.mem.eql(u8, ct, "HourToHour")) {
        const start_obj = if (obj.get("Start")) |v| if (v == .object) v.object else null else null;
        const end_obj = if (obj.get("End")) |v| if (v == .object) v.object else null else null;
        const cur_hour: i32 = if (ctx.quest_comp.info.env.time_locked)
            ctx.quest_comp.info.env.time_lock_hour
        else if (ctx.scene) |s|
            s.instance.map_time.hour
        else
            ctx.quest_comp.info.env.time_lock_hour;
        const cur_min: i32 = if (ctx.quest_comp.info.env.time_locked)
            ctx.quest_comp.info.env.time_lock_minute
        else if (ctx.scene) |s|
            s.instance.map_time.minute
        else
            ctx.quest_comp.info.env.time_lock_minute;
        const cur_total: i32 = cur_hour * 60 + cur_min;
        const s_h = if (start_obj) |so| jsonInt(so.get("Hour")) orelse 0 else 0;
        const s_m = if (start_obj) |so| jsonInt(so.get("Min")) orelse 0 else 0;
        const e_h = if (end_obj) |eo| jsonInt(eo.get("Hour")) orelse 24 else 24;
        const e_m = if (end_obj) |eo| jsonInt(eo.get("Min")) orelse 0 else 0;
        const start_total: i32 = s_h * 60 + s_m;
        const end_total: i32 = e_h * 60 + e_m;
        const in_span = if (start_total <= end_total)
            cur_total >= start_total and cur_total <= end_total
        else
            cur_total >= start_total or cur_total <= end_total;
        const is_ne = if (obj.get("Compare")) |cv| switch (cv) {
            .string => |s| std.mem.eql(u8, s, "Ne") or std.mem.eql(u8, s, "NotEq") or std.mem.eql(u8, s, "!="),
            .integer => |iv| iv != 0,
            else => false,
        } else false;
        return if (is_ne) !in_span else in_span;
    } else if (std.mem.eql(u8, ct, "CheckLevel")) {
        const req = jsonInt(obj.get("Level")) orelse 1;
        return ctx.basic_comp.info.attributes.level >= req;
    } else if (std.mem.eql(u8, ct, "CompareExploreLevel")) {
        const req = jsonInt(obj.get("Level")) orelse 1;
        const cmp = if (obj.get("Compare")) |v| (if (v == .string) v.string else "Ge") else "Ge";
        return compareInts(ctx.basic_comp.info.attributes.level, req, cmp);
    } else if (std.mem.eql(u8, ct, "CheckOriginWorldLevel")) {
        const req = jsonInt(obj.get("Level")) orelse 0;
        return ctx.basic_comp.info.attributes.origin_world_level >= req;
    } else if (std.mem.eql(u8, ct, "CheckCurWorldLevel")) {
        const req = jsonInt(obj.get("Level")) orelse 0;
        return ctx.basic_comp.info.attributes.cur_world_level >= req;
    } else if (std.mem.eql(u8, ct, "CompareDungeonId")) {
        const did = jsonInt(obj.get("DungeonId")) orelse return false;
        const scene = ctx.scene orelse return false;
        if (scene.instance_id <= 0 or did <= 0) return false;
        const cmp = if (obj.get("Compare")) |v| (if (v == .string) v.string else "Eq") else "Eq";
        const is_ne = std.mem.eql(u8, cmp, "Ne") or std.mem.eql(u8, cmp, "NotEq") or std.mem.eql(u8, cmp, "!=");
        const eq = scene.instance_id == did;
        return if (is_ne) !eq else eq;
    } else if (std.mem.eql(u8, ct, "CheckDungeonFinish")) {
        const did = jsonInt(obj.get("DungeonId")) orelse return false;
        const want_finished = if (obj.get("IsFinished")) |v| (if (v == .bool) v.bool else true) else true;
        return containsInt(ctx.quest_comp.info.completed_dungeons, did) == want_finished;
    } else if (std.mem.eql(u8, ct, "CheckInRange") or std.mem.eql(u8, ct, "CheckInRangeProperly")) {
        const want_in = if (obj.get("InRange")) |v| if (v == .bool) v.bool else true else true;
        return evalCheckInRange(ctx, obj) == want_in;
    } else if (std.mem.eql(u8, ct, "CheckSystemState")) {
        return evalSystemStateProvideCondition(ctx, obj);
    } else if (std.mem.eql(u8, ct, "CheckVehicleCondition")) {
        if (obj.get("Condition")) |sub| return evalSingleInlineCondition(ctx, sub);
        return false;
    } else if (std.mem.eql(u8, ct, "CheckInCombat")) {
        const want_in_combat = switch (obj.get("InCombat") orelse return false) {
            .bool => |value| value,
            else => return false,
        };
        const scene = ctx.scene orelse return false;
        return scene.player_in_battle == want_in_combat;
    } else if (std.mem.eql(u8, ct, "ComparePlayerMotionState") or std.mem.eql(u8, ct, "ComparePlayerMotionState2")) {
        const motion = if (obj.get("MotionState")) |v| (if (v == .string) v.string else "") else "";
        const cmp = if (obj.get("Compare")) |v| (if (v == .string) v.string else "Eq") else "Eq";
        const is_ne = std.mem.eql(u8, cmp, "Ne");
        if (!std.mem.eql(u8, cmp, "Eq") and !is_ne) return false;
        const matched = evalPlayerMotionState(ctx, motion) orelse return false;
        return if (is_ne) !matched else matched;
    } else if (std.mem.eql(u8, ct, "UsingVehicle")) {
        const want_used = if (obj.get("CheckIsBeingUsed")) |v| if (v == .bool) v.bool else true else true;
        const target_vid = jsonInt(obj.get("TargetVehicle")) orelse 0;
        const cur_vid: i64 = if (ctx.progress_comp) |pc| pc.info.getKeyInt("vehicle:cur") else 0;
        var is_used = if (target_vid > 0) cur_vid == target_vid else cur_vid > 0;
        if (!is_used) {
            if (ctx.scene) |sc| {
                const slice = sc.entities.slice();
                for (slice.items(.config), slice.items(.vehicle)) |cfg, veh_opt| {
                    if (veh_opt) |veh| {
                        if (veh.occupied and (target_vid <= 0 or cfg.config_id == target_vid)) {
                            is_used = true;
                            break;
                        }
                    }
                }
            }
        }
        return is_used == want_used;
    } else if (std.mem.eql(u8, ct, "PlayerInVehicle")) {
        const want_in = if (obj.get("CheckType")) |v| if (v == .bool) v.bool else true else true;
        const cur_vid: i64 = if (ctx.progress_comp) |pc| pc.info.getKeyInt("vehicle:cur") else 0;
        var in_veh = cur_vid > 0;
        if (!in_veh) {
            if (ctx.scene) |sc| {
                for (sc.entities.items(.vehicle)) |veh_opt| {
                    if (veh_opt) |veh| {
                        if (veh.occupied) {
                            in_veh = true;
                            break;
                        }
                    }
                }
            }
        }
        return in_veh == want_in;
    } else if (std.mem.eql(u8, ct, "CheckDataLayer")) {
        const dl_id = jsonInt(obj.get("DataLayerId")) orelse return false;
        const want_load = if (obj.get("IsLoad")) |v| if (v == .bool) v.bool else true else true;
        const is_loaded = containsInt(ctx.quest_comp.info.world.loaded_datalayers, dl_id) and
            !containsInt(ctx.quest_comp.info.world.unloaded_datalayers, dl_id);
        return is_loaded == want_load;
    } else if (std.mem.eql(u8, ct, "CompareLift")) {
        const eid = jsonInt(obj.get("EntityId")) orelse return false;
        const want_loc = jsonInt(obj.get("Location")) orelse return false;
        const cmp = if (obj.get("Compare")) |v| (if (v == .string) v.string else "Eq") else "Eq";
        const pc = ctx.progress_comp orelse return false;
        var buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&buf, "lift_target:{d}", .{eid}) catch return false;
        const cur_loc: i32 = @intCast(std.math.clamp(pc.info.getKeyInt(key), std.math.minInt(i32), std.math.maxInt(i32)));
        return compareInts(cur_loc, want_loc, cmp);
    } else if (std.mem.eql(u8, ct, "CheckTreasureBeenClaimed")) {
        const eid = jsonInt(obj.get("EntityId")) orelse return false;
        return ctx.quest_comp.info.hasOpenedChest(eid);
    } else if (std.mem.eql(u8, ct, "CheckSystemFunction")) {
        const sid = jsonInt(obj.get("SystemId")) orelse return false;
        const is_ne = if (obj.get("Compare")) |v| (if (v == .string) (std.mem.eql(u8, v.string, "Ne") or std.mem.eql(u8, v.string, "NotEq")) else false) else false;
        var open = ctx.quest_comp.info.hasUnlockedSystem(sid);
        if (!open) {
            if (ctx.assets) |a| {
                for (a.tables.function_condition.items) |fc| {
                    if (fc.FunctionId == sid) {
                        open = fc.FunctionId == 10053 or fc.OpenConditionId == 0 or evalConditionGroup(ctx, fc.OpenConditionId);
                        break;
                    }
                }
            }
        }
        return if (is_ne) !open else open;
    } else if (std.mem.eql(u8, ct, "CompareWeather")) {
        const wid = jsonInt(obj.get("WeatherId")) orelse return false;
        const cmp = if (obj.get("Compare")) |v| (if (v == .string) v.string else "Eq") else "Eq";
        return compareInts(ctx.quest_comp.info.env.weather_id, wid, cmp);
    } else if (std.mem.eql(u8, ct, "CheckLevelPlayState")) {
        const lpid = jsonInt(obj.get("LevelId")) orelse jsonInt(obj.get("LevelPlayId")) orelse return false;
        const want_st = jsonInt(obj.get("State")) orelse 2;
        const cmp = if (obj.get("Compare")) |v| (if (v == .string) v.string else "Eq") else "Eq";
        const cur_st: i32 = if (containsInt(ctx.quest_comp.info.completed_level_plays, lpid))
            3
        else blk: {
            if (ctx.progress_comp) |pc| {
                var buf: [48]u8 = undefined;
                const key = std.fmt.bufPrint(&buf, "lp_state:{d}", .{lpid}) catch break :blk 1;
                const v = pc.info.getKeyInt(key);
                if (v >= 0) break :blk @intCast(std.math.clamp(v, 0, 3));
            }
            break :blk 1;
        };
        return compareInts(cur_st, want_st, cmp);
    } else if (std.mem.eql(u8, ct, "CheckLevelPlayCompleteNumber")) {
        const level_id = jsonInt(obj.get("LevelId")) orelse return false;
        const required = jsonInt(obj.get("Number")) orelse return false;
        const cmp = if (obj.get("Compare")) |v| (if (v == .string) v.string else "Eq") else "Eq";
        // This server records a level play's completion once and emits
        // CompleteNumber=1. Do not invent repeat counts above that state.
        if (required > 1) return false;
        const count: i32 = @intFromBool(containsInt(ctx.quest_comp.info.completed_level_plays, level_id));
        return compareInts(count, required, cmp);
    } else if (std.mem.eql(u8, ct, "CheckEntitesExist")) {
        const want_exist = if (obj.get("IsExist")) |v| if (v == .bool) v.bool else true else true;
        return evalCheckEntitiesExist(ctx, obj) == want_exist;
    } else if (std.mem.eql(u8, ct, "CheckTargetSunSpiritCollected")) {
        const eid = jsonInt(obj.get("SunSpiritEntityId")) orelse return false;
        const want_col = if (obj.get("IsCollected")) |v| if (v == .bool) v.bool else true else true;
        var collected = ctx.quest_comp.info.hasOpenedChest(eid);
        if (!collected) {
            if (ctx.progress_comp) |pc| {
                var buf: [48]u8 = undefined;
                const key = std.fmt.bufPrint(&buf, "sun_spirit:{d}", .{eid}) catch return false;
                collected = pc.info.hasClaim(key) or pc.info.getKeyInt(key) > 0;
            }
        }
        return collected == want_col;
    } else if (std.mem.eql(u8, ct, "CheckCollectedSunSpiritCount")) {
        const dungeon_id = jsonInt(obj.get("DungeonId")) orelse 0;
        const area_id = jsonInt(obj.get("AreaId")) orelse 0;
        const cmp = getParamStr(obj, "Compare") orelse "Eq";
        const req_count = jsonInt(obj.get("Count")) orelse 0;
        const collected = countCollectedSunSpirits(ctx, dungeon_id, area_id);
        return compareInts(collected, req_count, cmp);
    } else if (std.mem.eql(u8, ct, "CheckIsSupportedWalking")) {
        const want_supported = if (obj.get("IsSupported")) |v| switch (v) {
            .bool => |b| b,
            else => return false,
        } else return false;
        const is_supported = if (ctx.progress_comp) |pc| pc.info.getKeyInt("npc:supported_walking") > 0 else false;
        return is_supported == want_supported;
    } else if (std.mem.eql(u8, ct, "CompareGameDifficulty")) {
        const cmp = getParamStr(obj, "CompareType") orelse "Eq";
        const req_diff = jsonInt(obj.get("Difficulty")) orelse return false;
        const cur_diff: i32 = if (ctx.progress_comp) |pc| blk: {
            const stored = pc.info.getKeyInt("game:difficulty");
            break :blk if (stored > 0) @intCast(stored) else 5; // EGameDifficultyType.Normal = 5
        } else 5;
        return compareInts(cur_diff, req_diff, cmp);
    } else if (std.mem.eql(u8, ct, "CheckPlayerSkillReady")) {
        return evalPlayerSkillReady(ctx, obj);
    } else if (std.mem.eql(u8, ct, "CheckEntityHasBuff") or std.mem.eql(u8, ct, "HasBuff")) {
        const buff_id = jsonInt(obj.get("BuffId")) orelse return false;
        const want_has_buff = if (obj.get("HasBuff")) |v| switch (v) {
            .bool => |b| b,
            else => return false,
        } else true;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Eq",
        } else "Eq";
        const scene = ctx.scene orelse return false;
        const entity_id = if (std.mem.eql(u8, ct, "CheckEntityHasBuff")) blk: {
            const target = obj.get("Target") orelse return false;
            if (target != .object) return false;
            const target_type = if (target.object.get("Type")) |v| switch (v) {
                .string => |s| s,
                else => return false,
            } else return false;
            if (!std.mem.eql(u8, target_type, "Monster")) return false;
            const config_id = jsonInt(target.object.get("MonsterEntityId")) orelse return false;
            break :blk scene.findEntityNetIdByConfigId(config_id) orelse return false;
        } else scene.currentRoleEntityId();
        const entity_index = scene.net_id_map.get(entity_id) orelse return false;
        if (entity_index >= scene.entities.len) return false;
        const has_buff = if (scene.entities.items(.buffs)[entity_index]) |buffs|
            buffs.hasBuffId(@intCast(buff_id))
        else
            false;
        if (!std.mem.eql(u8, ct, "CheckEntityHasBuff")) {
            return compareInts(if (has_buff) 1 else 0, if (want_has_buff) 1 else 0, cmp);
        }
        return has_buff == want_has_buff;
    } else if (std.mem.eql(u8, ct, "CheckGameplayTag")) {
        const tag_name = if (obj.get("GameplayTag")) |v| switch (v) {
            .string => |s| s,
            else => return false,
        } else return false;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Eq",
        } else "Eq";
        if (!std.mem.eql(u8, cmp, "Eq") and !std.mem.eql(u8, cmp, "Ne")) return false;
        const scene = ctx.scene orelse return false;
        const role_id = scene.currentRoleEntityId();
        const entity_index = scene.net_id_map.get(role_id) orelse return false;
        if (entity_index >= scene.entities.len) return false;
        const tag_component = scene.entities.items(.tag)[entity_index] orelse return false;
        const assets = ctx.assets orelse return false;
        const tag_id = gameplay_tags.idFromName(&assets.tables.gameplay_tag_remap, tag_name) catch return false;
        const has_tag = tag_component.hasTag(&assets.tables.gameplay_tag_parent, tag_id);
        return if (std.mem.eql(u8, cmp, "Eq")) has_tag else !has_tag;
    } else if (std.mem.eql(u8, ct, "CheckCertainFishingItemCount")) {
        const item_id = jsonInt(obj.get("Id")) orelse return false;
        const count = jsonInt(obj.get("Count")) orelse return false;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Ge",
        } else "Ge";
        return compareInts(itemCount(ctx, item_id), count, cmp);
    } else if (std.mem.eql(u8, ct, "CompareFishingTechLevel")) {
        const tech_id = jsonInt(obj.get("TechId")) orelse return false;
        const required_level = jsonInt(obj.get("TechLevel")) orelse return false;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => "Ge",
        } else "Ge";
        const progress = ctx.progress_comp orelse return false;
        const current_level = ProgressInfo.getIdCount(progress.info.fishing.tech_levels, tech_id);
        return compareInts(current_level, required_level, cmp);
    } else if (std.mem.eql(u8, ct, "CompleteCertainFishingEntrust")) {
        const entrust_id = jsonInt(obj.get("Id")) orelse return false;
        const required_count = jsonInt(obj.get("Count")) orelse return false;
        if (required_count <= 0) return true;
        // FishingEntrustHandInRequest stores one idempotent claim per entrust.
        // QuestNodeData only asks for Count 0 or 1; larger counts need a real
        // repeat-completion counter and must not be inferred from this claim.
        if (required_count > 1) return false;
        const progress = ctx.progress_comp orelse return false;
        var key_buf: [64]u8 = undefined;
        const key = std.fmt.bufPrint(&key_buf, "fish:entrust_done:{d}", .{entrust_id}) catch return false;
        return progress.info.hasClaim(key);
    } else if (std.mem.eql(u8, ct, "CompareFishingBoatState")) {
        const want_in_port = if (obj.get("IsStop")) |v| switch (v) {
            .bool => |b| b,
            else => return false,
        } else true;
        const progress = ctx.progress_comp orelse return false;
        // Client LevelConditionCompareFishingBoatState compares IsStop with
        // FishingShipData.IsShipInPort(); sailing state is the server's
        // persisted source for this field (FishingInfoResponse mirrors it).
        const is_in_port = progress.info.getKeyInt("fish:sailing") == 0;
        return want_in_port == is_in_port;
    } else if (std.mem.eql(u8, ct, "DungeonSubType")) {
        const subtype = jsonInt(obj.get("DungeonSubType")) orelse return false;
        const scene = ctx.scene orelse return false;
        const assets = ctx.assets orelse return false;
        const instance = assets.tables.instance_dungeon.getDataById(scene.instance_id) orelse return false;
        return instance.InstSubType == subtype;
    } else if (std.mem.eql(u8, ct, "CheckHookLockPoint")) {
        const ids = obj.get("EntityIds") orelse return false;
        if (ids != .array or ids.array.items.len == 0) return false;
        const progress = ctx.progress_comp orelse return false;
        for (ids.array.items) |value| {
            const entity_id = jsonInt(value) orelse continue;
            var key_buf: [48]u8 = undefined;
            const key = std.fmt.bufPrint(&key_buf, "hook_point:active:{d}", .{entity_id}) catch continue;
            if (progress.info.getKeyInt(key) != 0) return true;
        }
        return false;
    } else if (std.mem.eql(u8, ct, "CheckIsCharacterHoldingHands")) {
        const target = obj.get("Target") orelse return false;
        if (target != .object) return false;
        const target_type = if (target.object.get("Type")) |v| switch (v) {
            .string => |s| s,
            else => return false,
        } else return false;
        if (!std.mem.eql(u8, target_type, "Player")) return false;
        const want_holding = if (obj.get("IsHoldingHands")) |v| switch (v) {
            .bool => |b| b,
            else => return false,
        } else return false;
        const scene = ctx.scene orelse return false;
        const role_id = scene.currentRoleEntityId();
        if (role_id <= 0) return false;
        const progress = ctx.progress_comp orelse return false;
        var key_buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&key_buf, "hold_hand:{d}", .{role_id}) catch return false;
        return want_holding == (progress.info.getKeyInt(key) != 0);
    } else if (std.mem.eql(u8, ct, "CheckHonamiStoryActivateTalent")) {
        const talent_id = jsonInt(obj.get("TalentId")) orelse return false;
        const want_active = if (obj.get("IsActivated")) |v| switch (v) {
            .bool => |b| b,
            else => return false,
        } else return false;
        const progress = ctx.progress_comp orelse return false;
        var key_buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&key_buf, "honami:talent:{d}", .{talent_id}) catch return false;
        return want_active == progress.info.hasClaim(key);
    } else if (std.mem.eql(u8, ct, "CheckHonamiStoryLifeSupportLevel")) {
        const required = jsonInt(obj.get("Level")) orelse return false;
        const cmp = if (obj.get("Compare")) |v| switch (v) {
            .string => |s| s,
            else => return false,
        } else return false;
        const progress = ctx.progress_comp orelse return false;
        const current: i32 = @intCast(@max(1, progress.info.getKeyInt("honami:ls_lv")));
        return compareInts(current, required, cmp);
    } else if (std.mem.eql(u8, ct, "CheckHonamiStoryUnlockSlot")) {
        const role_slot = jsonInt(obj.get("RoleSlot")) orelse return false;
        const plugin_slot = jsonInt(obj.get("PluginSlot")) orelse return false;
        const want_unlocked = if (obj.get("IsUnlocked")) |v| switch (v) {
            .bool => |b| b,
            else => return false,
        } else return false;
        const progress = ctx.progress_comp orelse return false;
        var key_buf: [64]u8 = undefined;
        const key = std.fmt.bufPrint(&key_buf, "honami:slot:{d}:{d}", .{ role_slot, plugin_slot }) catch return false;
        return want_unlocked == progress.info.hasClaim(key);
    } else if (std.mem.eql(u8, ct, "CheckTargetAttribute")) {
        const option = obj.get("Option") orelse return false;
        if (option != .object) return false;
        const target_type = getParamStr(option.object, "Type") orelse return false;
        const target_option = getParamStr(option.object, "Option") orelse return false;
        if (!std.mem.eql(u8, target_type, "Player") or !std.mem.eql(u8, target_option, "AnyRole")) return false;
        const attributes = option.object.get("AttributeTypes") orelse return false;
        if (attributes != .array or attributes.array.items.len == 0) return false;
        const scene = ctx.scene orelse return false;
        const formation_idx = std.math.cast(usize, scene.formation_info.cur_formation) orelse return false;
        if (formation_idx >= scene.formation_info.formations.len) return false;
        const formation = scene.formation_info.formations[formation_idx];
        for (formation.roles) |maybe_role| {
            const role = maybe_role orelse continue;
            if (role.entity_id <= 0) continue;
            const entity_idx = scene.net_id_map.get(role.entity_id) orelse continue;
            if (entity_idx >= scene.entities.len) continue;
            const attr = scene.entities.items(.attribute)[entity_idx] orelse continue;
            var matches = true;
            for (attributes.array.items) |attribute_condition| {
                if (attribute_condition != .object) {
                    matches = false;
                    break;
                }
                const attribute_type = getParamStr(attribute_condition.object, "Type") orelse {
                    matches = false;
                    break;
                };
                if (!std.mem.eql(u8, attribute_type, "Health")) {
                    matches = false;
                    break;
                }
                const cmp = getParamStr(attribute_condition.object, "Compare") orelse {
                    matches = false;
                    break;
                };
                const value = attribute_condition.object.get("Value") orelse {
                    matches = false;
                    break;
                };
                const target_value: f64 = switch (value) {
                    .integer => |v| @floatFromInt(v),
                    .float => |v| v,
                    else => {
                        matches = false;
                        break;
                    },
                };
                const life_idx: usize = @intCast(@intFromEnum(pb.EAttributeType.Life));
                const max_idx: usize = @intCast(@intFromEnum(pb.EAttributeType.LifeMax));
                const current_life = attr.attributes[life_idx].current;
                const max_life = attr.attributes[max_idx].current;
                if (max_life <= 0 or !compareFloats(@as(f64, @floatFromInt(current_life)) / @as(f64, @floatFromInt(max_life)) * 100.0, target_value, cmp)) {
                    matches = false;
                    break;
                }
            }
            if (matches) return true;
        }
        return false;
    } else if (std.mem.eql(u8, ct, "CheckBvbChallengeState")) {
        const challenge_id = jsonInt(obj.get("ChallengeId")) orelse return false;
        const state = jsonInt(obj.get("ChallengeState")) orelse return false;
        // Client enum: Default=0, Failed=1, Succeed=2. QuestEngine records
        // completed BvB challenges only after successful completion.
        if (state != 2) return false;
        return containsInt(ctx.quest_comp.info.progress.completed_bvb, challenge_id);
    } else if (std.mem.eql(u8, ct, "RangeSphere")) {
        const center_val = obj.get("Center") orelse return false;
        if (center_val != .object) return false;
        const radius = jsonFloatVal(obj.get("Radius")) orelse return false;
        if (radius <= 0) return false;
        const cx = jsonFloatVal(center_val.object.get("X")) orelse return false;
        const cy = jsonFloatVal(center_val.object.get("Y")) orelse return false;
        const cz = jsonFloatVal(center_val.object.get("Z")) orelse 0;
        const p = currentPlayerPos3D(ctx) orelse return false;
        const dx = p[0] - cx;
        const dy = p[1] - cy;
        const dz = p[2] - cz;
        return dx * dx + dy * dy + dz * dz <= radius * radius;
    } else if (std.mem.eql(u8, ct, "CheckDirection")) {
        const scene = ctx.scene orelse return false;
        const dir_val = obj.get("Direction") orelse return true;
        if (dir_val != .object) return true;
        const target_yaw = jsonFloatVal(dir_val.object.get("Z")) orelse return true;
        const interval = jsonFloatVal(obj.get("AngleInterval")) orelse 60.0;
        if (interval >= 180.0) return true;
        const cur_yaw: f32 = if (scene.instance.players.len > 0) scene.instance.players[0].rotation[2] else return false;
        var diff = @mod(cur_yaw - target_yaw + 180.0, 360.0);
        if (diff < 0) diff += 360.0;
        diff = @abs(diff - 180.0);
        return diff <= interval;
    } else if (std.mem.eql(u8, ct, "CompareCalabashLevel")) {
        const a = ctx.assets orelse return false;
        const req = jsonInt(obj.get("CalabashLevel")) orelse jsonInt(obj.get("Level")) orelse 1;
        const cmp = getParamStr(obj, "Compare") orelse "Ge";
        return compareInts(currentCalabashLevel(a, ctx.basic_comp, ctx.echo_comp), req, cmp);
    } else if (std.mem.eql(u8, ct, "CheckLordGymFinish")) {
        const gym_id = jsonInt(obj.get("LordGymId")) orelse return false;
        const pc = ctx.progress_comp orelse return false;
        return pc.info.hasPassedLordGym(gym_id);
    } else if (std.mem.eql(u8, ct, "CheckCalabashDevelopReward")) {
        const monster_id = jsonInt(obj.get("MonsterId")) orelse return false;
        const need_dev = jsonInt(obj.get("Develop")) orelse 1;
        if (ctx.echo_comp.calabash_info.highestQualityForMonster(monster_id) >= need_dev) return true;
        const pc = ctx.progress_comp orelse return false;
        var kbuf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&kbuf, "calabash:dev:{d}", .{monster_id}) catch return false;
        return pc.info.getKeyInt(key) >= need_dev;
    } else if (std.mem.eql(u8, ct, "CheckSubLevelState")) {
        const check_list = obj.get("CheckList") orelse return false;
        if (check_list != .array or check_list.array.items.len == 0) return true;
        const req_count = jsonInt(obj.get("ConditionCount")) orelse 0;
        var matched: i32 = 0;
        for (check_list.array.items) |entry| {
            if (entry != .object) continue;
            const sname = getParamStr(entry.object, "SubLevelName") orelse continue;
            const want_state = getParamStr(entry.object, "SubLevelState") orelse "Enable";
            var is_loaded = false;
            for (ctx.quest_comp.info.world.loaded_sublevels) |lv| {
                if (std.mem.eql(u8, lv, sname)) {
                    is_loaded = true;
                    break;
                }
            }
            const want_loaded = std.mem.eql(u8, want_state, "Enable");
            if (is_loaded == want_loaded) matched += 1;
        }
        return if (req_count <= 0) matched == @as(i32, @intCast(check_list.array.items.len)) else matched >= req_count;
    } else if (std.mem.eql(u8, ct, "CheckPlayerIsInMotorcycleCruise")) {
        const want = if (obj.get("IsInMotorcycleCruise")) |v| (if (v == .bool) v.bool else true) else true;
        const pc = ctx.progress_comp orelse return !want;
        const cruising = pc.info.getKeyInt("motor:cruise") > 0;
        return cruising == want;
    } else if (std.mem.eql(u8, ct, "CheckSyncLevelPlayCompleteNumber")) {
        const sync_id = jsonInt(obj.get("SyncId")) orelse return false;
        const num = jsonInt(obj.get("Number")) orelse 0;
        const cmp = getParamStr(obj, "Compare") orelse "Eq";
        const pc = ctx.progress_comp orelse return compareInts(0, num, cmp);
        var kbuf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&kbuf, "sync_lp:{d}", .{sync_id}) catch return false;
        const cur: i32 = @intCast(std.math.clamp(pc.info.getKeyInt(key), 0, std.math.maxInt(i32)));
        return compareInts(cur, num, cmp);
    } else if (std.mem.eql(u8, ct, "CheckCrossSceneSyncKillMonsterFinished")) {
        const sync_id = jsonInt(obj.get("SyncId")) orelse return false;
        const bundle_id = jsonInt(obj.get("EntityBundleId")) orelse 0;
        const cmp = getParamStr(obj, "Compare") orelse "Eq";
        var cur: i32 = 0;
        if (ctx.progress_comp) |pc| {
            var kbuf: [64]u8 = undefined;
            const key = std.fmt.bufPrint(&kbuf, "cross_kill:{d}:{d}", .{ sync_id, bundle_id }) catch "";
            if (key.len > 0 and (pc.info.hasClaim(key) or pc.info.getKeyInt(key) > 0)) cur = 1;
        }
        return compareInts(cur, 1, cmp);
    } else if (std.mem.eql(u8, ct, "CheckGiantRegionCultivationTriggeredUpgradeLevel")) {
        const eco_type = getParamStr(obj, "EcoType") orelse return false;
        const eco_id: i32 = if (std.mem.eql(u8, eco_type, "Fire")) 0 else if (std.mem.eql(u8, eco_type, "Water")) 1 else if (std.mem.eql(u8, eco_type, "Wood")) 2 else return false;
        const pc = ctx.progress_comp orelse return false;
        var kbuf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&kbuf, "giant:upgrade:{d}", .{eco_id}) catch return false;
        const cur: i32 = @intCast(std.math.clamp(pc.info.getKeyInt(key), 0, std.math.maxInt(i32)));
        const levels = obj.get("Levels") orelse return cur > 0;
        if (levels != .array) return cur > 0;
        for (levels.array.items) |lv| {
            if ((jsonInt(lv) orelse -1) == cur) return true;
        }
        return false;
    } else if (std.mem.eql(u8, ct, "CheckTetrisEndlessScore")) {
        const did = jsonInt(obj.get("DungeonId")) orelse 0;
        const target = jsonInt(obj.get("TargetScore")) orelse 0;
        const cmp = getParamStr(obj, "Compare") orelse "Ge";
        var score: i32 = 0;
        if (ctx.progress_comp) |pc| {
            var kbuf: [48]u8 = undefined;
            const key = std.fmt.bufPrint(&kbuf, "tetris:score:{d}", .{did}) catch "";
            if (key.len > 0) score = @intCast(std.math.clamp(pc.info.getKeyInt(key), 0, std.math.maxInt(i32)));
        }
        return compareInts(score, target, cmp);
    } else if (std.mem.eql(u8, ct, "CheckDollGrabShowcaseItemComboAchieved")) {
        const eid = jsonInt(obj.get("ShowcaseEntityId")) orelse 0;
        const count = jsonInt(obj.get("Count")) orelse 1;
        const cmp = getParamStr(obj, "Compare") orelse "Ge";
        var cur: i32 = 0;
        if (ctx.progress_comp) |pc| {
            var kbuf: [48]u8 = undefined;
            const key = std.fmt.bufPrint(&kbuf, "doll:combo:{d}", .{eid}) catch "";
            if (key.len > 0) cur = @intCast(std.math.clamp(pc.info.getKeyInt(key), 0, std.math.maxInt(i32)));
        }
        return compareInts(cur, count, cmp);
    }
    return false;
}

threadlocal var quest_state_eval_depth: u8 = 0;

fn questStateValue(ctx: EvalContext, qid: i32) i32 {
    if (ctx.quest_comp.info.hasFinishedQuest(qid)) return 3;
    if (ctx.quest_comp.info.hasActiveQuest(qid)) return 2;
    if (quest_state_eval_depth < 4) {
        if (ctx.assets) |a| {
            if (a.tables.quest_data.getDataById(qid)) |q| {
                quest_state_eval_depth += 1;
                defer quest_state_eval_depth -= 1;
                if (evalQuestProvideConditions(ctx, q.Data.ProvideType)) return 1;
            }
        }
    }
    return 0;
}

fn evalInlineNodeStatus(ctx: EvalContext, qid: i32, nid: i32, want_st: i32) bool {
    if (want_st == 4) {
        if (qid > 0) {
            return ctx.quest_comp.info.hasFinishedChildQuest(qid, nid);
        }
        for (ctx.quest_comp.info.finished_child_quests) |rec| {
            if (rec.node_id == nid) return true;
        }
        for (ctx.quest_comp.info.active_quests) |aq| {
            if (QuestInfo.isNodeFinished(aq, nid)) return true;
        }
        return false;
    } else if (want_st == 1 or want_st == 2) {
        if (qid > 0) {
            return if (ctx.quest_comp.info.findActiveQuest(qid)) |aq| QuestInfo.isNodeActive(aq, nid) else false;
        }
        for (ctx.quest_comp.info.active_quests) |aq| {
            if (QuestInfo.isNodeActive(aq, nid)) return true;
        }
        return false;
    } else if (want_st == 0) {
        if (qid > 0) {
            if (ctx.quest_comp.info.hasFinishedChildQuest(qid, nid)) return false;
            if (ctx.quest_comp.info.findActiveQuest(qid)) |aq| {
                return !QuestInfo.isNodeActive(aq, nid) and !QuestInfo.isNodeFinished(aq, nid);
            }
            return true;
        }
        for (ctx.quest_comp.info.finished_child_quests) |rec| {
            if (rec.node_id == nid) return false;
        }
        for (ctx.quest_comp.info.active_quests) |aq| {
            if (QuestInfo.isNodeActive(aq, nid) or QuestInfo.isNodeFinished(aq, nid)) return false;
        }
        return true;
    }
    return false;
}

fn evalCheckEntitiesExist(ctx: EvalContext, obj: std.json.ObjectMap) bool {
    const scene = ctx.scene orelse return false;
    const ids_val = obj.get("EntityIds") orelse return false;
    if (ids_val != .array or ids_val.array.items.len == 0) return false;
    const slice = scene.entities.slice();
    const configs = slice.items(.config);
    const visibles = slice.items(.visible);
    for (ids_val.array.items) |id_val| {
        const eid = jsonInt(id_val) orelse continue;
        var found = false;
        for (configs, visibles) |cfg, vis_opt| {
            if (cfg.config_id == eid and vis_opt != null) {
                found = true;
                break;
            }
        }
        if (!found) return false;
    }
    return true;
}

pub fn evalPlayerMotionState(ctx: EvalContext, motion: []const u8) ?bool {
    const scene = ctx.scene orelse return null;
    const role_entity_id = scene.currentRoleEntityId();
    if (role_entity_id <= 0) return null;
    const entity_index = scene.net_id_map.get(role_entity_id) orelse return null;
    if (entity_index >= scene.entities.len) return null;
    const logic_state = scene.entities.items(.logic_state)[entity_index] orelse return null;

    const position_state = logic_state.position_state;
    const move_state = logic_state.move_state;
    const position_sub_state = logic_state.position_sub_state;
    if (std.mem.eql(u8, motion, "Ground")) return position_state == 0 and position_sub_state == 0;
    if (std.mem.eql(u8, motion, "Air")) return position_state == 2;
    if (std.mem.eql(u8, motion, "Climb")) return position_state == 1;
    if (std.mem.eql(u8, motion, "OnClimbing")) {
        return position_state == 1 and (move_state == 0 or move_state == 14 or move_state == 15);
    }
    if (std.mem.eql(u8, motion, "Water")) return position_state == 3;
    if (std.mem.eql(u8, motion, "WalkOnWater")) return position_state == 0 and position_sub_state == 1;
    if (std.mem.eql(u8, motion, "GlideInAir")) return position_state == 2 and move_state == 16;
    if (std.mem.eql(u8, motion, "FallInAir")) return position_state == 2 and move_state == 0;
    if (std.mem.eql(u8, motion, "Soaring")) return position_state == 2 and move_state == 28;
    if (std.mem.eql(u8, motion, "Gongduola")) return position_state == 5 and move_state == 31;

    const tag_name: []const u8 = if (std.mem.eql(u8, motion, "SwitchSkill"))
        "角色.Common.技能通用标识.QTE"
    else if (std.mem.eql(u8, motion, "UltimateDodge"))
        "角色.Common.技能通用标识.极限闪避"
    else if (std.mem.eql(u8, motion, "UltimateSkill"))
        "角色.Common.技能通用标识.大招"
    else
        return null;
    const assets = ctx.assets orelse return null;
    const tag_id = gameplay_tags.idFromName(&assets.tables.gameplay_tag_remap, tag_name) catch return null;
    const tag_component = scene.entities.items(.tag)[entity_index] orelse return null;
    return tag_component.hasTag(&assets.tables.gameplay_tag_parent, tag_id);
}

fn jsonFloatVal(v_opt: ?std.json.Value) ?f32 {
    const v = v_opt orelse return null;
    return switch (v) {
        .float => |f| @floatCast(f),
        .integer => |i| @floatFromInt(i),
        else => null,
    };
}

pub fn currentPlayerPos3D(ctx: EvalContext) ?[3]f32 {
    const scene = ctx.scene orelse return null;
    if (scene.instance.players.len > 0) {
        return scene.instance.players[0].location;
    }
    return scene.last_spatial_check_loc;
}

fn rotateInverseEuler(v: [3]f32, rot_deg: [3]f32) [3]f32 {
    if (rot_deg[0] == 0 and rot_deg[1] == 0 and rot_deg[2] == 0) return v;
    const deg2rad: f32 = std.math.pi / 180.0;
    const roll = rot_deg[0] * deg2rad;
    const pitch = rot_deg[1] * deg2rad;
    const yaw = rot_deg[2] * deg2rad;

    // Inverse of UE Z-Y-X rotation: R_x(-roll) * R_y(-pitch) * R_z(-yaw)
    const cy = @cos(yaw);
    const sy = @sin(yaw);
    const x1 = v[0] * cy + v[1] * sy;
    const y1 = -v[0] * sy + v[1] * cy;
    const z1 = v[2];

    const cp = @cos(pitch);
    const sp = @sin(pitch);
    const x2 = x1 * cp + z1 * sp;
    const y2 = y1;
    const z2 = -x1 * sp + z1 * cp;

    const cr = @cos(roll);
    const sr = @sin(roll);
    return .{
        x2,
        y2 * cr - z2 * sr,
        y2 * sr + z2 * cr,
    };
}

/// Evaluates oriented 3D containment for RangeComponent shapes (`Sphere`, `HollowSphere`,
/// `Cylinder`, `HollowCylinder`, `Box`, `Combination`, and fallback spherical/box shapes).
pub fn isPosInsideRangeShape(
    pos: [3]f32,
    base_pos: [3]f32,
    entity_rot: [3]f32,
    shape: Range.ShapeConfig,
    default_radius: f32,
    min_sphere_radius: f32,
) bool {
    return isPosInsideRangeShapeDepth(pos, base_pos, entity_rot, shape, default_radius, min_sphere_radius, 0);
}

fn isPosInsideRangeShapeDepth(
    pos: [3]f32,
    base_pos: [3]f32,
    entity_rot: [3]f32,
    shape: Range.ShapeConfig,
    default_radius: f32,
    min_sphere_radius: f32,
    depth: u8,
) bool {
    const stype = shape.Type orelse "";
    if (std.mem.eql(u8, stype, "Combination") or (shape.CombinationShapes != null and stype.len == 0)) {
        if (depth >= 4) return false;
        const subs = shape.CombinationShapes orelse return false;
        for (subs) |sub| {
            if (isPosInsideRangeShapeDepth(pos, base_pos, entity_rot, sub, default_radius, min_sphere_radius, depth + 1)) {
                return true;
            }
        }
        return false;
    }

    const center_off: [3]f32 = if (shape.Center) |c|
        .{ c.X orelse 0, c.Y orelse 0, c.Z orelse 0 }
    else
        .{ 0, 0, 0 };

    const rel: [3]f32 = .{
        pos[0] - base_pos[0],
        pos[1] - base_pos[1],
        pos[2] - base_pos[2],
    };

    const shape_rot: [3]f32 = if (shape.Rotator) |sr|
        .{ sr.X orelse 0, sr.Y orelse 0, sr.Z orelse 0 }
    else
        .{ 0, 0, 0 };

    // Transform world offset into entity-local frame, subtract shape center, then apply inverse shape 3D Euler rotation.
    const ent_local = rotateInverseEuler(rel, entity_rot);
    const shifted: [3]f32 = .{
        ent_local[0] - center_off[0],
        ent_local[1] - center_off[1],
        ent_local[2] - center_off[2],
    };
    const local = rotateInverseEuler(shifted, shape_rot);
    const dx = local[0];
    const dy = local[1];
    const dz = local[2];
    const has_box_size = shape.Size != null and ((shape.Size.?.X orelse 0) > 0 or (shape.Size.?.Y orelse 0) > 0 or (shape.Size.?.Z orelse 0) > 0);
    if (std.mem.eql(u8, stype, "Box") or (stype.len == 0 and has_box_size)) {
        const sz = shape.Size orelse return false;
        const sx = @max(sz.X orelse 0, 100.0);
        const sy = @max(sz.Y orelse 0, 100.0);
        const sz_v = @max(sz.Z orelse 0, 100.0);
        return @abs(dx) <= sx and @abs(dy) <= sy and @abs(dz) <= sz_v;
    }

    const r = @max(shape.Radius orelse default_radius, min_sphere_radius);
    if (std.mem.eql(u8, stype, "Cylinder") or std.mem.eql(u8, stype, "HollowCylinder")) {
        const xy_sq = dx * dx + dy * dy;
        const half_h: f32 = if (shape.Height) |h| (if (h > 0) h * 0.5 else @max(r, 1500.0)) else @max(r, 1500.0);
        if (xy_sq > r * r or @abs(dz) > half_h) return false;
        if (std.mem.eql(u8, stype, "HollowCylinder")) {
            const inner_r = shape.InnerRadius orelse 0;
            if (inner_r > 0 and xy_sq < inner_r * inner_r) return false;
        }
        return true;
    }

    const dist_sq = dx * dx + dy * dy + dz * dz;
    if (dist_sq > r * r) return false;
    if (std.mem.eql(u8, stype, "HollowSphere")) {
        const inner_r = shape.InnerRadius orelse 0;
        if (inner_r > 0 and dist_sq < inner_r * inner_r) return false;
    }
    return true;
}

fn evalCheckInRange(ctx: EvalContext, obj: std.json.ObjectMap) bool {
    const scene = ctx.scene orelse return false;
    const range_entities = obj.get("RangeEntities") orelse return false;
    if (range_entities != .array or range_entities.array.items.len == 0) return false;

    const player_pos = currentPlayerPos3D(ctx) orelse return false;
    const slice = scene.entities.slice();
    const configs = slice.items(.config);
    const positions = slice.items(.position);

    for (range_entities.array.items) |ent_val| {
        const eid = jsonInt(ent_val) orelse continue;
        var ent_pos: ?[3]f32 = null;
        var ent_rot: [3]f32 = .{ 0, 0, 0 };
        for (configs, positions) |cfg, pos_opt| {
            if (cfg.config_id == eid) {
                if (pos_opt) |p| {
                    ent_pos = p.location;
                    ent_rot = p.rotation;
                    break;
                }
            }
        }
        var shape: Range.ShapeConfig = .{};
        if (ctx.assets) |a| {
            if (a.findLevelEntity(scene.instance_id, eid)) |lcfg| {
                if (ent_pos == null) {
                    ent_pos = Assets.AssetIndexes.entityWorldPos(lcfg);
                    ent_rot = Assets.AssetIndexes.entityWorldRot(lcfg);
                }
                const maybe_rc = lcfg.ComponentsData.RangeComponent orelse
                    if (a.findTemplateConfig(lcfg.BlueprintType)) |tpl| tpl.ComponentsData.RangeComponent else null;
                if (maybe_rc) |rc| {
                    if (rc.Shape) |sh| shape = sh;
                }
            }
        }
        const pos = ent_pos orelse continue;
        if (isPosInsideRangeShape(player_pos, pos, ent_rot, shape, 1500.0, 0.0)) {
            return true;
        }
    }
    return false;
}

fn evalQuestState(ctx: EvalContext, qid: i32, st: i32) bool {
    return questStateValue(ctx, qid) == st;
}

fn evalQuestNodeStatus(ctx: EvalContext, qid: i32, nid: i32, st: i32) bool {
    const aq = ctx.quest_comp.info.findActiveQuest(qid);
    return switch (st) {
        0 => if (aq) |q| !QuestInfo.isNodeActive(q, nid) and !QuestInfo.isNodeFinished(q, nid) else !ctx.quest_comp.info.hasFinishedChildQuest(qid, nid),
        1, 2 => if (aq) |q| QuestInfo.isNodeActive(q, nid) else false,
        3 => ctx.quest_comp.info.hasFinishedChildQuest(qid, nid),
        else => false,
    };
}

pub fn isActivityOpen(ctx: EvalContext, activity_id: i32) bool {
    if (containsInt(ctx.quest_comp.info.progress.opened_activities, activity_id)) return true;
    const a = ctx.assets orelse return false;
    return a.tables.activity.getDataById(activity_id) != null;
}

fn scopedVarName(var_obj: std.json.ObjectMap, name: []const u8, buf: []u8) []const u8 {
    const src = getParamStr(var_obj, "Source") orelse "Constant";
    if (std.mem.eql(u8, src, "Other")) {
        const ref_type = getParamStr(var_obj, "RefType") orelse "";
        const ref_id = jsonInt(var_obj.get("RefId")) orelse 0;
        if (ref_type.len > 0 and ref_id != 0) {
            return std.fmt.bufPrint(buf, "{s}:{d}:{s}", .{ ref_type, ref_id, name }) catch name;
        }
    }
    return name;
}

fn lookupEntityConfigVar(ctx: EvalContext, obj: std.json.ObjectMap, name: []const u8) ?std.json.Value {
    const src = getParamStr(obj, "Source") orelse return null;
    if (!std.mem.eql(u8, src, "Other")) return null;
    const ref_type = getParamStr(obj, "RefType") orelse return null;
    if (!std.mem.eql(u8, ref_type, "Entity")) return null;
    const ref_id: i64 = if (obj.get("RefId")) |rv| switch (rv) {
        .integer => |i| i,
        .float => |f| @intFromFloat(f),
        else => return null,
    } else return null;
    const assets = ctx.assets orelse return null;
    const map_id: i32 = if (ctx.scene) |scene| assets.resolveMapConfigId(scene.instance_id) else 8;
    const cfg = assets.findLevelEntity(map_id, ref_id) orelse return null;
    const vc = cfg.ComponentsData.VarComponent orelse return null;
    if (vc.Disabled orelse false) return null;
    for (vc.Vars) |v| {
        if (std.mem.eql(u8, v.Name, name)) return v.Value;
    }
    return null;
}

fn resolveVarStr(ctx: EvalContext, var_opt: ?std.json.Value) ?[]const u8 {
    const v = var_opt orelse return null;
    if (v != .object) return null;
    const src = getParamStr(v.object, "Source") orelse "Constant";
    if (std.mem.eql(u8, src, "Constant")) {
        const val = v.object.get("Value") orelse return "";
        return switch (val) {
            .string => |s| s,
            else => null,
        };
    }
    const name = getParamStr(v.object, "Name") orelse return null;
    var buf: [128]u8 = undefined;
    const scoped = scopedVarName(v.object, name, &buf);
    const val = ctx.quest_comp.info.getVarStr(scoped);
    if (val.len > 0) return val;
    if (lookupEntityConfigVar(ctx, v.object, name)) |init_val| {
        if (init_val == .string) return init_val.string;
    }
    if (std.mem.eql(u8, scoped, name)) return val;
    return ctx.quest_comp.info.getVarStr(name);
}

fn resolveVarFloat(ctx: EvalContext, var_opt: ?std.json.Value) f64 {
    const v = var_opt orelse return 0.0;
    if (v != .object) return 0.0;
    const src = getParamStr(v.object, "Source") orelse "Constant";
    if (std.mem.eql(u8, src, "Constant")) {
        const val = v.object.get("Value") orelse return 0.0;
        return switch (val) {
            .float => |f| f,
            .integer => |i| @floatFromInt(i),
            .bool => |b| if (b) 1.0 else 0.0,
            else => 0.0,
        };
    }
    const name = getParamStr(v.object, "Name") orelse return 0.0;
    var buf: [128]u8 = undefined;
    const scoped = scopedVarName(v.object, name, &buf);
    const s = ctx.quest_comp.info.getVarStr(scoped);
    if (s.len > 0) {
        if (std.fmt.parseFloat(f64, s)) |f| return f else |_| {}
    }
    const iv = ctx.quest_comp.info.getVarInt(scoped);
    if (iv != 0) return @floatFromInt(iv);
    if (lookupEntityConfigVar(ctx, v.object, name)) |init_val| {
        return switch (init_val) {
            .float => |f| f,
            .integer => |i| @floatFromInt(i),
            .bool => |b| if (b) 1.0 else 0.0,
            else => 0.0,
        };
    }
    if (std.mem.eql(u8, scoped, name)) return 0.0;
    return @floatFromInt(ctx.quest_comp.info.getVarInt(name));
}

fn resolveVarValue(ctx: EvalContext, var_opt: ?std.json.Value) i32 {
    const v = var_opt orelse return 0;
    if (v != .object) return 0;
    const src = if (v.object.get("Source")) |s| switch (s) {
        .string => |str| str,
        else => "Constant",
    } else "Constant";

    if (std.mem.eql(u8, src, "Constant")) {
        if (v.object.get("Value")) |val| {
            return switch (val) {
                .integer => |i| std.math.cast(i32, i) orelse 0,
                .bool => |b| if (b) 1 else 0,
                else => 0,
            };
        }
        return 0;
    }
    if (v.object.get("Name")) |name_v| {
        if (name_v == .string) {
            var buf: [128]u8 = undefined;
            const scoped = scopedVarName(v.object, name_v.string, &buf);
            if (getParamStr(v.object, "Type")) |vt| {
                if (std.mem.eql(u8, vt, "Boolean") or std.mem.eql(u8, vt, "Bool")) {
                    if (ctx.quest_comp.info.getVarBool(scoped)) return 1;
                    if (!std.mem.eql(u8, scoped, name_v.string) and ctx.quest_comp.info.getVarBool(name_v.string)) return 1;
                }
            }
            const val = ctx.quest_comp.info.getVarInt(scoped);
            if (val != 0) return val;
            if (lookupEntityConfigVar(ctx, v.object, name_v.string)) |init_val| {
                return switch (init_val) {
                    .integer => |i| std.math.cast(i32, i) orelse 0,
                    .bool => |b| if (b) 1 else 0,
                    else => 0,
                };
            }
            if (std.mem.eql(u8, scoped, name_v.string)) return val;
            return ctx.quest_comp.info.getVarInt(name_v.string);
        }
    }
    return 0;
}

pub fn evalCompareVarCondition(ctx: EvalContext, obj: std.json.ObjectMap) bool {
    const cmp = getParamStr(obj, "Compare") orelse "Eq";
    const v1_opt = obj.get("Var1");
    const v2_opt = obj.get("Var2");
    const t1 = if (v1_opt) |v| (if (v == .object) getParamStr(v.object, "Type") else null) else null;
    const t2 = if (v2_opt) |v| (if (v == .object) getParamStr(v.object, "Type") else null) else null;
    const is_str = (t1 != null and std.mem.eql(u8, t1.?, "String")) or (t2 != null and std.mem.eql(u8, t2.?, "String"));
    if (is_str) {
        const s1 = resolveVarStr(ctx, v1_opt) orelse return false;
        const s2 = resolveVarStr(ctx, v2_opt) orelse return false;
        const eq = std.mem.eql(u8, s1, s2);
        if (std.mem.eql(u8, cmp, "Ne") or std.mem.eql(u8, cmp, "NotEq") or std.mem.eql(u8, cmp, "!=")) return !eq;
        if (std.mem.eql(u8, cmp, "Eq") or std.mem.eql(u8, cmp, "==")) return eq;
        return false;
    }
    const is_float = (t1 != null and std.mem.eql(u8, t1.?, "Float")) or (t2 != null and std.mem.eql(u8, t2.?, "Float"));
    if (is_float) {
        return compareFloats(resolveVarFloat(ctx, v1_opt), resolveVarFloat(ctx, v2_opt), cmp);
    }
    return compareInts(resolveVarValue(ctx, v1_opt), resolveVarValue(ctx, v2_opt), cmp);
}

fn compareInts(a: i32, b: i32, cmp: []const u8) bool {
    if (std.mem.eql(u8, cmp, "Eq") or std.mem.eql(u8, cmp, "==")) return a == b;
    if (std.mem.eql(u8, cmp, "Ne") or std.mem.eql(u8, cmp, "NotEq") or std.mem.eql(u8, cmp, "!=")) return a != b;
    if (std.mem.eql(u8, cmp, "Gt") or std.mem.eql(u8, cmp, "Great") or std.mem.eql(u8, cmp, ">")) return a > b;
    if (std.mem.eql(u8, cmp, "Ge") or std.mem.eql(u8, cmp, "GtEq") or std.mem.eql(u8, cmp, "GreatEq") or std.mem.eql(u8, cmp, ">=")) return a >= b;
    if (std.mem.eql(u8, cmp, "Lt") or std.mem.eql(u8, cmp, "Less") or std.mem.eql(u8, cmp, "<")) return a < b;
    if (std.mem.eql(u8, cmp, "Le") or std.mem.eql(u8, cmp, "LtEq") or std.mem.eql(u8, cmp, "LessEq") or std.mem.eql(u8, cmp, "<=")) return a <= b;
    return a == b;
}

fn compareFloats(a: f64, b: f64, cmp: []const u8) bool {
    if (std.mem.eql(u8, cmp, "Eq") or std.mem.eql(u8, cmp, "==")) return a == b;
    if (std.mem.eql(u8, cmp, "Ne") or std.mem.eql(u8, cmp, "NotEq") or std.mem.eql(u8, cmp, "!=")) return a != b;
    if (std.mem.eql(u8, cmp, "Gt") or std.mem.eql(u8, cmp, "Great") or std.mem.eql(u8, cmp, ">")) return a > b;
    if (std.mem.eql(u8, cmp, "Ge") or std.mem.eql(u8, cmp, "GtEq") or std.mem.eql(u8, cmp, "GreatEq") or std.mem.eql(u8, cmp, ">=")) return a >= b;
    if (std.mem.eql(u8, cmp, "Lt") or std.mem.eql(u8, cmp, "Less") or std.mem.eql(u8, cmp, "<")) return a < b;
    if (std.mem.eql(u8, cmp, "Le") or std.mem.eql(u8, cmp, "LtEq") or std.mem.eql(u8, cmp, "LessEq") or std.mem.eql(u8, cmp, "<=")) return a <= b;
    return false;
}

pub fn currentCalabashLevel(assets: *const Assets, basic_comp: *const PlayerBasicComponent, echo_comp: *const PlayerEchoComponent) i32 {
    return CalabashHelper.currentLevel(assets, basic_comp, echo_comp);
}

fn itemCount(ctx: EvalContext, item_id: i32) i32 {
    if (item_id == 2) return ctx.basic_comp.info.attributes.coin;
    if (item_id == 3) return @max(ctx.basic_comp.info.attributes.rare_coin, ctx.inventory_comp.info.normalItemCount(3));
    return ctx.inventory_comp.info.normalItemCount(item_id);
}

fn fishingPrestigeLevel(assets: *const Assets, exp: i32) i32 {
    var level: i32 = 1;
    for (assets.tables.fishing_reputation.items) |row| {
        if (exp >= row.Exp) level = @max(level, row.Level);
    }
    return level;
}

fn countDangoAboveLevel(assets: *const Assets, info: ProgressInfo, target_level: i32) i32 {
    const prefix = "abyss:lrole:";
    var count: i32 = 0;
    for (info.dynamic.ints) |entry| {
        if (!std.mem.startsWith(u8, entry.key, prefix)) continue;
        const id_text = entry.key[prefix.len..];
        const dango_id = std.fmt.parseInt(i32, id_text, 10) catch continue;
        if (assets.tables.abyss_little_role.getDataById(dango_id) == null) continue;
        if (entry.val > target_level) count += 1;
    }
    return count;
}

fn jsonHasType(obj: std.json.ObjectMap, expected: []const u8) bool {
    const type_value = obj.get("Type") orelse return false;
    return type_value == .string and std.mem.eql(u8, type_value.string, expected);
}

fn currentFormationRoleId(ctx: EvalContext, index: i32) ?i32 {
    if (index < 0) return null;
    const scene = ctx.scene orelse return null;
    const formation_index = std.math.cast(usize, scene.formation_info.cur_formation) orelse return null;
    if (formation_index >= scene.formation_info.formations.len) return null;
    const slot = std.math.cast(usize, index) orelse return null;
    const formation = scene.formation_info.formations[formation_index];
    if (slot >= formation.roles.len) return null;
    const role = formation.roles[slot] orelse return null;
    return role.role_id;
}

fn evalEntityGravityDirection(ctx: EvalContext, obj: std.json.ObjectMap) bool {
    // QuestNodeData uses the online host as the target and one of the six
    // world axes as the expected direction. This reads the server entity's
    // configured gravity; runtime gravity changes made only by the client
    // are not mirrored into the server scene yet.
    const target = obj.get("Target") orelse return false;
    if (target != .object or !jsonHasType(target.object, "OnlinePlayer")) return false;
    const target_option = target.object.get("OnlinePlayerConditionTargetOption") orelse return false;
    if (target_option != .object or !jsonHasType(target_option.object, "Host")) return false;

    const direction = obj.get("GravityDirection") orelse return false;
    if (direction != .object or !jsonHasType(direction.object, "WorldAxis")) return false;
    const axis = getParamStr(direction.object, "WorldAxis") orelse return false;
    const expected: [3]f32 = if (std.mem.eql(u8, axis, "PositiveX"))
        .{ 1, 0, 0 }
    else if (std.mem.eql(u8, axis, "NegativeX"))
        .{ -1, 0, 0 }
    else if (std.mem.eql(u8, axis, "PositiveY"))
        .{ 0, 1, 0 }
    else if (std.mem.eql(u8, axis, "NegativeY"))
        .{ 0, -1, 0 }
    else if (std.mem.eql(u8, axis, "PositiveZ"))
        .{ 0, 0, 1 }
    else if (std.mem.eql(u8, axis, "NegativeZ"))
        .{ 0, 0, -1 }
    else
        return false;

    const scene = ctx.scene orelse return false;
    const entity_id = scene.currentRoleEntityId();
    if (entity_id <= 0) return false;
    const entity_index = scene.net_id_map.get(entity_id) orelse return false;
    if (entity_index >= scene.entities.len) return false;
    // Client gravity components default to world down when no custom gravity
    // is configured; EntityPb serialization uses the same default.
    const actual: [3]f32 = if (scene.entities.items(.world_item)[entity_index].gravity) |gravity|
        gravity.location
    else
        .{ 0, 0, -1 };
    var matches = true;
    for (actual, expected) |actual_component, expected_component| {
        const rounded_actual = @round(actual_component * 1000.0) / 1000.0;
        const rounded_expected = @round(expected_component * 1000.0) / 1000.0;
        if (@abs(rounded_actual - rounded_expected) > 0.001) {
            matches = false;
            break;
        }
    }
    const invert = if (obj.get("IsNotInTargetDirection")) |value| switch (value) {
        .bool => |enabled| enabled,
        else => return false,
    } else false;
    return if (invert) !matches else matches;
}

fn evalFormationRoleInfo(ctx: EvalContext, obj: std.json.ObjectMap) bool {
    const option = obj.get("Option") orelse return false;
    if (option != .object) return false;
    const option_type = getParamStr(option.object, "Type") orelse return false;
    if (std.mem.eql(u8, option_type, "RoleLevel") or std.mem.eql(u8, option_type, "WeaponLevel")) {
        const target = option.object.get("Option") orelse return false;
        if (target != .object or !jsonHasType(target.object, "SpecifyRole")) return false;
        const slot = jsonInt(target.object.get("Index")) orelse return false;
        const required = jsonInt(target.object.get("Level")) orelse return false;
        const compare = getParamStr(target.object, "Compare") orelse "Ge";
        const role_id = currentFormationRoleId(ctx, slot) orelse return false;
        if (std.mem.eql(u8, option_type, "RoleLevel")) {
            const role_comp = ctx.role_comp orelse return false;
            const role = role_comp.role_map.get(role_id) orelse return false;
            return compareInts(role.level, required, compare);
        }
        const role_comp = ctx.role_comp orelse return false;
        const weapon_comp = ctx.weapon_comp orelse return false;
        const role = role_comp.role_map.get(role_id) orelse return false;
        const weapon = weapon_comp.weapon_map.get(role.weapon) orelse return false;
        return compareInts(weapon.level, required, compare);
    }
    if (std.mem.eql(u8, option_type, "HasEquippedVision")) {
        const target = option.object.get("Option") orelse return false;
        if (target != .string or !std.mem.eql(u8, target.string, "AnyRole")) return false;
        const scene = ctx.scene orelse return false;
        const formation_index = std.math.cast(usize, scene.formation_info.cur_formation) orelse return false;
        if (formation_index >= scene.formation_info.formations.len) return false;
        const formation = scene.formation_info.formations[formation_index];
        for (formation.roles) |maybe_role| {
            const role = maybe_role orelse continue;
            var echoes = ctx.echo_comp.echo_map.iterator();
            while (echoes.next()) |entry| {
                if (entry.value_ptr.role_id != null and entry.value_ptr.role_id.? == role.role_id and entry.value_ptr.equipped_pos >= 0) return true;
            }
        }
        return false;
    }
    if (std.mem.eql(u8, option_type, "HasUpgradableVision")) {
        const target = option.object.get("Option") orelse return false;
        if (target != .string or !std.mem.eql(u8, target.string, "AnyRole")) return false;
        const scene = ctx.scene orelse return false;
        const assets = ctx.assets orelse return false;
        const formation_index = std.math.cast(usize, scene.formation_info.cur_formation) orelse return false;
        if (formation_index >= scene.formation_info.formations.len) return false;
        const formation = scene.formation_info.formations[formation_index];
        for (formation.roles) |maybe_role| {
            const role = maybe_role orelse continue;
            var echoes = ctx.echo_comp.echo_map.iterator();
            while (echoes.next()) |entry| {
                const echo = entry.value_ptr.*;
                if (echo.role_id == null or echo.role_id.? != role.role_id or echo.equipped_pos < 0) continue;
                const item = assets.tables.phantom_item.getDataById(echo.id) orelse continue;
                const quality = assets.tables.phantom_quality.getDataById(item.QualityId) orelse continue;
                if (echo.level < quality.LevelLimit) return true;
            }
        }
        return false;
    }
    // FeatureCollection is a separate client feature system. The server has no
    // authoritative progress source for its collection level yet.
    return false;
}

fn isSunSpiritCollected(ctx: EvalContext, eid: i64) bool {
    if (std.math.cast(i32, eid)) |eid32| {
        if (ctx.quest_comp.info.hasOpenedChest(eid32)) return true;
    }
    if (ctx.progress_comp) |pc| {
        var buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&buf, "sun_spirit:{d}", .{eid}) catch return false;
        if (pc.info.hasClaim(key) or pc.info.getKeyInt(key) > 0) return true;
    }
    return false;
}

fn countCollectedSunSpirits(ctx: EvalContext, dungeon_id: i32, area_id: i32) i32 {
    const assets = ctx.assets orelse return 0;
    var count: i32 = 0;
    if (assets.getDb()) |db| {
        var scratch = std.heap.ArenaAllocator.init(assets.tables.arena.child_allocator);
        defer scratch.deinit();
        const rows = db.querySunSpiritEntities(scratch.allocator()) catch return 0;
        for (rows) |row| {
            if (dungeon_id > 0 and row.map_id != dungeon_id) continue;
            if (area_id > 0 and row.area_id != area_id) continue;
            if (isSunSpiritCollected(ctx, row.entity_id)) count += 1;
        }
        return count;
    }
    for (assets.tables.level_entity_config.items) |cfg| {
        if (dungeon_id > 0 and cfg.MapId != dungeon_id) continue;
        if (area_id > 0 and cfg.AreaId != area_id) continue;
        if (std.mem.indexOf(u8, cfg.BlueprintType, "RiLing") == null and
            std.mem.indexOf(u8, cfg.BlueprintType, "SunSpirit") == null) continue;
        if (isSunSpiritCollected(ctx, cfg.EntityId)) count += 1;
    }
    return count;
}

fn evalPlayerSkillReady(ctx: EvalContext, obj: std.json.ObjectMap) bool {
    const opt = obj.get("SkillOption") orelse return false;
    if (opt != .object) return false;
    const skill_type = getParamStr(opt.object, "Type") orelse return false;
    const scene = ctx.scene orelse return false;
    const role_net_id = scene.currentRoleEntityId();
    const idx = scene.net_id_map.get(role_net_id) orelse return false;
    const slice = scene.entities.slice();
    if (std.mem.eql(u8, skill_type, "UltimateSkill")) {
        const attr = slice.items(.attribute)[idx] orelse return false;
        const energy_idx: usize = @intCast(@intFromEnum(pb.EAttributeType.Energy));
        const max_idx: usize = @intCast(@intFromEnum(pb.EAttributeType.EnergyMax));
        const cur = attr.attributes[energy_idx].current;
        const max_val = attr.attributes[max_idx].current;
        return if (max_val > 0) cur >= max_val else cur > 0;
    }
    if (std.mem.eql(u8, skill_type, "ESkill")) {
        const attr = slice.items(.attribute)[idx] orelse return true;
        const life_idx: usize = @intCast(@intFromEnum(pb.EAttributeType.Life));
        return attr.attributes[life_idx].current > 0;
    }
    if (std.mem.eql(u8, skill_type, "VisionSkill")) {
        if (slice.items(.vision_skills)[idx]) |vs| {
            if (vs.vision_skills.len > 0) return true;
        }
        return scene.explore_tools_info.active_explore_skill != 0 or ctx.echo_comp.echo_map.count() > 0;
    }
    return false;
}

fn getParamStr(obj: std.json.ObjectMap, key: []const u8) ?[]const u8 {
    const val = obj.get(key) orelse return null;
    return switch (val) {
        .string => |s| s,
        else => null,
    };
}

fn getParamInt(obj: std.json.ObjectMap, key: []const u8) ?i32 {
    const s = getParamStr(obj, key) orelse return null;
    return std.fmt.parseInt(i32, s, 10) catch null;
}

fn jsonInt(v_opt: ?std.json.Value) ?i32 {
    const v = v_opt orelse return null;
    return switch (v) {
        .integer => |i| std.math.cast(i32, i),
        .float => |f| {
            if (!std.math.isFinite(f)) return null;
            const min_f: f64 = @floatFromInt(std.math.minInt(i32));
            const max_f: f64 = @floatFromInt(std.math.maxInt(i32));
            if (f < min_f or f > max_f) return null;
            return @intFromFloat(f);
        },
        .string => |s| std.fmt.parseInt(i32, s, 10) catch null,
        else => null,
    };
}

fn containsInt(slice: []const i32, val: i32) bool {
    for (slice) |x| {
        if (x == val) return true;
    }
    return false;
}

test "roleMeetsThreshold checks breach, level, favor, and resonance" {
    const RoleInfo = @import("../../fs/RoleInfo.zig");
    var resons = [_]RoleInfo.Reson{
        .{ .id = 1, .open = true, .increase = 1 },
        .{ .id = 2, .open = true, .increase = 1 },
        .{ .id = 3, .open = false, .increase = 0 },
    };
    const role: RoleInfo = .{
        .level = 60,
        .breakthrough = 3,
        .favor = 4,
        .resonant_chain_group_index = 2,
        .reson = &resons,
    };
    try std.testing.expect(roleMeetsThreshold(role, .level, 60));
    try std.testing.expect(!roleMeetsThreshold(role, .level, 70));
    try std.testing.expect(roleMeetsThreshold(role, .breach, 3));
    try std.testing.expect(roleMeetsThreshold(role, .favor, 4));
    try std.testing.expect(!roleMeetsThreshold(role, .favor, 5));
    try std.testing.expect(roleMeetsThreshold(role, .resonance, 2));
    try std.testing.expect(!roleMeetsThreshold(role, .resonance, 3));
}

test "compareInts handles symbol and keyword comparison operators uniformly" {
    try std.testing.expect(compareInts(0, 2, "<"));
    try std.testing.expect(!compareInts(2, 2, "<"));
    try std.testing.expect(compareInts(2, 2, "<="));
    try std.testing.expect(compareInts(5, 3, ">"));
    try std.testing.expect(compareInts(3, 3, "GtEq"));
    try std.testing.expect(compareInts(1, 2, "!="));
    try std.testing.expect(compareInts(2, 2, "=="));
}

test "evalConditionRecord evaluates direct 3.8.11 types and server_cond_specs" {
    const gpa = std.testing.allocator;
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const aalloc = arena.allocator();

    var basic: PlayerBasicComponent = .{ .player_id = 1, .info = .default };
    basic.info.attributes.level = 55;
    basic.info.attributes.origin_world_level = 6;
    basic.info.attributes.cur_world_level = 5;

    var echo: PlayerEchoComponent = .{ .player_id = 1, .echo_map = .empty, .preset_info = .default, .calabash_info = .default };
    var inv: PlayerInventoryComponent = .{ .player_id = 1, .info = .default };
    var map: PlayerMapComponent = .{ .player_id = 1, .info = .default };
    var quest: PlayerQuestComponent = .{ .player_id = 1, .info = .default };
    defer quest.info.deinit(gpa);
    try quest.info.setVarStr(gpa, "route", "B");

    var prog: PlayerProgressComponent = .{ .player_id = 1, .info = .default };
    defer prog.info.deinit(gpa);
    _ = try prog.info.markClaim(gpa, "abyss:pass:101");
    _ = try prog.info.markClaim(gpa, "rover_rogue:talent:7");
    try prog.info.setKeyInt(gpa, "pb:challenge:9", 1);

    const ctx: EvalContext = .{
        .assets = null,
        .basic_comp = &basic,
        .echo_comp = &echo,
        .inventory_comp = &inv,
        .map_comp = &map,
        .quest_comp = &quest,
        .progress_comp = &prog,
    };

    const p_lv = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"Level\":\"50\",\"Op\":\"Ge\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 1, .Type = "CheckLevelOp", .LimitParams = p_lv }));

    const p_lv_fail = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"Level\":\"60\",\"Op\":\"Ge\"}", .{});
    try std.testing.expect(!evalConditionRecord(ctx, .{ .Id = 2, .Type = "CheckLevelOp", .LimitParams = p_lv_fail }));

    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 3, .Type = "ReduceWorldLevel" }));
    basic.info.attributes.cur_world_level = 6;
    try std.testing.expect(!evalConditionRecord(ctx, .{ .Id = 4, .Type = "ReduceWorldLevel" }));

    const p_vd = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"VDName\":\"route\",\"VDValueString\":\"B\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 5, .Type = "VarDefineString", .LimitParams = p_vd }));

    const p_abyss = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"Id\":\"101\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 6, .Type = "AbyssPass", .LimitParams = p_abyss }));

    const p_rr = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"TalentId\":\"7\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 7, .Type = "RoverRogueTalentUnlock", .LimitParams = p_rr }));

    const p_pb = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"ChallengeId\":\"9\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 8, .Type = "PhantomBattleCompleteChallenge", .LimitParams = p_pb }));

    const p_miss = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"Id\":\"999\"}", .{});
    try std.testing.expect(!evalConditionRecord(ctx, .{ .Id = 9, .Type = "AbyssPass", .LimitParams = p_miss }));
}

test "evalConditionRecord handles 15 LevelPlay Condition types" {
    const gpa = std.testing.allocator;
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const aalloc = arena.allocator();

    var basic: PlayerBasicComponent = .{ .player_id = 1, .info = .default };
    var echo: PlayerEchoComponent = .{ .player_id = 1, .echo_map = .empty, .preset_info = .default, .calabash_info = .default };
    var inv: PlayerInventoryComponent = .{ .player_id = 1, .info = .default };
    var map: PlayerMapComponent = .{ .player_id = 1, .info = .default };
    var quest: PlayerQuestComponent = .{ .player_id = 1, .info = .default };
    defer quest.info.deinit(gpa);

    var prog: PlayerProgressComponent = .{ .player_id = 1, .info = .default };
    defer prog.info.deinit(gpa);
    try prog.info.setKeyInt(gpa, "actor_var:done", 1);
    try prog.info.setKeyInt(gpa, "simple_combat:kills", 1);
    try prog.info.setKeyInt(gpa, "flipper:fin", 1);
    try prog.info.setKeyInt(gpa, "kurotato:wave", 1);
    try prog.info.setKeyInt(gpa, "role_tower:stage", 1);
    try prog.info.setKeyInt(gpa, "rb_blk:fin", 1);
    try prog.info.setKeyInt(gpa, "surv:wave", 1);
    try prog.info.setKeyInt(gpa, "tetris:best", 500);
    try prog.info.setKeyInt(gpa, "trap:pass_wave", 1);
    try prog.info.setKeyInt(gpa, "handin_pm:cnt", 1);
    try prog.info.setKeyInt(gpa, "parallax:done", 1);
    try prog.info.setKeyInt(gpa, "timeline:done", 1);
    try prog.info.setKeyInt(gpa, "explore:photo_cnt", 1);
    try prog.info.setKeyInt(gpa, "walk_pat:done", 1);

    const ctx: EvalContext = .{
        .assets = null,
        .basic_comp = &basic,
        .echo_comp = &echo,
        .inventory_comp = &inv,
        .map_comp = &map,
        .quest_comp = &quest,
        .progress_comp = &prog,
    };

    _ = aalloc;
    inline for (.{
        "CompareActorVar",
        "CrossSceneSyncKillMonster",
        "FinishFlipper",
        "FinishKuroSimpleCombatKillMonster",
        "FinishKurotato",
        "FinishRoleTower",
        "FinishRollBlock",
        "FinishSurvivorsRouge",
        "FinishTetris",
        "FinishTrapDefense",
        "NewHandInItems",
        "ParallaxAlign",
        "PlaybackShowTimeline",
        "TakePicturesWithTimeScale",
        "WalkingPattern",
    }) |ctype| {
        try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 1, .Type = ctype, .NeedNum = 1 }));
    }
}

test "evalConditionRecord handles 3.8.11 dungeon, tower, and activity conditions" {
    const gpa = std.testing.allocator;
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const aalloc = arena.allocator();

    var basic: PlayerBasicComponent = .{ .player_id = 1, .info = .default };
    var echo: PlayerEchoComponent = .{ .player_id = 1, .echo_map = .empty, .preset_info = .default, .calabash_info = .default };
    var inv: PlayerInventoryComponent = .{ .player_id = 1, .info = .default };
    var map: PlayerMapComponent = .{ .player_id = 1, .info = .default };
    var quest: PlayerQuestComponent = .{ .player_id = 1, .info = .default };
    defer quest.info.deinit(gpa);
    try quest.info.markDungeonCompleted(gpa, 9101);

    var prog: PlayerProgressComponent = .{ .player_id = 1, .info = .default };
    defer prog.info.deinit(gpa);
    _ = try prog.info.markClaim(gpa, "role_tower:stage:3");
    _ = try prog.info.markClaim(gpa, "pb:challenge:12");
    _ = try prog.info.markClaim(gpa, "kurotato:done:4");
    try prog.info.setKeyInt(gpa, "babel:star:2", 9);
    try prog.info.setKeyInt(gpa, "cond_cnt:CiacconaChapterResult", 1);

    const ctx: EvalContext = .{
        .assets = null,
        .basic_comp = &basic,
        .echo_comp = &echo,
        .inventory_comp = &inv,
        .map_comp = &map,
        .quest_comp = &quest,
        .progress_comp = &prog,
    };

    const p_dung_ok = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"InstanceId\":\"9101\",\"Finish\":\"1\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 10, .Type = "CheckDungeonFinished", .LimitParams = p_dung_ok }));

    const p_dung_fail = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"InstanceId\":\"9102\",\"Finish\":\"1\"}", .{});
    try std.testing.expect(!evalConditionRecord(ctx, .{ .Id = 11, .Type = "NewCheckInstanceState", .LimitParams = p_dung_fail }));

    const p_rt = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"Id\":\"3\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 12, .Type = "RoleTowerPassStage", .LimitParams = p_rt }));

    const p_babel = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"Id\":\"2\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 13, .Type = "BabelTowerLevelFinishStar", .LimitParams = p_babel, .NeedNum = 9 }));

    const p_kuro = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"Level\":\"4\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 14, .Type = "KurotatoFinishLevel", .LimitParams = p_kuro }));

    const p_pb = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"ChallengeId\":\"12\"}", .{});
    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 15, .Type = "PhantomBattleCompleteChallenge", .LimitParams = p_pb }));

    try std.testing.expect(evalConditionRecord(ctx, .{ .Id = 16, .Type = "CiacconaChapterResult", .NeedNum = 1 }));
}
