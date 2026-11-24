const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Transaction = @import("../handlers.zig").Transaction;
const Connection = @import("../Connection.zig");
const Assets = @import("../../data/Assets.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");

const FileSystem = common.FileSystem;
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const Scene = @import("../../logic/Scene.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const GravityHelper = @import("../../logic/helpers/gravity.zig");
const lahai_roi_dungeon_id = 906;

fn appendTrackedIds(arena: std.mem.Allocator, map_comp: *const PlayerMapComponent) !std.ArrayList(i32) {
    var ids: std.ArrayList(i32) = .empty;
    try ids.appendSlice(arena, map_comp.info.tracked_mark_ids);
    return ids;
}

fn appendCustomMarks(arena: std.mem.Allocator, map_comp: *const PlayerMapComponent) !std.ArrayList(pb.MarkPointInfo) {
    var marks: std.ArrayList(pb.MarkPointInfo) = .empty;
    for (map_comp.info.custom_marks) |saved_mark| {
        try marks.append(arena, saved_mark.toPb());
    }
    return marks;
}

fn appendExploreProgressRewardIds(arena: std.mem.Allocator, map_comp: *const PlayerMapComponent) !std.ArrayList(i32) {
    var reward_ids: std.ArrayList(i32) = .empty;
    try reward_ids.appendSlice(arena, map_comp.info.claimed_area_rewards);
    return reward_ids;
}

const CountryExploreScoreData = struct {
    score: i32,
    received: std.ArrayList(pb.CountryExploreScoreReceived),
};

fn computeAreaExplorePercent(
    assets: *const Assets,
    map_comp: *const PlayerMapComponent,
    progress_comp: *const PlayerProgressComponent,
    area_id: i32,
) i32 {
    var pct: i32 = 0;

    // 1. Progress from claimed area stage rewards (25/50/75/100)
    for (map_comp.info.claimed_area_rewards) |rid| {
        if (assets.tables.explore_progress_reward.getDataById(rid)) |cfg| {
            if (cfg.Area == area_id and cfg.NeedExploreProgress > pct) {
                pct = cfg.NeedExploreProgress;
            }
        }
    }

    // 2. Progress from claimed area score steps (20/40/60/80/100)
    if (assets.tables.explore_score.getDataById(area_id)) |es| {
        var kbuf: [64]u8 = undefined;
        var it = es.Score.map.iterator();
        while (it.next()) |kv| {
            const step = kv.key_ptr.*;
            const key = std.fmt.bufPrint(&kbuf, "explore_score:{d}:{d}", .{ area_id, step }) catch continue;
            if (progress_comp.info.hasClaim(key) and step > pct) {
                pct = step;
            }
        }
    }

    // 3. Progress from unlocked teleporters belonging to this area
    var total_tp: i32 = 0;
    var unlocked_tp: i32 = 0;
    for (assets.tables.teleporter.items) |tp| {
        const lcfg = assets.findLevelEntity(tp.MapId, tp.TeleportEntityConfigId) orelse continue;
        if (lcfg.AreaId != area_id) continue;
        total_tp += 1;
        for (map_comp.info.unlocked_teleporters) |uid| {
            if (uid == tp.Id) {
                unlocked_tp += 1;
                break;
            }
        }
    }
    if (total_tp > 0) {
        const tp_pct = @divTrunc(unlocked_tp * 100, total_tp);
        if (tp_pct > pct) pct = tp_pct;
    }

    // 4. Minimum non-zero progress when area is unlocked
    if (pct == 0) {
        for (map_comp.info.unlocked_areas) |aid| {
            if (aid == area_id) {
                pct = 10;
                break;
            }
        }
    }

    return std.math.clamp(pct, 0, 100);
}

fn computeCountryExploreScore(
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    country_id: i32,
) i32 {
    var total_score: i32 = 0;
    var kbuf: [64]u8 = undefined;
    for (assets.tables.area.items) |area| {
        if (area.CountryId != country_id) continue;
        const es = assets.tables.explore_score.getDataById(area.AreaId) orelse continue;
        var it = es.Score.map.iterator();
        while (it.next()) |kv| {
            const step = kv.key_ptr.*;
            const pts = kv.value_ptr.*;
            const key = std.fmt.bufPrint(&kbuf, "explore_score:{d}:{d}", .{ area.AreaId, step }) catch continue;
            if (progress_comp.info.hasClaim(key)) {
                total_score += pts;
            }
        }
    }
    return total_score;
}

fn appendMaxExploreLevelEntries(
    arena: std.mem.Allocator,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    country_filter: ?i32,
) !std.ArrayList(pb.CountryExploreLevel) {
    var levels: std.ArrayList(pb.CountryExploreLevel) = .empty;

    for (assets.tables.explore_reward.items) |reward| {
        if (country_filter) |country_id| {
            if (reward.Country != country_id) continue;
        }

        const already = for (levels.items) |entry| {
            if (entry.CountryId == reward.Country) break true;
        } else false;
        if (already) continue;

        const score = computeCountryExploreScore(assets, progress_comp, reward.Country);
        var lvl: i32 = 0;
        var max_lvl: i32 = 0;
        for (assets.tables.explore_reward.items) |r| {
            if (r.Country != reward.Country) continue;
            if (r.ExploreLevel > max_lvl) max_lvl = r.ExploreLevel;
            if (r.NeedScore > 0 and score >= r.NeedScore and r.ExploreLevel + 1 > lvl) {
                lvl = r.ExploreLevel + 1;
            }
        }
        try levels.append(arena, .{
            .CountryId = reward.Country,
            .ExploreLevel = @min(lvl, max_lvl),
        });
    }

    return levels;
}

fn buildCountryExploreScoreData(
    arena: std.mem.Allocator,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    country_id: i32,
) !CountryExploreScoreData {
    var received: std.ArrayList(pb.CountryExploreScoreReceived) = .empty;
    var total_score: i32 = 0;
    var kbuf: [64]u8 = undefined;

    for (assets.tables.area.items) |area| {
        if (area.CountryId != country_id) continue;
        const es = assets.tables.explore_score.getDataById(area.AreaId) orelse continue;
        var steps: std.ArrayList(i32) = .empty;
        var it = es.Score.map.iterator();
        while (it.next()) |kv| {
            const step = kv.key_ptr.*;
            const pts = kv.value_ptr.*;
            const key = std.fmt.bufPrint(&kbuf, "explore_score:{d}:{d}", .{ area.AreaId, step }) catch continue;
            if (progress_comp.info.hasClaim(key)) {
                total_score += pts;
                try steps.append(arena, step);
            }
        }
        if (steps.items.len > 0) {
            try received.append(arena, .{
                .AreaId = area.AreaId,
                .ExploreProgress = steps,
            });
        }
    }

    return .{
        .score = total_score,
        .received = received,
    };
}

pub fn onMapTraceRequest(
    txn: *Transaction(pb.MapTraceRequest),
    map_comp: *PlayerMapComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
) !void {
    try map_comp.info.trackMark(alloc.gpa, txn.message.MarkId);
    try map_comp.save(fs, alloc.arena);

    txn.respond(.{
        .ErrorCode = .Success,
        .MarkId = txn.message.MarkId,
    });
}

pub fn onMapCancelTraceRequest(
    txn: *Transaction(pb.MapCancelTraceRequest),
    map_comp: *PlayerMapComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
) !void {
    try map_comp.info.untrackMark(alloc.gpa, txn.message.MarkId);
    try map_comp.save(fs, alloc.arena);

    txn.respond(.{
        .ErrorCode = .Success,
        .MarkId = txn.message.MarkId,
    });
}

pub fn onMapTraceInfoRequest(
    txn: *Transaction(pb.MapTraceInfoRequest),
    map_comp: *PlayerMapComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const marks = try appendCustomMarks(alloc.arena, map_comp);
    if (marks.items.len != 0) {
        try txn.conn.push(alloc.gpa, io, pb.MapMarkInfoNotify{ .InfoList = marks });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .MarkIdList = try appendTrackedIds(alloc.arena, map_comp),
    });
}

pub fn onMapMarkRequest(
    txn: *Transaction(pb.MapMarkRequest),
    map_comp: *PlayerMapComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const request_mark = txn.message.MarkPointRequestInfo orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };

    const saved_mark = try map_comp.info.addCustomMark(alloc.gpa, request_mark);
    if (request_mark.IsTrace) {
        try map_comp.info.trackMark(alloc.gpa, saved_mark.id);
    }
    try map_comp.save(fs, alloc.arena);

    const mark = saved_mark.toPb();

    try txn.conn.push(alloc.gpa, io, pb.MapMarkAddNotify{ .Info = mark });

    txn.respond(.{
        .ErrorCode = .Success,
        .Info = mark,
    });
}

pub fn onRemoveMapMarkRequest(
    txn: *Transaction(pb.RemoveMapMarkRequest),
    map_comp: *PlayerMapComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try map_comp.info.deleteCustomMarks(alloc.gpa, txn.message.MarkList.items);
    try map_comp.save(fs, alloc.arena);

    try txn.conn.push(alloc.gpa, io, pb.MapMarkInfoNotify{
        .InfoList = try appendCustomMarks(alloc.arena, map_comp),
    });

    txn.respond(.{
        .ErrorCode = .Success,
        .MarkList = txn.message.MarkList,
    });
}

pub fn onMapRemoveMarkRequest(
    txn: *Transaction(pb.MapRemoveMarkRequest),
    map_comp: *PlayerMapComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try map_comp.info.deleteCustomMarks(alloc.gpa, txn.message.MarkIds.items);
    try map_comp.save(fs, alloc.arena);
    try txn.conn.push(alloc.gpa, io, pb.MapMarkInfoNotify{
        .InfoList = try appendCustomMarks(alloc.arena, map_comp),
    });
    txn.respond(.{ .ErrorCode = .Success, .MarkIds = txn.message.MarkIds });
}

pub fn onMapMarkInfoRequest(
    txn: *Transaction(pb.MapMarkInfoRequest),
    map_comp: *const PlayerMapComponent,
    alloc: mem.Alloc,
) !void {
    const tracked = try appendTrackedIds(alloc.arena, map_comp);
    var show_ids: std.ArrayList(pb.MapMarkShowInfo) = .empty;
    for (tracked.items) |mid| {
        try show_ids.append(alloc.arena, .{ .MarkId = mid, .ShowFlag = 1 });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .InfoList = try appendCustomMarks(alloc.arena, map_comp),
        .ShowMarkIds = show_ids,
    });
}

pub fn onMapRemoveAllMarkRequest(
    txn: *Transaction(pb.MapRemoveAllMarkRequest),
    map_comp: *PlayerMapComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var removed: std.ArrayList(i32) = .empty;
    for (map_comp.info.custom_marks) |mark| {
        if (txn.message.MarkType == 0 or mark.mark_type == txn.message.MarkType) {
            try removed.append(alloc.arena, mark.id);
        }
    }
    if (removed.items.len > 0) {
        try map_comp.info.deleteCustomMarks(alloc.gpa, removed.items);
        try map_comp.save(fs, alloc.arena);
        try txn.conn.push(alloc.gpa, io, pb.MapMarkInfoNotify{
            .InfoList = try appendCustomMarks(alloc.arena, map_comp),
        });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .MarkIdList = removed,
    });
}

pub fn onMapUnlockFieldInfoRequest(
    txn: *Transaction(pb.MapUnlockFieldInfoRequest),
    map_comp: *PlayerMapComponent,
    fs: *FileSystem,
    assets: *const Assets,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    if (try map_comp.info.ensureDefaultUnlocks(alloc.gpa)) {
        try map_comp.save(fs, alloc.arena);
    }

    var fields: std.ArrayList(i32) = .empty;
    for (map_comp.info.unlocked_fogs) |fog| {
        try fields.append(alloc.arena, fog);
    }

    var multi_maps: std.ArrayList(i32) = .empty;
    var map_blocks: std.ArrayList(i32) = .empty;

    for (assets.tables.multi_map.items) |multi| {
        try multi_maps.append(alloc.arena, multi.Id);
    }
    for (assets.tables.map_block_info.items) |block| {
        try map_blocks.append(alloc.arena, block.BlockId);
    }

    try txn.conn.push(alloc.gpa, io, pb.MultiMapInfoNotify{
        .UnlockMultiMapId = multi_maps,
        .UnlockBlockId = map_blocks,
    });

    txn.respond(.{
        .ErrorCode = .Success,
        .FieldId = fields,
    });
}

pub fn unlockFogAndSiblings(
    conn: *Connection,
    assets: *const Assets,
    map_comp: *PlayerMapComponent,
    gpa: std.mem.Allocator,
    io: std.Io,
    fog_id: i32,
) !bool {
    if (fog_id <= 0) return false;
    var changed = false;
    if (try map_comp.info.unlockFog(gpa, fog_id)) {
        changed = true;
        conn.pushBestEffort(gpa, io, pb.MapUnlockFieldNotify{ .FieldId = fog_id });
    }
    if (assets.tables.map_fog.getDataById(fog_id)) |fog_cfg| {
        if (fog_cfg.AreaId > 0) {
            for (assets.tables.map_fog.items) |sibling| {
                if (sibling.AreaId == fog_cfg.AreaId and sibling.Fog > 0) {
                    if (try map_comp.info.unlockFog(gpa, sibling.Fog)) {
                        changed = true;
                        conn.pushBestEffort(gpa, io, pb.MapUnlockFieldNotify{ .FieldId = sibling.Fog });
                    }
                }
            }
        }
    }
    return changed;
}

pub fn onExploreProgressRequest(
    txn: *Transaction(pb.ExploreProgressRequest),
    assets: *const Assets,
    map_comp: *const PlayerMapComponent,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try txn.conn.push(alloc.gpa, io, pb.ExploreLevelNotify{
        .CountryExploreLevel = try appendMaxExploreLevelEntries(alloc.arena, assets, progress_comp, null),
    });

    try txn.conn.push(alloc.gpa, io, pb.ExploreProgressRewardIdsNotify{
        .AreaStageRewardDataList = try appendExploreProgressRewardIds(alloc.arena, map_comp),
    });

    var area_progress: std.ArrayList(pb.AreaExploreInfo) = .empty;

    for (txn.message.AreaIds.items) |area_id| {
        try area_progress.append(alloc.arena, .{
            .AreaId = area_id,
            .ExplorePercent = computeAreaExplorePercent(assets, map_comp, progress_comp, area_id),
        });
    }

    txn.respond(.{
        .AreaProgress = area_progress,
    });
}

pub fn onReceiveAreaStageRewardAsyncRequest(
    txn: *Transaction(pb.ReceiveAreaStageRewardAsyncRequest),
    assets: *const Assets,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var claimed_list: std.ArrayList(i32) = .empty;
    var dirty = false;
    const reward_ctx: RewardService.RewardContext = .{
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .conn = txn.conn,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    };

    for (txn.message.AreaStageRewardDataList.items) |reward_cfg_id| {
        const cfg = assets.tables.explore_progress_reward.getDataById(reward_cfg_id) orelse continue;
        if (try map_comp.info.claimAreaReward(alloc.gpa, reward_cfg_id)) {
            dirty = true;
            if (cfg.DropReward > 0) {
                _ = try RewardService.grantDropPackagePopup(reward_ctx, cfg.DropReward, 19500);
            }
            try claimed_list.append(alloc.arena, reward_cfg_id);
        }
    }
    if (dirty) {
        try map_comp.save(fs, alloc.arena);
    }

    txn.respond(.{
        .AreaStageRewardDataList = claimed_list,
    });
}

pub fn onExploreProgressRewardRequest(
    txn: *Transaction(pb.ExploreProgressRewardRequest),
    assets: *const Assets,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var claimed_ids: std.ArrayList(i32) = .empty;
    var dirty = false;
    const reward_ctx: RewardService.RewardContext = .{
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .conn = txn.conn,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    };

    for (txn.message.RewardIds.items) |reward_id| {
        const cfg = assets.tables.explore_progress_reward.getDataById(reward_id) orelse continue;
        if (!try map_comp.info.claimAreaReward(alloc.gpa, reward_id)) continue;
        dirty = true;
        if (cfg.DropReward > 0) {
            _ = try RewardService.grantDropPackagePopup(reward_ctx, cfg.DropReward, 19500);
        }
        try claimed_ids.append(alloc.arena, reward_id);
    }
    if (dirty) try map_comp.save(fs, alloc.arena);
    txn.respond(.{ .RewardIds = claimed_ids });
}

pub fn onCountryExploreScoreInfoRequest(
    txn: *Transaction(pb.CountryExploreScoreInfoRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const country_id = txn.message.CountryId;
    const score_data = try buildCountryExploreScoreData(alloc.arena, assets, progress_comp, country_id);
    const levels = try appendMaxExploreLevelEntries(alloc.arena, assets, progress_comp, country_id);

    try txn.conn.push(alloc.gpa, io, pb.ExploreLevelNotify{ .CountryExploreLevel = levels });
    txn.respond(.{
        .ExploreScore = score_data.score,
        .CountryExploreScoreReceived = score_data.received,
    });
}

fn claimExploreScoreStep(
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    area_id: i32,
    progress_step: i32,
) !bool {
    const key = try std.fmt.allocPrint(alloc.arena, "explore_score:{d}:{d}", .{ area_id, progress_step });
    if (!try progress_comp.info.markClaim(alloc.gpa, key)) return false;

    const table_id: i64 = if (assets.tables.explore_score.getDataById(area_id) != null)
        area_id
    else
        progress_step;
    _ = try RewardService.grantTableReward(reward_ctx, "ExploreScoreReward", table_id, false, true, 1, &.{
        .{ .item_id = 3, .count = 40 },
        .{ .item_id = 2, .count = 15000 },
    }, null);
    return true;
}

pub fn onExploreScoreRewardRequest(
    txn: *Transaction(pb.ExploreScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    if (try claimExploreScoreStep(alloc, assets, progress_comp, reward_ctx, txn.message.AreaId, txn.message.ExploreProgress)) {
        try progress_comp.save(fs, alloc.arena);
        try txn.conn.push(alloc.gpa, io, pb.ExploreLevelNotify{
            .CountryExploreLevel = try appendMaxExploreLevelEntries(alloc.arena, assets, progress_comp, null),
        });
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMultiExploreScoreRewardRequest(
    txn: *Transaction(pb.MultiExploreScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var dirty = false;
    for (txn.message.nBs.items) |entry| {
        if (try claimExploreScoreStep(alloc, assets, progress_comp, reward_ctx, entry.key, entry.value)) {
            dirty = true;
        }
    }
    if (dirty) {
        try progress_comp.save(fs, alloc.arena);
        try txn.conn.push(alloc.gpa, io, pb.ExploreLevelNotify{
            .CountryExploreLevel = try appendMaxExploreLevelEntries(alloc.arena, assets, progress_comp, null),
        });
    }
    txn.respond(.{
        .ErrorCode = .Success,
    });
}

pub fn onMulExploreScoreRewardRequest(
    txn: *Transaction(pb.MulExploreScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var dirty = false;
    for (txn.message.Rewards.items) |entry| {
        if (try claimExploreScoreStep(alloc, assets, progress_comp, reward_ctx, entry.key, entry.value)) {
            dirty = true;
        }
    }
    if (dirty) {
        try progress_comp.save(fs, alloc.arena);
        try txn.conn.push(alloc.gpa, io, pb.ExploreLevelNotify{
            .CountryExploreLevel = try appendMaxExploreLevelEntries(alloc.arena, assets, progress_comp, null),
        });
    }
    txn.respond(.{ .ErrorCode = .Success });
}

fn trackRangeArea(
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    map_comp: *PlayerMapComponent,
    quest_ctx: QuestEngine.QuestContext,
    range_entity_id: i64,
    is_enter: bool,
    is_player_range: bool,
    trigger_events: bool,
    out_results: *std.ArrayList(pb.MapEntry(i32, pb.ErrorCode)),
    progress_dirty: *bool,
) !void {
    // AccessRangeResultType: Trigger=0, Trample=1, DynamicPortal=2, EffectArea=3, AirPassage=4, Range=5, GravityFlip=6, RollBlock=7
    var area_id: i32 = 0;
    var resolved_in_world = false;
    var has_range_comp = false;
    var has_trample = false;
    var has_portal = false;
    var has_air_passage = false;
    var has_gravity_flip = false;
    var has_gravity_volume = false;
    var resolved_gravity_dir: ?[3]f32 = null;
    var has_roll_block = false;
    var cfg_id: i32 = std.math.cast(i32, range_entity_id) orelse 0;
    var trigger_comp: ?@import("../../data/tables/entity_components/Trigger.zig") = null;
    var client_trigger_comp: ?@import("../../data/tables/entity_components/Extra.zig").ClientTriggerComponent = null;
    if (scene_opt.*) |*scene| {
        if (scene.net_id_map.contains(range_entity_id)) resolved_in_world = true;
        const resolved = scene.findConfigIdByNetId(range_entity_id) orelse range_entity_id;
        cfg_id = std.math.cast(i32, resolved) orelse cfg_id;
        if (assets.findLevelEntity(scene.instance_id, resolved)) |lcfg| {
            resolved_in_world = true;
            area_id = lcfg.AreaId;
            var components = lcfg.ComponentsData;
            if (assets.findTemplateConfig(lcfg.BlueprintType)) |tpl| {
                tpl.ComponentsData.mergeInto(&components);
            }
            has_range_comp = components.RangeComponent != null;
            has_portal = components.PortalComponent != null or components.NoRenderPortalComponent != null;
            if (components.GravityFlipComponent) |gf| {
                has_gravity_flip = !(gf.Disabled orelse false);
                if (has_gravity_flip) {
                    const default_flip: i32 = if (gf.DefaultGravity) |dg|
                        @intCast(@import("../../data/tables/entity_components/Components.zig").gravityFlipFromDefault(dg) orelse 0)
                    else
                        0;
                    const flip_val: i32 = if (scene.net_id_map.get(range_entity_id)) |ridx|
                        (scene.entities.items(.world_item)[ridx].gravity_flip orelse default_flip)
                    else
                        default_flip;
                    resolved_gravity_dir = switch (flip_val) {
                        0 => .{ 0, 0, 1 },
                        1 => .{ 0, 0, -1 },
                        2 => .{ -1, 0, 0 },
                        3 => .{ 1, 0, 0 },
                        else => null,
                    };
                }
            }
            if (resolved_gravity_dir == null and components.BaseInfoComponent != null) {
                const gres = GravityHelper.resolveComponentsGravity(&components, Assets.AssetIndexes.entityWorldRot(lcfg));
                if (gres.configured) {
                    has_gravity_volume = true;
                    resolved_gravity_dir = gres.dir;
                }
            } else if (has_gravity_flip) {
                has_gravity_volume = true;
            }
            trigger_comp = components.TriggerComponent;
            has_air_passage = std.mem.indexOf(u8, lcfg.BlueprintType, "AirPassage") != null;
            has_roll_block = std.mem.indexOf(u8, lcfg.BlueprintType, "RollBlock") != null;
            for ([2][]const @import("../../data/tables/entity_components/Extra.zig").Entry{
                components.extra,
                components.template_extra,
            }) |extra_slice| {
                for (extra_slice) |entry| {
                    switch (entry) {
                        .TrampleComponent => |tc| if (!(tc.Disabled orelse false)) {
                            has_trample = true;
                        },
                        .AirPassageComponent => has_air_passage = true,
                        .RollBlockComponent, .RollBlockFloorComponent, .RollBlockItemComponent => has_roll_block = true,
                        .ClientTriggerComponent => |ct| {
                            client_trigger_comp = ct.*;
                            if (!(ct.Disabled orelse false) and components.SplineComponent != null) {
                                has_air_passage = true;
                            }
                        },
                        else => {},
                    }
                }
            }
        }
        if (resolved_in_world or is_player_range) {
            try out_results.append(alloc.arena, .{ .key = 5, .value = .Success });
        }
        if (is_player_range) {
            if (scene.net_id_map.get(range_entity_id)) |ent_idx| {
                scene.entities.items(.world_item)[ent_idx].in_range_player_id = if (is_enter) scene.player_id else 0;
            }
            if (has_gravity_volume) {
                const role_nid = scene.currentRoleEntityId();
                if (scene.net_id_map.get(role_nid)) |role_idx| {
                    if (is_enter) {
                        if (resolved_gravity_dir) |gdir| {
                            scene.entities.items(.world_item)[role_idx].gravity = .{ .location = gdir };
                        }
                    } else {
                        scene.entities.items(.world_item)[role_idx].gravity = null;
                    }
                }
                try QuestEngine.onConditionStateChanged(quest_ctx);
            }
        }
    } else {
        try out_results.append(alloc.arena, .{ .key = 5, .value = .Success });
    }
    if (has_trample) {
        try out_results.append(alloc.arena, .{ .key = 1, .value = .Success });
    }
    if (has_portal) {
        try out_results.append(alloc.arena, .{ .key = 2, .value = .Success });
    }
    if (has_range_comp) {
        try out_results.append(alloc.arena, .{ .key = 3, .value = .Success });
    }
    if (has_air_passage) {
        try out_results.append(alloc.arena, .{ .key = 4, .value = .Success });
    }
    if (has_gravity_flip) {
        try out_results.append(alloc.arena, .{ .key = 6, .value = .Success });
    }
    if (has_roll_block) {
        try out_results.append(alloc.arena, .{ .key = 7, .value = .Success });
    }

    if (trigger_comp) |trig| {
        if (!(trig.Disabled orelse false)) {
            if (trigger_events) {
                if (scene_opt.*) |*scene| {
                    if (scene.net_id_map.get(range_entity_id)) |ent_idx| {
                        if (is_enter) {
                            scene.entities.items(.world_item)[ent_idx].trigger_count +|= 1;
                        } else {
                            scene.entities.items(.world_item)[ent_idx].exit_trigger_count +|= 1;
                        }
                    }
                }
            }
            try out_results.append(alloc.arena, .{ .key = 0, .value = .Success });
            if (trigger_events) {
                if (try QuestEngine.handleEntityRangeTrigger(quest_ctx, range_entity_id, cfg_id, trig, is_enter)) {
                    progress_dirty.* = true;
                }
            }
        }
    }
    if (client_trigger_comp) |ct| {
        if (trigger_events) {
            try QuestEngine.handleClientTriggerActions(quest_ctx, range_entity_id, cfg_id, ct, is_enter);
            progress_dirty.* = true;
        }
    }

    if (is_enter and area_id > 0) {
        if (try map_comp.info.unlockArea(alloc.gpa, area_id)) {
            try map_comp.save(fs, alloc.arena);
        }
    }
}

pub fn onPlayerAccessRangeRequest(
    txn: *Transaction(pb.PlayerAccessRangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    map_comp: *PlayerMapComponent,
    progress_comp: *PlayerProgressComponent,
    quest_ctx: QuestEngine.QuestContext,
) !void {
    const rtype = txn.message.AccessRangeType orelse .RangeEnter;
    const is_enter = rtype == .RangeEnter or rtype == .RangeInit;
    var results: std.ArrayList(pb.MapEntry(i32, pb.ErrorCode)) = .empty;
    var progress_dirty = false;
    try trackRangeArea(alloc, fs, assets, scene_opt, map_comp, quest_ctx, txn.message.RangeEntityId, is_enter, true, true, &results, &progress_dirty);
    if (progress_dirty) {
        try progress_comp.save(fs, alloc.arena);
    }

    txn.respond(.{
        .AccessRangeResult = .{
            .AccessEntityId = txn.message.RangeEntityId,
            .AccessRangeType = txn.message.AccessRangeType,
            .AccessRangeResults = results,
        },
        .RangeEntityId = txn.message.RangeEntityId,
        .ErrorCode = .Success,
    });
}

pub fn onEntityAccessRangeRequest(
    txn: *Transaction(pb.EntityAccessRangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    map_comp: *PlayerMapComponent,
    progress_comp: *PlayerProgressComponent,
    quest_ctx: QuestEngine.QuestContext,
) !void {
    const rtype = txn.message.RangeType orelse .RangeEnter;
    const is_enter = rtype == .RangeEnter or rtype == .RangeInit;
    var infos: std.ArrayList(pb.EntityAccessInfo) = .empty;
    var range_results: std.ArrayList(pb.MapEntry(i32, pb.ErrorCode)) = .empty;
    var progress_dirty = false;
    try trackRangeArea(alloc, fs, assets, scene_opt, map_comp, quest_ctx, txn.message.EntityId, is_enter, false, false, &range_results, &progress_dirty);
    const next_val: i64 = if (is_enter) 1 else 0;
    for (txn.message.EntitiesToCheck.items) |eid| {
        const ekey = try std.fmt.allocPrint(alloc.arena, "range:ent:{d}:{d}", .{ txn.message.EntityId, eid });
        if (progress_comp.info.getKeyInt(ekey) != next_val) {
            try progress_comp.info.setKeyInt(alloc.gpa, ekey, next_val);
            progress_dirty = true;
        }
        if (scene_opt.*) |*scene| {
            if (scene.net_id_map.get(txn.message.EntityId)) |ridx| {
                var wi = &scene.entities.items(.world_item)[ridx];
                if (is_enter) {
                    var found = false;
                    const n_ents = @min(@as(usize, wi.in_range_entity_len), wi.in_range_entity_ids.len);
                    for (wi.in_range_entity_ids[0..n_ents]) |cur| {
                        if (cur == eid) {
                            found = true;
                            break;
                        }
                    }
                    if (!found and wi.in_range_entity_len < wi.in_range_entity_ids.len) {
                        wi.in_range_entity_ids[wi.in_range_entity_len] = eid;
                        wi.in_range_entity_len += 1;
                    }
                } else {
                    var i: usize = 0;
                    while (i < wi.in_range_entity_len) {
                        if (wi.in_range_entity_ids[i] == eid) {
                            wi.in_range_entity_len -= 1;
                            wi.in_range_entity_ids[i] = wi.in_range_entity_ids[wi.in_range_entity_len];
                            wi.in_range_entity_ids[wi.in_range_entity_len] = 0;
                        } else {
                            i += 1;
                        }
                    }
                }
            }
        }
        try infos.append(alloc.arena, .{
            .EntityId = eid,
            .RangeType = txn.message.RangeType,
            .AcessRangeResults = range_results,
        });
    }
    if (progress_dirty) {
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .EntityId = txn.message.EntityId,
        .Info = infos,
    });
}

pub fn onInitRangeRequest(
    txn: *Transaction(pb.InitRangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    map_comp: *PlayerMapComponent,
    progress_comp: *PlayerProgressComponent,
    quest_ctx: QuestEngine.QuestContext,
) !void {
    var range_results: std.ArrayList(pb.MapEntry(i32, pb.ErrorCode)) = .empty;
    var progress_dirty = false;
    try trackRangeArea(alloc, fs, assets, scene_opt, map_comp, quest_ctx, txn.message.EntityId, txn.message.IsPlayerInRange, true, txn.message.IsPlayerInRange, &range_results, &progress_dirty);
    var infos: std.ArrayList(pb.EntityAccessInfo) = .empty;
    for (txn.message.EntitiesToRequest.items) |eid| {
        const ekey = try std.fmt.allocPrint(alloc.arena, "range:ent:{d}:{d}", .{ txn.message.EntityId, eid });
        if (progress_comp.info.getKeyInt(ekey) != 1) {
            try progress_comp.info.setKeyInt(alloc.gpa, ekey, 1);
            progress_dirty = true;
        }
        try infos.append(alloc.arena, .{
            .EntityId = eid,
            .RangeType = .RangeInit,
            .AcessRangeResults = range_results,
        });
    }
    if (progress_dirty) {
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .EntityId = txn.message.EntityId,
        .Info = infos,
        .PlayerAccessRangeResult = .{
            .EntityId = txn.message.EntityId,
            .RangeType = .RangeInit,
            .AcessRangeResults = range_results,
        },
    });
}

pub fn onInfrV2InfoRequest(
    txn: *Transaction(pb.InfrV2InfoRequest),
    assets: *const Assets,
    alloc: mem.Alloc,
) !void {
    var tree_list: std.ArrayList(pb.InfrV2OneTree) = .empty;

    for (assets.tables.infr_v2_tree_build.items) |cfg| {
        try tree_list.append(alloc.arena, .{
            .TreeId = cfg.Id,
            .status = .InfrV2StatusComplete,
            .CompleteTime = 0,
        });
    }

    const village_data = pb.InfrV2FirePb{
        .FireExp = 0,
        .FireLevel = 1,
        .FireLevelReachTime = 0,
        .FireStatus = .InfrV2StatusComplete,
    };

    const infr_data = pb.InfrV2Pb{
        .FireInfo = village_data,
        .TreeInfo = .{
            .Trees = tree_list,
            .ManualTraceTree = 0,
        },
    };

    txn.respond(.{
        .ErrorCode = .Success,
        .InfrInfo = infr_data,
    });
}

pub fn onInfrInfoRequest(
    txn: *Transaction(pb.InfrInfoRequest),
    assets: *const Assets,
    alloc: mem.Alloc,
) !void {
    var roads: std.ArrayList(pb.InfrOneRoad) = .empty;
    for (assets.tables.infr_road_build.items) |cfg| {
        if (cfg.DungeonId != lahai_roi_dungeon_id) continue;

        try roads.append(alloc.arena, .{
            .RoadId = cfg.Id,
            .status = .InfrStatusComplete,
            .CompleteTime = 0,
            .TotalGiftCount = 0,
            .LastGiftTime = 0,
        });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .InfrInfo = .{
            .RoadInfo = .{ .Roads = roads },
        },
    });
}

pub fn onMapTravelLevelUpRequest(
    txn: *Transaction(pb.MapTravelLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const next_lvl = @as(i32, @intCast(try progress_comp.info.addKeyInt(alloc.gpa, "map_travel:lvl", 1)));
    try progress_comp.save(fs, alloc.arena);
    _ = try RewardService.grantTableReward(
        reward_ctx,
        "MapTravelLevel",
        next_lvl,
        false,
        true,
        1,
        &.{ .{ .item_id = 3, .count = 50 }, .{ .item_id = 2, .count = 10000 } },
        null,
    );
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMapTravelFullRewardRequest(
    txn: *Transaction(pb.MapTravelFullRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    io: std.Io,
) !void {
    if (try progress_comp.info.markClaim(alloc.gpa, "map_travel:full_reward")) {
        try progress_comp.save(fs, alloc.arena);
        const rows = assets.reward_index.getRows(io, "MapTravelLevel");
        const cfg_key: i64 = if (rows.len > 0) rows[0].key else 1;
        _ = try RewardService.grantTableReward(
            reward_ctx,
            "MapTravelLevel",
            cfg_key,
            false,
            true,
            1,
            &.{
                .{ .item_id = 3, .count = 160 },
                .{ .item_id = 2, .count = 30000 },
            },
            null,
        );
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMulMapTravelSoarRewardRequest(
    txn: *Transaction(pb.MulMapTravelSoarRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var claimed: std.ArrayList(i32) = .empty;
    var dirty = false;
    var kbuf: [48]u8 = undefined;
    for (txn.message.TaskIds.items) |tid| {
        const k = try std.fmt.bufPrint(&kbuf, "map_travel_soar:{d}", .{tid});
        if (!(try progress_comp.info.markClaim(alloc.gpa, k))) continue;
        dirty = true;
        try claimed.append(alloc.arena, tid);
        _ = try RewardService.grantTableReward(reward_ctx, "MapTravelTask", tid, false, true, 1, &.{.{ .item_id = 3, .count = 30 }}, null);
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .TaskIds = claimed });
}

pub fn onMulMapTravelTaskRewardRequest(
    txn: *Transaction(pb.MulMapTravelTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var claimed: std.ArrayList(i32) = .empty;
    var dirty = false;
    var kbuf: [48]u8 = undefined;
    for (txn.message.TaskId.items) |tid| {
        const k = try std.fmt.bufPrint(&kbuf, "map_travel_task:{d}", .{tid});
        if (!(try progress_comp.info.markClaim(alloc.gpa, k))) continue;
        dirty = true;
        try claimed.append(alloc.arena, tid);
        _ = try RewardService.grantTableReward(reward_ctx, "MapTravelTask", tid, false, true, 1, &.{.{ .item_id = 3, .count = 30 }}, null);
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .TaskId = claimed });
}

pub fn onMapReplaceMarkRequest(
    txn: *Transaction(pb.MapReplaceMarkRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    map_comp: *PlayerMapComponent,
) !void {
    for (map_comp.info.custom_marks) |*mark| {
        if (mark.id == txn.message.MarkId) {
            mark.config_id = txn.message.ConfigId;
            try map_comp.save(fs, alloc.arena);
            break;
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .MarkId = txn.message.MarkId,
        .ConfigId = txn.message.ConfigId,
    });
}

pub fn onMapMarkInfoUpdateRequest(
    txn: *Transaction(pb.MapMarkInfoUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    map_comp: *PlayerMapComponent,
) !void {
    if (txn.message.Pos) |pos| {
        for (map_comp.info.custom_marks) |*mark| {
            if (mark.id == txn.message.MarkId) {
                mark.x = pos.X;
                mark.y = pos.Y;
                mark.z = pos.Z;
                try map_comp.save(fs, alloc.arena);
                break;
            }
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}
