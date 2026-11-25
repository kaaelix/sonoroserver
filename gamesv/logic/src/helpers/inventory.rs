use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::inventory_info::InventoryInfo;
use std::collections::BTreeMap;

pub fn add_default_progression_items(info: &mut InventoryInfo, assets: &Assets) {
    add_default_gacha_items(info, assets);

    for entry in &assets.tables.skill_tree.items {
        ensure_consume_items(info, &entry.Consume);
    }
    for entry in &assets.tables.weapon_exp_item.items {
        ensure_normal_item(info, entry.Id, 777);
    }
    for entry in &assets.tables.role_exp_item.items {
        ensure_normal_item(info, entry.Id, 999);
    }
    for entry in &assets.tables.weapon_breach.items {
        ensure_consume_items(info, &entry.Consume);
    }
    for entry in &assets.tables.role_breach.items {
        ensure_consume_items(info, &entry.BreachConsume);
    }
    for entry in &assets.tables.skill_level.items {
        ensure_consume_items(info, &entry.Consume);
    }
    for entry in &assets.tables.motor_tech_tree.items {
        ensure_normal_item(info, entry.TpItemId, 777);
    }
    for scene in &assets.tables.motor_scene.items {
        if scene.UnlockItem != 0 {
            ensure_normal_item(info, scene.UnlockItem, 1);
        }
    }

    add_default_echo_materials(info, assets);
}

fn add_default_echo_materials(info: &mut InventoryInfo, assets: &Assets) {
    for entry in &assets.tables.phantom_exp_item.items {
        ensure_normal_item(info, entry.ItemId, 777);
    }
    for entry in &assets.tables.phantom_quality.items {
        for &id in entry.IdentifyCost.keys() {
            ensure_normal_item(info, id, 777);
        }
    }
    for entry in &assets.tables.phantom_rarity.items {
        for &id in entry.PolishCost.keys() {
            ensure_normal_item(info, id, 777);
        }
    }
    for entry in &assets.tables.phantom_vice_polish_config.items {
        for &id in entry.Cost.keys() {
            ensure_normal_item(info, id, 777);
        }
    }
}

fn add_default_gacha_items(info: &mut InventoryInfo, assets: &Assets) {
    for gacha in &assets.tables.gacha.items {
        let pool_id = first_gacha_pool_id(assets, gacha.Id);
        let Some(view) = assets.tables.gacha_view_info.get_data_by_id(&pool_id) else {
            continue;
        };
        ensure_normal_item(info, gacha_currency_item_id(gacha.Id, view.Type), 777);
    }
}

fn first_gacha_pool_id(assets: &Assets, gacha_id: i32) -> i32 {
    let mut best_id = 0;
    let mut best_sort = i32::MAX;
    for pool in &assets.tables.gacha_pool.items {
        if pool.GachaId != gacha_id {
            continue;
        }
        if pool.Sort < best_sort || (pool.Sort == best_sort && (best_id == 0 || pool.Id < best_id)) {
            best_id = pool.Id;
            best_sort = pool.Sort;
        }
    }
    best_id
}

pub fn gacha_currency_item_id(gacha_id: i32, view_type: i32) -> i32 {
    match view_type {
        1 | 4 | 5 => 50001,
        2 | 7 | 9 | 13 => 50002,
        3 | 8 | 10 | 14 => 50005,
        11 => 50007,
        12 => 50008,
        6 if gacha_id == 5 => 50006,
        _ => 50001,
    }
}

fn ensure_consume_items(info: &mut InventoryInfo, consume_map: &BTreeMap<i32, i32>) {
    for &id in consume_map.keys() {
        if id > 0 {
            ensure_normal_item(info, id, 777);
        }
    }
}

fn ensure_normal_item(info: &mut InventoryInfo, id: i32, count: i32) {
    if id <= 0 || is_player_currency(id) || count <= 0 || info.normal_item_count(id) >= count {
        return;
    }
    if let Some(item) = info.normal_items.iter_mut().find(|i| i.id == id) {
        item.count = count;
        return;
    }
    info.add_normal_item(id, count);
}

fn is_player_currency(id: i32) -> bool {
    (1..=6).contains(&id)
}
