use std::collections::{BTreeMap, HashMap};
use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_fs::inventory_info::InventoryInfo;
use sonoro_proto::enums::{PlayerAttrKey, PlayerAttrType};
use sonoro_proto::pb::{
    NormalItem as PbNormalItem, NormalItemUpdateNotify, PlayerAttr, PlayerAttrNotify,
    PlayerAttrValueOneof,
};

use crate::component::comp_util;
use crate::component::player::{PlayerBasicComponent, PlayerInventoryComponent};
use crate::sink::PushSink;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CostItem {
    pub id: i32,
    pub count: i32,
}

pub fn append_int_map(list: &mut Vec<CostItem>, consume_map: &BTreeMap<i32, i32>) {
    for (&id, &count) in consume_map {
        if id > 0 && count > 0 {
            list.push(CostItem { id, count });
        }
    }
}

pub fn has_costs(
    basic_comp: &PlayerBasicComponent,
    inventory_comp: &PlayerInventoryComponent,
    costs: &[CostItem],
) -> bool {
    let mut totals: HashMap<i32, i64> = HashMap::new();
    for entry in costs {
        if entry.id <= 0 || entry.count <= 0 {
            continue;
        }
        *totals.entry(entry.id).or_default() += i64::from(entry.count);
    }

    for (id, needed) in totals {
        let owned = match id {
            2 => i64::from(basic_comp.info.attributes.coin),
            3 => i64::from(
                basic_comp
                    .info
                    .attributes
                    .rare_coin
                    .max(inventory_comp.info.normal_item_count(3)),
            ),
            4 => i64::from(basic_comp.info.attributes.cash_coin),
            _ => i64::from(inventory_comp.info.normal_item_count(id)),
        };
        if owned < needed {
            return false;
        }
    }
    true
}

pub fn consume_costs<S: PushSink + ?Sized>(
    sink: &mut S,
    fs: &FileSystem,
    basic_comp: &mut PlayerBasicComponent,
    inventory_comp: &mut PlayerInventoryComponent,
    costs: &[CostItem],
) -> io::Result<bool> {
    if !has_costs(basic_comp, inventory_comp, costs) {
        return Ok(false);
    }

    let mut totals: HashMap<i32, i32> = HashMap::new();
    let mut ordered_ids = Vec::new();
    for entry in costs {
        if entry.id <= 0 || entry.count <= 0 {
            continue;
        }
        totals
            .entry(entry.id)
            .and_modify(|v| *v += entry.count)
            .or_insert_with(|| {
                ordered_ids.push(entry.id);
                entry.count
            });
    }
    if ordered_ids.is_empty() {
        return Ok(true);
    }

    let mut basic_dirty = false;
    let mut inv_dirty = false;
    let mut updated_items = Vec::new();
    let mut attr_updates = Vec::new();

    for id in ordered_ids {
        let count = totals.get(&id).copied().unwrap_or(0);
        if count <= 0 {
            continue;
        }
        if id == 2 {
            basic_comp.info.attributes.coin -= count;
            basic_dirty = true;
            updated_items.push(PbNormalItem {
                Id: 2,
                Count: basic_comp.info.attributes.coin,
                ExpireTime: 0,
            });
            attr_updates.push(PlayerAttr {
                Key: Some(PlayerAttrKey::Coin),
                ValueType: Some(PlayerAttrType::Int32),
                Value: Some(PlayerAttrValueOneof::Int32Value(
                    basic_comp.info.attributes.coin,
                )),
            });
        } else if id == 3 {
            let next_val = basic_comp
                .info
                .attributes
                .rare_coin
                .max(inventory_comp.info.normal_item_count(3))
                - count;
            basic_comp.info.attributes.rare_coin = next_val;
            basic_dirty = true;
            let _ = inventory_comp.info.consume_normal_item(3, count);
            inv_dirty = true;
            updated_items.push(PbNormalItem {
                Id: 3,
                Count: next_val,
                ExpireTime: 0,
            });
            attr_updates.push(PlayerAttr {
                Key: Some(PlayerAttrKey::RareCoin),
                ValueType: Some(PlayerAttrType::Int32),
                Value: Some(PlayerAttrValueOneof::Int32Value(next_val)),
            });
        } else if id == 4 {
            basic_comp.info.attributes.cash_coin -= count;
            basic_dirty = true;
            updated_items.push(PbNormalItem {
                Id: 4,
                Count: basic_comp.info.attributes.cash_coin,
                ExpireTime: 0,
            });
            attr_updates.push(PlayerAttr {
                Key: Some(PlayerAttrKey::CashCoin),
                ValueType: Some(PlayerAttrType::Int32),
                Value: Some(PlayerAttrValueOneof::Int32Value(
                    basic_comp.info.attributes.cash_coin,
                )),
            });
        } else {
            let _ = inventory_comp.info.consume_normal_item(id, count);
            inv_dirty = true;
            updated_items.push(PbNormalItem {
                Id: id,
                Count: inventory_comp.info.normal_item_count(id),
                ExpireTime: 0,
            });
        }
    }

    if inv_dirty {
        let inv_path = format!(
            "player/{}/{}",
            inventory_comp.player_id,
            InventoryInfo::DATA_PATH
        );
        comp_util::save_struct(fs, &inventory_comp.info, &inv_path)?;
    }
    if basic_dirty {
        let basic_path = format!("player/{}/basic_info", basic_comp.player_id);
        comp_util::save_struct(fs, &basic_comp.info, &basic_path)?;
    }

    if !updated_items.is_empty() {
        sink.push(
            NormalItemUpdateNotify::MSG_ID,
            &NormalItemUpdateNotify {
                NormalItemList: updated_items,
                NoTips: true,
            },
        );
    }
    if !attr_updates.is_empty() {
        sink.push(
            PlayerAttrNotify::MSG_ID,
            &PlayerAttrNotify {
                Attributes: attr_updates,
            },
        );
    }

    Ok(true)
}
