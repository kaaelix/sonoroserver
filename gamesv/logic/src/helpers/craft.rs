use std::collections::BTreeMap;
use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::CostEntry;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::SingleItemInfo;

use crate::component::player::{
    PlayerBasicComponent, PlayerEchoComponent, PlayerInventoryComponent, PlayerWeaponComponent,
};
use crate::helpers::consume::{self, CostItem};
use crate::helpers::reward::{self, RewardContext, RewardItem};
use crate::sink::PushSink;

pub struct CraftContext<'a, S: PushSink + ?Sized> {
    pub sink: &'a mut S,
    pub fs: &'a FileSystem,
    pub assets: &'a Assets,
    pub basic_comp: &'a mut PlayerBasicComponent,
    pub inventory_comp: &'a mut PlayerInventoryComponent,
    pub weapon_comp: &'a mut PlayerWeaponComponent,
    pub echo_comp: &'a mut PlayerEchoComponent,
}

#[derive(Debug, Clone, Default)]
pub struct CraftOutcome {
    pub ok: bool,
    pub err: ErrorCode,
    pub items: Vec<SingleItemInfo>,
}

pub fn build_scaled_costs(entries: &[CostEntry], count: i32) -> Result<Vec<CostItem>, &'static str> {
    let mut costs = Vec::new();
    for entry in entries {
        if entry.ItemId <= 0 || entry.Count <= 0 {
            continue;
        }
        let total = entry.Count.checked_mul(count).ok_or("Overflow")?;
        costs.push(CostItem {
            id: entry.ItemId,
            count: total,
        });
    }
    Ok(costs)
}

pub fn consume_and_grant<S: PushSink + ?Sized>(
    ctx: &mut CraftContext<'_, S>,
    costs: &[CostItem],
    reward_item_id: i32,
    reward_count: i32,
    reason: i32,
) -> io::Result<CraftOutcome> {
    if reward_item_id <= 0 || reward_count <= 0 {
        return Ok(CraftOutcome {
            ok: false,
            err: ErrorCode::RequestParamError,
            items: Vec::new(),
        });
    }
    let paid = consume::consume_costs(
        ctx.sink,
        ctx.fs,
        ctx.basic_comp,
        ctx.inventory_comp,
        costs,
    )?;
    if !paid {
        return Ok(CraftOutcome {
            ok: false,
            err: ErrorCode::ErrItemNotEnough,
            items: Vec::new(),
        });
    }

    let mut rctx = RewardContext {
        fs: ctx.fs,
        assets: ctx.assets,
        sink: ctx.sink,
        basic_comp: ctx.basic_comp,
        inventory_comp: ctx.inventory_comp,
        weapon_comp: ctx.weapon_comp,
        echo_comp: ctx.echo_comp,
        progress_comp: None,
    };
    let _ = reward::grant_items(
        &mut rctx,
        0,
        false,
        &[RewardItem {
            item_id: reward_item_id,
            count: reward_count,
        }],
        reason,
    )?;

    Ok(CraftOutcome {
        ok: true,
        err: ErrorCode::Success,
        items: vec![SingleItemInfo {
            ItemId: reward_item_id,
            ItemNum: reward_count,
        }],
    })
}

pub fn add_proficiency_and_update_level(
    level: &mut i32,
    total_proficiency: &mut i32,
    gain: i32,
    rows: impl IntoIterator<Item = (i32, i32)>,
) -> bool {
    if gain <= 0 {
        return false;
    }
    *total_proficiency += gain;
    let mut best_lvl = (*level).max(1);
    for (id, completeness) in rows {
        if *total_proficiency >= completeness && id > best_lvl {
            best_lvl = id;
        }
    }
    if best_lvl != *level {
        *level = best_lvl;
        true
    } else {
        false
    }
}

pub fn grant_drop_and_collect_items<S: PushSink + ?Sized>(
    ctx: &mut CraftContext<'_, S>,
    drop_id: i32,
    fallback_item_id: i32,
    fallback_count: i32,
    reason: i32,
    out_map: &mut BTreeMap<i32, i32>,
) -> io::Result<()> {
    let mut rctx = RewardContext {
        fs: ctx.fs,
        assets: ctx.assets,
        sink: ctx.sink,
        basic_comp: ctx.basic_comp,
        inventory_comp: ctx.inventory_comp,
        weapon_comp: ctx.weapon_comp,
        echo_comp: ctx.echo_comp,
        progress_comp: None,
    };
    let preview = reward::preview_drop_items(ctx.assets, drop_id);
    if !preview.is_empty() {
        let _ = reward::grant_drop_package(&mut rctx, drop_id, reason)?;
        for item in preview {
            *out_map.entry(item.item_id).or_insert(0) += item.count;
        }
        return Ok(());
    }
    if fallback_item_id > 0 && fallback_count > 0 {
        let _ = reward::grant_items(
            &mut rctx,
            0,
            true,
            &[RewardItem {
                item_id: fallback_item_id,
                count: fallback_count,
            }],
            reason,
        )?;
        *out_map.entry(fallback_item_id).or_insert(0) += fallback_count;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn craft_helper_add_proficiency_and_update_level_advances_by_threshold() {
        let rows = [(1, 0), (2, 500), (3, 1500)];
        let mut lvl = 1;
        let mut prof = 0;
        assert!(!add_proficiency_and_update_level(
            &mut lvl, &mut prof, 200, rows
        ));
        assert_eq!(lvl, 1);
        assert!(add_proficiency_and_update_level(
            &mut lvl, &mut prof, 400, rows
        ));
        assert_eq!(lvl, 2);
        assert!(add_proficiency_and_update_level(
            &mut lvl, &mut prof, 1000, rows
        ));
        assert_eq!(lvl, 3);
    }
}
