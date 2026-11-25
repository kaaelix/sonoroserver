use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_gamesv_fs::weapon_item::WeaponItem;
use sonoro_proto::enums::{PlayerAttrKey, PlayerAttrType};
use sonoro_proto::pb::{
    MapUnlockFieldNotify, PbGetRoleListNotify, PlayerAttr, PlayerAttrNotify,
    PlayerAttrValueOneof, TeleportUpdateNotify, WeaponItemAddNotify,
};

use crate::commands::respond;
use crate::component::comp_util;
use crate::component::player::{
    PlayerBasicComponent, PlayerEchoComponent, PlayerInventoryComponent, PlayerMapComponent,
    PlayerRoleComponent, PlayerWeaponComponent,
};
use crate::event_queue::EventQueue;
use crate::events::Event;
use crate::helpers::quest_types::QuestContext;
use crate::helpers::reward::{self, RewardContext, RewardItem};
use crate::helpers::{
    quest_actions, quest_engine, role as role_helper, role_attribute_sync, role_stats,
};
use crate::scene::Scene;
use crate::sink::PushSink;

pub const fn breach_for_level(lv: i32) -> i32 {
    if lv > 80 {
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
    }
}

fn push_all_owned_roles<S: PushSink + ?Sized>(
    sink: &mut S,
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
) {
    let selected_main_role = role_helper::selected_main_role_id(
        assets,
        role_comp,
        basic_comp.info.attributes.sex,
        basic_comp.info.selected_main_role_id,
    );
    let mut role_list = Vec::with_capacity(role_comp.role_map.len());
    let role_entries: Vec<(i32, RoleInfo)> = role_comp
        .role_map
        .iter()
        .filter(|&(&rid, _)| (1000..=1999).contains(&rid))
        .filter(|&(&rid, _)| {
            selected_main_role
                .is_none_or(|selected| !role_helper::is_main_role(assets, rid) || rid == selected)
        })
        .map(|(&rid, info)| (rid, info.clone()))
        .collect();
    for (rid, role_info) in &role_entries {
        if let Ok(pb_role) = role_stats::to_client_role_info(
            assets,
            role_comp,
            *rid,
            role_info,
            weapon_comp,
            echo_comp,
        ) {
            role_list.push(pb_role);
        }
    }
    if !role_list.is_empty() {
        sink.push(
            PbGetRoleListNotify::MSG_ID,
            &PbGetRoleListNotify {
                RoleList: role_list,
            },
        );
    }
}

fn save_role(fs: &FileSystem, player_id: i32, role_id: i32, role: &RoleInfo) -> io::Result<()> {
    let path = format!("player/{player_id}/{}/{role_id}", RoleInfo::DATA_DIR);
    comp_util::save_struct(fs, role, &path)
}

fn max_resonant_chain_index(assets: &Assets, group_id: i32) -> i32 {
    if group_id <= 0 {
        return 6;
    }
    let max_idx = assets
        .tables
        .resonant_chain
        .items
        .iter()
        .filter(|c| c.GroupId == group_id)
        .map(|c| c.GroupIndex)
        .max()
        .unwrap_or(0);
    if max_idx > 0 { max_idx } else { 6 }
}

fn max_skill_level_for_id(assets: &Assets, skill_id: i32) -> i32 {
    assets
        .tables
        .skill
        .get_data_by_id(&skill_id)
        .filter(|s| s.MaxSkillLevel > 0)
        .map_or(10, |s| s.MaxSkillLevel)
}

fn set_role_max_skills_and_chain(
    assets: &Assets,
    role_id: i32,
    role: &mut RoleInfo,
    chain_opt: Option<i32>,
) {
    let Some(rc) = assets.tables.resolve_base_role_info(role_id) else {
        return;
    };
    let max_chain = max_resonant_chain_index(assets, rc.ResonantChainGroupId);
    role.resonant_chain_group_index = chain_opt.map_or(max_chain, |c| c.clamp(0, max_chain));

    if role.skill_node_state.is_empty() {
        let _ = role.add_default_skills(assets, role_id);
    }
    for sn in &mut role.skill_node_state {
        sn.active = true;
    }

    let mut skill_pairs: Vec<(i32, i32)> = Vec::new();
    for node in &assets.tables.skill_tree.items {
        if node.NodeGroup != rc.SkillTreeGroupId || node.SkillId <= 0 {
            continue;
        }
        if !skill_pairs.iter().any(|&(id, _)| id == node.SkillId) {
            skill_pairs.push((node.SkillId, max_skill_level_for_id(assets, node.SkillId)));
        }
    }
    if !skill_pairs.is_empty() {
        role.skills = skill_pairs;
    } else {
        for s in &mut role.skills {
            s.1 = max_skill_level_for_id(assets, s.0);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn sync_role_on_scene_or_notify<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    fs: &FileSystem,
    assets: &Assets,
    sink: &mut S,
    scene_opt: Option<&mut Scene>,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    role_id: i32,
) -> io::Result<()> {
    let player_id = role_comp.player_id;
    events.enqueue(Event::RoleInfoModified { player_id, role_id });
    if let Some(scene) = scene_opt {
        role_attribute_sync::refresh_role(
            sink,
            fs,
            scene,
            assets,
            role_comp,
            weapon_comp,
            echo_comp,
            role_id,
        )
    } else {
        role_attribute_sync::push_role_info_notify(
            sink,
            assets,
            role_comp,
            weapon_comp,
            echo_comp,
            role_id,
        );
        Ok(())
    }
}

pub mod world_level {
    use super::*;

    pub const ALIAS: &str = "wl";
    pub const DESCRIPTION: &str = "sets SOL3 Phase / World Level (1..8).\nusage: world_level [1..8]";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        fs: &FileSystem,
        sink: &mut S,
        basic_comp: &mut PlayerBasicComponent,
        target_wl: i32,
    ) -> io::Result<()> {
        let player_id = basic_comp.player_id;
        let wl = target_wl.clamp(1, 8);
        basic_comp.info.attributes.origin_world_level = wl;
        basic_comp.info.attributes.cur_world_level = wl;
        role_helper::save_basic_info(fs, basic_comp)?;

        sink.push(
            PlayerAttrNotify::MSG_ID,
            &PlayerAttrNotify {
                Attributes: vec![
                    PlayerAttr {
                        Key: Some(PlayerAttrKey::OriginWorldLevel),
                        ValueType: Some(PlayerAttrType::Int32),
                        Value: Some(PlayerAttrValueOneof::Int32Value(wl)),
                    },
                    PlayerAttr {
                        Key: Some(PlayerAttrKey::CurWorldLevel),
                        ValueType: Some(PlayerAttrType::Int32),
                        Value: Some(PlayerAttrValueOneof::Int32Value(wl)),
                    },
                ],
            },
        );
        respond(events, player_id, format!("set SOL3 / World Level to {wl}"));
        Ok(())
    }
}

pub mod unlock_roles {
    use super::*;

    pub const ALIAS: &str = "uroles";
    pub const DESCRIPTION: &str =
        "unlocks all playable resonators or one role by id.\nusage: unlock_roles [all|role_id?]";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        fs: &FileSystem,
        assets: &Assets,
        sink: &mut S,
        basic_comp: &mut PlayerBasicComponent,
        role_comp: &mut PlayerRoleComponent,
        weapon_comp: &mut PlayerWeaponComponent,
        echo_comp: &mut PlayerEchoComponent,
        target_opt: Option<&str>,
    ) -> io::Result<()> {
        let player_id = role_comp.player_id;
        let token = target_opt.unwrap_or("all");
        let mut added = 0usize;
        let mut starter_weapons = Vec::new();

        if !token.eq_ignore_ascii_case("all") {
            let Ok(raw_id) = token.parse::<i32>() else {
                respond(events, player_id, format!("invalid role_id: {token}"));
                return Ok(());
            };
            let rid = if raw_id >= 10_000_000 {
                raw_id - 10_000_000
            } else {
                raw_id
            };
            let Some(rc) = assets.tables.role_info.get_data_by_id(&rid) else {
                respond(events, player_id, format!("unknown role id: {rid}"));
                return Ok(());
            };
            if rc.IsTrial || rc.RoleType != 1 || !(1000..=1999).contains(&rid) {
                respond(
                    events,
                    player_id,
                    format!("role id {rid} is not a playable character"),
                );
                return Ok(());
            }
            role_comp.trial_roles.remove(&rid);
            if role_helper::is_main_role(assets, rid) {
                let target_main = role_helper::main_role_id_for_element(
                    assets,
                    basic_comp.info.attributes.sex,
                    rc.ElementId,
                )
                .unwrap_or(rid);
                let fallback_main = if basic_comp.info.attributes.sex == 0 {
                    1502
                } else {
                    1501
                };
                let src_main = role_helper::selected_main_role_id(
                    assets,
                    role_comp,
                    basic_comp.info.attributes.sex,
                    basic_comp.info.selected_main_role_id,
                )
                .unwrap_or(fallback_main);
                let _ = role_helper::transfer_main_role_state(
                    fs,
                    assets,
                    None,
                    basic_comp,
                    role_comp,
                    weapon_comp,
                    echo_comp,
                    src_main,
                    target_main,
                )
                .map_err(io::Error::other)?;
                role_helper::save_basic_info(fs, basic_comp)?;
                added = 1;
            } else if let std::collections::btree_map::Entry::Vacant(entry) =
                role_comp.role_map.entry(rid)
            {
                let new_role = RoleInfo::create_default(assets, rid).map_err(io::Error::other)?;
                save_role(fs, player_id, rid, &new_role)?;
                entry.insert(new_role);
                added = 1;
            }
            if role_helper::ensure_role_weapon(fs, assets, role_comp, weapon_comp, rid)
                .unwrap_or(false)
                && let Some(rinfo) = role_comp.role_map.get(&rid)
                && let Some(witem) = weapon_comp.weapon_map.get(&rinfo.weapon)
            {
                starter_weapons.push(witem.to_proto(rinfo.weapon));
            }
            if !starter_weapons.is_empty() {
                sink.push(
                    WeaponItemAddNotify::MSG_ID,
                    &WeaponItemAddNotify {
                        WeaponItemList: starter_weapons,
                        AddFromRole: true,
                        Reason: 1,
                    },
                );
            }
            push_all_owned_roles(sink, assets, basic_comp, role_comp, weapon_comp, echo_comp);
            respond(
                events,
                player_id,
                format!("unlocked role {rid} (new={added})"),
            );
            return Ok(());
        }

        let selected_main = role_helper::selected_main_role_id(
            assets,
            role_comp,
            basic_comp.info.attributes.sex,
            basic_comp.info.selected_main_role_id,
        );
        for rc in &assets.tables.role_info.items {
            if !(1000..=1999).contains(&rc.Id) || rc.IsTrial || rc.RoleType != 1 {
                continue;
            }
            if role_helper::is_main_role(assets, rc.Id) {
                if let Some(sel) = selected_main {
                    if rc.Id != sel {
                        continue;
                    }
                } else if !role_helper::is_main_role_for_gender(
                    assets,
                    basic_comp.info.attributes.sex,
                    rc.Id,
                ) {
                    continue;
                }
            }
            role_comp.trial_roles.remove(&rc.Id);
            if let std::collections::btree_map::Entry::Vacant(entry) =
                role_comp.role_map.entry(rc.Id)
            {
                let Ok(new_role) = RoleInfo::create_default(assets, rc.Id) else {
                    continue;
                };
                save_role(fs, player_id, rc.Id, &new_role)?;
                entry.insert(new_role);
                added += 1;
            }
            if role_helper::ensure_role_weapon(fs, assets, role_comp, weapon_comp, rc.Id)
                .unwrap_or(false)
                && let Some(rinfo) = role_comp.role_map.get(&rc.Id)
                && let Some(witem) = weapon_comp.weapon_map.get(&rinfo.weapon)
            {
                starter_weapons.push(witem.to_proto(rinfo.weapon));
            }
        }
        if !starter_weapons.is_empty() {
            sink.push(
                WeaponItemAddNotify::MSG_ID,
                &WeaponItemAddNotify {
                    WeaponItemList: starter_weapons,
                    AddFromRole: true,
                    Reason: 1,
                },
            );
        }
        push_all_owned_roles(sink, assets, basic_comp, role_comp, weapon_comp, echo_comp);
        respond(
            events,
            player_id,
            format!(
                "unlocked {added} new resonator(s) (total owned={})",
                role_comp.role_map.len()
            ),
        );
        Ok(())
    }
}

pub mod role_max {
    use super::*;

    pub const ALIAS: &str = "rmax";
    pub const DESCRIPTION: &str = "maxes level, breakthrough, resonance chain S6, and skills.\nusage: role_max [current|team|all|role_id?] [level?]";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        fs: &FileSystem,
        assets: &Assets,
        sink: &mut S,
        mut scene_opt: Option<&mut Scene>,
        role_comp: &mut PlayerRoleComponent,
        weapon_comp: &PlayerWeaponComponent,
        echo_comp: &PlayerEchoComponent,
        subject_opt: Option<&str>,
        lv_opt: Option<i32>,
    ) -> io::Result<()> {
        let player_id = role_comp.player_id;
        let subject = subject_opt.unwrap_or("team");
        let lv = lv_opt.unwrap_or(90).clamp(1, 90);
        let breach = breach_for_level(lv);
        let mut updated = 0usize;

        let target_ids: Vec<i32> = if subject.eq_ignore_ascii_case("all") {
            role_comp
                .role_map
                .keys()
                .copied()
                .filter(|rid| (1000..=1999).contains(rid))
                .collect()
        } else if subject.eq_ignore_ascii_case("current") {
            scene_opt
                .as_deref()
                .and_then(|sc| {
                    usize::try_from(sc.formation_info.cur_formation)
                        .ok()
                        .and_then(|idx| sc.formation_info.formations.get(idx))
                })
                .filter(|f| f.cur_role > 0)
                .map(|f| vec![f.cur_role])
                .unwrap_or_default()
        } else if subject.eq_ignore_ascii_case("team") {
            scene_opt
                .as_deref()
                .and_then(|sc| {
                    usize::try_from(sc.formation_info.cur_formation)
                        .ok()
                        .and_then(|idx| sc.formation_info.formations.get(idx))
                })
                .map(|f| f.roles.iter().flatten().map(|r| r.role_id).collect())
                .unwrap_or_default()
        } else if let Ok(parsed) = subject.parse::<i32>() {
            vec![if parsed >= 10_000_000 {
                parsed - 10_000_000
            } else {
                parsed
            }]
        } else {
            Vec::new()
        };

        for rid in target_ids {
            let Some(role) = role_comp.role_map.get_mut(&rid) else {
                continue;
            };
            role.level = lv;
            role.breakthrough = breach;
            role.exp = 0;
            set_role_max_skills_and_chain(assets, rid, role, None);
            let _ = role.refresh_scaled_properties(assets, rid);
            let role_snapshot = role.clone();
            save_role(fs, player_id, rid, &role_snapshot)?;
            sync_role_on_scene_or_notify(
                events,
                fs,
                assets,
                sink,
                scene_opt.as_deref_mut(),
                role_comp,
                weapon_comp,
                echo_comp,
                rid,
            )?;
            updated += 1;
        }

        if updated > 0 {
            events.enqueue(Event::UpdateFormations {
                player_id,
                formation_id: 0,
                role_ids: Vec::new(),
            });
        }
        respond(
            events,
            player_id,
            format!("maxxed {updated} resonator(s) to Lv.{lv} S6 + Max Skills"),
        );
        Ok(())
    }
}

pub mod role_chain {
    use super::*;

    pub const ALIAS: &str = "s";
    pub const DESCRIPTION: &str =
        "sets resonance chain rank (0..6).\nusage: role_chain [current|team|all|role_id] [0..6]";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        fs: &FileSystem,
        assets: &Assets,
        sink: &mut S,
        mut scene_opt: Option<&mut Scene>,
        role_comp: &mut PlayerRoleComponent,
        weapon_comp: &PlayerWeaponComponent,
        echo_comp: &PlayerEchoComponent,
        subject: &str,
        rank: i32,
    ) -> io::Result<()> {
        let player_id = role_comp.player_id;
        let clamped = rank.clamp(0, 6);
        let mut updated = 0usize;

        let target_ids: Vec<i32> = if subject.eq_ignore_ascii_case("all") {
            role_comp
                .role_map
                .keys()
                .copied()
                .filter(|rid| (1000..=1999).contains(rid))
                .collect()
        } else if subject.eq_ignore_ascii_case("current") {
            scene_opt
                .as_deref()
                .and_then(|sc| {
                    usize::try_from(sc.formation_info.cur_formation)
                        .ok()
                        .and_then(|idx| sc.formation_info.formations.get(idx))
                })
                .map(|f| vec![f.cur_role])
                .unwrap_or_default()
        } else if subject.eq_ignore_ascii_case("team") {
            scene_opt
                .as_deref()
                .and_then(|sc| {
                    usize::try_from(sc.formation_info.cur_formation)
                        .ok()
                        .and_then(|idx| sc.formation_info.formations.get(idx))
                })
                .map(|f| f.roles.iter().flatten().map(|r| r.role_id).collect())
                .unwrap_or_default()
        } else if let Ok(parsed) = subject.parse::<i32>() {
            vec![if parsed >= 10_000_000 {
                parsed - 10_000_000
            } else {
                parsed
            }]
        } else {
            Vec::new()
        };

        for rid in target_ids {
            let Some(role) = role_comp.role_map.get_mut(&rid) else {
                continue;
            };
            role.resonant_chain_group_index = clamped;
            let role_snapshot = role.clone();
            save_role(fs, player_id, rid, &role_snapshot)?;
            sync_role_on_scene_or_notify(
                events,
                fs,
                assets,
                sink,
                scene_opt.as_deref_mut(),
                role_comp,
                weapon_comp,
                echo_comp,
                rid,
            )?;
            updated += 1;
        }
        respond(
            events,
            player_id,
            format!("set Resonance Chain S{clamped} on {updated} role(s)"),
        );
        Ok(())
    }
}

pub mod weapon_max {
    use super::*;

    pub const ALIAS: &str = "wmax";
    pub const DESCRIPTION: &str =
        "maxes equipped or all weapons to Lv.90 R5.\nusage: weapon_max [equipped|current|all] [level?] [rank?]";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        fs: &FileSystem,
        assets: &Assets,
        sink: &mut S,
        mut scene_opt: Option<&mut Scene>,
        role_comp: &mut PlayerRoleComponent,
        weapon_comp: &mut PlayerWeaponComponent,
        echo_comp: &PlayerEchoComponent,
        scope_opt: Option<&str>,
        lv_opt: Option<i32>,
        rank_opt: Option<i32>,
    ) -> io::Result<()> {
        let player_id = role_comp.player_id;
        let scope = scope_opt.unwrap_or("equipped");
        let lv = lv_opt.unwrap_or(90).clamp(1, 90);
        let rank = rank_opt.unwrap_or(5).clamp(1, 5);
        let breach = breach_for_level(lv);

        let cur_weapon_incr = scene_opt
            .as_deref()
            .and_then(|sc| {
                usize::try_from(sc.formation_info.cur_formation)
                    .ok()
                    .and_then(|idx| sc.formation_info.formations.get(idx))
            })
            .and_then(|f| role_comp.role_map.get(&f.cur_role))
            .map_or(0, |r| r.weapon);

        let mut updated_weapons = Vec::new();
        let mut touched_roles = Vec::new();
        for (&incr_id, w) in &mut weapon_comp.weapon_map {
            let matched = if scope.eq_ignore_ascii_case("all") {
                true
            } else if scope.eq_ignore_ascii_case("current") {
                incr_id == cur_weapon_incr
            } else {
                w.role_id.is_some()
            };
            if !matched {
                continue;
            }
            let max_reson = assets
                .tables
                .weapon_conf
                .get_data_by_id(&w.id)
                .filter(|c| c.ResonLevelLimit > 0)
                .map_or(5, |c| c.ResonLevelLimit);
            w.level = lv;
            w.breach = breach;
            w.exp = 0;
            w.reson_level = rank.min(max_reson);

            let path = format!("player/{player_id}/{}/{incr_id}", WeaponItem::DATA_DIR);
            comp_util::save_struct(fs, w, &path)?;
            events.enqueue(Event::WeaponInfoModified { player_id, incr_id });
            updated_weapons.push(w.to_proto(incr_id));
            if let Some(rid) = w.role_id {
                touched_roles.push(rid);
            }
        }

        let updated_count = updated_weapons.len();
        if !updated_weapons.is_empty() {
            sink.push(
                WeaponItemAddNotify::MSG_ID,
                &WeaponItemAddNotify {
                    WeaponItemList: updated_weapons,
                    AddFromRole: false,
                    Reason: 0,
                },
            );
        }
        for rid in touched_roles {
            sync_role_on_scene_or_notify(
                events,
                fs,
                assets,
                sink,
                scene_opt.as_deref_mut(),
                role_comp,
                weapon_comp,
                echo_comp,
                rid,
            )?;
        }
        respond(
            events,
            player_id,
            format!("upgraded {updated_count} weapon(s) to Lv.{lv} R{rank}"),
        );
        Ok(())
    }
}

pub mod quest_pick {
    use super::*;

    pub const ALIAS: &str = "qpick";
    pub const DESCRIPTION: &str =
        "picks/starts and tracks a quest by ID.\nusage: quest_pick [quest_id] [force?]";

    pub fn call<S: PushSink + ?Sized>(
        qctx: &mut QuestContext<'_, S>,
        quest_id: i32,
        force_opt: Option<bool>,
    ) -> io::Result<()> {
        let player_id = qctx.basic_comp.player_id;
        let ok = quest_engine::gm_pick_quest(qctx, quest_id, force_opt.unwrap_or(true))?;
        if !ok {
            respond(qctx.events, player_id, format!("unknown quest id {quest_id}"));
            return Ok(());
        }
        respond(
            qctx.events,
            player_id,
            format!("picked and tracked quest {quest_id}"),
        );
        Ok(())
    }
}

pub mod quest_step {
    use super::*;

    pub const ALIAS: &str = "qstep";
    pub const DESCRIPTION: &str =
        "skips the active objective node of current/target quest.\nusage: quest_step [quest_id?]";

    pub fn call<S: PushSink + ?Sized>(
        qctx: &mut QuestContext<'_, S>,
        quest_id_opt: Option<i32>,
    ) -> io::Result<()> {
        let player_id = qctx.basic_comp.player_id;
        let qid = quest_engine::gm_skip_quest_node(qctx, quest_id_opt)?;
        if qid == 0 {
            respond(qctx.events, player_id, "no active quest node found to step");
            return Ok(());
        }
        respond(
            qctx.events,
            player_id,
            format!("skipped current step of quest {qid}"),
        );
        Ok(())
    }
}

pub mod quest_skip {
    use super::*;

    pub const ALIAS: &str = "qskip";
    pub const DESCRIPTION: &str =
        "skips/completes entire tracked or specified quest.\nusage: quest_skip [quest_id?]";

    pub fn call<S: PushSink + ?Sized>(
        qctx: &mut QuestContext<'_, S>,
        quest_id_opt: Option<i32>,
    ) -> io::Result<()> {
        let player_id = qctx.basic_comp.player_id;
        let qid = quest_engine::gm_skip_quest(qctx, quest_id_opt)?;
        if qid == 0 {
            respond(qctx.events, player_id, "no matching quest found to skip");
            return Ok(());
        }
        respond(
            qctx.events,
            player_id,
            format!("completed/skipped quest {qid}"),
        );
        Ok(())
    }
}

pub mod unlock_tp {
    use super::*;

    pub const ALIAS: &str = "tpall";
    pub const DESCRIPTION: &str =
        "unlocks all Resonance Beacons / Nexus teleporters and map fog.\nusage: unlock_tp";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        fs: &FileSystem,
        assets: &Assets,
        sink: &mut S,
        map_comp: &mut PlayerMapComponent,
    ) -> io::Result<()> {
        let player_id = map_comp.player_id;
        let mut tp_added = 0usize;
        let mut fog_added = 0usize;

        for tp in &assets.tables.teleporter.items {
            if map_comp.info.unlock_teleporter(tp.Id) {
                tp_added += 1;
            }
            if tp.FogId > 0 && map_comp.info.unlock_fog(tp.FogId) {
                fog_added += 1;
                sink.push(
                    MapUnlockFieldNotify::MSG_ID,
                    &MapUnlockFieldNotify { FieldId: tp.FogId },
                );
            }
        }
        for fog in &assets.tables.map_fog.items {
            if fog.Fog > 0 && map_comp.info.unlock_fog(fog.Fog) {
                fog_added += 1;
                sink.push(
                    MapUnlockFieldNotify::MSG_ID,
                    &MapUnlockFieldNotify { FieldId: fog.Fog },
                );
            }
        }
        if tp_added > 0 || fog_added > 0 {
            map_comp.save(fs)?;
        }
        let tp_ids = map_comp.info.unlocked_teleporters.clone();
        let total = tp_ids.len();
        sink.push(
            TeleportUpdateNotify::MSG_ID,
            &TeleportUpdateNotify { Ids: tp_ids },
        );
        respond(
            events,
            player_id,
            format!("unlocked +{tp_added} teleporters (+{fog_added} map fog zones, total={total})"),
        );
        Ok(())
    }
}

pub mod unlock_func {
    use super::*;

    pub const ALIAS: &str = "funcall";
    pub const DESCRIPTION: &str =
        "unlocks all system menus/functions (Convene, Events, Tower, Co-op, Synthesis).\nusage: unlock_func";

    pub fn call<S: PushSink + ?Sized>(qctx: &mut QuestContext<'_, S>) -> io::Result<()> {
        let player_id = qctx.basic_comp.player_id;
        let mut added = 0usize;
        for fc in &qctx.assets.tables.function_condition.items {
            if fc.FunctionId <= 0 || !fc.IsOn {
                continue;
            }
            if qctx.quest_comp.info.unlock_system(fc.FunctionId) {
                added += 1;
            }
        }
        quest_actions::push_func_open(qctx);
        qctx.quest_comp.save(qctx.fs)?;
        let total = qctx.quest_comp.info.unlocked_systems.len();
        respond(
            qctx.events,
            player_id,
            format!("unlocked +{added} functions (total={total})"),
        );
        Ok(())
    }
}

pub mod money {
    use super::*;

    pub const ALIAS: &str = "cash";
    pub const DESCRIPTION: &str =
        "grants Astrite, Shell Credit, Lunite, Tides, and Waveplates.\nusage: money";

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
    ) -> io::Result<()> {
        let player_id = basic_comp.player_id;
        const ITEMS: &[RewardItem] = &[
            RewardItem {
                item_id: 3,
                count: 160_000,
            },
            RewardItem {
                item_id: 2,
                count: 10_000_000,
            },
            RewardItem {
                item_id: 4,
                count: 64_800,
            },
            RewardItem {
                item_id: 50002,
                count: 500,
            },
            RewardItem {
                item_id: 50005,
                count: 500,
            },
            RewardItem {
                item_id: 50001,
                count: 500,
            },
            RewardItem {
                item_id: 50013,
                count: 5000,
            },
            RewardItem {
                item_id: 5,
                count: 1000,
            },
        ];
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
        let _ = reward::grant_items(&mut rctx, 0, true, ITEMS, 1)?;
        respond(
            events,
            player_id,
            "Granted currencies: +160k Astrite, +10M Shell Credit, +64.8k Lunite, +500 Tides, +1000 Waveplates",
        );
        Ok(())
    }
}

pub mod time_set {
    use super::*;

    pub const ALIAS: &str = "time";
    pub const DESCRIPTION: &str =
        "sets in-game clock hour and minute.\nusage: time_set [0..23] [0..59?]";

    pub fn call<S: PushSink + ?Sized>(
        qctx: &mut QuestContext<'_, S>,
        hour: i32,
        minute_opt: Option<i32>,
    ) -> io::Result<()> {
        let player_id = qctx.basic_comp.player_id;
        let h = hour.rem_euclid(24);
        let m = minute_opt.unwrap_or(0).rem_euclid(60);
        quest_engine::set_scene_time(qctx, h, m, true)?;
        qctx.quest_comp.save(qctx.fs)?;
        respond(
            qctx.events,
            player_id,
            format!("set in-game time to {h:02}:{m:02}"),
        );
        Ok(())
    }
}

pub mod repair_account {
    use super::*;

    pub const ALIAS: &str = "repair";
    pub const DESCRIPTION: &str = "repairs broken quest unlocks, orphaned trial roles/locks, Rover & RoleInfo weapons.\nusage: repair_account";

    pub fn call<S: PushSink + ?Sized>(qctx: &mut QuestContext<'_, S>) -> io::Result<()> {
        let player_id = qctx.basic_comp.player_id;
        let rep = quest_engine::reconcile_account_state(qctx, true)?;
        if let Some(role_comp) = qctx.role_comp.as_deref_mut() {
            role_helper::push_role_metadata_notifies(qctx.sink, qctx.assets, qctx.basic_comp, role_comp);
            push_all_owned_roles(
                qctx.sink,
                qctx.assets,
                qctx.basic_comp,
                role_comp,
                qctx.weapon_comp,
                qctx.echo_comp,
            );
        }
        respond(
            qctx.events,
            player_id,
            format!(
                "repaired account: quests={}, systems=+{}, trials_cleared={}, locks_cleared={}, leaked_roles_removed={}, roles_fixed={}, weapons_fixed={}",
                rep.repaired_quests,
                rep.unlocked_systems,
                rep.removed_trial_roles,
                rep.unlocked_locks,
                rep.role_summary.removed_leaks,
                rep.role_summary.fixed_roles,
                rep.role_summary.fixed_weapons,
            ),
        );
        Ok(())
    }
}

pub mod switch_rover {
    use super::*;

    pub const ALIAS: &str = "rover";
    pub const DESCRIPTION: &str =
        "switches Rover element safely (spectro|havoc|aero|electro).\nusage: switch_rover [spectro|havoc|aero|electro]";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        fs: &FileSystem,
        assets: &Assets,
        sink: &mut S,
        mut scene_opt: Option<&mut Scene>,
        basic_comp: &mut PlayerBasicComponent,
        role_comp: &mut PlayerRoleComponent,
        weapon_comp: &mut PlayerWeaponComponent,
        echo_comp: &mut PlayerEchoComponent,
        element_arg: &str,
    ) -> io::Result<()> {
        let player_id = basic_comp.player_id;
        let elem_id = if element_arg.eq_ignore_ascii_case("spectro")
            || element_arg == "1501"
            || element_arg == "1502"
        {
            5
        } else if element_arg.eq_ignore_ascii_case("havoc")
            || element_arg == "1604"
            || element_arg == "1605"
        {
            6
        } else if element_arg.eq_ignore_ascii_case("aero")
            || element_arg == "1406"
            || element_arg == "1408"
        {
            4
        } else if element_arg.eq_ignore_ascii_case("electro")
            || element_arg == "1309"
            || element_arg == "1310"
        {
            3
        } else {
            respond(
                events,
                player_id,
                format!("unknown Rover element '{element_arg}' (use spectro|havoc|aero|electro)"),
            );
            return Ok(());
        };

        let gender = if basic_comp.info.attributes.sex == 0 {
            0
        } else {
            1
        };
        let Some(target_id) = role_helper::main_role_id_for_element(assets, gender, elem_id) else {
            respond(
                events,
                player_id,
                format!("Rover element {elem_id} not found for gender {gender}"),
            );
            return Ok(());
        };
        let default_main = if gender == 0 { 1502 } else { 1501 };
        let source_id = role_helper::selected_main_role_id(
            assets,
            role_comp,
            gender,
            basic_comp.info.selected_main_role_id,
        )
        .unwrap_or(default_main);

        let res = role_helper::transfer_main_role_state(
            fs,
            assets,
            scene_opt.as_deref_mut(),
            basic_comp,
            role_comp,
            weapon_comp,
            echo_comp,
            source_id,
            target_id,
        )
        .map_err(io::Error::other)?;

        if res.basic_changed {
            role_helper::save_basic_info(fs, basic_comp)?;
        }
        if res.formation_changed
            && let Some(sc) = scene_opt.as_deref()
        {
            let _ = sc.save_persistent(fs);
        }

        role_helper::push_role_metadata_notifies(sink, assets, basic_comp, role_comp);
        push_all_owned_roles(sink, assets, basic_comp, role_comp, weapon_comp, echo_comp);
        respond(
            events,
            player_id,
            format!("switched Rover from {source_id} to {target_id} ({element_arg})"),
        );
        Ok(())
    }
}
