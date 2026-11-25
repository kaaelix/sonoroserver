use std::collections::HashSet;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::gift_package::GiftPackage;
use sonoro_gamesv_data::tables::item_info::ItemInfo as ItemConfig;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::component::player::{PlayerEchoComponent, PlayerWeaponComponent};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{consume, energy, quest_engine, reward, role as role_helper, role_stats};
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    AddCountItemInfo, DecomposeItemInfo, GetInstExchangeRewardRequest,
    GetInstExchangeRewardResponse, GiftPackPreviewRequest, GiftPackPreviewResponse,
    HandInItemRequest, HandInItemResponse, HandInMingSuRequest, HandInMingSuResponse,
    ItemDecomposePreviewRequest, ItemDecomposePreviewResponse, ItemDecomposeRequest,
    ItemDecomposeResponse, ItemDisuseRequest,
    ItemDisuseResponse, ItemFuncValueUpdateNotify, ItemGiftUseRequest, ItemGiftUseResponse,
    ItemLockRequest, ItemLockResponse, ItemUseRequest, ItemUseResponse, MingSuHandRewardRequest,
    MingSuHandRewardResponse, NewHandInItemRequest, NewHandInItemResponse, NormalItem,
    NormalItemRequest, NormalItemResponse, PbGetRoleListNotify, PhantomItem,
    PhantomItemRemoveNotify, WeaponItemAddNotify, WeaponItemRemoveNotify,
};

use super::State;

fn append_normal_item(list: &mut Vec<NormalItem>, id: i32, count: i32) {
    if count <= 0 {
        return;
    }
    if let Some(item) = list.iter_mut().find(|item| item.Id == id) {
        item.Count = count;
        return;
    }
    list.push(NormalItem {
        Id: id,
        Count: count,
        ExpireTime: 0,
    });
}

pub fn handle_normal_item_request(
    state: &mut State,
    _req: NormalItemRequest,
) -> Result<NormalItemResponse, String> {
    let mut normal_items = state.player_components.inventory.info.normal_item_list();
    append_normal_item(
        &mut normal_items,
        2,
        state.player_components.basic.info.attributes.coin,
    );
    append_normal_item(
        &mut normal_items,
        3,
        state.player_components.basic.info.attributes.rare_coin,
    );
    Ok(NormalItemResponse {
        NormalItemList: normal_items,
    })
}

pub fn handle_item_lock_request(
    state: &mut State,
    req: ItemLockRequest,
) -> Result<ItemLockResponse, String> {
    let func_value = if req.ItemId == 1 { 1 } else { 0 };
    if let Some(item) = state.player_components.weapon.weapon_map.get_mut(&req.IncrId) {
        item.func_value = func_value;
        PlayerWeaponComponent::save_all(
            &state.fs,
            state.player_components.weapon.player_id,
            &state.player_components.weapon.weapon_map,
        )
        .map_err(|e| e.to_string())?;
        state.conn.push(
            ItemFuncValueUpdateNotify::MSG_ID,
            &ItemFuncValueUpdateNotify {
                IncrId: req.IncrId,
                FuncValue: func_value,
            },
        );
        return Ok(ItemLockResponse {
            ErrorCode: Some(ErrorCode::Success),
        });
    }

    let Some(item) = state.player_components.echo.echo_map.get_mut(&req.IncrId) else {
        return Ok(ItemLockResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
        });
    };
    item.func_value = func_value;
    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_components.echo.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;
    state.conn.push(
        ItemFuncValueUpdateNotify::MSG_ID,
        &ItemFuncValueUpdateNotify {
            IncrId: req.IncrId,
            FuncValue: func_value,
        },
    );
    Ok(ItemLockResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn apply_item_func_value_update(
    state: &mut State,
    incr_id: i32,
    new_func: i32,
) -> Result<ErrorCode, String> {
    if let Some(item) = state.player_components.echo.echo_map.get_mut(&incr_id) {
        item.func_value = new_func;
        PlayerEchoComponent::save_all(
            &state.fs,
            state.player_components.echo.player_id,
            &state.player_components.echo.echo_map,
        )
        .map_err(|e| e.to_string())?;
    } else if let Some(weapon) = state.player_components.weapon.weapon_map.get_mut(&incr_id) {
        weapon.func_value = new_func;
        PlayerWeaponComponent::save_all(
            &state.fs,
            state.player_components.weapon.player_id,
            &state.player_components.weapon.weapon_map,
        )
        .map_err(|e| e.to_string())?;
    } else {
        return Ok(ErrorCode::ErrPhantomItemNotExist);
    }

    state.conn.push(
        ItemFuncValueUpdateNotify::MSG_ID,
        &ItemFuncValueUpdateNotify {
            IncrId: incr_id,
            FuncValue: new_func,
        },
    );
    Ok(ErrorCode::Success)
}

pub fn handle_item_disuse_request(
    state: &mut State,
    req: ItemDisuseRequest,
) -> Result<ItemDisuseResponse, String> {
    let new_func = if req.Oper == 1 { 2 } else { 0 };
    let err = apply_item_func_value_update(state, req.IncrId, new_func)?;
    Ok(ItemDisuseResponse { ErrCode: Some(err) })
}

fn find_gift_package(assets: &Assets, item_cfg: &ItemConfig) -> Option<GiftPackage> {
    for key in [2, 4, 8] {
        if let Some(&pkg_id) = item_cfg.Parameters.get(&key)
            && let Some(pkg) = assets.tables.gift_package.get_data_by_id(&pkg_id)
        {
            return Some(pkg.into_owned());
        }
    }
    None
}

fn roll_random_usize(upper: usize) -> Result<usize, String> {
    if upper <= 1 {
        return Ok(0);
    }
    let mut bytes = [0u8; 8];
    getrandom::fill(&mut bytes).map_err(|e| format!("rng failed: {e}"))?;
    Ok((u64::from_le_bytes(bytes) % upper as u64) as usize)
}

fn expand_gift_package(
    pkg: &GiftPackage,
    open_count: i32,
) -> Result<Vec<reward::RewardItem>, String> {
    let mut out = Vec::new();
    if pkg.Content.is_empty() || open_count <= 0 {
        return Ok(out);
    }

    if pkg.Type == 0 {
        for (&item_id, &cnt) in pkg.Content.iter() {
            if item_id > 0 && cnt > 0 {
                out.push(reward::RewardItem {
                    item_id,
                    count: cnt * open_count,
                });
            }
        }
        return Ok(out);
    }

    let mut pool_ids = Vec::new();
    let mut pool_counts = Vec::new();
    let mut pool_weights = Vec::new();
    for (&item_id, &cnt) in pkg.Content.iter() {
        if item_id <= 0 || cnt <= 0 {
            continue;
        }
        pool_ids.push(item_id);
        pool_counts.push(cnt);
        let w = pkg.Weight.get(&item_id).copied().unwrap_or(1).max(1);
        pool_weights.push(w as usize);
    }
    if pool_ids.is_empty() {
        return Ok(out);
    }

    let picks_per_open = pkg.AvailableNum.max(1) as usize;
    for _ in 0..open_count {
        if matches!(pkg.Type, 1 | 5 | 6) {
            let mut picked = vec![false; pool_ids.len()];
            let target = picks_per_open.min(pool_ids.len());
            for _ in 0..target {
                let start_idx = roll_random_usize(pool_ids.len())?;
                for step in 0..pool_ids.len() {
                    let idx = (start_idx + step) % pool_ids.len();
                    if !picked[idx] {
                        picked[idx] = true;
                        out.push(reward::RewardItem {
                            item_id: pool_ids[idx],
                            count: pool_counts[idx],
                        });
                        break;
                    }
                }
            }
        } else {
            let total_weight: usize = pool_weights.iter().sum();
            for _ in 0..picks_per_open {
                let mut target = if total_weight > 0 {
                    roll_random_usize(total_weight)?
                } else {
                    0
                };
                let mut chosen_idx = 0usize;
                for (idx, &w) in pool_weights.iter().enumerate() {
                    if target < w {
                        chosen_idx = idx;
                        break;
                    }
                    target -= w;
                }
                out.push(reward::RewardItem {
                    item_id: pool_ids[chosen_idx],
                    count: pool_counts[chosen_idx],
                });
            }
        }
    }

    Ok(out)
}

fn role_waveband_id(assets: &Assets, role_id: i32) -> Option<i32> {
    let role = assets.tables.role_info.get_data_by_id(&role_id)?;
    if role.ResonantChainGroupId == 0 {
        return None;
    }
    for chain in &assets.tables.resonant_chain.items {
        if chain.GroupId == role.ResonantChainGroupId && chain.GroupIndex == 1 {
            return chain.ActivateConsume.keys().next().copied();
        }
    }
    None
}

fn grant_gift_rewards(state: &mut State, raw_items: &[reward::RewardItem]) -> Result<(), String> {
    let mut normal_items = Vec::new();
    let mut starter_weapons = Vec::new();
    let mut has_new_roles = false;

    for &entry in raw_items {
        if entry.item_id <= 0 || entry.count <= 0 {
            continue;
        }
        if state
            .assets
            .tables
            .role_info
            .get_data_by_id(&entry.item_id)
            .is_some()
        {
            for _ in 0..entry.count {
                if state.player_components.role.role_map.contains_key(&entry.item_id) {
                    let wb = role_waveband_id(&state.assets, entry.item_id).unwrap_or(50003);
                    normal_items.push(reward::RewardItem {
                        item_id: wb,
                        count: if wb == 50003 { 3 } else { 1 },
                    });
                } else {
                    let new_role = RoleInfo::create_default(&state.assets, entry.item_id)
                        .map_err(|e| e.to_string())?;
                    state
                        .player_components
                        .role
                        .role_map
                        .insert(entry.item_id, new_role.clone());
                    let role_path = format!(
                        "player/{}/role/{}",
                        state.player_components.role.player_id, entry.item_id
                    );
                    comp_util::save_struct(&state.fs, &new_role, &role_path)
                        .map_err(|e| e.to_string())?;
                    if role_helper::ensure_role_weapon(
                        &state.fs,
                        &state.assets,
                        &mut state.player_components.role,
                        &mut state.player_components.weapon,
                        entry.item_id,
                    )
                    .unwrap_or(false)
                        && let Some(rinfo) = state.player_components.role.role_map.get(&entry.item_id)
                        && let Some(witem) = state.player_components.weapon.weapon_map.get(&rinfo.weapon)
                    {
                        starter_weapons.push(witem.to_proto(rinfo.weapon));
                    }
                    has_new_roles = true;
                }
            }
        } else {
            normal_items.push(entry);
        }
    }

    if !starter_weapons.is_empty() {
        state.conn.push(
            WeaponItemAddNotify::MSG_ID,
            &WeaponItemAddNotify {
                WeaponItemList: starter_weapons,
                AddFromRole: true,
                Reason: 1,
            },
        );
    }

    if !normal_items.is_empty() {
        let mut rctx = state.reward_context();
        reward::grant_items(&mut rctx, 0, true, &normal_items, 1).map_err(|e| e.to_string())?;
    }

    if has_new_roles {
        let selected_main_role = role_helper::selected_main_role_id(
            &state.assets,
            &state.player_components.role,
            state.player_components.basic.info.attributes.sex,
            state.player_components.basic.info.selected_main_role_id,
        );
        let role_ids: Vec<i32> = state
            .player_components
            .role
            .role_map
            .keys()
            .copied()
            .filter(|&id| {
                (1000..=1999).contains(&id)
                    && selected_main_role.is_none_or(|selected| {
                        !role_helper::is_main_role(&state.assets, id) || id == selected
                    })
            })
            .collect();
        let mut role_list = Vec::with_capacity(role_ids.len());
        for role_id in role_ids {
            let Some(role_info) = state.player_components.role.role_map.get(&role_id).cloned()
            else {
                continue;
            };
            let pb_role = role_stats::to_client_role_info(
                &state.assets,
                &mut state.player_components.role,
                role_id,
                &role_info,
                &state.player_components.weapon,
                &state.player_components.echo,
            )
            .map_err(|e| e.to_string())?;
            role_list.push(pb_role);
        }
        state.conn.push(
            PbGetRoleListNotify::MSG_ID,
            &PbGetRoleListNotify {
                RoleList: role_list,
            },
        );
    }

    Ok(())
}

pub fn handle_item_use_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ItemUseRequest,
) -> Result<ItemUseResponse, String> {
    let item_id = req.ItemId;
    let count = req.Count;
    if item_id <= 0 || count <= 0 {
        return Ok(ItemUseResponse {
            ErrCode: Some(ErrorCode::ErrItemCount),
        });
    }

    if let Some(cfg) = state
        .assets
        .tables
        .item_info
        .get_data_by_id(&item_id)
        .map(|r| r.into_owned())
    {
        if cfg.UseLevel > 0 && state.player_components.basic.info.attributes.level < cfg.UseLevel {
            return Ok(ItemUseResponse {
                ErrCode: Some(ErrorCode::ErrItemUseLevelLimit),
            });
        }
        if cfg.UseCountLimit > 0 && count > cfg.UseCountLimit {
            return Ok(ItemUseResponse {
                ErrCode: Some(ErrorCode::ErrGiftPackUseLimit),
            });
        }

        if cfg.Parameters.get(&10).copied() == Some(1) {
            let costs = [consume::CostItem { id: item_id, count }];
            if !consume::has_costs(
                &state.player_components.basic,
                &state.player_components.inventory,
                &costs,
            ) {
                return Ok(ItemUseResponse {
                    ErrCode: Some(ErrorCode::ErrItemNotEnough),
                });
            }
            let waveplate_gain = 60 * count;
            if !energy::can_add_waveplate(&mut state.player_components.basic, waveplate_gain) {
                return Ok(ItemUseResponse {
                    ErrCode: Some(ErrorCode::ErrEnergyMaxCharge),
                });
            }
            let consumed = {
                let State {
                    conn,
                    fs,
                    player_components,
                    ..
                } = state;
                consume::consume_costs(
                    conn,
                    fs,
                    &mut player_components.basic,
                    &mut player_components.inventory,
                    &costs,
                )
                .map_err(|e| e.to_string())?
            };
            if !consumed {
                return Ok(ItemUseResponse {
                    ErrCode: Some(ErrorCode::ErrItemNotEnough),
                });
            }
            let State {
                conn,
                fs,
                player_components,
                ..
            } = state;
            let _ = energy::add_waveplate(conn, fs, &mut player_components.basic, waveplate_gain)
                .map_err(|e| e.to_string())?;
            let mut qctx = state.quest_context(events);
            quest_engine::on_use_item(&mut qctx, item_id, count).map_err(|e| e.to_string())?;
            return Ok(ItemUseResponse {
                ErrCode: Some(ErrorCode::Success),
            });
        }

        if let Some(pkg) = find_gift_package(&state.assets, &cfg) {
            let costs = [consume::CostItem { id: item_id, count }];
            if !consume::has_costs(
                &state.player_components.basic,
                &state.player_components.inventory,
                &costs,
            ) {
                return Ok(ItemUseResponse {
                    ErrCode: Some(ErrorCode::ErrItemNotEnough),
                });
            }
            let expanded = expand_gift_package(&pkg, count)?;
            let consumed = {
                let State {
                    conn,
                    fs,
                    player_components,
                    ..
                } = state;
                consume::consume_costs(
                    conn,
                    fs,
                    &mut player_components.basic,
                    &mut player_components.inventory,
                    &costs,
                )
                .map_err(|e| e.to_string())?
            };
            if !consumed {
                return Ok(ItemUseResponse {
                    ErrCode: Some(ErrorCode::ErrItemNotEnough),
                });
            }
            grant_gift_rewards(state, &expanded)?;
            let mut qctx = state.quest_context(events);
            quest_engine::on_use_item(&mut qctx, item_id, count).map_err(|e| e.to_string())?;
            return Ok(ItemUseResponse {
                ErrCode: Some(ErrorCode::Success),
            });
        }

        if !cfg.ConvertItems.is_empty() {
            let costs = [consume::CostItem { id: item_id, count }];
            if !consume::has_costs(
                &state.player_components.basic,
                &state.player_components.inventory,
                &costs,
            ) {
                return Ok(ItemUseResponse {
                    ErrCode: Some(ErrorCode::ErrItemNotEnough),
                });
            }
            let rewards: Vec<reward::RewardItem> = cfg
                .ConvertItems
                .iter()
                .filter(|&(&k, &v)| k > 0 && v > 0)
                .map(|(&k, &v)| reward::RewardItem {
                    item_id: k,
                    count: v * count,
                })
                .collect();
            let consumed = {
                let State {
                    conn,
                    fs,
                    player_components,
                    ..
                } = state;
                consume::consume_costs(
                    conn,
                    fs,
                    &mut player_components.basic,
                    &mut player_components.inventory,
                    &costs,
                )
                .map_err(|e| e.to_string())?
            };
            if !consumed {
                return Ok(ItemUseResponse {
                    ErrCode: Some(ErrorCode::ErrItemNotEnough),
                });
            }
            grant_gift_rewards(state, &rewards)?;
            let mut qctx = state.quest_context(events);
            quest_engine::on_use_item(&mut qctx, item_id, count).map_err(|e| e.to_string())?;
            return Ok(ItemUseResponse {
                ErrCode: Some(ErrorCode::Success),
            });
        }

        if !matches!(cfg.ItemType, 7 | 8 | 13) && cfg.ShowUseButton {
            let costs = [consume::CostItem { id: item_id, count }];
            let consumed = {
                let State {
                    conn,
                    fs,
                    player_components,
                    ..
                } = state;
                consume::consume_costs(
                    conn,
                    fs,
                    &mut player_components.basic,
                    &mut player_components.inventory,
                    &costs,
                )
                .map_err(|e| e.to_string())?
            };
            if !consumed {
                return Ok(ItemUseResponse {
                    ErrCode: Some(ErrorCode::ErrItemNotEnough),
                });
            }
        }
    }

    let mut qctx = state.quest_context(events);
    quest_engine::on_use_item(&mut qctx, item_id, count).map_err(|e| e.to_string())?;
    Ok(ItemUseResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_item_gift_use_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ItemGiftUseRequest,
) -> Result<ItemGiftUseResponse, String> {
    let item_id = req.ItemId;
    let count = req.Count;
    if item_id <= 0 || count <= 0 {
        return Ok(ItemGiftUseResponse {
            ErrorCode: Some(ErrorCode::ErrItemCount),
            ..Default::default()
        });
    }

    let costs = [consume::CostItem { id: item_id, count }];
    if !consume::has_costs(
        &state.player_components.basic,
        &state.player_components.inventory,
        &costs,
    ) {
        return Ok(ItemGiftUseResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
            ..Default::default()
        });
    }

    let mut rewards = Vec::new();
    if let Some(cfg) = state
        .assets
        .tables
        .item_info
        .get_data_by_id(&item_id)
        .map(|r| r.into_owned())
        && let Some(pkg) = find_gift_package(&state.assets, &cfg)
    {
        for &sel_id in &req.SelectedId {
            if sel_id <= 0 {
                continue;
            }
            let unit_count = pkg.Content.get(&sel_id).copied().unwrap_or(1).max(1);
            rewards.push(reward::RewardItem {
                item_id: sel_id,
                count: unit_count * count,
            });
        }
        if rewards.is_empty() {
            rewards = expand_gift_package(&pkg, count)?;
        }
    }
    if rewards.is_empty() {
        for &sel_id in &req.SelectedId {
            if sel_id > 0 {
                rewards.push(reward::RewardItem {
                    item_id: sel_id,
                    count,
                });
            }
        }
    }

    let consumed = {
        let State {
            conn,
            fs,
            player_components,
            ..
        } = state;
        consume::consume_costs(
            conn,
            fs,
            &mut player_components.basic,
            &mut player_components.inventory,
            &costs,
        )
        .map_err(|e| e.to_string())?
    };
    if !consumed {
        return Ok(ItemGiftUseResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
            ..Default::default()
        });
    }

    grant_gift_rewards(state, &rewards)?;
    let mut qctx = state.quest_context(events);
    quest_engine::on_use_item(&mut qctx, item_id, count).map_err(|e| e.to_string())?;

    let infos = rewards
        .into_iter()
        .map(|r| AddCountItemInfo {
            Id: r.item_id,
            Count: r.count,
            IncrId: 0,
        })
        .collect();
    Ok(ItemGiftUseResponse {
        ErrorCode: Some(ErrorCode::Success),
        Infos: infos,
    })
}

fn calc_decompose_rewards(
    assets: &Assets,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    items: &[DecomposeItemInfo],
) -> Vec<reward::RewardItem> {
    let mut rewards = Vec::new();
    for entry in items {
        if entry.IncrId > 0 {
            if let Some(w) = weapon_comp.weapon_map.get(&entry.IncrId) {
                let tier = w.level.max(1);
                rewards.push(reward::RewardItem {
                    item_id: 43010001,
                    count: (tier / 2).max(1),
                });
                rewards.push(reward::RewardItem {
                    item_id: 2,
                    count: tier * 100,
                });
            } else if let Some(e) = echo_comp.echo_map.get(&entry.IncrId) {
                let quality = assets
                    .tables
                    .phantom_item
                    .get_data_by_id(&e.id)
                    .map_or(2, |p| p.QualityId.max(1));
                rewards.push(reward::RewardItem {
                    item_id: 41100011,
                    count: quality.max(1),
                });
                rewards.push(reward::RewardItem {
                    item_id: 2,
                    count: quality * 250 + e.level * 100,
                });
            }
        } else if entry.ItemId > 0 && entry.Count > 0 {
            rewards.push(reward::RewardItem {
                item_id: 2,
                count: entry.Count * 100,
            });
        }
    }
    rewards
}

pub fn handle_item_decompose_preview_request(
    state: &mut State,
    req: ItemDecomposePreviewRequest,
) -> Result<ItemDecomposePreviewResponse, String> {
    let rewards = calc_decompose_rewards(
        &state.assets,
        &state.player_components.weapon,
        &state.player_components.echo,
        &req.ItemList,
    );
    let mut item_map = std::collections::BTreeMap::new();
    for r in rewards {
        *item_map.entry(r.item_id).or_insert(0) += r.count;
    }
    Ok(ItemDecomposePreviewResponse {
        ErrorCode: Some(ErrorCode::Success),
        ItemMap: item_map,
    })
}

pub fn handle_item_decompose_request(
    state: &mut State,
    req: ItemDecomposeRequest,
) -> Result<ItemDecomposeResponse, String> {
    if req.ItemList.is_empty() {
        return Ok(ItemDecomposeResponse {
            ErrCode: Some(ErrorCode::ErrItemCount),
        });
    }

    let mut seen_incr = HashSet::new();
    let mut costs = Vec::new();
    let mut incr_weapons = Vec::new();
    let mut incr_echoes = Vec::new();

    for entry in &req.ItemList {
        if entry.IncrId > 0 {
            if !seen_incr.insert(entry.IncrId) {
                return Ok(ItemDecomposeResponse {
                    ErrCode: Some(ErrorCode::ErrPhantomConsumeItemIncrDuplicate),
                });
            }
            if let Some(w) = state.player_components.weapon.weapon_map.get(&entry.IncrId) {
                if w.func_value == 1 || w.role_id.is_some() {
                    return Ok(ItemDecomposeResponse {
                        ErrCode: Some(if w.role_id.is_some() {
                            ErrorCode::ErrPhantomDecomposeEquiped
                        } else {
                            ErrorCode::ErrPhantomLevelUpMaterialLock
                        }),
                    });
                }
                incr_weapons.push(entry.IncrId);
            } else if let Some(e) = state.player_components.echo.echo_map.get(&entry.IncrId) {
                if e.func_value == 1 || e.role_id.is_some() {
                    return Ok(ItemDecomposeResponse {
                        ErrCode: Some(if e.role_id.is_some() {
                            ErrorCode::ErrPhantomDecomposeEquiped
                        } else {
                            ErrorCode::ErrPhantomLevelUpMaterialLock
                        }),
                    });
                }
                incr_echoes.push(entry.IncrId);
            } else {
                return Ok(ItemDecomposeResponse {
                    ErrCode: Some(ErrorCode::ErrPhantomItemNotExist),
                });
            }
        } else if entry.ItemId > 0 && entry.Count > 0 {
            costs.push(consume::CostItem {
                id: entry.ItemId,
                count: entry.Count,
            });
        } else {
            return Ok(ItemDecomposeResponse {
                ErrCode: Some(ErrorCode::ErrItemCount),
            });
        }
    }

    let rewards = calc_decompose_rewards(
        &state.assets,
        &state.player_components.weapon,
        &state.player_components.echo,
        &req.ItemList,
    );

    if !costs.is_empty() {
        let ok = {
            let State {
                conn,
                fs,
                player_components,
                ..
            } = state;
            consume::consume_costs(
                conn,
                fs,
                &mut player_components.basic,
                &mut player_components.inventory,
                &costs,
            )
            .map_err(|e| e.to_string())?
        };
        if !ok {
            return Ok(ItemDecomposeResponse {
                ErrCode: Some(ErrorCode::ErrItemNotEnough),
            });
        }
    }

    for &incr_id in &incr_weapons {
        state.player_components.weapon.weapon_map.remove(&incr_id);
        PlayerWeaponComponent::delete_weapon_file(
            &state.fs,
            state.player_components.weapon.player_id,
            incr_id,
        )
        .map_err(|e| e.to_string())?;
    }
    for &incr_id in &incr_echoes {
        state.player_components.echo.echo_map.remove(&incr_id);
        PlayerEchoComponent::delete_echo_file(
            &state.fs,
            state.player_components.echo.player_id,
            incr_id,
        )
        .map_err(|e| e.to_string())?;
    }

    if !incr_weapons.is_empty() {
        PlayerWeaponComponent::save_all(
            &state.fs,
            state.player_components.weapon.player_id,
            &state.player_components.weapon.weapon_map,
        )
        .map_err(|e| e.to_string())?;
        state.conn.push(
            WeaponItemRemoveNotify::MSG_ID,
            &WeaponItemRemoveNotify {
                WeaponItemIncrIdList: incr_weapons,
            },
        );
    }
    if !incr_echoes.is_empty() {
        PlayerEchoComponent::save_all(
            &state.fs,
            state.player_components.echo.player_id,
            &state.player_components.echo.echo_map,
        )
        .map_err(|e| e.to_string())?;
        state.conn.push(
            PhantomItemRemoveNotify::MSG_ID,
            &PhantomItemRemoveNotify {
                PhantomItemIncrIdList: incr_echoes,
            },
        );
    }
    if !rewards.is_empty() {
        let mut rctx = state.reward_context();
        reward::grant_items(&mut rctx, 0, true, &rewards, 1).map_err(|e| e.to_string())?;
    }
    Ok(ItemDecomposeResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_gift_pack_preview_request(
    state: &mut State,
    req: GiftPackPreviewRequest,
) -> Result<GiftPackPreviewResponse, String> {
    let mut preview = std::collections::BTreeMap::new();
    if let Some(pkg) = state
        .assets
        .tables
        .gift_package
        .get_data_by_id(&req.GiftPackId)
    {
        for &k in pkg.Content.keys() {
            if state
                .assets
                .tables
                .phantom_item
                .get_data_by_id(&k)
                .is_some()
            {
                preview.insert(
                    k,
                    PhantomItem {
                        Id: k,
                        PhantomLevel: 0,
                        ..Default::default()
                    },
                );
            }
        }
    }
    Ok(GiftPackPreviewResponse {
        PhantomPreview: preview,
    })
}

pub fn handle_get_inst_exchange_reward_request(
    state: &mut State,
    req: GetInstExchangeRewardRequest,
) -> Result<GetInstExchangeRewardResponse, String> {
    let cnt = req.ExchangeCount;
    let Some(waveplate_cost) = cnt.checked_mul(60) else {
        return Ok(GetInstExchangeRewardResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    if cnt <= 0 {
        return Ok(GetInstExchangeRewardResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    let ok = {
        let State {
            conn,
            fs,
            player_components,
            ..
        } = state;
        energy::consume_waveplate(conn, fs, &mut player_components.basic, waveplate_cost)
            .map_err(|e| e.to_string())?
    };
    if !ok {
        return Ok(GetInstExchangeRewardResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
        });
    }
    let _ = state
        .player_components
        .progress
        .info
        .add_key_int("inst_exchange_count", i64::from(cnt));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut rctx = state.reward_context();
    reward::grant_items(
        &mut rctx,
        0,
        true,
        &[
            reward::RewardItem {
                item_id: 3,
                count: cnt * 20,
            },
            reward::RewardItem {
                item_id: 2,
                count: cnt * 5000,
            },
        ],
        1,
    )
    .map_err(|e| e.to_string())?;
    Ok(GetInstExchangeRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_hand_in_item_request(
    state: &mut State,
    events: &mut EventQueue,
    req: HandInItemRequest,
) -> Result<HandInItemResponse, String> {
    let mut costs = Vec::new();
    for hand_in in &req.HandInItem {
        for info in &hand_in.HandItemInfo {
            if info.ItemId > 0 && info.Num > 0 {
                costs.push(consume::CostItem {
                    id: info.ItemId,
                    count: info.Num,
                });
            }
        }
    }
    if !costs.is_empty() {
        let ok = {
            let State {
                conn,
                fs,
                player_components,
                ..
            } = state;
            consume::consume_costs(
                conn,
                fs,
                &mut player_components.basic,
                &mut player_components.inventory,
                &costs,
            )
            .map_err(|e| e.to_string())?
        };
        if !ok {
            return Ok(HandInItemResponse {
                ErrorId: Some(ErrorCode::ErrItemNotEnough),
            });
        }
    }
    if req.NodeId > 0 {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_submit_node(
            &mut qctx,
            req.TreeOwnerId.max(0),
            req.TreeIncId,
            req.NodeId,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(HandInItemResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_new_hand_in_item_request(
    state: &mut State,
    events: &mut EventQueue,
    req: NewHandInItemRequest,
) -> Result<NewHandInItemResponse, String> {
    if let Some(pm) = req.ProjectionMachine
        && pm.ItemId > 0
    {
        let cost = [consume::CostItem {
            id: pm.ItemId,
            count: 1,
        }];
        let ok = {
            let State {
                conn,
                fs,
                player_components,
                ..
            } = state;
            consume::consume_costs(
                conn,
                fs,
                &mut player_components.basic,
                &mut player_components.inventory,
                &cost,
            )
            .map_err(|e| e.to_string())?
        };
        if !ok {
            return Ok(NewHandInItemResponse {
                ErrorCode: Some(ErrorCode::ErrItemNotEnough),
            });
        }
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("handin_pm:{}", pm.EntityCfgId), i64::from(pm.ItemId));
        let _ = state
            .player_components
            .progress
            .info
            .add_key_int("handin_pm:cnt", 1);
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;

        let mut qctx = state.quest_context(events);
        quest_engine::events::on_condition_event(
            &mut qctx,
            quest_engine::events::QuestEventKind::NewHandInItems,
            pm.EntityCfgId,
            pm.ItemId,
        )
        .map_err(|e| e.to_string())?;
        quest_engine::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    }
    Ok(NewHandInItemResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_hand_in_ming_su_request(
    state: &mut State,
    events: &mut EventQueue,
    req: HandInMingSuRequest,
) -> Result<HandInMingSuResponse, String> {
    let Some(pool) = state
        .assets
        .tables
        .dragon_pool
        .get_data_by_id(&req.DragonPoolId)
        .map(|r| r.into_owned())
    else {
        return Ok(HandInMingSuResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            ..Default::default()
        });
    };
    let lvl_k = format!("mingsu:lvl:{}", req.DragonPoolId);
    let inj_k = format!("mingsu:inj:{}", req.DragonPoolId);
    let cur_lvl = state
        .player_components
        .progress
        .info
        .get_key_int(&lvl_k)
        .max(0) as usize;
    if cur_lvl >= pool.Goal.len() {
        return Ok(HandInMingSuResponse {
            ErrorCode: Some(ErrorCode::Success),
            DragonPoolId: req.DragonPoolId,
            Level: cur_lvl as i32,
            InjectedCoreItemCount: state
                .player_components
                .progress
                .info
                .get_key_int(&inj_k)
                .max(0) as i32,
            LevelGains: 0,
        });
    }

    let core_id = pool.CoreId;
    if core_id > 0 {
        let owned = state.player_components.inventory.info.normal_item_count(core_id);
        if owned <= 0 {
            return Ok(HandInMingSuResponse {
                ErrorCode: Some(ErrorCode::ErrItemNotEnough),
                ..Default::default()
            });
        }
        let cur_inj = state
            .player_components
            .progress
            .info
            .get_key_int(&inj_k)
            .max(0) as i32;
        let next_goal = pool.Goal[cur_lvl];
        let needed = (next_goal - cur_inj).max(1);
        let take = owned.min(needed);
        let cost = [consume::CostItem {
            id: core_id,
            count: take,
        }];
        let ok = {
            let State {
                conn,
                fs,
                player_components,
                ..
            } = state;
            consume::consume_costs(
                conn,
                fs,
                &mut player_components.basic,
                &mut player_components.inventory,
                &cost,
            )
            .map_err(|e| e.to_string())?
        };
        if !ok {
            return Ok(HandInMingSuResponse {
                ErrorCode: Some(ErrorCode::ErrItemNotEnough),
                ..Default::default()
            });
        }
        let total_inj = cur_inj + take;
        state
            .player_components
            .progress
            .info
            .set_key_int(&inj_k, i64::from(total_inj));
        let mut new_lvl = cur_lvl;
        while new_lvl < pool.Goal.len() && total_inj >= pool.Goal[new_lvl] {
            new_lvl += 1;
        }
        let gains = (new_lvl - cur_lvl) as i32;
        state
            .player_components
            .progress
            .info
            .set_key_int(&lvl_k, new_lvl as i64);
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        if gains > 0 {
            let mut qctx = state.quest_context(events);
            if let Err(err) = quest_engine::on_condition_state_changed(&mut qctx) {
                tracing::warn!("on_hand_in_ming_su hook error: {err}");
            }
        }
        return Ok(HandInMingSuResponse {
            ErrorCode: Some(ErrorCode::Success),
            DragonPoolId: req.DragonPoolId,
            Level: new_lvl as i32,
            InjectedCoreItemCount: total_inj,
            LevelGains: gains,
        });
    }

    let next_lvl = state
        .player_components
        .progress
        .info
        .add_key_int(&lvl_k, 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    if let Err(err) = quest_engine::on_condition_state_changed(&mut qctx) {
        tracing::warn!("on_hand_in_ming_su hook error: {err}");
    }
    Ok(HandInMingSuResponse {
        ErrorCode: Some(ErrorCode::Success),
        DragonPoolId: req.DragonPoolId,
        Level: next_lvl,
        InjectedCoreItemCount: next_lvl,
        LevelGains: 1,
    })
}

pub fn handle_ming_su_hand_reward_request(
    state: &mut State,
    req: MingSuHandRewardRequest,
) -> Result<MingSuHandRewardResponse, String> {
    let pool = state
        .assets
        .tables
        .dragon_pool
        .get_data_by_id(&req.DragonPoolId)
        .map(|r| r.into_owned());
    let lvl_k = format!("mingsu:lvl:{}", req.DragonPoolId);
    let cur_lvl = state
        .player_components
        .progress
        .info
        .get_key_int(&lvl_k)
        .max(0) as i32;
    if cur_lvl <= 0 {
        return Ok(MingSuHandRewardResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            LevelGains: 0,
        });
    }
    let clm_k = format!("mingsu:clm:{}:{cur_lvl}", req.DragonPoolId);
    if state.player_components.progress.info.mark_claim(&clm_k) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let drop_id = pool
            .as_ref()
            .and_then(|p| p.DropIds.get((cur_lvl - 1) as usize).copied())
            .unwrap_or(0);
        let mut rctx = state.reward_context();
        if drop_id > 0 {
            reward::grant_drop_package_popup(&mut rctx, drop_id, 19500)
                .map_err(|e| e.to_string())?;
        } else {
            reward::grant_items(
                &mut rctx,
                0,
                true,
                &[
                    reward::RewardItem {
                        item_id: 3,
                        count: 50,
                    },
                    reward::RewardItem {
                        item_id: 2,
                        count: 15000,
                    },
                ],
                19500,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(MingSuHandRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        LevelGains: cur_lvl,
    })
}
