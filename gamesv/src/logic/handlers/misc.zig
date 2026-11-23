const pb = @import("proto").pb;
const std = @import("std");
const mem = @import("../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const EventQueue = @import("../EventQueue.zig");
const Scene = @import("../Scene.zig");
const Connection = @import("../../network/Connection.zig");
const PlayerRoleComponent = @import("../component/player/PlayerRoleComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const PlayerCosmeticComponent = @import("../component/player/PlayerCosmeticComponent.zig");
const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
const PlayerMailComponent = @import("../component/player/PlayerMailComponent.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const ConditionEvaluator = @import("../helpers/condition.zig");
const CalabashHelper = @import("../helpers/calabash.zig");
const EnergyHelper = @import("../helpers/energy.zig");
const QuestEngine = @import("../helpers/quest_engine.zig");
const phantom_projector = @import("../helpers/phantom_projector.zig");
const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");
const activity_handler = @import("../../network/handlers/activity.zig");

pub fn pushData(
    _: EventQueue.Dequeue(.push_data),
    conn: *Connection,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    cosmetic_comp: *PlayerCosmeticComponent,
    map_comp: *PlayerMapComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    quest_comp: *PlayerQuestComponent,
    mail_comp: *PlayerMailComponent,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    var role_notify: pb.RoleConfigInfoNotify = .default;
    var iterator = role_comp.role_map.iterator();

    while (iterator.next()) |role| {
        try role_notify.RoleConfigs.append(alloc.arena, pb.RoleConfigInfo{ .RoleId = role.key_ptr.*, .SkillBranch = role.value_ptr.skill_branch });
    }

    try conn.push(alloc.gpa, io, role_notify);
    try ConditionEvaluator.pushFuncOpenNotify(conn, alloc, assets, basic_comp, echo_comp, inventory_comp, map_comp, quest_comp, role_comp, io);

    activity_handler.pushActivityUpdateNotify(conn, alloc, assets, progress_comp, .{
        .assets = assets,
        .basic_comp = basic_comp,
        .echo_comp = echo_comp,
        .inventory_comp = inventory_comp,
        .map_comp = map_comp,
        .quest_comp = quest_comp,
        .role_comp = role_comp,
        .progress_comp = progress_comp,
    }, basic_comp.player_id, io);

    try conn.push(alloc.gpa, io, pb.AdviceSettingNotify{ .IsShow = false });
    try conn.push(alloc.gpa, io, pb.ControlInfoNotify{});
    try conn.push(alloc.gpa, io, try buildInstDataNotify(alloc.arena, assets, quest_comp));
    try conn.push(alloc.gpa, io, try CalabashHelper.buildMsgNotify(alloc.arena, assets, basic_comp, echo_comp));
    try conn.push(alloc.gpa, io, try phantom_projector.buildUnlockNotify(alloc, assets, echo_comp.calabash_info, cosmetic_comp.info));
    try conn.push(alloc.gpa, io, pb.CalabashLevelsRewardNotify{
        .RewardedLevels = try intList(echo_comp.calabash_info.rewarded_levels, alloc.arena),
    });

    var open_pkg: std.ArrayList(i32) = .empty;
    defer open_pkg.deinit(alloc.gpa);
    for (0..8) |i| {
        try open_pkg.append(alloc.gpa, @intCast(i));
    }
    try conn.push(alloc.gpa, io, pb.ItemPkgOpenNotify{
        .OpenPkg = open_pkg,
    });

    const now_ms = EnergyHelper.nowSeconds(io) * 1000;
    var buff_cds: std.ArrayList(pb.BuffItem) = .empty;
    for (progress_comp.info.dynamic.ints) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "buff_cd:")) continue;
        if (entry.val <= now_ms) continue;
        const item_id = std.fmt.parseInt(i32, entry.key["buff_cd:".len..], 10) catch continue;
        try buff_cds.append(alloc.arena, .{ .ItemId = item_id, .CdTime = entry.val });
    }
    try conn.push(alloc.gpa, io, pb.BuffItemNotify{ .ItemBuffList = buff_cds });

    try EnergyHelper.syncAndSave(alloc.arena, fs, basic_comp, io);
    try EnergyHelper.pushEnergyNotify(conn, alloc, basic_comp, io);

    const now_sec = @divFloor(now_ms, 1000);
    if (try mail_comp.info.pruneExpired(alloc.gpa, now_sec)) {
        try mail_comp.save(alloc.arena, fs);
    }

    try conn.push(alloc.gpa, io, try buildLevelPlayInfoNotify(alloc, assets, quest_comp));
    try conn.push(alloc.gpa, io, try buildMapMarkInfoNotify(alloc, assets, map_comp));
    try conn.push(alloc.gpa, io, pb.PlayerVarNotify{});
    try conn.push(alloc.gpa, io, pb.RoguelikeCurrencyNotify{});
    try conn.push(alloc.gpa, io, pb.PassiveSkillNotify{});
    try conn.push(alloc.gpa, io, try mail_comp.info.toProtoNotify(alloc.arena));
    try conn.push(alloc.gpa, io, pb.SettingNotify{});
    try conn.push(alloc.gpa, io, pb.MoonChasingTrackMoonHandbookRewardNotify{});
    try conn.push(alloc.gpa, io, pb.MoonChasingTargetGetCountNotify{});
    try conn.push(alloc.gpa, io, pb.SilenceNpcNotify{});
}

fn containsInt(values: []const i32, needle: i32) bool {
    for (values) |v| {
        if (v == needle) return true;
    }
    return false;
}

pub fn buildLevelPlayInfoNotify(
    alloc: mem.Alloc,
    assets: *const Assets,
    quest_comp: *const PlayerQuestComponent,
) !pb.LevelPlayInfoNotify {
    var seen: std.AutoHashMapUnmanaged(i32, void) = .empty;
    var infos: std.ArrayList(pb.LevelPlayInfo) = .empty;

    for (assets.tables.map_mark.items) |mark| {
        if (mark.RelativeType != 1 or mark.RelativeId <= 0) continue;
        if (mark.ObjectType != 19 and mark.ObjectType != 28 and mark.ObjectType != 29) continue;
        if (assets.tables.level_play_data.getDataById(mark.RelativeId) == null) continue;
        if (seen.contains(mark.RelativeId)) continue;
        try seen.put(alloc.arena, mark.RelativeId, {});
        const done = QuestInfo.containsInt(quest_comp.info.completed_level_plays, mark.RelativeId);
        try infos.append(alloc.arena, .{
            .Id = mark.RelativeId,
            .IsFirst = !done,
            .State = if (done) 3 else 1,
        });
    }

    return .{ .LevelPlayInfo = infos };
}

fn buildMapMarkInfoNotify(
    alloc: mem.Alloc,
    assets: *const Assets,
    map_comp: *const PlayerMapComponent,
) !pb.MapMarkInfoNotify {
    var boss_entity_positions: std.AutoHashMapUnmanaged(i64, pb.Vector) = .empty;
    for (assets.tables.map_mark.items) |mark| {
        if (mark.MapId <= 0 or mark.EntityConfigId == 0) continue;
        if (mark.ObjectType != 19 and mark.ObjectType != 29) continue;
        if (boss_entity_positions.contains(mark.EntityConfigId)) continue;
        const cfg = assets.findLevelEntity(mark.MapId, mark.EntityConfigId) orelse continue;
        const pos = Assets.AssetIndexes.entityWorldPos(cfg) orelse continue;
        try boss_entity_positions.put(alloc.arena, cfg.EntityId, .{
            .X = pos[0],
            .Y = pos[1],
            .Z = pos[2],
        });
    }

    var show_marks: std.ArrayList(pb.MapMarkShowInfo) = .empty;
    var unlock_marks: std.ArrayList(i32) = .empty;
    var entity_marks: std.ArrayList(pb.EntityMapMarkInfoPb) = .empty;

    for (map_comp.info.unlocked_teleporters) |tp_id| {
        try unlock_marks.append(alloc.arena, tp_id);
        try show_marks.append(alloc.arena, .{ .MarkId = tp_id, .ShowFlag = 1 });
    }

    for (assets.tables.map_mark.items) |mark| {
        const is_boss_or_challenge = (mark.ObjectType == 19 or mark.ObjectType == 28 or mark.ObjectType == 29);
        const is_teleporter = (mark.ObjectType == 5 or mark.ObjectType == 6);

        if (is_boss_or_challenge) {
            try show_marks.append(alloc.arena, .{ .MarkId = mark.MarkId, .ShowFlag = 1 });
            try unlock_marks.append(alloc.arena, mark.MarkId);
            if (mark.EntityConfigId != 0) {
                if (std.math.cast(i32, mark.EntityConfigId)) |inst_id32| {
                    if (boss_entity_positions.get(mark.EntityConfigId)) |pos| {
                        try entity_marks.append(alloc.arena, .{
                            .InstId = inst_id32,
                            .TemplateId = mark.RelativeId,
                            .Pos = pos,
                        });
                    }
                }
            }
        } else if (!is_teleporter and mark.CreateOnStart == 1 and (mark.FogHide == 0 or containsInt(map_comp.info.unlocked_fogs, mark.FogHide))) {
            try show_marks.append(alloc.arena, .{ .MarkId = mark.MarkId, .ShowFlag = 1 });
        } else if (is_teleporter and !containsInt(map_comp.info.unlocked_teleporters, mark.MarkId) and mark.FogHide != 0 and containsInt(map_comp.info.unlocked_fogs, mark.FogHide)) {
            try show_marks.append(alloc.arena, .{ .MarkId = mark.MarkId, .ShowFlag = 1 });
        }
    }

    return .{
        .ShowMarkIds = show_marks,
        .UnlockMarkIds = unlock_marks,
        .EntityMapMarkInfo = entity_marks,
    };
}

pub fn buildInstDataNotify(
    arena: std.mem.Allocator,
    assets: *const Assets,
    quest_comp: *const PlayerQuestComponent,
) !pb.InstDataNotify {
    var enter_infos: std.ArrayList(pb.InstEnterInfoPb) = .empty;
    var seen_shares: std.AutoHashMapUnmanaged(i32, void) = .empty;
    for (quest_comp.info.completed_dungeons) |inst_id| {
        if (inst_id <= 0) continue;
        // InstDataNotify expects InstanceEnterControl ShareId; skip dungeons without enter-count config.
        const dg = assets.tables.instance_dungeon.getDataById(inst_id) orelse continue;
        if (dg.EnterControlId <= 0) continue;
        if (seen_shares.contains(dg.EnterControlId)) continue;
        try seen_shares.put(arena, dg.EnterControlId, {});
        try enter_infos.append(arena, .{
            .Id = dg.EnterControlId,
            .ChallengedTimes = 1,
        });
    }
    return .{ .EnterInfos = enter_infos };
}

fn intList(values: []const i32, arena: std.mem.Allocator) !std.ArrayList(i32) {
    var list: std.ArrayList(i32) = .empty;
    try list.appendSlice(arena, values);
    return list;
}

/// Re-pushes tracked quest state and active dynamic interact options after a silent reconnect.
pub fn handleReconnectResync(
    _: EventQueue.Dequeue(.reconnect_resync),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.resyncOnReconnect(qctx);
}

/// Re-evaluates quest unlock conditions and FuncOpen flags after progression changes (e.g. union level up).
pub fn handleConditionStateRefresh(
    _: EventQueue.Dequeue(.condition_state_refresh),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.onProgressionUnlockChanged(qctx);
}
