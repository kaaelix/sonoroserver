use std::cell::Cell;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::asset_indexes::{entity_world_pos, entity_world_rot};
use sonoro_gamesv_data::tables::entity_components::extra::ExtraEntry;
use sonoro_gamesv_data::tables::level_entity_config::LevelEntityConfig;
use sonoro_proto::pb::{TimeScheduleIdleNotify, TimeScheduleTriggerNotify};

use crate::commands::entity as entity_cmd;
use crate::helpers::condition::{EvalContext, eval_inline_condition_block};
use crate::helpers::quest_types::{QuestContext, json_bool, json_float, json_int, json_str};
use crate::scene::Scene;
use sonoro_proto::enums::EAttributeType;

thread_local! {
    static CONTROLLER_TICK_ACTIVE: Cell<bool> = const { Cell::new(false) };
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
}

fn merged_components(
    assets: &Assets,
    cfg: &LevelEntityConfig,
) -> sonoro_gamesv_data::tables::entity_components::components::Components {
    let mut merged = cfg.ComponentsData.clone();
    if let Some(tpl) = assets.find_template_config(&cfg.BlueprintType) {
        tpl.ComponentsData.merge_into(&mut merged);
    }
    merged
}

fn eval_context_for_entity<'a, S: crate::sink::PushSink + ?Sized>(
    qctx: &'a QuestContext<'_, S>,
    self_entity_config_id: i32,
) -> EvalContext<'a> {
    let mut eval = qctx.make_eval_context();
    eval.self_entity_config_id = self_entity_config_id;
    eval
}

fn json_i64_list(val: Option<&Value>) -> Vec<i64> {
    val.and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_i64).collect())
        .unwrap_or_default()
}

fn is_entity_destroyed_or_dead(
    qctx: &QuestContext<'_, impl crate::sink::PushSink + ?Sized>,
    cfg_id: i32,
) -> bool {
    if cfg_id <= 0 {
        return false;
    }
    if qctx.quest_comp.info.world.hidden_entities.contains(&cfg_id) {
        return true;
    }
    qctx.progress_comp
        .as_ref()
        .is_some_and(|p| p.info.get_key_int(&format!("entity_dead:{cfg_id}")) != 0)
}

fn clear_destroyed_entity<S: crate::sink::PushSink + ?Sized>(
    qctx: &mut QuestContext<'_, S>,
    cfg_id: i32,
) {
    qctx.quest_comp
        .info
        .world
        .hidden_entities
        .retain(|&id| id != cfg_id);
    if let Some(prog) = qctx.progress_comp.as_deref_mut() {
        prog.info.set_key_int(&format!("entity_dead:{cfg_id}"), 0);
    }
}

fn try_spawn_level_entity_near_player<S: crate::sink::PushSink + ?Sized>(
    qctx: &mut QuestContext<'_, S>,
    cfg_id: i64,
    loc_override: Option<[f32; 3]>,
) -> Option<i64> {
    let scene = qctx.scene.as_deref_mut()?;
    let cfg_i32 = i32::try_from(cfg_id).ok()?;
    if let Some(existing) = scene.find_entity_net_id_by_config_id(cfg_i32) {
        return Some(existing);
    }
    let progress_info = qctx
        .progress_comp
        .as_ref()
        .map(|p| p.info.clone())
        .unwrap_or_default();
    let handle = entity_cmd::spawn_with_location_and_progress(
        qctx.events,
        scene,
        qctx.fs,
        qctx.assets,
        qctx.sink,
        qctx.basic_comp,
        cfg_id,
        None,
        None,
        None,
        loc_override,
        false,
        &progress_info,
        false,
        false,
    )
    .ok()
    .flatten()?;
    Some(handle.net_id)
}

fn next_fixed_daily_due_ms(now_ms: i64, hours: i64, minutes: i64, seconds: i64) -> i64 {
    let now_s = (now_ms / 1000).max(0);
    let secs_in_day = 86_400i64;
    let target_tod = hours.clamp(0, 23) * 3600 + minutes.clamp(0, 59) * 60 + seconds.clamp(0, 59);
    let day_start = now_s - (now_s % secs_in_day);
    let mut candidate = day_start + target_tod;
    if candidate <= now_s {
        candidate += secs_in_day;
    }
    candidate * 1000
}

fn tick_refresh_groups<S: crate::sink::PushSink + ?Sized>(
    qctx: &mut QuestContext<'_, S>,
    controllers: &[std::sync::Arc<LevelEntityConfig>],
    now_ms: i64,
) -> bool {
    let mut changed = false;
    for ctrl in controllers {
        let components = merged_components(qctx.assets, ctrl);
        let Some(ref refresh) = components.RefreshGroupComponent else {
            continue;
        };
        if refresh.Disabled.unwrap_or(false) {
            continue;
        }
        let rule = refresh.RefreshRule.as_object();
        let rule_type = rule.and_then(|r| json_str(r.get("Type"))).unwrap_or("");
        if rule_type != "CD" && rule_type != "FixedDateTime" {
            continue;
        }
        let mut child_ids = json_i64_list(
            refresh
                .RefreshContent
                .as_object()
                .and_then(|o| o.get("EntityIds")),
        );
        for &eid in &refresh.EntityIds {
            if !child_ids.contains(&eid) {
                child_ids.push(eid);
            }
        }
        if child_ids.is_empty() {
            child_ids.push(ctrl.EntityId);
        }

        let any_destroyed = child_ids.iter().any(|&eid| {
            i32::try_from(eid).is_ok_and(|id| is_entity_destroyed_or_dead(qctx, id))
        });
        let due_key = format!("refresh:due:{}", ctrl.EntityId);
        let due_ms = qctx
            .progress_comp
            .as_ref()
            .map_or(0, |p| p.info.get_key_int(&due_key));

        if !any_destroyed {
            if due_ms != 0
                && let Some(prog) = qctx.progress_comp.as_deref_mut()
            {
                prog.info.set_key_int(&due_key, 0);
                changed = true;
            }
            continue;
        }

        if due_ms <= 0 {
            let next_due = match rule_type {
                "CD" => {
                    let cd_sec = rule.and_then(|r| json_int(r.get("Cd"))).unwrap_or(86_400).max(1);
                    now_ms.saturating_add(i64::from(cd_sec) * 1000)
                }
                "FixedDateTime" => {
                    let h = i64::from(rule.and_then(|r| json_int(r.get("Hours"))).unwrap_or(4));
                    let m = i64::from(rule.and_then(|r| json_int(r.get("Minutes"))).unwrap_or(0));
                    let s = i64::from(rule.and_then(|r| json_int(r.get("Seconds"))).unwrap_or(0));
                    next_fixed_daily_due_ms(now_ms, h, m, s)
                }
                _ => continue,
            };
            if let Some(prog) = qctx.progress_comp.as_deref_mut() {
                prog.info.set_key_int(&due_key, next_due);
                changed = true;
            }
            continue;
        }

        if now_ms < due_ms {
            continue;
        }

        if let Some(prog) = qctx.progress_comp.as_deref_mut() {
            prog.info.set_key_int(&due_key, 0);
            changed = true;
        }
        for &eid in &child_ids {
            if let Ok(cid) = i32::try_from(eid) {
                clear_destroyed_entity(qctx, cid);
                let _ = try_spawn_level_entity_near_player(qctx, eid, None);
            }
        }
    }
    changed
}

fn tick_entity_batch_refreshes<S: crate::sink::PushSink + ?Sized>(
    qctx: &mut QuestContext<'_, S>,
    controllers: &[std::sync::Arc<LevelEntityConfig>],
) -> bool {
    let mut changed = false;
    for ctrl in controllers {
        let components = merged_components(qctx.assets, ctrl);
        let Some(batch_comp) = components
            .extra
            .iter()
            .chain(components.template_extra.iter())
            .find_map(|e| match e {
                ExtraEntry::EntityBatchRefreshComponent(b) => Some(b),
                _ => None,
            })
        else {
            continue;
        };

        let Some(ref cfg_val) = batch_comp.EntityBatchRefresh else {
            continue;
        };
        let Some(cfg_obj) = cfg_val.as_object() else {
            continue;
        };
        let Some(batches) = cfg_obj.get("EntityBatches").and_then(Value::as_array) else {
            continue;
        };
        if batches.is_empty() {
            continue;
        }
        let restart = json_bool(cfg_obj.get("IsRestartAfterAllBatchesFinished")).unwrap_or(false);

        let eval = eval_context_for_entity(qctx, i32::try_from(ctrl.EntityId).unwrap_or(0));
        let end_hit = batch_comp
            .EndCondition
            .as_ref()
            .is_some_and(|cond| eval_inline_condition_block(&eval, cond));
        let active_key = format!("batch_refresh:active:{}", ctrl.EntityId);
        let idx_key = format!("batch_refresh:idx:{}", ctrl.EntityId);
        let done_key = format!("batch_refresh:done:{}", ctrl.EntityId);

        let done = qctx
            .progress_comp
            .as_ref()
            .is_some_and(|p| p.info.get_key_int(&done_key) != 0);
        if end_hit {
            if let Some(prog) = qctx.progress_comp.as_deref_mut()
                && prog.info.get_key_int(&active_key) != 0
            {
                prog.info.set_key_int(&active_key, 0);
                changed = true;
            }
            continue;
        }
        if done {
            continue;
        }

        let mut active = qctx
            .progress_comp
            .as_ref()
            .is_some_and(|p| p.info.get_key_int(&active_key) != 0);
        if !active {
            let start_ok = batch_comp
                .StartCondition
                .as_ref()
                .map_or(true, |cond| eval_inline_condition_block(&eval, cond));
            if !start_ok {
                continue;
            }
            active = true;
            if let Some(prog) = qctx.progress_comp.as_deref_mut() {
                prog.info.set_key_int(&active_key, 1);
                prog.info.set_key_int(&idx_key, 0);
                changed = true;
            }
        }
        if !active {
            continue;
        }

        let cur_idx = qctx
            .progress_comp
            .as_ref()
            .map_or(0, |p| p.info.get_key_int(&idx_key))
            .max(0) as usize;
        if cur_idx >= batches.len() {
            continue;
        }

        let batch_eids = batches[cur_idx]
            .as_object()
            .map(|o| json_i64_list(o.get("EntityIds")))
            .unwrap_or_default();
        let spawned_key = format!("batch_refresh:spawned:{}:{}", ctrl.EntityId, cur_idx);
        let was_spawned = qctx
            .progress_comp
            .as_ref()
            .is_some_and(|p| p.info.get_key_int(&spawned_key) != 0);

        if !was_spawned {
            for &eid in &batch_eids {
                if let Ok(cid) = i32::try_from(eid) {
                    clear_destroyed_entity(qctx, cid);
                    let _ = try_spawn_level_entity_near_player(qctx, eid, None);
                }
            }
            if let Some(prog) = qctx.progress_comp.as_deref_mut() {
                prog.info.set_key_int(&spawned_key, 1);
                changed = true;
            }
            continue;
        }

        let all_finished = !batch_eids.is_empty()
            && batch_eids.iter().all(|&eid| {
                i32::try_from(eid).is_ok_and(|cid| {
                    is_entity_destroyed_or_dead(qctx, cid)
                        || qctx
                            .scene
                            .as_deref()
                            .is_none_or(|s| s.find_entity_net_id_by_config_id(cid).is_none())
                })
            });
        if all_finished {
            let next_idx = cur_idx + 1;
            if next_idx < batches.len() {
                if let Some(prog) = qctx.progress_comp.as_deref_mut() {
                    prog.info.set_key_int(&idx_key, next_idx as i64);
                    prog.info
                        .set_key_int(&format!("batch_refresh:spawned:{}:{}", ctrl.EntityId, next_idx), 0);
                    changed = true;
                }
            } else if restart {
                if let Some(prog) = qctx.progress_comp.as_deref_mut() {
                    prog.info.set_key_int(&idx_key, 0);
                    for i in 0..batches.len() {
                        prog.info
                            .set_key_int(&format!("batch_refresh:spawned:{}:{}", ctrl.EntityId, i), 0);
                    }
                    changed = true;
                }
            } else if let Some(prog) = qctx.progress_comp.as_deref_mut() {
                prog.info.set_key_int(&active_key, 0);
                prog.info.set_key_int(&done_key, 1);
                changed = true;
            }
        }
    }
    changed
}

fn collect_spawn_points(
    assets: &Assets,
    controllers: &[std::sync::Arc<LevelEntityConfig>],
) -> Vec<([f32; 3], i64)> {
    let mut pts = Vec::new();
    for cfg in controllers {
        let components = merged_components(assets, cfg);
        for e in components.extra.iter().chain(components.template_extra.iter()) {
            if let ExtraEntry::DynamicSpawnMonsterPointComponent(pt) = e
                && let Some(pos) = entity_world_pos(cfg)
            {
                pts.push((pos, pt.TagId.unwrap_or(0)));
            }
        }
    }
    pts
}

fn pick_random_summon_ids(assets: &Assets, pool_id: i32, count: usize, seed: u64) -> Vec<i64> {
    let Some(entry) = assets
        .random_summons()
        .iter()
        .find(|v| json_int(v.get("Id")) == Some(pool_id))
    else {
        return Vec::new();
    };
    let Some(random_arr) = entry.get("Random").and_then(Value::as_array) else {
        return Vec::new();
    };
    let mut weighted: Vec<(i64, i64)> = Vec::new();
    let mut total_weight: i64 = 0;
    for r in random_arr {
        if let Some(ints) = r.get("ArrayInt").and_then(Value::as_array)
            && !ints.is_empty()
        {
            let target_id = ints[0].as_i64().unwrap_or(0);
            let weight = ints.get(1).and_then(Value::as_i64).unwrap_or(1).max(1);
            if target_id > 0 {
                total_weight += weight;
                weighted.push((target_id, weight));
            }
        }
    }
    if weighted.is_empty() || total_weight <= 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let roll = ((seed.wrapping_add((i as u64).wrapping_mul(2_654_435_761))) % (total_weight as u64))
            as i64;
        let mut acc = 0i64;
        let mut chosen = weighted[0].0;
        for &(id, w) in &weighted {
            acc += w;
            if roll < acc {
                chosen = id;
                break;
            }
        }
        out.push(chosen);
    }
    out
}

fn spawn_monster_target_for_dynamic<S: crate::sink::PushSink + ?Sized>(
    qctx: &mut QuestContext<'_, S>,
    map_id: i32,
    target_id: i64,
    loc: [f32; 3],
    rot: [f32; 3],
    seq_seed: i64,
) -> Option<i64> {
    if qctx.assets.find_level_entity(map_id, target_id).is_some() {
        return try_spawn_level_entity_near_player(qctx, target_id, Some(loc));
    }
    let tpl = i32::try_from(target_id)
        .ok()
        .and_then(|id| qctx.assets.find_template_config_by_id(id))
        .or_else(|| qctx.assets.find_template_config(&target_id.to_string()));
    if let Some(tpl) = tpl {
        let scene = qctx.scene.as_deref_mut()?;
        return entity_cmd::spawn_synthetic_template_entity(
            scene,
            qctx.fs,
            qctx.assets,
            qctx.sink,
            qctx.basic_comp,
            &tpl.BlueprintType,
            seq_seed,
            loc,
            rot,
        );
    }
    None
}

fn is_net_entity_alive(scene: Option<&Scene>, net_id: i64) -> bool {
    if net_id <= 0 {
        return false;
    }
    let Some(scene) = scene else {
        return false;
    };
    let Some(&idx) = scene.net_id_map.get(&net_id) else {
        return false;
    };
    scene.entities[idx]
        .attribute
        .as_ref()
        .is_none_or(|attr| attr.get_current(EAttributeType::Life) > 0)
}

fn tick_dynamic_spawn_monsters<S: crate::sink::PushSink + ?Sized>(
    qctx: &mut QuestContext<'_, S>,
    controllers: &[std::sync::Arc<LevelEntityConfig>],
    now_ms: i64,
) -> bool {
    let mut changed = false;
    let map_id = qctx.scene.as_deref().map_or(0, |s| s.instance_id);
    if map_id <= 0 {
        return false;
    }
    let spawn_points = collect_spawn_points(qctx.assets, controllers);

    for ctrl in controllers {
        let ctrl_id = ctrl.EntityId;
        if let Ok(cid) = i32::try_from(ctrl_id)
            && is_entity_destroyed_or_dead(qctx, cid)
        {
            continue;
        }
        let components = merged_components(qctx.assets, ctrl);
        let Some(dyn_comp) = components
            .extra
            .iter()
            .chain(components.template_extra.iter())
            .find_map(|e| match e {
                ExtraEntry::DynamicSpawnMonsterComponent(d) => Some(d),
                _ => None,
            })
        else {
            continue;
        };
        if dyn_comp.Disabled.unwrap_or(false) {
            continue;
        }

        let ctrl_pos = entity_world_pos(ctrl).unwrap_or([0.0; 3]);
        let ctrl_rot = entity_world_rot(ctrl);

        for (g_idx, grp_val) in dyn_comp.Group.iter().enumerate() {
            let Some(grp) = grp_val.as_object() else {
                continue;
            };
            let g_active_key = format!("dyn_spawn:{ctrl_id}:g:{g_idx}:active");
            let g_start_key = format!("dyn_spawn:{ctrl_id}:g:{g_idx}:start");
            let was_active = qctx
                .progress_comp
                .as_ref()
                .is_some_and(|p| p.info.get_key_int(&g_active_key) != 0);

            let end_hit = {
                let eval = eval_context_for_entity(qctx, i32::try_from(ctrl_id).unwrap_or(0));
                grp.get("EndCondition")
                    .is_some_and(|c| eval_inline_condition_block(&eval, c))
            };
            if end_hit {
                if was_active
                    && let Some(prog) = qctx.progress_comp.as_deref_mut()
                {
                    prog.info.set_key_int(&g_active_key, 0);
                    changed = true;
                }
                continue;
            }
            let cond_ok = {
                let eval = eval_context_for_entity(qctx, i32::try_from(ctrl_id).unwrap_or(0));
                grp.get("Condition")
                    .map_or(true, |c| eval_inline_condition_block(&eval, c))
            };
            if !cond_ok {
                continue;
            }
            if !was_active {
                if let Some(prog) = qctx.progress_comp.as_deref_mut() {
                    prog.info.set_key_int(&g_active_key, 1);
                    prog.info.set_key_int(&g_start_key, now_ms);
                    changed = true;
                }
            }

            let Some(batches) = grp.get("BatchConfigs").and_then(Value::as_array) else {
                continue;
            };
            let loop_group = json_bool(grp.get("Loop")).unwrap_or(false);
            let group_start_ms = qctx
                .progress_comp
                .as_ref()
                .map_or(now_ms, |p| p.info.get_key_int(&g_start_key));

            let mut all_batches_done = !batches.is_empty();
            for batch_val in batches {
                let Some(batch) = batch_val.as_object() else {
                    continue;
                };
                let b_id = json_int(batch.get("Id")).unwrap_or(0);
                let b_state_key = format!("dyn_spawn:{ctrl_id}:g:{g_idx}:b:{b_id}:st");
                let b_state = qctx
                    .progress_comp
                    .as_ref()
                    .map_or(0, |p| p.info.get_key_int(&b_state_key));
                // 0 = pending, 1 = active/spawned, 2 = completed
                if b_state == 2 {
                    continue;
                }
                all_batches_done = false;

                let pre_dep_ids: Vec<i64> = batch
                    .get("PreCondition")
                    .and_then(Value::as_object)
                    .and_then(|pc| pc.get("Ids"))
                    .and_then(Value::as_array)
                    .map(|ids| ids.iter().filter_map(Value::as_i64).collect())
                    .unwrap_or_default();
                let deps_done = pre_dep_ids.iter().all(|&dep_id| {
                    qctx.progress_comp.as_ref().is_some_and(|p| {
                        p.info.get_key_int(&format!(
                            "dyn_spawn:{ctrl_id}:g:{g_idx}:b:{dep_id}:st"
                        )) == 2
                    })
                });
                if !deps_done {
                    continue;
                }

                let delay_ms = (json_float(batch.get("DelayTime")).unwrap_or(0.0) * 1000.0) as i64;
                if b_state == 0 && now_ms < group_start_ms.saturating_add(delay_ms) {
                    continue;
                }

                let b_spawned_at_key = format!("dyn_spawn:{ctrl_id}:g:{g_idx}:b:{b_id}:at");
                let b_killed_key = format!("dyn_spawn:{ctrl_id}:g:{g_idx}:b:{b_id}:killed");
                let b_slot_cnt_key = format!("dyn_spawn:{ctrl_id}:g:{g_idx}:b:{b_id}:cnt");

                if b_state == 0 {
                    let radius = batch
                        .get("PointChooseType")
                        .and_then(Value::as_object)
                        .and_then(|p| p.get("RangeConstraint"))
                        .and_then(Value::as_object)
                        .and_then(|r| json_float(r.get("Radius")))
                        .unwrap_or(2000.0);
                    let r2 = radius * radius;
                    let candidate_locs: Vec<[f32; 3]> = spawn_points
                        .iter()
                        .filter_map(|&(pt, _)| {
                            let dx = pt[0] - ctrl_pos[0];
                            let dy = pt[1] - ctrl_pos[1];
                            let dz = pt[2] - ctrl_pos[2];
                            (dx * dx + dy * dy + dz * dz <= r2).then_some(pt)
                        })
                        .collect();

                    let mut target_spawns: Vec<i64> = Vec::new();
                    if let Some(mc) = batch.get("MonsterChooseType").and_then(Value::as_object) {
                        if let Some(pools) = mc.get("RandomPoolConfigs").and_then(Value::as_array) {
                            for p_val in pools {
                                let Some(p_obj) = p_val.as_object() else {
                                    continue;
                                };
                                let pid = json_int(p_obj.get("Id")).unwrap_or(0);
                                let cnt = json_int(p_obj.get("Count")).unwrap_or(1).max(1) as usize;
                                let chosen = pick_random_summon_ids(
                                    qctx.assets,
                                    pid,
                                    cnt,
                                    (now_ms as u64) ^ (ctrl_id as u64),
                                );
                                if !chosen.is_empty() {
                                    target_spawns.extend(chosen);
                                } else if pid > 0 {
                                    for _ in 0..cnt {
                                        target_spawns.push(i64::from(pid));
                                    }
                                }
                            }
                        }
                        if let Some(eids) = mc.get("EntityIds").and_then(Value::as_array) {
                            target_spawns.extend(eids.iter().filter_map(Value::as_i64));
                        }
                    }

                    let mut spawned_net_ids = Vec::new();
                    for (idx, tid) in target_spawns.iter().copied().enumerate() {
                        let loc = candidate_locs
                            .get(idx % candidate_locs.len().max(1))
                            .copied()
                            .unwrap_or(ctrl_pos);
                        let synth_seq = (ctrl_id.abs() % 10_000_000) * 100
                            + (g_idx as i64) * 10
                            + (idx as i64);
                        if let Some(nid) = spawn_monster_target_for_dynamic(
                            qctx,
                            map_id,
                            tid,
                            loc,
                            ctrl_rot,
                            synth_seq,
                        ) {
                            spawned_net_ids.push(nid);
                        }
                    }
                    if let Some(prog) = qctx.progress_comp.as_deref_mut() {
                        prog.info.set_key_int(&b_state_key, 1);
                        prog.info.set_key_int(&b_spawned_at_key, now_ms);
                        prog.info.set_key_int(&b_killed_key, 0);
                        prog.info
                            .set_key_int(&b_slot_cnt_key, spawned_net_ids.len() as i64);
                        for (s_idx, nid) in spawned_net_ids.iter().enumerate() {
                            prog.info.set_key_int(
                                &format!("dyn_spawn:{ctrl_id}:g:{g_idx}:b:{b_id}:nid:{s_idx}"),
                                *nid,
                            );
                        }
                        changed = true;
                    }
                    continue;
                }

                // Track kills from active slots
                let slot_cnt = qctx
                    .progress_comp
                    .as_ref()
                    .map_or(0, |p| p.info.get_key_int(&b_slot_cnt_key))
                    .max(0) as usize;
                let slot_nids: Vec<(usize, String, i64)> = (0..slot_cnt)
                    .map(|s_idx| {
                        let nkey = format!("dyn_spawn:{ctrl_id}:g:{g_idx}:b:{b_id}:nid:{s_idx}");
                        let nid = qctx
                            .progress_comp
                            .as_ref()
                            .map_or(0, |p| p.info.get_key_int(&nkey));
                        (s_idx, nkey, nid)
                    })
                    .collect();
                let mut alive = 0usize;
                let mut dead_keys = Vec::new();
                for &(_, ref nkey, nid) in &slot_nids {
                    if nid <= 0 {
                        continue;
                    }
                    if is_net_entity_alive(qctx.scene.as_deref(), nid) {
                        alive += 1;
                    } else {
                        dead_keys.push(nkey.clone());
                    }
                }
                let newly_dead = dead_keys.len() as i64;
                if newly_dead > 0
                    && let Some(prog) = qctx.progress_comp.as_deref_mut()
                {
                    for nkey in dead_keys {
                        prog.info.set_key_int(&nkey, 0);
                    }
                    let total_killed = prog.info.get_key_int(&b_killed_key) + newly_dead;
                    prog.info.set_key_int(&b_killed_key, total_killed);
                    changed = true;
                }

                let cc = batch.get("CompleteCondition").and_then(Value::as_object);
                let cc_type = cc.and_then(|c| json_int(c.get("Type"))).unwrap_or(0);
                let killed = qctx
                    .progress_comp
                    .as_ref()
                    .map_or(0, |p| p.info.get_key_int(&b_killed_key));
                let spawned_at = qctx
                    .progress_comp
                    .as_ref()
                    .map_or(now_ms, |p| p.info.get_key_int(&b_spawned_at_key));
                let batch_done = match cc_type {
                    1 => {
                        let dur_ms = (cc.and_then(|c| json_float(c.get("Duration"))).unwrap_or(0.0)
                            * 1000.0) as i64;
                        now_ms >= spawned_at.saturating_add(dur_ms)
                    }
                    2 => {
                        let qty = i64::from(cc.and_then(|c| json_int(c.get("Quantity"))).unwrap_or(1));
                        killed >= qty || (slot_cnt > 0 && alive == 0)
                    }
                    _ => slot_cnt == 0 || alive == 0,
                };
                if batch_done
                    && let Some(prog) = qctx.progress_comp.as_deref_mut()
                {
                    prog.info.set_key_int(&b_state_key, 2);
                    changed = true;
                }
            }

            if all_batches_done && loop_group {
                let batch_ids: Vec<i32> = batches
                    .iter()
                    .filter_map(|bv| bv.as_object().and_then(|b| json_int(b.get("Id"))))
                    .collect();
                if let Some(prog) = qctx.progress_comp.as_deref_mut() {
                    prog.info.set_key_int(&g_start_key, now_ms);
                    for b_id in batch_ids {
                        prog.info.set_key_int(
                            &format!("dyn_spawn:{ctrl_id}:g:{g_idx}:b:{b_id}:st"),
                            0,
                        );
                    }
                    changed = true;
                }
            }
        }
    }
    changed
}

fn parse_schedule_hm_minutes(raw: &str) -> Option<i32> {
    let mut parts = raw.trim().split(':');
    let h: i32 = parts.next()?.parse().ok()?;
    let m: i32 = parts.next()?.parse().ok()?;
    Some(h.rem_euclid(24) * 60 + m.clamp(0, 59))
}

pub fn tick_time_schedules<S: crate::sink::PushSink + ?Sized>(
    qctx: &mut QuestContext<'_, S>,
    hour: i32,
    minute: i32,
) -> io::Result<()> {
    let Some(map_id) = qctx.scene.as_deref().map(|s| s.instance_id) else {
        return Ok(());
    };
    let controllers = qctx.assets.world_controller_entities(map_id);
    let cur_mins = hour.rem_euclid(24) * 60 + minute.clamp(0, 59);
    let mut actions: Vec<(String, i32, Vec<i64>)> = Vec::new();
    {
        let schedules = qctx.assets.time_schedules();
        for ctrl in controllers.iter() {
            let components = merged_components(qctx.assets, ctrl);
            let Some(ts_comp) = components
                .extra
                .iter()
                .chain(components.template_extra.iter())
                .find_map(|e| match e {
                    ExtraEntry::TimeScheduleComponent(t) => Some(t),
                    _ => None,
                })
            else {
                continue;
            };
            let eval = eval_context_for_entity(qctx, i32::try_from(ctrl.EntityId).unwrap_or(0));

            for (idx, cfg_val) in ts_comp.TimeScheduleConfigs.iter().enumerate() {
                let Some(cfg) = cfg_val.as_object() else {
                    continue;
                };
                let tag = json_str(cfg.get("TimeScheduleTag")).unwrap_or("");
                if tag.is_empty() {
                    continue;
                }
                let target_cfgs = json_i64_list(cfg.get("TargetEntityIds"));
                if target_cfgs.is_empty() {
                    continue;
                }
                let net_ids: Vec<i64> = qctx
                    .scene
                    .as_deref()
                    .map(|scene| {
                        target_cfgs
                            .iter()
                            .filter_map(|&eid| {
                                i32::try_from(eid)
                                    .ok()
                                    .and_then(|c| scene.find_entity_net_id_by_config_id(c))
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                if net_ids.is_empty() {
                    continue;
                }

                let cond_ok = cfg
                    .get("Conditions")
                    .map_or(true, |c| eval_inline_condition_block(&eval, c));

                let mut matching: Vec<(i32, i32)> = schedules
                    .iter()
                    .filter(|s| s.TimeScheduleTag == tag)
                    .filter_map(|s| parse_schedule_hm_minutes(&s.StartTime).map(|m| (m, s.Id)))
                    .collect();
                matching.sort_by_key(|&(m, id)| (m, id));

                let active_sched_id = if cond_ok && !matching.is_empty() {
                    matching
                        .iter()
                        .rev()
                        .find(|&&(m, _)| m <= cur_mins)
                        .or_else(|| matching.last())
                        .map(|&(_, id)| id)
                        .unwrap_or(0)
                } else {
                    0
                };

                let state_key = format!("time_sched:{}:{idx}", ctrl.EntityId);
                let prev_id = qctx
                    .progress_comp
                    .as_ref()
                    .map_or(-1, |p| p.info.get_key_int(&state_key));
                if prev_id != i64::from(active_sched_id) {
                    actions.push((state_key, active_sched_id, net_ids));
                }
            }
        }
    }

    let mut changed = false;
    for (state_key, active_sched_id, net_ids) in actions {
        if let Some(prog) = qctx.progress_comp.as_deref_mut() {
            prog.info.set_key_int(&state_key, i64::from(active_sched_id));
            changed = true;
        }
        if active_sched_id > 0 {
            qctx.sink.push(
                TimeScheduleTriggerNotify::MSG_ID,
                &TimeScheduleTriggerNotify {
                    TimeScheduleId: active_sched_id,
                    EntityIds: net_ids,
                },
            );
        } else {
            qctx.sink.push(
                TimeScheduleIdleNotify::MSG_ID,
                &TimeScheduleIdleNotify { EntityIds: net_ids },
            );
        }
    }

    if changed && let Some(prog) = qctx.progress_comp.as_deref_mut() {
        prog.save(qctx.fs)?;
    }
    Ok(())
}

pub fn tick_world_controllers<S: crate::sink::PushSink + ?Sized>(
    qctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let entered = CONTROLLER_TICK_ACTIVE.with(|flag| {
        if flag.get() {
            false
        } else {
            flag.set(true);
            true
        }
    });
    if !entered {
        return Ok(());
    }

    struct ResetGuard;
    impl Drop for ResetGuard {
        fn drop(&mut self) {
            CONTROLLER_TICK_ACTIVE.with(|f| f.set(false));
        }
    }
    let _guard = ResetGuard;

    let Some((map_id, hour, minute)) = qctx
        .scene
        .as_deref()
        .map(|s| (s.instance_id, s.instance.map_time.hour, s.instance.map_time.minute))
    else {
        return Ok(());
    };
    let controllers = qctx.assets.world_controller_entities(map_id);
    if controllers.is_empty() {
        return Ok(());
    }

    let now_ms = now_millis();
    let mut changed = false;
    changed |= tick_refresh_groups(qctx, &controllers, now_ms);
    changed |= tick_entity_batch_refreshes(qctx, &controllers);
    changed |= tick_dynamic_spawn_monsters(qctx, &controllers, now_ms);
    tick_time_schedules(qctx, hour, minute)?;

    if changed && let Some(prog) = qctx.progress_comp.as_deref_mut() {
        prog.save(qctx.fs)?;
    }
    Ok(())
}
