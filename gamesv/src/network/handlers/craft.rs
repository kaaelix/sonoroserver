use std::collections::BTreeMap;

use sonoro_gamesv_fs::progress_info::{IdRoleCount, ProgressInfo};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{consume, craft, quest_engine};
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    BatchSynthesisItemRequest, BatchSynthesisItemResponse, CookFoodRequest, CookFoodResponse,
    CookFormulaRequest, CookFormulaResponse, CookingDataRequest, CookingDataResponse, CookingInfo,
    CookingInfoUpdateNotify, ForgeFormulaUnlockRequest, ForgeFormulaUnlockResponse,
    ForgeInfoRequest, ForgeInfoResponse, ForgeItemInfoUpdateNotify, ForgeItemRequest,
    ForgeItemResponse, OneForgeConfig, OneForgeInfo, OneFormulaConfig, OneSynthesisConfig,
    OneSynthesisInfo, SingleFoodFormulaInfo, SingleProcessedFoodFormulaInfo,
    SynthesisFormulaUnlockRequest, SynthesisFormulaUnlockResponse, SynthesisInfoRequest,
    SynthesisInfoResponse, SynthesisItemRequest, SynthesisItemResponse, SynthesisLevelInfo,
    SynthesisLevelRewardRequest, SynthesisLevelRewardResponse, SynthesisLevelUpdateNotify,
};

use super::State;

pub fn handle_forge_info_request(
    state: &mut State,
    _req: ForgeInfoRequest,
) -> Result<ForgeInfoResponse, String> {
    let craft_state = &state.player_components.progress.info.craft;
    let mut list = Vec::new();
    let mut configs = Vec::with_capacity(state.assets.tables.forge_formula.items.len());
    for row in &state.assets.tables.forge_formula.items {
        configs.push(OneForgeConfig {
            Id: row.Id,
            ..Default::default()
        });
        let rec = ProgressInfo::find_id_role_count(&craft_state.forge_counts, row.Id);
        let unlocked = row.Unlock || craft_state.unlocked_forge.contains(&row.Id);
        if unlocked || rec.is_some() {
            list.push(OneForgeInfo {
                Id: row.Id,
                LastRoleId: rec.as_ref().map_or(0, |r| r.last_role_id),
                LimitForgeCount: rec.as_ref().map_or(0, |r| r.count),
                ..Default::default()
            });
        }
    }
    Ok(ForgeInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        ForgeInfoList: list,
        ForgeConfigs: configs,
        LimitRefreshTime: 0,
    })
}

pub fn handle_forge_formula_unlock_request(
    state: &mut State,
    req: ForgeFormulaUnlockRequest,
) -> Result<ForgeFormulaUnlockResponse, String> {
    let id = req.Id;
    let Some(row) = state
        .assets
        .tables
        .forge_formula
        .get_data_by_id(&id)
        .map(|r| r.into_owned())
    else {
        return Ok(ForgeFormulaUnlockResponse {
            Code: Some(ErrorCode::ErrForgeConfigNotFound),
            Id: id,
        });
    };
    if row.Unlock
        || state
            .player_components
            .progress
            .info
            .craft
            .unlocked_forge
            .contains(&id)
    {
        return Ok(ForgeFormulaUnlockResponse {
            Code: Some(ErrorCode::ErrForgeUnlocked),
            Id: id,
        });
    }
    if row.FormulaItemId > 0 {
        let costs = [consume::CostItem {
            id: row.FormulaItemId,
            count: 1,
        }];
        let paid = {
            let fs = &state.fs;
            let conn = &mut state.conn;
            let basic_comp = &mut state.player_components.basic;
            let inventory_comp = &mut state.player_components.inventory;
            consume::consume_costs(conn, fs, basic_comp, inventory_comp, &costs)
                .map_err(|e| e.to_string())?
        };
        if !paid {
            return Ok(ForgeFormulaUnlockResponse {
                Code: Some(ErrorCode::ErrItemNotEnough),
                Id: id,
            });
        }
    }
    ProgressInfo::append_unique_id(
        &mut state.player_components.progress.info.craft.unlocked_forge,
        id,
    );
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(ForgeFormulaUnlockResponse {
        Code: Some(ErrorCode::Success),
        Id: id,
    })
}

pub fn handle_forge_item_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ForgeItemRequest,
) -> Result<ForgeItemResponse, String> {
    let count = req.Count;
    if count <= 0 {
        return Ok(ForgeItemResponse {
            Code: Some(ErrorCode::ErrItemCount),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    }
    let Some(row) = state
        .assets
        .tables
        .forge_formula
        .get_data_by_id(&req.Id)
        .map(|r| r.into_owned())
    else {
        return Ok(ForgeItemResponse {
            Code: Some(ErrorCode::ErrForgeConfigNotFound),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    };
    let unlocked = row.Unlock
        || state
            .player_components
            .progress
            .info
            .craft
            .unlocked_forge
            .contains(&row.Id);
    if !unlocked {
        return Ok(ForgeItemResponse {
            Code: Some(ErrorCode::ErrForgeLocked),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    }

    let Ok(costs) = craft::build_scaled_costs(&row.ConsumeItems, count) else {
        return Ok(ForgeItemResponse {
            Code: Some(ErrorCode::ErrItemCount),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    };

    let outcome = {
        let mut ctx = state.craft_context();
        craft::consume_and_grant(&mut ctx, &costs, row.ItemId, count, 2)
            .map_err(|e| e.to_string())?
    };
    if !outcome.ok {
        return Ok(ForgeItemResponse {
            Code: Some(outcome.err),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    }

    ProgressInfo::add_id_role_count(
        &mut state.player_components.progress.info.craft.forge_counts,
        row.Id,
        count,
        req.RoleId,
    );
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_ui_action(&mut qctx, "Forging") {
            tracing::warn!("on_ui_action(Forging) error: {err}");
        }
    }

    let forge_cnt = ProgressInfo::find_id_role_count(
        &state.player_components.progress.info.craft.forge_counts,
        row.Id,
    )
    .map_or(count, |r| r.count);
    state.conn.push(
        ForgeItemInfoUpdateNotify::MSG_ID,
        &ForgeItemInfoUpdateNotify {
            ForgeInfoList: vec![OneForgeInfo {
                Id: row.Id,
                LastRoleId: req.RoleId,
                LimitForgeCount: forge_cnt,
                ..Default::default()
            }],
        },
    );

    Ok(ForgeItemResponse {
        Code: Some(ErrorCode::Success),
        Id: row.Id,
        RoleId: req.RoleId,
        ItemInfos: outcome.items,
        ..Default::default()
    })
}

fn build_synth_info_pb(id: i32, limit_count: i32, rec: Option<IdRoleCount>) -> OneSynthesisInfo {
    let cnt = rec.as_ref().map_or(0, |r| r.count);
    OneSynthesisInfo {
        Id: id,
        Count: cnt,
        LastRoleId: rec.map_or(0, |r| r.last_role_id),
        LimitCount: limit_count,
        LimitSynthesisCount: cnt,
        ..Default::default()
    }
}

pub fn handle_synthesis_info_request(
    state: &mut State,
    _req: SynthesisInfoRequest,
) -> Result<SynthesisInfoResponse, String> {
    let craft_state = &state.player_components.progress.info.craft;
    let mut infos = Vec::new();
    let mut configs = Vec::with_capacity(state.assets.tables.synthesis_formula.items.len());
    for row in &state.assets.tables.synthesis_formula.items {
        configs.push(OneSynthesisConfig {
            Id: row.Id,
            ..Default::default()
        });
        let rec = ProgressInfo::find_id_role_count(&craft_state.synth_counts, row.Id);
        let unlocked = row.Unlock || craft_state.unlocked_synth.contains(&row.Id);
        if unlocked || rec.is_some() {
            infos.push(build_synth_info_pb(row.Id, row.LimitCount, rec));
        }
    }
    Ok(SynthesisInfoResponse {
        LevelInfo: Some(SynthesisLevelInfo {
            Level: craft_state.synth_level,
            TotalProficiency: craft_state.synth_proficiency,
        }),
        SynthesisConfigs: configs,
        Code: Some(ErrorCode::Success),
        SynthesisInfoList: infos,
        LimitRefreshTime: 0,
    })
}

pub fn handle_synthesis_formula_unlock_request(
    state: &mut State,
    req: SynthesisFormulaUnlockRequest,
) -> Result<SynthesisFormulaUnlockResponse, String> {
    let id = req.Id;
    let Some(row) = state
        .assets
        .tables
        .synthesis_formula
        .get_data_by_id(&id)
        .map(|r| r.into_owned())
    else {
        return Ok(SynthesisFormulaUnlockResponse {
            Code: Some(ErrorCode::ErrSynthesisConfigNotFound),
            Id: id,
        });
    };
    if row.Unlock
        || state
            .player_components
            .progress
            .info
            .craft
            .unlocked_synth
            .contains(&id)
    {
        return Ok(SynthesisFormulaUnlockResponse {
            Code: Some(ErrorCode::ErrSynthesisUnlocked),
            Id: id,
        });
    }
    if row.FormulaItemId > 0 {
        let costs = [consume::CostItem {
            id: row.FormulaItemId,
            count: 1,
        }];
        let paid = {
            let fs = &state.fs;
            let conn = &mut state.conn;
            let basic_comp = &mut state.player_components.basic;
            let inventory_comp = &mut state.player_components.inventory;
            consume::consume_costs(conn, fs, basic_comp, inventory_comp, &costs)
                .map_err(|e| e.to_string())?
        };
        if !paid {
            return Ok(SynthesisFormulaUnlockResponse {
                Code: Some(ErrorCode::ErrItemNotEnough),
                Id: id,
            });
        }
    }
    ProgressInfo::append_unique_id(
        &mut state.player_components.progress.info.craft.unlocked_synth,
        id,
    );
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(SynthesisFormulaUnlockResponse {
        Code: Some(ErrorCode::Success),
        Id: id,
    })
}

pub fn handle_synthesis_item_request(
    state: &mut State,
    events: &mut EventQueue,
    req: SynthesisItemRequest,
) -> Result<SynthesisItemResponse, String> {
    let count = req.Count;
    if count <= 0 {
        return Ok(SynthesisItemResponse {
            Code: Some(ErrorCode::ErrItemCount),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    }
    let Some(row) = state
        .assets
        .tables
        .synthesis_formula
        .get_data_by_id(&req.Id)
        .map(|r| r.into_owned())
    else {
        return Ok(SynthesisItemResponse {
            Code: Some(ErrorCode::ErrSynthesisConfigNotFound),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    };
    let unlocked = row.Unlock
        || state
            .player_components
            .progress
            .info
            .craft
            .unlocked_synth
            .contains(&row.Id);
    if !unlocked {
        return Ok(SynthesisItemResponse {
            Code: Some(ErrorCode::ErrSynthesisLocked),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    }
    let prev_cnt = ProgressInfo::find_id_role_count(
        &state.player_components.progress.info.craft.synth_counts,
        row.Id,
    )
    .map_or(0, |r| r.count);
    if row.LimitCount > 0 && prev_cnt.saturating_add(count) > row.LimitCount {
        return Ok(SynthesisItemResponse {
            Code: Some(ErrorCode::ErrSynthesisCountLimit),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    }

    let Ok(costs) = craft::build_scaled_costs(&row.ConsumeItems, count) else {
        return Ok(SynthesisItemResponse {
            Code: Some(ErrorCode::ErrItemCount),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    };

    let outcome = {
        let mut ctx = state.craft_context();
        craft::consume_and_grant(&mut ctx, &costs, row.ItemId, count, 2)
            .map_err(|e| e.to_string())?
    };
    if !outcome.ok {
        return Ok(SynthesisItemResponse {
            Code: Some(outcome.err),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    }

    let prof_left = if row.MaxProficiencyCount > 0 {
        (row.MaxProficiencyCount - prev_cnt).max(0)
    } else {
        count
    };
    let prof_cnt = count.min(prof_left);
    let prof_gain = if row.Proficiency > 0 && prof_cnt > 0 {
        row.Proficiency * prof_cnt
    } else {
        0
    };
    let level_rows: Vec<(i32, i32)> = state
        .assets
        .tables
        .synthesis_level
        .items
        .iter()
        .map(|r| (r.Id, r.Completeness))
        .collect();
    let (leveled, synth_level, synth_proficiency) = {
        let craft_state = &mut state.player_components.progress.info.craft;
        ProgressInfo::add_id_role_count(&mut craft_state.synth_counts, row.Id, count, req.RoleId);
        let leveled = craft::add_proficiency_and_update_level(
            &mut craft_state.synth_level,
            &mut craft_state.synth_proficiency,
            prof_gain,
            level_rows,
        );
        (leveled, craft_state.synth_level, craft_state.synth_proficiency)
    };
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_ui_action(&mut qctx, "Synthetic") {
            tracing::warn!("on_ui_action(Synthetic) error: {err}");
        }
    }

    if leveled {
        state.conn.push(
            SynthesisLevelUpdateNotify::MSG_ID,
            &SynthesisLevelUpdateNotify {
                LevelInfo: Some(SynthesisLevelInfo {
                    Level: synth_level,
                    TotalProficiency: synth_proficiency,
                }),
            },
        );
    }

    Ok(SynthesisItemResponse {
        Code: Some(ErrorCode::Success),
        Id: row.Id,
        RoleId: req.RoleId,
        ItemInfos: outcome.items,
        ..Default::default()
    })
}

pub fn handle_batch_synthesis_item_request(
    state: &mut State,
    req: BatchSynthesisItemRequest,
) -> Result<BatchSynthesisItemResponse, String> {
    let mut out_items = BTreeMap::new();
    let mut any_changed = false;
    let mut leveled = false;
    let level_rows: Vec<(i32, i32)> = state
        .assets
        .tables
        .synthesis_level
        .items
        .iter()
        .map(|r| (r.Id, r.Completeness))
        .collect();

    for batch_entry in &req.BatchItems {
        if batch_entry.Count <= 0 {
            continue;
        }
        let Some(row) = state
            .assets
            .tables
            .synthesis_formula
            .get_data_by_id(&batch_entry.Id)
            .map(|r| r.into_owned())
        else {
            continue;
        };
        let unlocked = row.Unlock
            || state
                .player_components
                .progress
                .info
                .craft
                .unlocked_synth
                .contains(&row.Id);
        if !unlocked {
            continue;
        }
        let prev_cnt = ProgressInfo::find_id_role_count(
            &state.player_components.progress.info.craft.synth_counts,
            row.Id,
        )
        .map_or(0, |r| r.count);
        if row.LimitCount > 0 && prev_cnt.saturating_add(batch_entry.Count) > row.LimitCount {
            continue;
        }
        let Ok(costs) = craft::build_scaled_costs(&row.ConsumeItems, batch_entry.Count) else {
            continue;
        };
        let outcome = {
            let mut ctx = state.craft_context();
            craft::consume_and_grant(&mut ctx, &costs, row.ItemId, batch_entry.Count, 2)
                .map_err(|e| e.to_string())?
        };
        if !outcome.ok {
            continue;
        }

        any_changed = true;
        let prof_left = if row.MaxProficiencyCount > 0 {
            (row.MaxProficiencyCount - prev_cnt).max(0)
        } else {
            batch_entry.Count
        };
        let prof_cnt = batch_entry.Count.min(prof_left);
        let prof_gain = if row.Proficiency > 0 && prof_cnt > 0 {
            row.Proficiency * prof_cnt
        } else {
            0
        };
        {
            let craft_state = &mut state.player_components.progress.info.craft;
            ProgressInfo::add_id_role_count(
                &mut craft_state.synth_counts,
                row.Id,
                batch_entry.Count,
                0,
            );
            if craft::add_proficiency_and_update_level(
                &mut craft_state.synth_level,
                &mut craft_state.synth_proficiency,
                prof_gain,
                level_rows.iter().copied(),
            ) {
                leveled = true;
            }
        }
        *out_items.entry(row.ItemId).or_insert(0) += batch_entry.Count;
    }

    if any_changed {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut q_events = EventQueue::new();
        {
            let mut qctx = state.quest_context(&mut q_events);
            if let Err(err) = quest_engine::events::on_ui_action(&mut qctx, "Synthetic") {
                tracing::warn!("on_ui_action(Synthetic) error: {err}");
            }
        }
        super::drain_event_queue_best_effort(&mut q_events, state);
        if leveled {
            let craft_state = &state.player_components.progress.info.craft;
            state.conn.push(
                SynthesisLevelUpdateNotify::MSG_ID,
                &SynthesisLevelUpdateNotify {
                    LevelInfo: Some(SynthesisLevelInfo {
                        Level: craft_state.synth_level,
                        TotalProficiency: craft_state.synth_proficiency,
                    }),
                },
            );
        }
    }

    Ok(BatchSynthesisItemResponse {
        Items: out_items,
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_synthesis_level_reward_request(
    state: &mut State,
    _req: SynthesisLevelRewardRequest,
) -> Result<SynthesisLevelRewardResponse, String> {
    let cur_lvl = state.player_components.progress.info.craft.synth_level;
    let claimed_lvl = state
        .player_components
        .progress
        .info
        .craft
        .synth_claimed_level;
    let drop_ids: Vec<i32> = state
        .assets
        .tables
        .synthesis_level
        .items
        .iter()
        .filter(|r| r.Id <= cur_lvl && r.Id > claimed_lvl && r.DropIds > 0)
        .map(|r| r.DropIds)
        .collect();

    let mut reward_map = BTreeMap::new();
    for drop_id in drop_ids {
        let mut ctx = state.craft_context();
        craft::grant_drop_and_collect_items(&mut ctx, drop_id, 0, 0, 2, &mut reward_map)
            .map_err(|e| e.to_string())?;
    }
    if cur_lvl > claimed_lvl {
        state
            .player_components
            .progress
            .info
            .craft
            .synth_claimed_level = cur_lvl;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    let craft_state = &state.player_components.progress.info.craft;
    Ok(SynthesisLevelRewardResponse {
        Code: Some(ErrorCode::Success),
        LevelInfo: Some(SynthesisLevelInfo {
            Level: craft_state.synth_level,
            TotalProficiency: craft_state.synth_proficiency,
        }),
    })
}

pub fn handle_cooking_data_request(
    state: &mut State,
    _req: CookingDataRequest,
) -> Result<CookingDataResponse, String> {
    let craft_state = &state.player_components.progress.info.craft;
    let mut formulas = Vec::new();
    let mut configs = Vec::with_capacity(state.assets.tables.cook_formula.items.len());
    for row in &state.assets.tables.cook_formula.items {
        configs.push(OneFormulaConfig {
            Id: row.Id,
            ..Default::default()
        });
        let rec = ProgressInfo::find_id_role_count(&craft_state.cook_counts, row.Id);
        let unlocked = row.Unlock || craft_state.unlocked_cook.contains(&row.Id);
        if unlocked || rec.is_some() {
            formulas.push(SingleFoodFormulaInfo {
                Id: row.Id,
                CookCount: rec.as_ref().map_or(0, |r| r.count),
                LastRoleId: rec.as_ref().map_or(0, |r| r.last_role_id),
                ..Default::default()
            });
        }
    }

    let mut processed = Vec::with_capacity(state.assets.tables.cook_processed.items.len());
    for row in &state.assets.tables.cook_processed.items {
        let unlocked = row.Unlock || craft_state.unlocked_processed.contains(&row.Id);
        processed.push(SingleProcessedFoodFormulaInfo {
            Id: row.Id,
            LockState: unlocked,
            Interations: row.InterationId.clone(),
            UnlockParams: Vec::new(),
        });
    }

    Ok(CookingDataResponse {
        ProcessedFoodFormulaInfos: processed,
        FormulaConfigs: configs,
        FoodFormulaInfos: formulas,
        Code: Some(ErrorCode::Success),
        LimitRefreshTime: 0,
        CookingInfo: Some(CookingInfo {
            CookingLevel: craft_state.cook_level,
            TotalProficiencys: craft_state.cook_proficiency,
        }),
    })
}

pub fn handle_cook_formula_request(
    state: &mut State,
    req: CookFormulaRequest,
) -> Result<CookFormulaResponse, String> {
    let id = req.CookFormulaId;
    if let Some(cf) = state
        .assets
        .tables
        .cook_formula
        .get_data_by_id(&id)
        .map(|r| r.into_owned())
    {
        let already_unlocked = state
            .player_components
            .progress
            .info
            .craft
            .unlocked_cook
            .contains(&id);
        if cf.FormulaItemId > 0 && !cf.Unlock && !already_unlocked {
            let costs = [consume::CostItem {
                id: cf.FormulaItemId,
                count: 1,
            }];
            let paid = {
                let fs = &state.fs;
                let conn = &mut state.conn;
                let basic_comp = &mut state.player_components.basic;
                let inventory_comp = &mut state.player_components.inventory;
                consume::consume_costs(conn, fs, basic_comp, inventory_comp, &costs)
                    .map_err(|e| e.to_string())?
            };
            if !paid {
                return Ok(CookFormulaResponse {
                    Code: Some(ErrorCode::ErrItemNotEnough),
                    CookFormulaId: id,
                });
            }
        }
        ProgressInfo::append_unique_id(
            &mut state.player_components.progress.info.craft.unlocked_cook,
            id,
        );
    } else if state
        .assets
        .tables
        .cook_processed
        .get_data_by_id(&id)
        .is_some()
    {
        ProgressInfo::append_unique_id(
            &mut state
                .player_components
                .progress
                .info
                .craft
                .unlocked_processed,
            id,
        );
    } else {
        return Ok(CookFormulaResponse {
            Code: Some(ErrorCode::ErrCookingFormulaNotFound),
            CookFormulaId: id,
        });
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(CookFormulaResponse {
        Code: Some(ErrorCode::Success),
        CookFormulaId: id,
    })
}

pub fn handle_cook_food_request(
    state: &mut State,
    events: &mut EventQueue,
    req: CookFoodRequest,
) -> Result<CookFoodResponse, String> {
    let count = req.CookCount;
    if count <= 0 {
        return Ok(CookFoodResponse {
            Code: Some(ErrorCode::ErrCookingCount),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    }

    let (out_item_id, prof_per, max_prof_cnt, raw_costs) = if let Some(cf) = state
        .assets
        .tables
        .cook_formula
        .get_data_by_id(&req.Id)
        .map(|r| r.into_owned())
    {
        if !cf.Unlock
            && !state
                .player_components
                .progress
                .info
                .craft
                .unlocked_cook
                .contains(&req.Id)
        {
            return Ok(CookFoodResponse {
                Code: Some(ErrorCode::ErrCookingFormulaNotFound),
                Id: req.Id,
                RoleId: req.RoleId,
                ..Default::default()
            });
        }
        (
            cf.FoodItemId,
            cf.Proficiency,
            cf.MaxProficiencyCount,
            cf.ConsumeItems,
        )
    } else if let Some(cp) = state
        .assets
        .tables
        .cook_processed
        .get_data_by_id(&req.Id)
        .map(|r| r.into_owned())
    {
        if !cp.Unlock
            && !state
                .player_components
                .progress
                .info
                .craft
                .unlocked_processed
                .contains(&req.Id)
        {
            return Ok(CookFoodResponse {
                Code: Some(ErrorCode::ErrCookingProcessNotFound),
                Id: req.Id,
                RoleId: req.RoleId,
                ..Default::default()
            });
        }
        (cp.FinalItemId, 10, 0, cp.ConsumeItemsId)
    } else {
        return Ok(CookFoodResponse {
            Code: Some(ErrorCode::ErrCookingFormulaNotFound),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    };

    let Ok(costs) = craft::build_scaled_costs(&raw_costs, count) else {
        return Ok(CookFoodResponse {
            Code: Some(ErrorCode::ErrCookingCount),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    };

    let outcome = {
        let mut ctx = state.craft_context();
        craft::consume_and_grant(&mut ctx, &costs, out_item_id, count, 2)
            .map_err(|e| e.to_string())?
    };
    if !outcome.ok {
        return Ok(CookFoodResponse {
            Code: Some(outcome.err),
            Id: req.Id,
            RoleId: req.RoleId,
            ..Default::default()
        });
    }

    let level_rows: Vec<(i32, i32)> = state
        .assets
        .tables
        .cook_level
        .items
        .iter()
        .map(|r| (r.Id, r.Completeness))
        .collect();
    let (leveled, cook_level, cook_proficiency) = {
        let craft_state = &mut state.player_components.progress.info.craft;
        let prev_cnt = ProgressInfo::find_id_role_count(&craft_state.cook_counts, req.Id)
            .map_or(0, |r| r.count);
        let prof_left = if max_prof_cnt > 0 {
            (max_prof_cnt - prev_cnt).max(0)
        } else {
            count
        };
        let prof_cnt = count.min(prof_left);
        let prof_gain = if prof_per > 0 && prof_cnt > 0 {
            prof_per * prof_cnt
        } else {
            0
        };
        ProgressInfo::add_id_role_count(&mut craft_state.cook_counts, req.Id, count, req.RoleId);
        let leveled = craft::add_proficiency_and_update_level(
            &mut craft_state.cook_level,
            &mut craft_state.cook_proficiency,
            prof_gain,
            level_rows,
        );
        (leveled, craft_state.cook_level, craft_state.cook_proficiency)
    };
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_ui_action(&mut qctx, "Cook") {
            tracing::warn!("on_ui_action(Cook) error: {err}");
        }
    }

    if leveled {
        state.conn.push(
            CookingInfoUpdateNotify::MSG_ID,
            &CookingInfoUpdateNotify {
                CookingInfo: Some(CookingInfo {
                    CookingLevel: cook_level,
                    TotalProficiencys: cook_proficiency,
                }),
            },
        );
    }

    Ok(CookFoodResponse {
        ItemInfos: outcome.items,
        Code: Some(ErrorCode::Success),
        Id: req.Id,
        RoleId: req.RoleId,
        ..Default::default()
    })
}
