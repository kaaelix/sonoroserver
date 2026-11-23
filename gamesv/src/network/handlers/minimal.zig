// typed no-op handlers for unimplemented client requests so the game doesnt freeze.
const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerSceneComponent = @import("../../logic/component/player/PlayerSceneComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const Scene = @import("../../logic/Scene.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const Connection = @import("../../network/Connection.zig");
const Transaction = @import("../handlers.zig").Transaction;
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const attributes_helper = @import("../../logic/helpers/attributes.zig");
const pb_kv = @import("pb_kv.zig");

const FileSystem = common.FileSystem;

const ph_ba_success_code: pb.ErrorCode = .Success;

pub fn onDollSmallMapInfoRequest(
    txn: *Transaction(pb.DollSmallMapInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
    scene_comp: *const PlayerSceneComponent,
) !void {
    _ = txn.message;
    var scores: std.ArrayList(pb.SceneUnlimitedScoreInfo) = .empty;
    if (progress_comp.info.getKeyInt("doll:score") > 0) {
        try scores.append(alloc.arena, .{ .instId = 8 });
    }
    var by_showcase: std.AutoHashMapUnmanaged(i32, std.ArrayList(i32)) = .empty;
    for (progress_comp.info.dynamic.claims) |c| {
        if (std.mem.startsWith(u8, c, "doll:item:")) {
            var it = std.mem.splitScalar(u8, c["doll:item:".len..], ':');
            const sc_str = it.next() orelse continue;
            const d_str = it.next() orelse continue;
            const showcase_id = std.fmt.parseInt(i32, sc_str, 10) catch continue;
            const doll_id = std.fmt.parseInt(i32, d_str, 10) catch continue;
            if (showcase_id > 0 and doll_id > 0) {
                const gop = try by_showcase.getOrPut(alloc.arena, showcase_id);
                if (!gop.found_existing) gop.value_ptr.* = .empty;
                try gop.value_ptr.append(alloc.arena, doll_id);
            }
        } else if (std.mem.startsWith(u8, c, "doll:delivery:")) {
            if (std.fmt.parseInt(i32, c["doll:delivery:".len..], 10)) |showcase_id| {
                if (showcase_id > 0) {
                    const gop = try by_showcase.getOrPut(alloc.arena, showcase_id);
                    if (!gop.found_existing) gop.value_ptr.* = .empty;
                }
            } else |_| {}
        }
    }
    var delivery_infos: std.ArrayList(pb.DollDeliveryInfo) = .empty;
    var it = by_showcase.iterator();
    while (it.next()) |kv| {
        try delivery_infos.append(alloc.arena, .{
            .ShowcaseEntityId = kv.key_ptr.*,
            .DeliveredDolls = kv.value_ptr.*,
        });
    }
    var scene_deliveries: std.ArrayList(pb.SceneDollDeliveryInfo) = .empty;
    if (delivery_infos.items.len > 0) {
        const inst_id: i32 = if (scene_comp.last_scene_info.instance_id > 0) scene_comp.last_scene_info.instance_id else 8;
        try scene_deliveries.append(alloc.arena, .{
            .instId = inst_id,
            .DollDeliveryInfos = delivery_infos,
        });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .SceneUnlimitedScoreInfos = scores,
        .SceneDollDeliveryInfos = scene_deliveries,
    });
}

pub fn onRoleVisionRecommendDataRequest(
    txn: *Transaction(pb.RoleVisionRecommendDataRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
) !void {
    _ = txn.message;
    var list: std.ArrayList(pb.VisionFetterRecommendInfo) = .empty;
    var groups: std.ArrayList(pb.RecommendFetterGroupInfo) = .empty;
    var count: usize = 0;
    for (assets.tables.phantom_fetter_group.items) |fg| {
        try groups.append(alloc.arena, .{ .RecommendFetterGroupId = fg.Id, .CountNeed = 5 });
        count += 1;
        if (count >= 3) break;
    }
    var combos: std.ArrayList(pb.CostCombinationInfo) = .empty;
    try combos.append(alloc.arena, .{ .CostId = 43311, .Usage = 1 });
    try list.append(alloc.arena, .{
        .Usage = 1,
        .RecommendFetterGroupInfos = groups,
        .CostCombinationInfos = combos,
    });
    txn.respond(.{ .ErrorCode = .Success, .VisionFetterRecommendInfo = list });
}

pub fn onRoleVisionRecommendAttrRequest(
    txn: *Transaction(pb.RoleVisionRecommendAttrRequest),
    alloc: mem.Alloc,
) !void {
    _ = txn.message;
    var list: std.ArrayList(pb.CostVisionAttrRecommendInfo) = .empty;
    for ([_]struct { cost: i32, main: i32 }{
        .{ .cost = 4, .main = 20001 },
        .{ .cost = 3, .main = 7 },
        .{ .cost = 1, .main = 1 },
    }) |spec| {
        var mains: std.ArrayList(pb.VisionAttrRecommendInfo) = .empty;
        try mains.append(alloc.arena, .{ .AttrType = spec.main, .AddType = 2, .Usage = 1 });
        var subs: std.ArrayList(pb.VisionAttrRecommendInfo) = .empty;
        try subs.append(alloc.arena, .{ .AttrType = 20001, .AddType = 2, .Usage = 1 });
        try subs.append(alloc.arena, .{ .AttrType = 20002, .AddType = 2, .Usage = 1 });
        try list.append(alloc.arena, .{
            .Cost = spec.cost,
            .GetMainAttrRecommendInfo = mains,
            .GetSubAttrRecommendInfo = subs,
        });
    }
    txn.respond(.{ .ErrorCode = .Success, .VisionAttrRecommendInfos = list });
}

pub fn onRoleVisionMainPhantomRequest(
    txn: *Transaction(pb.RoleVisionMainPhantomRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
) !void {
    _ = txn.message;
    var list: std.ArrayList(pb.MainPhantomRecommendInfo) = .empty;
    for (assets.tables.phantom_item.items) |pi| {
        if (pi.MonsterId > 0 and pi.FetterGroup.len > 0) {
            try list.append(alloc.arena, .{
                .Usage = 1,
                .MonsterId = pi.MonsterId,
                .FetterGroupId = pi.FetterGroup[0],
            });
            if (list.items.len >= 3) break;
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .RecommendInfo = list });
}

pub fn onPhBaPlanUsePlanRequest(
    txn: *Transaction(pb.PhBaPlanUsePlanRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .ErrorCode = ph_ba_success_code,
        .SelfPlanCode = "local-plan",
        .UsePlan = pb_kv.loadMsg(alloc.arena, progress_comp, "phba:use_plan", pb.PhBaOneAllSuitPlan) orelse emptyUsePlan(),
        .FiveStarSwitch = progress_comp.info.getKeyInt("phba_plan:5star") != 0,
        .TowPlanSame = true,
    });
}

pub fn onPhBaPlanSaveUsePlanRequest(
    txn: *Transaction(pb.PhBaPlanSaveUsePlanRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistOptMsg(alloc, fs, progress_comp, "phba:use_plan", txn.message.UsePlan);
    _ = try progress_comp.info.markClaim(alloc.gpa, "phba_plan:saved");
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = ph_ba_success_code,
        .SelfPlanCode = "local-plan",
        .TowPlanSame = true,
    });
}

pub fn onPhBaPlanFindPlanRequest(
    txn: *Transaction(pb.PhBaPlanFindPlanRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = ph_ba_success_code,
        .Plan = pb_kv.loadMsg(alloc.arena, progress_comp, "phba:use_plan", pb.PhBaOneAllSuitPlan) orelse emptyUsePlan(),
        .TowPlanSame = true,
    });
}

pub fn onPhBaPlanUpdatePlanRequest(
    txn: *Transaction(pb.PhBaPlanUpdatePlanRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.markClaim(alloc.gpa, "phba_plan:updated");
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = ph_ba_success_code, .SelfPlanCode = "local-plan" });
}

pub fn onPhBaPlanSetFiveStarSwitchRequest(
    txn: *Transaction(pb.PhBaPlanSetFiveStarSwitchRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "phba_plan:5star", if (txn.message.Open) 1 else 0);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "phba_plan:5star_toggle", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = ph_ba_success_code });
}

pub fn onPhBaPlanSetPlanStatusRequest(
    txn: *Transaction(pb.PhBaPlanSetPlanStatusRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    for (txn.message.SetInfo.items) |si| {
        const k = try std.fmt.allocPrint(alloc.arena, "phba_plan:suit:{d}", .{si.Suit});
        try progress_comp.info.setKeyInt(alloc.gpa, k, if (si.Open) 1 else 0);
    }
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "phba_plan:status", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = ph_ba_success_code,
        .TowPlanSame = true,
    });
}

pub fn onPhantomFuncValueBatchRequest(
    txn: *Transaction(pb.PhantomFuncValueBatchRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
    io: std.Io,
) !void {
    const new_func: i32 = switch (txn.message.FuncValue orelse .Reset) {
        .Locked => 1,
        .Disuse => 2,
        .Reset => 0,
    };
    var changed = false;
    for (txn.message.IncrId.items) |incr_id| {
        const item = echo_comp.echo_map.getPtr(incr_id) orelse continue;
        item.func_value = new_func;
        changed = true;
        try txn.conn.push(alloc.gpa, io, pb.ItemFuncValueUpdateNotify{
            .IncrId = incr_id,
            .FuncValue = new_func,
        });
    }
    if (changed) {
        try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhBaPlanBatchOperRequest(
    txn: *Transaction(pb.PhBaPlanBatchOperRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    echo_comp: *@import("../../logic/component/player/PlayerEchoComponent.zig"),
    io: std.Io,
) !void {
    for (txn.message.BatchOper.items) |op| {
        const new_func: i32 = switch (op.Oper orelse .Reset) {
            .Locked => 1,
            .Disuse => 2,
            .Reset => 0,
        };
        for (op.IncrId.items) |incr_id| {
            if (echo_comp.echo_map.getPtr(incr_id)) |item| {
                item.func_value = new_func;
                try txn.conn.push(alloc.gpa, io, pb.ItemFuncValueUpdateNotify{
                    .IncrId = incr_id,
                    .FuncValue = new_func,
                });
            }
        }
    }
    try @import("../../logic/component/player/PlayerEchoComponent.zig").saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    txn.respond(.{ .errCode = .Success });
}

pub fn onPhantomManageConfigRequest(
    txn: *Transaction(pb.PhantomManageConfigRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var locks: std.ArrayList(pb.PhantomManageConfig) = .empty;
    var disuses: std.ArrayList(pb.PhantomManageConfig) = .empty;
    var i: i32 = 0;
    while (i < 5) : (i += 1) {
        const lk = try std.fmt.allocPrint(alloc.arena, "phantom_mgr:lock:{d}", .{i});
        if (pb_kv.loadMsg(alloc.arena, progress_comp, lk, pb.PhantomManageConfig)) |cfg| {
            try locks.append(alloc.arena, cfg);
        }
        const dk = try std.fmt.allocPrint(alloc.arena, "phantom_mgr:disuse:{d}", .{i});
        if (pb_kv.loadMsg(alloc.arena, progress_comp, dk, pb.PhantomManageConfig)) |cfg| {
            try disuses.append(alloc.arena, cfg);
        }
    }
    txn.respond(.{
        .AutoLock = locks,
        .AutoDisuse = disuses,
    });
}

pub fn onPhantomSettingBatchUpdateRequest(
    txn: *Transaction(pb.PhantomSettingBatchUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    for (txn.message.Settings.items) |entry| {
        const setting = entry.Setting orelse continue;
        const pfx = if ((entry.SettingType orelse .AutoLock) == .AutoDisuse) "phantom_mgr:disuse" else "phantom_mgr:lock";
        const key = try std.fmt.allocPrint(alloc.arena, "{s}:{d}", .{ pfx, setting.Index });
        try pb_kv.saveMsg(alloc, fs, progress_comp, key, setting);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .Settings = txn.message.Settings,
    });
}

pub fn onPhantomManageConfigUpdateRequest(
    txn: *Transaction(pb.PhantomManageConfigUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.Setting) |setting| {
        const pfx = if ((txn.message.SettingType orelse .AutoLock) == .AutoDisuse) "phantom_mgr:disuse" else "phantom_mgr:lock";
        const key = try std.fmt.allocPrint(alloc.arena, "{s}:{d}", .{ pfx, setting.Index });
        try pb_kv.saveMsg(alloc, fs, progress_comp, key, setting);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .Setting = txn.message.Setting,
    });
}



pub fn onGetDetectionLabelInfoRequest(
    txn: *Transaction(pb.GetDetectionLabelInfoRequest),
    assets: *const Assets,
    alloc: mem.Alloc,
) !void {
    var guide_ids: std.ArrayList(i32) = .empty;
    for (assets.tables.dungeon_detection.items) |dd| {
        if (dd.GuideId > 0 and std.mem.indexOfScalar(i32, guide_ids.items, dd.GuideId) == null) {
            try guide_ids.append(alloc.arena, dd.GuideId);
        }
    }
    for (assets.tables.silent_area_detection.items) |sd| {
        if (sd.GuideId > 0 and std.mem.indexOfScalar(i32, guide_ids.items, sd.GuideId) == null) {
            try guide_ids.append(alloc.arena, sd.GuideId);
        }
    }
    txn.respond(.{
        .UnlockLabelInfo = .{
            .UnlockedGuideIds = guide_ids,
        },
    });
}

pub fn onDetectionRequest(
    txn: *Transaction(pb.DetectionRequest),
) !void {
    txn.respond(.{
        .DetectionConfId = txn.message.DetectionConfId,
    });
}

pub fn onSelectDetectionTargetRequest(
    txn: *Transaction(pb.SelectDetectionTargetRequest),
) !void {
    txn.respond(.{
        .SelectDetectionTarget = txn.message.SelectDetectionTarget,
    });
}

pub fn onMonsterDetectionInfoRequest(
    txn: *Transaction(pb.MonsterDetectionInfoRequest),
) !void {
    txn.respond(.{ .DetectionTarget = .empty });
}

pub fn onLevelPlayVarAsyncRequest(
    txn: *Transaction(pb.LevelPlayVarAsyncRequest),
    alloc: mem.Alloc,
    quest_comp: *const @import("../../logic/component/player/PlayerQuestComponent.zig"),
) !void {
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = .Success,
        .Vars = try @import("world_puzzle.zig").buildLevelPlayVars(alloc.arena, quest_comp),
    });
}

pub fn onRoleDevelopConfigRequest(txn: *Transaction(pb.RoleDevelopConfigRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success, .Configs = .default });
}

pub fn onServerPlayStationPlayOnlyStateRequest(
    txn: *Transaction(pb.ServerPlayStationPlayOnlyStateRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    txn.respond(.{
        .CrossPlayEnabled = progress_comp.info.getKeyInt("psn:only_play") != 0,
    });
}

pub fn onXboxSettingRequest(
    txn: *Transaction(pb.XboxSettingRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    txn.respond(.{
        .MatchXboxUser = progress_comp.info.getKeyInt("setting:xbox_match") != 0,
        .XboxBlockUserCreatedContentViewing = progress_comp.info.getKeyInt("setting:xbox_block_ugc") != 0,
    });
}

pub fn onPhotoMemoryRequest(
    txn: *Transaction(pb.PhotoMemoryRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    io: std.Io,
) !void {
    _ = txn.message;
    var items: std.ArrayList(pb.FragmentMemoryItem) = .empty;
    for (assets.reward_index.getRows(io, "PhotoMemory")) |row| {
        const id: i32 = @intCast(row.key);
        const k = try std.fmt.allocPrint(alloc.arena, "photo_mem:{d}", .{id});
        try items.append(alloc.arena, .{
            .Id = id,
            .IsUnlock = progress_comp.info.hasClaim(k),
        });
        if (items.items.len >= 64) break;
    }
    txn.respond(.{ .Item = items });
}

pub fn onSheriffZoneInfoRequest(
    txn: *Transaction(pb.SheriffZoneInfoRequest),
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var zones: std.ArrayList(pb.SheriffZoneInfo) = .empty;
    const clue_cnt = progress_comp.info.countClaimsPrefix("sheriff_clue:");
    if (clue_cnt > 0) {
        try zones.append(alloc.arena, .{ .ZoneId = 1, .ItemCount = clue_cnt });
    }
    txn.respond(.{ .errorCode = .Success, .ZoneInfos = zones });
}

pub fn onWebSignRequest(
    txn: *Transaction(pb.WebSignRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    basic_comp: *const PlayerBasicComponent,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.markClaim(alloc.gpa, "web_sign:claimed");
    try progress_comp.save(fs, alloc.arena);
    const token = try std.fmt.allocPrint(alloc.arena, "session_token_{d}", .{basic_comp.player_id});
    txn.respond(.{ .NoticeSign = token });
}

pub fn onClientPullResourcePackageRequest(txn: *Transaction(pb.ClientPullResourcePackageRequest)) !void {
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onFormationAttrRequest(
    txn: *Transaction(pb.FormationAttrRequest),
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try txn.conn.push(alloc.gpa, io, pb.FormationAttrNotify{
        .Duration = txn.message.Duration,
        .FormationAttrs = txn.message.FormationAttrs,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onCombatMessagePrefixInfoRequest(txn: *Transaction(pb.CombatMessagePrefixInfoRequest)) !void {
    txn.respond(.{ .PrefixInfo = "" });
}

pub fn onPrivateChatOperateRequest(
    txn: *Transaction(pb.PrivateChatOperateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "chat:priv_op:{d}", .{txn.message.TargetPlayerId});
    const op: i64 = if (txn.message.OperateType) |ot| @intFromEnum(ot) else 0;
    try pb_kv.persistInt(alloc, fs, progress_comp, key, op);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEntityPatrolStopRequest(
    txn: *Transaction(pb.EntityPatrolStopRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "wp:patrol_stop:{d}", .{txn.message.EntityId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onGivebackInfoRequest(
    txn: *Transaction(pb.GivebackInfoRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try pb_kv.persistClaim(alloc, fs, progress_comp, "giveback:viewed");
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onValidTimeItemRequest(txn: *Transaction(pb.ValidTimeItemRequest)) !void {
    txn.respond(.{ .ItemList = .empty });
}

pub fn onExchangeRewardRequest(
    txn: *Transaction(pb.ExchangeRewardRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var data: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    var share_counts: std.AutoHashMapUnmanaged(i32, i32) = .empty;
    for (progress_comp.info.dynamic.claims) |c| {
        if (std.mem.startsWith(u8, c, "exchange:")) {
            if (std.fmt.parseInt(i32, c["exchange:".len..], 10)) |cid| {
                try data.append(alloc.arena, .{ .key = cid, .value = 1 });
                if (assets.tables.exchange_reward.getDataById(cid)) |cfg| {
                    if (cfg.SharedId > 0) {
                        const gop = try share_counts.getOrPut(alloc.arena, cfg.SharedId);
                        gop.value_ptr.* = if (gop.found_existing) gop.value_ptr.* + 1 else 1;
                    }
                }
            } else |_| {}
        } else if (std.mem.startsWith(u8, c, "exchange_share:")) {
            if (std.fmt.parseInt(i32, c["exchange_share:".len..], 10)) |sid| {
                const gop = try share_counts.getOrPut(alloc.arena, sid);
                gop.value_ptr.* = if (gop.found_existing) gop.value_ptr.* + 1 else 1;
            } else |_| {}
        }
    }
    var share_data: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    var it = share_counts.iterator();
    while (it.next()) |entry| {
        try share_data.append(alloc.arena, .{ .key = entry.key_ptr.*, .value = entry.value_ptr.* });
    }
    txn.respond(.{
        .ExchangeRewardData = data,
        .ExchangeShareData = share_data,
    });
}

pub fn onUpdatePlayStationBlockAccountRequest(txn: *Transaction(pb.UpdatePlayStationBlockAccountRequest)) !void {
    txn.respond(.{});
}

pub fn onPlayerXboxBlockListRequest(txn: *Transaction(pb.PlayerXboxBlockListRequest)) !void {
    txn.respond(.{});
}

pub fn onTeamRecommendRequest(txn: *Transaction(pb.TeamRecommendRequest)) !void {
    txn.respond(.{ .TeamRecommendList = .empty });
}

pub fn onItemExchangeInfoRequest(
    txn: *Transaction(pb.ItemExchangeInfoRequest),
    alloc: mem.Alloc,
) !void {
    const limit_items = [_]i32{ 3, 50001, 50002, 50005, 50007, 50008 };
    var infos: std.ArrayList(pb.ItemExchangeInfo) = .empty;
    for (limit_items) |item_id| {
        try infos.append(alloc.arena, .{
            .ItemId = item_id,
            .TodayTimes = 0,
            .TotalTimes = 0,
            .DailyLimit = 100,
            .TotalLimit = 0,
        });
    }
    txn.respond(.{ .ItemExchangeInfos = infos });
}

pub fn onPayInfoRequest(txn: *Transaction(pb.PayInfoRequest)) !void {
    txn.respond(.{ .Version = txn.message.Version, .ErrorCode = .Success });
}

pub fn onBtnStateRequest(
    txn: *Transaction(pb.BtnStateRequest),
    alloc: mem.Alloc,
) !void {
    var results: std.ArrayList(pb.ButtonEnableResult) = .empty;
    for (txn.message.Types.items) |bt| {
        try results.append(alloc.arena, .{ .Type = bt, .Enabled = true });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .Type = txn.message.Type,
        .Enabled = true,
        .Result = results,
    });
}

pub fn onButtonIsEnableRequest(
    txn: *Transaction(pb.ButtonIsEnableRequest),
    alloc: mem.Alloc,
) !void {
    var results: std.ArrayList(pb.ButtonEnableResult) = .empty;
    for (txn.message.Types_.items) |button_type| {
        try results.append(alloc.arena, .{ .Type = button_type, .Enable = true });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .Type = txn.message.Type,
        .Enable = true,
        .Result = results,
    });
}

fn resolveLevelPlayExploratoryType(
    assets: *const Assets,
    mark_subtypes: *const std.AutoHashMapUnmanaged(i32, i32),
    level_play_id: i32,
) i32 {
    if (assets.tables.level_play_data.getDataById(level_play_id)) |lp| {
        if (lp.Data.ExploratoryDegree > 0) return lp.Data.ExploratoryDegree;
    }
    return mark_subtypes.get(level_play_id) orelse 0;
}

pub fn onLevelPlayStateRequest(
    txn: *Transaction(pb.LevelPlayStateRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    quest_comp: *const @import("../../logic/component/player/PlayerQuestComponent.zig"),
    progress_comp: *const PlayerProgressComponent,
) !void {
    var mark_subtypes: std.AutoHashMapUnmanaged(i32, i32) = .empty;
    for (assets.tables.map_mark.items) |mark| {
        if (mark.RelativeType == 1 and mark.RelativeId > 0 and mark.RelativeSubType > 0) {
            try mark_subtypes.put(alloc.arena, mark.RelativeId, mark.RelativeSubType);
        }
    }

    const filter_inst = txn.message.ArenaId > 0 or txn.message.InstId > 0;
    var dict: std.ArrayList(pb.MapEntry(i32, pb.LevelPlayStateMsg)) = .empty;
    var seen: std.AutoHashMapUnmanaged(i32, void) = .empty;

    for (quest_comp.info.completed_level_plays) |lpid| {
        if (filter_inst) {
            if (assets.tables.level_play_data.getDataById(lpid)) |lp| {
                if ((txn.message.ArenaId > 0 and lp.Data.LevelId != txn.message.ArenaId) or
                    (txn.message.InstId > 0 and lp.Data.InstanceId != txn.message.InstId))
                {
                    continue;
                }
            }
        }
        if ((try seen.getOrPut(alloc.arena, lpid)).found_existing) continue;
        try dict.append(alloc.arena, .{ .key = lpid, .value = .{
            .LevelPlayEntityId = lpid,
            .ExploratoryType = resolveLevelPlayExploratoryType(assets, &mark_subtypes, lpid),
            .StateType = 2,
            .CompleteNumber = 1,
            .IsUnlocked = true,
            .LevelPlayMarkUnlock = true,
        } });
    }
    for (progress_comp.info.dynamic.claims) |c| {
        if (std.mem.startsWith(u8, c, "lp:open:")) {
            const lpid = std.fmt.parseInt(i32, c["lp:open:".len..], 10) catch continue;
            if (filter_inst) {
                if (assets.tables.level_play_data.getDataById(lpid)) |lp| {
                    if ((txn.message.ArenaId > 0 and lp.Data.LevelId != txn.message.ArenaId) or
                        (txn.message.InstId > 0 and lp.Data.InstanceId != txn.message.InstId))
                    {
                        continue;
                    }
                }
            }
            if ((try seen.getOrPut(alloc.arena, lpid)).found_existing) continue;
            try dict.append(alloc.arena, .{ .key = lpid, .value = .{
                .LevelPlayEntityId = lpid,
                .ExploratoryType = resolveLevelPlayExploratoryType(assets, &mark_subtypes, lpid),
                .StateType = 1,
                .IsUnlocked = true,
                .LevelPlayMarkUnlock = true,
            } });
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .LevelPlayStateDict = dict });
}

pub fn onLevelPlayVarRequest(
    txn: *Transaction(pb.LevelPlayVarRequest),
    alloc: mem.Alloc,
    quest_comp: *const @import("../../logic/component/player/PlayerQuestComponent.zig"),
) !void {
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = .Success,
        .Vars = try @import("world_puzzle.zig").buildLevelPlayVars(alloc.arena, quest_comp),
    });
}

pub fn onSecGetReportData2FlowRequest(txn: *Transaction(pb.SecGetReportData2FlowRequest)) !void {
    if (std.mem.indexOf(u8, txn.message.ReportData, "ZPROBE") != null) {
        std.log.scoped(.zprobe).warn("ReportData ZPROBE: {s}", .{txn.message.ReportData});
    }
    txn.respond(.{
        .Error = .{
            .ErrorCode = .Success,
            .ErrorParams = .empty,
        },
    });
}

pub fn onSceneBlockSplitPlayerNeedBlockPush(_: *Transaction(pb.SceneBlockSplitPlayerNeedBlockPush)) !void {}

pub fn onVersionInfoPush(_: *Transaction(pb.VersionInfoPush)) !void {}

pub fn onBlackScreenStatePush(
    txn: *Transaction(pb.BlackScreenStatePush),
    scene: *Scene,
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    scene.client_black_screen_open = txn.message.IsOpen;
    if (txn.message.IsOpen and
        scene.active_flow_inc_id == 0 and
        std.mem.startsWith(u8, txn.message.BlackScreenReason, "SkipPlot_"))
    {
        scene.client_black_screen_open = false;
        conn.pushBestEffort(alloc.gpa, io, pb.FlowServerSkipNotify{
            .FadeOutScreen = true,
        });
    }
}

pub fn onAdviceSetRequest(
    txn: *Transaction(pb.AdviceSetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "advice:show", if (txn.message.IsShow) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .IsShow = txn.message.IsShow,
        .ErrorCode = .Success,
    });
}

pub fn onEntityFollowTrackRequest(
    txn: *Transaction(pb.EntityFollowTrackRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "wp:follow_track:{d}", .{txn.message.EntityId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFollowTrackRequest(
    txn: *Transaction(pb.FollowTrackRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "wp:follow_track:{d}", .{txn.message.EntityId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEntityOnLandedRequest(txn: *Transaction(pb.EntityOnLandedRequest)) !void {
    txn.respond(.{});
}

pub fn onCombatMessagePostInfoRequest(txn: *Transaction(pb.CombatMessagePostInfoRequest)) !void {
    txn.respond(.{ .PostInfo = "" });
}

pub fn onRoleOperateSelfBgmRequest(
    txn: *Transaction(pb.RoleOperateSelfBgmRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "role_bgm:{d}", .{txn.message.RoleId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, if (txn.message.IsOpen) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .RoleId = txn.message.RoleId,
        .IsOpen = txn.message.IsOpen,
    });
}

fn emptyUsePlan() pb.PhBaOneAllSuitPlan {
    return .{ .SuitPlanList = std.ArrayList(pb.PhBaOneSuitPlan).empty };
}

pub fn onCdKeyVerifyRequest(
    txn: *Transaction(pb.CdKeyVerifyRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const raw = std.mem.trim(u8, txn.message.CdKey, " \t\r\n");
    if (raw.len < 4) {
        txn.respond(.{ .ErrorCode = .ErrCdKeyLength });
        return;
    }
    const valid = std.ascii.eqlIgnoreCase(raw, "WUTHERINGGIFT") or
        std.ascii.eqlIgnoreCase(raw, "SOLARIS3811") or
        std.ascii.eqlIgnoreCase(raw, "WW3811");
    if (!valid) {
        txn.respond(.{ .ErrorCode = .ErrCdKeyNotFound });
        return;
    }
    if (progress_comp.info.hasClaimedCdKey(raw)) {
        txn.respond(.{ .ErrorCode = .ErrCdKeyEachPlayerMaxCount });
        return;
    }
    _ = try progress_comp.info.claimCdKey(alloc.gpa, raw);
    try progress_comp.save(fs, alloc.arena);

    _ = try RewardService.grantItems(.{
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .conn = txn.conn,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    }, 0, true, &.{
        .{ .item_id = 3, .count = 300 },
        .{ .item_id = 50002, .count = 2 },
        .{ .item_id = 2, .count = 50000 },
    }, 2);

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDrownRequest(
    txn: *Transaction(pb.DrownRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "player:drown_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDrownEndTeleportRequest(
    txn: *Transaction(pb.DrownEndTeleportRequest),
    fs: *FileSystem,
    scene: *Scene,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try RoleHelper.reviveAndHealFormation(txn.conn, scene, role_comp, events, fs, alloc, io);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEnterViewDirectionRequest(
    txn: *Transaction(pb.EnterViewDirectionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try pb_kv.persistInt(alloc, fs, progress_comp, "player:view_dir", 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onExitViewDirectionRequest(
    txn: *Transaction(pb.ExitViewDirectionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try pb_kv.persistInt(alloc, fs, progress_comp, "player:view_dir", 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onClientCurrentRoleReportRequest(txn: *Transaction(pb.ClientCurrentRoleReportRequest)) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .CurrentEntityId = txn.message.CurrentEntityId,
        .PlayerId = txn.message.PlayerId,
    });
}

pub fn onCharacterDeathPush(
    txn: *Transaction(pb.CharacterDeathPush),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    scene_opt: *?Scene,
    pc: *PlayerProgressComponent,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    qctx: QuestEngine.QuestContext,
) !void {
    _ = try pc.info.addKeyInt(alloc.gpa, "combat:char_death_cnt", 1);
    try pc.save(fs, alloc.arena);
    if (scene_opt.*) |*scene| {
        var target_nid: i64 = scene.currentRoleEntityId();
        if (txn.message.RoleId > 0) {
            const slice = scene.entities.slice();
            for (slice.items(.entity_id), slice.items(.config)) |eid, cfg| {
                if (cfg.entity_type == .player and cfg.config_type == .character and cfg.config_id == txn.message.RoleId) {
                    target_nid = eid.net_id;
                    break;
                }
            }
        }
        if (scene.net_id_map.get(target_nid)) |idx| {
            const cfg = scene.entities.items(.config)[idx];
            var cpack: std.ArrayList(pb.CombatReceiveData) = .empty;
            if (scene.entities.items(.attribute)[idx]) |*attr| {
                _ = attributes_helper.set_life_unconditional(attr, 0);
                var changed: std.ArrayList(pb.EAttributeType) = .empty;
                try changed.append(alloc.arena, .Life);
                try attributes_helper.generate_attr_messages(&cpack, target_nid, attr, &changed, alloc, io);
            }
            try scene.setBattleEntityActive(alloc.gpa, target_nid, false);
            try cpack.append(alloc.arena, .{ .Message = .{ .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = target_nid },
                .Message = .{ .EntityLivingStatusNotify = .{
                    .Id = target_nid,
                    .LivingStatus = .Dead,
                } },
            } } });
            if (cpack.items.len > 0) {
                try txn.conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = cpack });
            }
            if (cfg.config_type == .character) {
                if (role_comp.role_map.getPtr(cfg.config_id)) |role_info| {
                    role_info.current_hp = 0;
                    events.enqueueBestEffort(.role_info_modified, .{ .role_id = cfg.config_id });
                    RoleHelper.saveRoleInfo(fs, alloc.arena, role_comp, cfg.config_id) catch {};
                }
            }
            events.enqueueBestEffort(.update_formations, .{});
            RoleHelper.maybePushPartyWipeNotify(txn.conn, scene, alloc, io);
            scene.markFsmDirty(alloc.gpa, target_nid, Scene.Entity.FsmComponent.WakeReason.attribute) catch {};
        }
    }
    QuestEngine.onConditionStateChanged(qctx) catch {};
}
pub fn onPlayerNetStatePush(_: *Transaction(pb.PlayerNetStatePush)) !void {}
pub fn onRobotDestinationPush(_: *Transaction(pb.RobotDestinationPush)) !void {}
pub fn onNetTestDataPush(_: *Transaction(pb.NetTestDataPush)) !void {}
pub fn onMaxMessageIdPush(_: *Transaction(pb.MaxMessageIdPush)) !void {}

pub fn onPlayerGameTokenRequest(
    txn: *Transaction(pb.PlayerGameTokenRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    _ = txn.message;
    const existing = progress_comp.info.getKeyStr("gm:sign_token");
    const token = if (existing.len >= 16) existing else blk: {
        var random_bytes: [12]u8 = undefined;
        std.Io.random(io, &random_bytes);
        const hex = std.fmt.bytesToHex(random_bytes, .lower);
        const dup = try alloc.arena.dupe(u8, &hex);
        try progress_comp.info.setKeyStr(alloc.gpa, "gm:sign_token", dup);
        try progress_comp.save(fs, alloc.arena);
        break :blk dup;
    };
    txn.respond(.{ .GameToken = token });
}
