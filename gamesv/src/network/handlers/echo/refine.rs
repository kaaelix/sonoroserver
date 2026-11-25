use std::collections::HashSet;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::calabash_level::CalabashLevel;
use sonoro_gamesv_data::tables::phantom_item::PhantomItem as PhantomItemConfig;
use sonoro_gamesv_fs::echo_info::{self, EchoInfo, ZigDefaultPrng};
use sonoro_gamesv_fs::special_item_incr;
use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::component::player::PlayerEchoComponent;
use sonoro_gamesv_logic::helpers::{calabash, reward};
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    AddCountItemInfo, PhantomBatchDirectRefiningRequest, PhantomBatchDirectRefiningResponse,
    PhantomBatchRefiningRequest, PhantomBatchRefiningResponse, PhantomItem,
    PhantomItemAddNotify, PhantomItemRemoveNotify, PhantomRefiningRequest,
    PhantomRefiningResponse, PhantomStackConsumeEntry,
};

use super::super::State;

#[derive(Default)]
struct RefineResult {
    items: Vec<AddCountItemInfo>,
    extra_items: Vec<AddCountItemInfo>,
    cost_phantoms: Vec<PhantomItem>,
}

pub fn handle_phantom_refining_request(
    state: &mut State,
    req: PhantomRefiningRequest,
) -> Result<PhantomRefiningResponse, String> {
    match execute_refine(state, &req.StackEntries, None)? {
        Err(code) => Ok(PhantomRefiningResponse {
            ErrorCode: Some(code),
            ..Default::default()
        }),
        Ok(res) => Ok(PhantomRefiningResponse {
            ErrorCode: Some(ErrorCode::Success),
            Items: res.items,
            ExtraItems: res.extra_items,
            CostPhantoms: res.cost_phantoms,
        }),
    }
}

pub fn handle_phantom_batch_refining_request(
    state: &mut State,
    req: PhantomBatchRefiningRequest,
) -> Result<PhantomBatchRefiningResponse, String> {
    match execute_refine(state, &req.StackEntries, None)? {
        Err(code) => Ok(PhantomBatchRefiningResponse {
            ErrorCode: Some(code),
            ..Default::default()
        }),
        Ok(res) => Ok(PhantomBatchRefiningResponse {
            ErrorCode: Some(ErrorCode::Success),
            Items: res.items,
            ExtraItems: res.extra_items,
            CostPhantoms: res.cost_phantoms,
        }),
    }
}

pub fn handle_phantom_batch_direct_refining_request(
    state: &mut State,
    req: PhantomBatchDirectRefiningRequest,
) -> Result<PhantomBatchDirectRefiningResponse, String> {
    let week_times = state
        .player_components
        .echo
        .calabash_info
        .direct_refine_week_times;
    if req.TargetFetterGroupId <= 0
        || !has_fetter_candidate(&state.assets, req.TargetFetterGroupId)
    {
        return Ok(PhantomBatchDirectRefiningResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
            DirectRefineWeekTimes: week_times,
            ..Default::default()
        });
    }
    let requested_batches = (req.StackEntries.len() / 5) as i32;
    if requested_batches > 0 && week_times + requested_batches > 10 {
        return Ok(PhantomBatchDirectRefiningResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomRefiningMaxCount),
            DirectRefineWeekTimes: week_times,
            ..Default::default()
        });
    }

    match execute_refine(state, &req.StackEntries, Some(req.TargetFetterGroupId))? {
        Err(code) => Ok(PhantomBatchDirectRefiningResponse {
            ErrorCode: Some(code),
            DirectRefineWeekTimes: state
                .player_components
                .echo
                .calabash_info
                .direct_refine_week_times,
            ..Default::default()
        }),
        Ok(res) => Ok(PhantomBatchDirectRefiningResponse {
            ErrorCode: Some(ErrorCode::Success),
            Items: res.items,
            CostPhantoms: res.cost_phantoms,
            DirectRefineWeekTimes: state
                .player_components
                .echo
                .calabash_info
                .direct_refine_week_times,
        }),
    }
}

fn execute_refine(
    state: &mut State,
    entries: &[PhantomStackConsumeEntry],
    target_fetter: Option<i32>,
) -> Result<Result<RefineResult, ErrorCode>, String> {
    if entries.len() < 5 || !entries.len().is_multiple_of(5) {
        return Ok(Err(ErrorCode::ErrPhantomRefiningCount));
    }
    if entries.len() > 100 {
        return Ok(Err(ErrorCode::ErrPhantomRefiningMaxCount));
    }

    let mut seen = HashSet::with_capacity(entries.len());
    for entry in entries {
        if entry.Count != 1 {
            return Ok(Err(ErrorCode::ErrPhantomConsumeItemCount));
        }
        if !seen.insert(entry.IncrId) {
            return Ok(Err(ErrorCode::ErrPhantomRefiningDulplicate));
        }
        let Some(item) = state.player_components.echo.echo_map.get(&entry.IncrId) else {
            return Ok(Err(ErrorCode::ErrPhantomItemNotExist));
        };
        if item.func_value == 1 {
            return Ok(Err(ErrorCode::ErrPhantomLevelUpMaterialLock));
        }
        if item.role_id.is_some() {
            return Ok(Err(ErrorCode::ErrPhantomDecomposeEquiped));
        }
        if item.level != 0 || item.exp != 0 {
            return Ok(Err(ErrorCode::ErrPhantomRefiningDeveloped));
        }
    }

    let mut seed_bytes = [0u8; 8];
    getrandom::fill(&mut seed_bytes).map_err(|e| format!("rng failed: {e}"))?;
    let mut rng = ZigDefaultPrng::init(u64::from_le_bytes(seed_bytes));

    let calabash_lvl = calabash::current_level(
        &state.assets,
        &state.player_components.basic,
        &state.player_components.echo,
    );
    let calabash_cfg = state
        .assets
        .tables
        .calabash_level
        .get_data_by_id(&calabash_lvl)
        .map(|r| r.into_owned());

    let mut res = RefineResult::default();
    let mut removed_ids = Vec::with_capacity(entries.len());
    for entry in entries {
        let inc_id = entry.IncrId;
        if let Some(old) = state.player_components.echo.echo_map.remove(&inc_id) {
            res.cost_phantoms.push(old.to_proto(inc_id));
        }
        PlayerEchoComponent::delete_echo_file(&state.fs, state.player_id, inc_id)
            .map_err(|e| e.to_string())?;
        removed_ids.push(inc_id);
    }

    let mut added_protos = Vec::new();
    let mut bonus_union_exp = 0i32;
    let mut calabash_dirty = false;

    let refine_batches = entries.len() / 5;
    for _ in 0..refine_batches {
        let main_quality = roll_quality(calabash_cfg.as_ref(), &mut rng);
        let Some(produced) = pick_echo_candidate(
            &state.assets,
            &state.player_components.echo,
            main_quality,
            target_fetter,
            &mut rng,
        ) else {
            break;
        };
        let (incr_id, proto, discovery) = grant_refined_echo(
            &state.fs,
            &state.assets,
            &mut state.player_components.echo,
            &produced,
            target_fetter,
            &mut rng,
        )?;
        added_protos.push(proto);
        res.items.push(AddCountItemInfo {
            Id: produced.ItemId,
            Count: 1,
            IncrId: incr_id,
        });
        if discovery.changed {
            calabash_dirty = true;
        }
        bonus_union_exp += discovery.union_exp;

        if target_fetter.is_none()
            && let Some(cfg) = calabash_cfg.as_ref()
            && cfg.TempCatchGain > 0
            && rng.less_than_u32(1000) < cfg.TempCatchGain as u32
        {
            let extra_quality = roll_quality(Some(cfg), &mut rng);
            if let Some(extra_cfg) = pick_echo_candidate(
                &state.assets,
                &state.player_components.echo,
                extra_quality,
                None,
                &mut rng,
            ) {
                let (extra_incr_id, extra_proto, extra_discovery) = grant_refined_echo(
                    &state.fs,
                    &state.assets,
                    &mut state.player_components.echo,
                    &extra_cfg,
                    None,
                    &mut rng,
                )?;
                added_protos.push(extra_proto);
                res.extra_items.push(AddCountItemInfo {
                    Id: extra_cfg.ItemId,
                    Count: 1,
                    IncrId: extra_incr_id,
                });
                if extra_discovery.changed {
                    calabash_dirty = true;
                }
                bonus_union_exp += extra_discovery.union_exp;
            }
        }
    }

    if target_fetter.is_some() && refine_batches > 0 {
        state
            .player_components
            .echo
            .calabash_info
            .direct_refine_week_times += refine_batches as i32;
        calabash_dirty = true;
    }

    state.conn.push(
        PhantomItemRemoveNotify::MSG_ID,
        &PhantomItemRemoveNotify {
            PhantomItemIncrIdList: removed_ids,
        },
    );
    if !added_protos.is_empty() {
        state.conn.push(
            PhantomItemAddNotify::MSG_ID,
            &PhantomItemAddNotify {
                PhantomItemList: added_protos,
                Reason: 0,
            },
        );
    }
    if bonus_union_exp > 0 {
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            false,
            &[reward::RewardItem {
                item_id: 1,
                count: bonus_union_exp,
            }],
            0,
        )
        .map_err(|e| e.to_string())?;
    }
    if calabash_dirty {
        PlayerEchoComponent::save_calabash(
            &state.fs,
            state.player_id,
            &state.player_components.echo.calabash_info,
        )
        .map_err(|e| e.to_string())?;
        let notify = calabash::build_msg_notify(
            &state.assets,
            &state.player_components.basic,
            &state.player_components.echo,
        );
        state
            .conn
            .push(sonoro_proto::pb::CalabashMsgNotify::MSG_ID, &notify);
    }

    Ok(Ok(res))
}

fn grant_refined_echo(
    fs: &sonoro_common::fs::FileSystem,
    assets: &Assets,
    echo_comp: &mut PlayerEchoComponent,
    cfg: &PhantomItemConfig,
    forced_fetter: Option<i32>,
    rng: &mut ZigDefaultPrng,
) -> Result<(i32, PhantomItem, calabash::DiscoveryOutcome), String> {
    let fetter = forced_fetter
        .unwrap_or_else(|| cfg.FetterGroup[rng.less_than_usize(cfg.FetterGroup.len())]);
    let incr_id = special_item_incr::next(fs, echo_comp.player_id).map_err(|e| e.to_string())?;
    let echo_item = echo_info::build_default_echo(assets, cfg, fetter);
    let proto = echo_item.to_proto(incr_id);
    echo_comp.echo_map.insert(incr_id, echo_item.clone());
    let e_path = format!(
        "player/{}/{}/{}",
        echo_comp.player_id,
        EchoInfo::DATA_DIR,
        incr_id
    );
    comp_util::save_struct(fs, &echo_item, &e_path).map_err(|e| e.to_string())?;
    let discovery = calabash::record_echo_discovery(assets, echo_comp, cfg.MonsterId, cfg.QualityId);
    Ok((incr_id, proto, discovery))
}

fn roll_quality(calabash_cfg: Option<&CalabashLevel>, rng: &mut ZigDefaultPrng) -> i32 {
    let Some(cfg) = calabash_cfg else {
        return 2;
    };
    let total: i32 = cfg
        .QualityDropWeight
        .values()
        .copied()
        .filter(|&w| w > 0)
        .sum();
    if total <= 0 {
        return 2;
    }
    let mut roll = rng.less_than_u32(total as u32) as i32;
    for (&k, &w) in cfg.QualityDropWeight.iter() {
        if w <= 0 {
            continue;
        }
        if roll < w {
            return k;
        }
        roll -= w;
    }
    2
}

fn has_fetter_candidate(assets: &Assets, fetter_id: i32) -> bool {
    assets
        .tables
        .phantom_item
        .items
        .iter()
        .any(|item| echo_info::valid_default_owned_item(assets, item) && item.FetterGroup.contains(&fetter_id))
}

fn pick_echo_candidate(
    assets: &Assets,
    echo_comp: &PlayerEchoComponent,
    quality: i32,
    target_fetter: Option<i32>,
    rng: &mut ZigDefaultPrng,
) -> Option<PhantomItemConfig> {
    let discovered: Vec<&PhantomItemConfig> = assets
        .tables
        .phantom_item
        .items
        .iter()
        .filter(|item| {
            echo_info::valid_default_owned_item(assets, item)
                && item.QualityId == quality
                && target_fetter.is_none_or(|f| item.FetterGroup.contains(&f))
                && calabash::discovered_max_quality(assets, echo_comp, item.MonsterId) > 0
        })
        .collect();
    if !discovered.is_empty() {
        let idx = rng.less_than_usize(discovered.len());
        return Some(discovered[idx].clone());
    }

    let fallback: Vec<&PhantomItemConfig> = assets
        .tables
        .phantom_item
        .items
        .iter()
        .filter(|item| {
            echo_info::valid_default_owned_item(assets, item)
                && item.QualityId == quality
                && target_fetter.is_none_or(|f| item.FetterGroup.contains(&f))
        })
        .collect();
    if fallback.is_empty() {
        return None;
    }
    let idx = rng.less_than_usize(fallback.len());
    Some(fallback[idx].clone())
}
