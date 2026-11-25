use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::inventory_info::InventoryInfo;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_gamesv_fs::special_item_incr;
use sonoro_gamesv_fs::weapon_item::WeaponItem;
use sonoro_proto::enums::{PlayerAttrKey, PlayerAttrType};
use sonoro_proto::pb::{
    NormalItem as PbNormalItem, NormalItemAddNotify, NormalItemUpdateNotify, PbGetRoleListNotify,
    PlayerAttr, PlayerAttrNotify, PlayerAttrValueOneof, WeaponItem as PbWeaponItem,
    WeaponItemAddNotify, WeaponItemRemoveNotify,
};

use crate::commands::respond;
use crate::component::comp_util;
use crate::component::player::{
    PlayerBasicComponent, PlayerEchoComponent, PlayerInventoryComponent, PlayerRoleComponent,
    PlayerWeaponComponent,
};
use crate::event_queue::EventQueue;
use crate::events::{BuffAdditionEntry, EntityHandle, Event};
use crate::helpers::quest_types::QuestContext;
use crate::helpers::reward::{self, RewardContext, RewardItem};
use crate::helpers::{quest_engine, role as role_helper, role_attribute_sync, role_stats};
use crate::scene::Scene;
use crate::sink::PushSink;

pub mod give {
    use super::*;

    pub const ALIAS: &str = "item";
    pub const DESCRIPTION: &str =
        "grants items, currencies, weapons, or echoes by id.\nusage: give [item_id] [count?]";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        fs: &FileSystem,
        assets: &Assets,
        sink: &mut S,
        basic_comp: &mut PlayerBasicComponent,
        inventory_comp: &mut PlayerInventoryComponent,
        weapon_comp: &mut PlayerWeaponComponent,
        echo_comp: &mut PlayerEchoComponent,
        item_id: i32,
        count_opt: Option<i32>,
    ) -> io::Result<()> {
        let player_id = basic_comp.player_id;
        let count = count_opt.unwrap_or(1);
        if item_id <= 0 || count <= 0 {
            respond(events, player_id, "invalid item_id or count");
            return Ok(());
        }
        let mut rctx = RewardContext {
            fs,
            assets,
            sink,
            basic_comp,
            inventory_comp,
            weapon_comp,
            echo_comp,
            progress_comp: None,
        };
        let res = reward::grant_items(&mut rctx, 0, true, &[RewardItem { item_id, count }], 1)?;
        if res.granted_count == 0 {
            respond(events, player_id, format!("unknown or ungrantable item {item_id}"));
            return Ok(());
        }
        respond(events, player_id, format!("granted {count}x item {item_id}"));
        Ok(())
    }
}

pub mod level {
    use super::*;

    pub const ALIAS: &str = "lv";
    pub const DESCRIPTION: &str =
        "sets union level or role level.\nusage: level [union|current|team|role_id] [target_level]";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        mut scene: Option<&mut Scene>,
        fs: &FileSystem,
        assets: &Assets,
        sink: &mut S,
        basic_comp: &mut PlayerBasicComponent,
        role_comp: &mut PlayerRoleComponent,
        weapon_comp: &PlayerWeaponComponent,
        echo_comp: &PlayerEchoComponent,
        subject: &str,
        target_level: i32,
    ) -> io::Result<()> {
        let player_id = basic_comp.player_id;
        if subject.eq_ignore_ascii_case("union") {
            let lv = target_level.clamp(1, 80);
            basic_comp.info.attributes.level = lv;
            basic_comp.info.attributes.exp = 0;
            role_helper::save_basic_info(fs, basic_comp)?;

            sink.push(
                PlayerAttrNotify::MSG_ID,
                &PlayerAttrNotify {
                    Attributes: vec![
                        PlayerAttr {
                            Key: Some(PlayerAttrKey::Level),
                            ValueType: Some(PlayerAttrType::Int32),
                            Value: Some(PlayerAttrValueOneof::Int32Value(lv)),
                        },
                        PlayerAttr {
                            Key: Some(PlayerAttrKey::Exp),
                            ValueType: Some(PlayerAttrType::Int32),
                            Value: Some(PlayerAttrValueOneof::Int32Value(0)),
                        },
                    ],
                },
            );
            events.enqueue(Event::ConditionStateRefresh {
                player_id,
                entity_config_id: 0,
            });
            respond(events, player_id, format!("set union level to {lv}"));
            return Ok(());
        }

        let lv = target_level.clamp(1, 90);
        let breach = if lv > 80 {
            6
        } else if lv > 70 {
            5
        } else if lv > 60 {
            4
        } else if lv > 50 {
            3
        } else if lv > 40 {
            2
        } else if lv > 20 {
            1
        } else {
            0
        };

        let mut ids = Vec::new();
        if subject.eq_ignore_ascii_case("current") {
            if let Some(sc) = scene.as_deref()
                && let Ok(idx) = usize::try_from(sc.formation_info.cur_formation)
                && let Some(form) = sc.formation_info.formations.get(idx)
                && form.cur_role > 0
            {
                ids.push(form.cur_role);
            }
        } else if subject.eq_ignore_ascii_case("team") {
            if let Some(sc) = scene.as_deref()
                && let Ok(idx) = usize::try_from(sc.formation_info.cur_formation)
                && let Some(form) = sc.formation_info.formations.get(idx)
            {
                for mr in form.roles.iter().flatten() {
                    ids.push(mr.role_id);
                }
            }
        } else if let Ok(parsed_id) = subject.parse::<i32>() {
            ids.push(parsed_id);
        }

        if ids.is_empty() {
            respond(
                events,
                player_id,
                "usage: level [union|current|team|role_id] [1..90]",
            );
            return Ok(());
        }

        let mut updated = 0usize;
        for role_id in ids {
            let Some(role) = role_comp.role_map.get_mut(&role_id) else {
                continue;
            };
            role.level = lv;
            role.breakthrough = role.breakthrough.max(breach);
            role.exp = 0;
            let _ = role.refresh_scaled_properties(assets, role_id);
            let r_path = format!("player/{player_id}/{}/{role_id}", RoleInfo::DATA_DIR);
            comp_util::save_struct(fs, role, &r_path)?;
            events.enqueue(Event::RoleInfoModified { player_id, role_id });
            if let Some(sc) = scene.as_deref_mut() {
                role_attribute_sync::refresh_role(
                    sink,
                    fs,
                    sc,
                    assets,
                    role_comp,
                    weapon_comp,
                    echo_comp,
                    role_id,
                )?;
            } else {
                role_attribute_sync::push_role_info_notify(
                    sink,
                    assets,
                    role_comp,
                    weapon_comp,
                    echo_comp,
                    role_id,
                );
            }
            updated += 1;
        }

        if updated == 0 {
            respond(events, player_id, format!("no matching roles found for {subject}"));
            return Ok(());
        }
        events.enqueue(Event::UpdateFormations {
            player_id,
            formation_id: 0,
            role_ids: Vec::new(),
        });
        respond(
            events,
            player_id,
            format!("set level {lv} for {updated} role(s)"),
        );
        Ok(())
    }
}

pub mod resonance_item {
    use super::*;

    pub const ALIAS: &str = "waveband";
    pub const DESCRIPTION: &str = "grants or clears resonance chain items.\nusage: waveband [current|full_role_id] [count|clear?]\ncount defaults to 6.";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        assets: &Assets,
        fs: &FileSystem,
        sink: &mut S,
        scene: Option<&Scene>,
        inventory_comp: &mut PlayerInventoryComponent,
        role_comp: &mut PlayerRoleComponent,
        weapon_comp: &PlayerWeaponComponent,
        echo_comp: &PlayerEchoComponent,
        subject: Option<&str>,
        action: Option<&str>,
    ) -> io::Result<()> {
        let player_id = inventory_comp.player_id;
        let Some(token) = subject else {
            respond(events, player_id, "no character specified");
            return Ok(());
        };
        if token.eq_ignore_ascii_case("all") {
            respond(events, player_id, "no character specified");
            return Ok(());
        }

        if action.is_some_and(|v| v.eq_ignore_ascii_case("clear")) {
            return clear_wavebands(
                events,
                assets,
                fs,
                sink,
                scene,
                inventory_comp,
                role_comp,
                weapon_comp,
                echo_comp,
                token,
            );
        }

        let Ok(item_count) = parse_count(action, 6) else {
            respond(events, player_id, "count must be a number or clear");
            return Ok(());
        };
        let Some(resolved) = resolve_item_id(events, player_id, assets, scene, token) else {
            return Ok(());
        };
        let old_count = inventory_comp.info.normal_item_count(resolved.item_id);
        inventory_comp.info.add_normal_item(resolved.item_id, item_count);
        save_inventory_info(fs, inventory_comp)?;

        let new_total = inventory_comp.info.normal_item_count(resolved.item_id);
        let updated_items = vec![PbNormalItem {
            Id: resolved.item_id,
            Count: new_total,
            ExpireTime: 0,
        }];
        if old_count == 0 {
            sink.push(
                NormalItemAddNotify::MSG_ID,
                &NormalItemAddNotify {
                    NormalItemList: updated_items,
                    NoTips: true,
                    Reason: 0,
                },
            );
        } else {
            sink.push(
                NormalItemUpdateNotify::MSG_ID,
                &NormalItemUpdateNotify {
                    NormalItemList: updated_items,
                    NoTips: true,
                },
            );
        }

        respond(
            events,
            player_id,
            format!(
                "granted {item_count} wavebands for {token} ({}, total {new_total})",
                resolved.item_id
            ),
        );
        Ok(())
    }
}

pub mod weapon_item {
    use super::*;

    pub const ALIAS: &str = "weapon";
    pub const DESCRIPTION: &str = "grants or clears weapon copies.\nusage: weapon [current|weapon_id] [count|clear?]\ncount defaults to 4.";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        assets: &Assets,
        fs: &FileSystem,
        sink: &mut S,
        weapon_comp: &mut PlayerWeaponComponent,
        role_comp: &PlayerRoleComponent,
        scene: Option<&Scene>,
        subject: Option<&str>,
        action: Option<&str>,
    ) -> io::Result<()> {
        let player_id = weapon_comp.player_id;
        let Some(token) = subject else {
            respond(events, player_id, "weapon must be specified");
            return Ok(());
        };
        if token.eq_ignore_ascii_case("all") {
            respond(events, player_id, "weapon must be specified");
            return Ok(());
        }
        let Some(weapon_id) =
            resolve_weapon_id(events, player_id, assets, scene, role_comp, weapon_comp, token)
        else {
            return Ok(());
        };
        if action.is_some_and(|v| v.eq_ignore_ascii_case("clear")) {
            return clear_weapon_copies(
                events,
                assets,
                fs,
                sink,
                scene,
                weapon_comp,
                weapon_id,
            );
        }

        let Ok(count) = parse_count(action, 4) else {
            respond(events, player_id, "count must be a number or clear");
            return Ok(());
        };

        let mut added = Vec::new();
        add_weapon_copies(fs, assets, weapon_comp, weapon_id, count, &mut added)?;
        push_weapon_adds(sink, added);
        respond(
            events,
            player_id,
            format!("granted {count} copies of weapon {weapon_id}"),
        );
        Ok(())
    }
}

pub mod unlockres {
    use super::*;

    pub const DESCRIPTION: &str =
        "Mark missing quest resources as confirmed and unlock locked quests";

    pub fn call<S: PushSink + ?Sized>(qctx: &mut QuestContext<'_, S>) -> io::Result<()> {
        let player_id = qctx.basic_comp.player_id;
        let missing = quest_engine::collect_missing_resource_quests(qctx);
        quest_engine::sync_quest_resource_availability(qctx, &[], &missing, false)?;
        respond(
            qctx.events,
            player_id,
            format!("unlocked {} missing-resource quests", missing.len()),
        );
        Ok(())
    }
}

fn parse_count(action: Option<&str>, default: i32) -> Result<i32, std::num::ParseIntError> {
    match action {
        Some(v) => v.parse::<i32>().map(|n| n.max(1)),
        None => Ok(default),
    }
}

fn save_inventory_info(
    fs: &FileSystem,
    inventory_comp: &PlayerInventoryComponent,
) -> io::Result<()> {
    let path = format!(
        "player/{}/{}",
        inventory_comp.player_id,
        InventoryInfo::DATA_PATH
    );
    comp_util::save_struct(fs, &inventory_comp.info, &path)
}

fn save_weapon_info(
    fs: &FileSystem,
    player_id: i32,
    incr_id: i32,
    weapon: &WeaponItem,
) -> io::Result<()> {
    let path = format!("player/{player_id}/{}/{incr_id}", WeaponItem::DATA_DIR);
    comp_util::save_struct(fs, weapon, &path)
}

fn add_weapon_copies(
    fs: &FileSystem,
    assets: &Assets,
    weapon_comp: &mut PlayerWeaponComponent,
    weapon_id: i32,
    count: i32,
    added: &mut Vec<PbWeaponItem>,
) -> io::Result<()> {
    let player_id = weapon_comp.player_id;
    for _ in 0..count.max(0) {
        let incr_id = special_item_incr::next(fs, player_id)?;
        let weapon = WeaponItem {
            id: weapon_id,
            func_value: WeaponItem::default_func_value(assets, weapon_id),
            ..Default::default()
        };
        save_weapon_info(fs, player_id, incr_id, &weapon)?;
        added.push(weapon.to_proto(incr_id));
        weapon_comp.weapon_map.insert(incr_id, weapon);
    }
    Ok(())
}

fn push_weapon_adds<S: PushSink + ?Sized>(sink: &mut S, added: Vec<PbWeaponItem>) {
    if added.is_empty() {
        return;
    }
    sink.push(
        WeaponItemAddNotify::MSG_ID,
        &WeaponItemAddNotify {
            WeaponItemList: added,
            AddFromRole: false,
            Reason: 0,
        },
    );
}

fn current_weapon_id(
    events: &mut EventQueue,
    player_id: i32,
    scene: Option<&Scene>,
    role_comp: &PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
) -> Option<i32> {
    let Some(sc) = scene else {
        respond(events, player_id, "no current formation");
        return None;
    };
    let Ok(idx) = usize::try_from(sc.formation_info.cur_formation) else {
        respond(events, player_id, "no current formation");
        return None;
    };
    let Some(form) = sc.formation_info.formations.get(idx) else {
        respond(events, player_id, "no current formation");
        return None;
    };
    let role_id = form.cur_role;
    let Some(role) = role_comp.role_map.get(&role_id) else {
        respond(events, player_id, "current role has no saved data");
        return None;
    };
    let Some(weapon) = weapon_comp.weapon_map.get(&role.weapon) else {
        respond(events, player_id, "weapon must be specified");
        return None;
    };
    Some(weapon.id)
}

fn resolve_weapon_id(
    events: &mut EventQueue,
    player_id: i32,
    assets: &Assets,
    scene: Option<&Scene>,
    role_comp: &PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    token: &str,
) -> Option<i32> {
    if token.eq_ignore_ascii_case("current") {
        return current_weapon_id(events, player_id, scene, role_comp, weapon_comp);
    }
    let Ok(weapon_id) = token.parse::<i32>() else {
        respond(events, player_id, "weapon id must be a number or current");
        return None;
    };
    if assets.tables.weapon_conf.get_data_by_id(&weapon_id).is_none() {
        respond(events, player_id, format!("unknown weapon id: {weapon_id}"));
        return None;
    }
    Some(weapon_id)
}

fn clear_weapon_copies<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    assets: &Assets,
    fs: &FileSystem,
    sink: &mut S,
    scene: Option<&Scene>,
    weapon_comp: &mut PlayerWeaponComponent,
    weapon_id: i32,
) -> io::Result<()> {
    let player_id = weapon_comp.player_id;
    let mut removed_ids = Vec::new();
    let mut updated_items = Vec::new();

    for (&incr_id, w) in &mut weapon_comp.weapon_map {
        if w.id != weapon_id {
            continue;
        }
        if w.role_id.is_none() {
            removed_ids.push(incr_id);
            continue;
        }
        if w.reson_level != 1 {
            if let Some(config) = assets.tables.weapon_conf.get_data_by_id(&weapon_id) {
                let old_reson = assets.tables.get_weapon_reson(config.ResonId, w.reson_level);
                let new_reson = assets.tables.get_weapon_reson(config.ResonId, 1);
                if let (Some(role_id), Some(sc)) = (w.role_id, scene) {
                    for (idx, storage) in sc.entities.iter().enumerate() {
                        if storage.config.config_id != role_id {
                            continue;
                        }
                        let Some(buffs) = storage.buffs.as_ref() else {
                            continue;
                        };
                        let entity = EntityHandle {
                            index: idx,
                            net_id: storage.net_id,
                        };
                        if let Some(old) = old_reson {
                            let handles: Vec<i32> = old
                                .Effect
                                .iter()
                                .filter_map(|&bid| buffs.get_by_buff_id(bid).map(|b| b.HandleId))
                                .collect();
                            if !handles.is_empty() {
                                events.enqueue(Event::BuffRemoval {
                                    entity,
                                    handle_ids: handles,
                                    natural_expiration: None,
                                });
                            }
                        }
                        if let Some(new) = new_reson {
                            let buffs_to_add: Vec<BuffAdditionEntry> = new
                                .Effect
                                .iter()
                                .map(|&id| BuffAdditionEntry {
                                    id,
                                    is_active: true,
                                    stack_count: 1,
                                    ..Default::default()
                                })
                                .collect();
                            if !buffs_to_add.is_empty() {
                                events.enqueue(Event::BuffAddition {
                                    target: entity,
                                    instigator: entity,
                                    buffs: buffs_to_add,
                                });
                            }
                        }
                        break;
                    }
                }
            }
            w.reson_level = 1;
            save_weapon_info(fs, player_id, incr_id, w)?;
            events.enqueue(Event::WeaponInfoModified { player_id, incr_id });
            updated_items.push(w.to_proto(incr_id));
        }
    }

    for &incr_id in &removed_ids {
        weapon_comp.weapon_map.remove(&incr_id);
        let _ = fs.delete_file(&format!(
            "player/{player_id}/{}/{incr_id}",
            WeaponItem::DATA_DIR
        ));
    }

    if !removed_ids.is_empty() {
        sink.push(
            WeaponItemRemoveNotify::MSG_ID,
            &WeaponItemRemoveNotify {
                WeaponItemIncrIdList: removed_ids,
            },
        );
    }
    push_weapon_adds(sink, updated_items);
    respond(events, player_id, format!("cleared weapon {weapon_id}"));
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn clear_wavebands<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    assets: &Assets,
    fs: &FileSystem,
    sink: &mut S,
    scene: Option<&Scene>,
    inventory_comp: &mut PlayerInventoryComponent,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    token: &str,
) -> io::Result<()> {
    let player_id = inventory_comp.player_id;
    if token.eq_ignore_ascii_case("all") {
        respond(events, player_id, "no character specified");
        return Ok(());
    }

    let Some(resolved) = resolve_item_id(events, player_id, assets, scene, token) else {
        return Ok(());
    };
    let _ = inventory_comp.info.remove_normal_item(resolved.item_id);
    let updated_items = vec![PbNormalItem {
        Id: resolved.item_id,
        Count: 0,
        ExpireTime: 0,
    }];

    if let Some(role_id) = resolved.role_id
        && let Some(role) = role_comp.role_map.get_mut(&role_id)
    {
        if role.resonant_chain_group_index != 0 {
            if let Some(sc) = scene {
                for (idx, storage) in sc.entities.iter().enumerate() {
                    if storage.config.config_id != role_id {
                        continue;
                    }
                    let Some(buffs) = storage.buffs.as_ref() else {
                        continue;
                    };
                    let mut handles_to_remove = Vec::new();
                    for chain in &assets.tables.resonant_chain.items {
                        if chain.GroupId != role_id
                            || chain.GroupIndex > role.resonant_chain_group_index
                        {
                            continue;
                        }
                        for &buff_id in &chain.BuffIds {
                            if let Some(buff) = buffs.get_by_buff_id(buff_id) {
                                handles_to_remove.push(buff.HandleId);
                            }
                        }
                    }
                    if !handles_to_remove.is_empty() {
                        events.enqueue(Event::BuffRemoval {
                            entity: EntityHandle {
                                index: idx,
                                net_id: storage.net_id,
                            },
                            handle_ids: handles_to_remove,
                            natural_expiration: None,
                        });
                    }
                    break;
                }
            }
            role.resonant_chain_group_index = 0;
            events.enqueue(Event::RoleInfoModified { player_id, role_id });
        }
        let role_snapshot = role.clone();

        if let Ok(pb_role) = role_stats::to_client_role_info(
            assets,
            role_comp,
            role_id,
            &role_snapshot,
            weapon_comp,
            echo_comp,
        ) {
            sink.push(
                PbGetRoleListNotify::MSG_ID,
                &PbGetRoleListNotify {
                    RoleList: vec![pb_role],
                },
            );
        }
    }

    save_inventory_info(fs, inventory_comp)?;
    sink.push(
        NormalItemUpdateNotify::MSG_ID,
        &NormalItemUpdateNotify {
            NormalItemList: updated_items,
            NoTips: true,
        },
    );
    respond(events, player_id, format!("cleared wavebands for {token}"));
    Ok(())
}

struct ResolvedItem {
    item_id: i32,
    role_id: Option<i32>,
}

fn resolve_item_id(
    events: &mut EventQueue,
    player_id: i32,
    assets: &Assets,
    scene: Option<&Scene>,
    token: &str,
) -> Option<ResolvedItem> {
    if token.eq_ignore_ascii_case("current") {
        let Some(sc) = scene else {
            respond(events, player_id, "no current formation");
            return None;
        };
        let Ok(idx) = usize::try_from(sc.formation_info.cur_formation) else {
            respond(events, player_id, "no current formation");
            return None;
        };
        let Some(form) = sc.formation_info.formations.get(idx) else {
            respond(events, player_id, "no current formation");
            return None;
        };
        return role_to_item(events, player_id, assets, form.cur_role);
    }

    if let Ok(id) = token.parse::<i32>() {
        if id < 10_000_000 {
            respond(events, player_id, format!("use the full character id: {id}"));
            return None;
        }
        return role_to_item(events, player_id, assets, id - 10_000_000);
    }

    respond(
        events,
        player_id,
        "character id must be current or a full id",
    );
    None
}

fn role_to_item(
    events: &mut EventQueue,
    player_id: i32,
    assets: &Assets,
    role_id: i32,
) -> Option<ResolvedItem> {
    let Some(role) = assets.tables.role_info.get_data_by_id(&role_id) else {
        respond(events, player_id, format!("unknown role id: {role_id}"));
        return None;
    };
    if role.ResonantChainGroupId == 0 {
        respond(
            events,
            player_id,
            format!("role {role_id} has no resonance chain group"),
        );
        return None;
    }
    let Some(item_id) = first_activate_item(assets, role.ResonantChainGroupId) else {
        respond(
            events,
            player_id,
            format!("role {role_id} has no resonance chain item"),
        );
        return None;
    };
    Some(ResolvedItem {
        item_id,
        role_id: Some(role_id),
    })
}

fn first_activate_item(assets: &Assets, group_id: i32) -> Option<i32> {
    for chain in &assets.tables.resonant_chain.items {
        if chain.GroupId != group_id || chain.GroupIndex != 1 {
            continue;
        }
        return chain.ActivateConsume.0.keys().next().copied();
    }
    None
}
