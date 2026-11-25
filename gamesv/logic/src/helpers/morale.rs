use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_proto::pb::MoraleInfoNotify;

use crate::component::player::{
    PlayerBasicComponent, PlayerInventoryComponent, PlayerProgressComponent,
};
use crate::sink::PushSink;

pub const DEFAULT_KEEP_ITEM_ID: i32 = 47;
pub const DEFAULT_TEMP_ITEM_ID: i32 = 48;
pub const DEFAULT_BATTLE_SCORE_ID: i32 = 50001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayConfig {
    pub play_id: i32,
    pub keep_item_id: i32,
    pub temp_item_id: i32,
    pub battle_score_id: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SyncResult {
    pub level: i32,
    pub exp: i32,
    pub temp_level: i32,
    pub temp_exp: i32,
    pub is_start: bool,
    pub changed: bool,
}

pub fn resolve_play_config(assets: Option<&Assets>, play_id: i32) -> PlayConfig {
    let target_id = if play_id > 0 { play_id } else { 1 };
    if let Some(a) = assets
        && let Some(row) = a.tables.morale_play.get_data_by_id(&target_id)
    {
        let keep_id = row
            .ExpItemId
            .first()
            .copied()
            .filter(|&v| v > 0)
            .unwrap_or(DEFAULT_KEEP_ITEM_ID);
        let temp_id = row
            .ExpItemId
            .get(1)
            .copied()
            .filter(|&v| v > 0)
            .unwrap_or(DEFAULT_TEMP_ITEM_ID);
        let score_id = if row.BattleScoreId > 0 {
            row.BattleScoreId
        } else {
            DEFAULT_BATTLE_SCORE_ID
        };
        return PlayConfig {
            play_id: row.Id,
            keep_item_id: keep_id,
            temp_item_id: temp_id,
            battle_score_id: score_id,
        };
    }
    PlayConfig {
        play_id: target_id,
        keep_item_id: DEFAULT_KEEP_ITEM_ID,
        temp_item_id: DEFAULT_TEMP_ITEM_ID,
        battle_score_id: DEFAULT_BATTLE_SCORE_ID,
    }
}

pub fn compute_keep_level(assets: Option<&Assets>, exp: i32) -> i32 {
    let Some(a) = assets else {
        return 1;
    };
    if a.tables.morale_keep_level.items.is_empty() {
        return 1;
    }
    let mut max_level = 1;
    let mut level = 1;
    for row in &a.tables.morale_keep_level.items {
        max_level = max_level.max(row.Level);
        if row.Level > 0 && exp >= row.Experience {
            level = level.max(row.Level + 1);
        }
    }
    level.min(max_level)
}

pub fn compute_temp_level(assets: Option<&Assets>, battle_score_id: i32, temp_exp: i32) -> i32 {
    let Some(a) = assets else {
        return 0;
    };
    let Some(score_cfg) = a.tables.battle_score_conf.get_data_by_id(&battle_score_id) else {
        return 0;
    };
    let mut level = 0;
    for row in &a.tables.battle_score_level_conf.items {
        if row.GroupId != score_cfg.LevelGroupId {
            continue;
        }
        if let Some(&lower) = row.LowerUpperLimits.first()
            && temp_exp >= lower
        {
            level = level.max(row.Level);
        }
    }
    level
}

pub fn resolve_current_keep_level(
    assets: Option<&Assets>,
    inventory_comp: &PlayerInventoryComponent,
    progress_comp: Option<&PlayerProgressComponent>,
) -> i32 {
    let active_play = progress_comp
        .map(|pc| pc.info.get_key_int("morale:active_play"))
        .filter(|&v| v > 0 && v <= i64::from(i32::MAX))
        .map_or(1, |v| v as i32);
    let cfg = resolve_play_config(assets, active_play);
    let keep_exp = inventory_comp.info.normal_item_count(cfg.keep_item_id);
    let computed_level = compute_keep_level(assets, keep_exp);
    let saved_level = progress_comp
        .map(|pc| pc.info.get_key_int("morale:level"))
        .filter(|&v| v > 0)
        .map_or(1, |v| v.min(i64::from(i32::MAX)) as i32);
    if keep_exp > 0 {
        computed_level.max(saved_level)
    } else {
        saved_level
    }
}

#[allow(clippy::too_many_arguments)]
pub fn sync_from_inventory<S: PushSink + ?Sized>(
    sink: Option<&mut S>,
    fs: &FileSystem,
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    inventory_comp: &PlayerInventoryComponent,
    progress_comp: Option<&mut PlayerProgressComponent>,
    play_id_opt: Option<i32>,
    is_start_override: Option<bool>,
    push_notify: bool,
) -> io::Result<SyncResult> {
    let resolved_play_id = play_id_opt.unwrap_or_else(|| {
        progress_comp
            .as_ref()
            .map(|pc| pc.info.get_key_int("morale:active_play"))
            .filter(|&v| v > 0 && v <= i64::from(i32::MAX))
            .map_or(1, |v| v as i32)
    });
    let cfg = resolve_play_config(Some(assets), resolved_play_id);
    let keep_exp = inventory_comp.info.normal_item_count(cfg.keep_item_id);
    let temp_exp = inventory_comp.info.normal_item_count(cfg.temp_item_id);
    let level = compute_keep_level(Some(assets), keep_exp);
    let temp_level = compute_temp_level(Some(assets), cfg.battle_score_id, temp_exp);

    let mut is_start = is_start_override.unwrap_or(false);
    let mut changed = is_start_override.is_some();

    if let Some(pc) = progress_comp {
        let play_key = format!("morale:play:{}", cfg.play_id);
        if is_start_override.is_none() {
            is_start = pc.info.get_key_int(&play_key) > 0;
        } else {
            pc.info.set_key_int(&play_key, if is_start { 1 } else { 0 });
            pc.info.set_key_int(
                "morale:active_play",
                if is_start { i64::from(cfg.play_id) } else { 0 },
            );
        }

        let prev_level = pc.info.get_key_int("morale:level");
        let prev_exp = pc.info.get_key_int("morale:exp");
        let prev_temp_lvl = pc.info.get_key_int("morale:temp_level");
        let prev_temp_exp = pc.info.get_key_int("morale:temp_exp");
        if prev_level != i64::from(level)
            || prev_exp != i64::from(keep_exp)
            || prev_temp_lvl != i64::from(temp_level)
            || prev_temp_exp != i64::from(temp_exp)
            || is_start_override.is_some()
        {
            pc.info.set_key_int("morale:level", i64::from(level));
            pc.info.set_key_int("morale:exp", i64::from(keep_exp));
            pc.info
                .set_key_int("morale:temp_level", i64::from(temp_level));
            pc.info.set_key_int("morale:temp_exp", i64::from(temp_exp));
            pc.save(fs)?;
            if prev_level != i64::from(level) {
                changed = true;
            }
        }
    }

    if push_notify && let Some(s) = sink {
        s.push(
            MoraleInfoNotify::MSG_ID,
            &MoraleInfoNotify {
                Reason: 0,
                TempMoraleExp: temp_exp,
                IndomitableLevel: 1,
                IsStart: is_start,
                KeepMoraleLevel: level,
                ExpRatio: 100,
                MoraleConfigId: cfg.play_id,
                PlayerId: basic_comp.player_id,
                TempMoraleLevel: temp_level,
                KeepMoraleExp: keep_exp,
            },
        );
    }

    Ok(SyncResult {
        level,
        exp: keep_exp,
        temp_level,
        temp_exp,
        is_start,
        changed,
    })
}
