const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Assets = @import("../../data/Assets.zig");
const Connection = @import("../../network/Connection.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");

pub const default_keep_item_id: i32 = 47;
pub const default_temp_item_id: i32 = 48;
pub const default_battle_score_id: i32 = 50001;

pub const PlayConfig = struct {
    play_id: i32,
    keep_item_id: i32,
    temp_item_id: i32,
    battle_score_id: i32,
};

pub const SyncResult = struct {
    level: i32,
    exp: i32,
    temp_level: i32,
    temp_exp: i32,
    is_start: bool,
    changed: bool,
};

/// Resolves the MoralePlay configuration and its permanent/temporary EXP item IDs.
pub fn resolvePlayConfig(assets: ?*const Assets, play_id: i32) PlayConfig {
    const target_id: i32 = if (play_id > 0) play_id else 1;
    if (assets) |a| {
        if (a.tables.morale_play.getDataById(target_id)) |row| {
            const keep_id = if (row.ExpItemId.len >= 1 and row.ExpItemId[0] > 0) row.ExpItemId[0] else default_keep_item_id;
            const temp_id = if (row.ExpItemId.len >= 2 and row.ExpItemId[1] > 0) row.ExpItemId[1] else default_temp_item_id;
            const score_id = if (row.BattleScoreId > 0) row.BattleScoreId else default_battle_score_id;
            return .{
                .play_id = row.Id,
                .keep_item_id = keep_id,
                .temp_item_id = temp_id,
                .battle_score_id = score_id,
            };
        }
    }
    return .{
        .play_id = target_id,
        .keep_item_id = default_keep_item_id,
        .temp_item_id = default_temp_item_id,
        .battle_score_id = default_battle_score_id,
    };
}

/// Computes the permanent Morale level from `MoraleKeepLevel`.
/// Each row `Level = L, Experience = E` marks the cumulative EXP needed to advance from `L` to `L + 1`.
pub fn computeKeepLevel(assets: ?*const Assets, exp: i32) i32 {
    const a = assets orelse return 1;
    if (a.tables.morale_keep_level.items.len == 0) return 1;

    var max_level: i32 = 1;
    var level: i32 = 1;
    for (a.tables.morale_keep_level.items) |row| {
        if (row.Level > max_level) max_level = row.Level;
        if (row.Level > 0 and exp >= row.Experience) {
            level = @max(level, row.Level + 1);
        }
    }
    return @min(level, max_level);
}

/// Computes the temporary Morale level from `BattleScoreConf` + `BattleScoreLevelConf`.
pub fn computeTempLevel(assets: ?*const Assets, battle_score_id: i32, temp_exp: i32) i32 {
    const a = assets orelse return 0;
    const score_cfg = a.tables.battle_score_conf.getDataById(battle_score_id) orelse return 0;
    var level: i32 = 0;
    for (a.tables.battle_score_level_conf.items) |row| {
        if (row.GroupId != score_cfg.LevelGroupId) continue;
        if (row.LowerUpperLimits.len >= 1 and temp_exp >= row.LowerUpperLimits[0]) {
            level = @max(level, row.Level);
        }
    }
    return level;
}

/// Returns the player's current keep Morale level from inventory EXP or persisted progress state.
pub fn resolveCurrentKeepLevel(
    assets: ?*const Assets,
    inventory_comp: *const PlayerInventoryComponent,
    progress_comp: ?*const PlayerProgressComponent,
) i32 {
    const cfg = resolvePlayConfig(assets, if (progress_comp) |pc| blk: {
        const active_play = pc.info.getKeyInt("morale:active_play");
        break :blk if (active_play > 0 and active_play <= std.math.maxInt(i32)) @intCast(active_play) else 1;
    } else 1);
    const keep_exp = inventory_comp.info.normalItemCount(cfg.keep_item_id);
    const computed_level = computeKeepLevel(assets, keep_exp);
    const saved_level: i32 = if (progress_comp) |pc| blk: {
        const v = pc.info.getKeyInt("morale:level");
        break :blk if (v > 0) @intCast(@min(v, std.math.maxInt(i32))) else 1;
    } else 1;
    return if (keep_exp > 0) @max(computed_level, saved_level) else saved_level;
}

/// Recomputes permanent/temporary Morale state from inventory items, persists progress keys, and optionally pushes `MoraleInfoNotify`.
pub fn syncFromInventory(
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    conn: ?*Connection,
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    inventory_comp: *const PlayerInventoryComponent,
    progress_comp: ?*PlayerProgressComponent,
    play_id_opt: ?i32,
    is_start_override: ?bool,
    push_notify: bool,
) !SyncResult {
    const resolved_play_id: i32 = play_id_opt orelse if (progress_comp) |pc| blk: {
        const active = pc.info.getKeyInt("morale:active_play");
        break :blk if (active > 0 and active <= std.math.maxInt(i32)) @intCast(active) else 1;
    } else 1;
    const cfg = resolvePlayConfig(assets, resolved_play_id);
    const keep_exp = inventory_comp.info.normalItemCount(cfg.keep_item_id);
    const temp_exp = inventory_comp.info.normalItemCount(cfg.temp_item_id);
    const level = computeKeepLevel(assets, keep_exp);
    const temp_level = computeTempLevel(assets, cfg.battle_score_id, temp_exp);

    var is_start = is_start_override orelse false;
    var changed = is_start_override != null;

    if (progress_comp) |pc| {
        const play_key = try std.fmt.allocPrint(alloc.arena, "morale:play:{d}", .{cfg.play_id});
        if (is_start_override == null) {
            is_start = pc.info.getKeyInt(play_key) > 0;
        } else {
            try pc.info.setKeyInt(alloc.gpa, play_key, if (is_start) 1 else 0);
            try pc.info.setKeyInt(alloc.gpa, "morale:active_play", if (is_start) cfg.play_id else 0);
        }

        const prev_level = pc.info.getKeyInt("morale:level");
        const prev_exp = pc.info.getKeyInt("morale:exp");
        const prev_temp_lvl = pc.info.getKeyInt("morale:temp_level");
        const prev_temp_exp = pc.info.getKeyInt("morale:temp_exp");
        if (prev_level != level or prev_exp != keep_exp or prev_temp_lvl != temp_level or prev_temp_exp != temp_exp or is_start_override != null) {
            try pc.info.setKeyInt(alloc.gpa, "morale:level", level);
            try pc.info.setKeyInt(alloc.gpa, "morale:exp", keep_exp);
            try pc.info.setKeyInt(alloc.gpa, "morale:temp_level", temp_level);
            try pc.info.setKeyInt(alloc.gpa, "morale:temp_exp", temp_exp);
            try pc.save(fs, alloc.arena);
            if (prev_level != level) changed = true;
        }
    }

    if (push_notify) {
        if (conn) |c| {
            try c.push(alloc.gpa, io, pb.MoraleInfoNotify{
                .Reason = 0,
                .TempMoraleExp = temp_exp,
                .IndomitableLevel = 1,
                .IsStart = is_start,
                .KeepMoraleLevel = level,
                .ExpRatio = 100,
                .MoraleConfigId = cfg.play_id,
                .PlayerId = basic_comp.player_id,
                .TempMoraleLevel = temp_level,
                .KeepMoraleExp = keep_exp,
            });
        }
    }

    return .{
        .level = level,
        .exp = keep_exp,
        .temp_level = temp_level,
        .temp_exp = temp_exp,
        .is_start = is_start,
        .changed = changed,
    };
}
