use std::cell::Cell;
use std::collections::HashMap;
use std::io;

use serde_json::{Map, Value};
use sonoro_proto::pb::{AlertArea, AlertAreaNotify};

use crate::helpers::quest_types::{
    ActionCallbacks, QuestContext, json_bool, json_i64, json_int, json_str,
};
use crate::scene::now_millis;
use crate::sink::PushSink;

thread_local! {
    static ALERT_NOTIFY_DEPTH: Cell<u8> = const { Cell::new(0) };
}

#[derive(Debug, Clone, Default)]
pub struct AlertAreaRuntime {
    pub area_id: i32,
    pub alert_value: f64,
    pub change_speed: f64,
    pub enabled: bool,
    pub enable_ui: bool,
    pub visible_ui: bool,
    pub auto_disable_dungeon_id: i32,
    pub auto_disable_quest_id: i32,
    pub last_tick_ms: i64,
}

impl AlertAreaRuntime {
    pub fn to_pb(&self) -> AlertArea {
        AlertArea {
            AreaId: self.area_id,
            AlertValue: self.alert_value as f32,
            ChangeSpeed: self.change_speed as f32,
            EnableUi: self.enable_ui,
            VisibleUi: self.visible_ui,
        }
    }
}

fn json_f64_any(v_opt: Option<&Value>) -> Option<f64> {
    let v = v_opt?;
    match v {
        Value::Number(n) => n.as_f64().filter(|f| f.is_finite()),
        Value::String(s) => s.parse::<f64>().ok().filter(|f| f.is_finite()),
        Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        _ => None,
    }
}

fn resolve_alert_value_mod(cur: f64, min_val: f64, max_val: f64, cfg: &Map<String, Value>) -> f64 {
    match json_int(cfg.get("Type")).unwrap_or(0) {
        0 => cur + json_f64_any(cfg.get("DeltaValue")).unwrap_or(0.0),
        1 => max_val,
        2 => min_val,
        3 => json_f64_any(cfg.get("CustomValue")).unwrap_or(cur),
        _ => cur,
    }
}

fn sync_area_to_progress<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    area: &AlertAreaRuntime,
) -> io::Result<()> {
    let Some(progress) = ctx.progress_comp.as_deref_mut() else {
        return Ok(());
    };
    let val_key = format!("alert:area:{}", area.area_id);
    let en_key = format!("alert_enable:{}", area.area_id);
    let val_i64 = area.alert_value.round() as i64;
    let mut dirty = false;
    if progress.info.dynamic.ints.iter().find(|kv| kv.key == val_key).map(|kv| kv.val) != Some(val_i64) {
        progress.info.set_key_int(&val_key, val_i64);
        dirty = true;
    }
    let en_i64 = i64::from(area.enabled);
    if progress.info.dynamic.ints.iter().find(|kv| kv.key == en_key).map(|kv| kv.val) != Some(en_i64) {
        progress.info.set_key_int(&en_key, en_i64);
        dirty = true;
    }
    if dirty {
        progress.save(ctx.fs)?;
    }
    Ok(())
}

fn advance_area_ticks(area: &mut AlertAreaRuntime, min_v: f64, max_v: f64, now_ms: i64) -> bool {
    if !area.enabled || area.change_speed == 0.0 {
        area.last_tick_ms = now_ms;
        return false;
    }
    if area.last_tick_ms <= 0 {
        area.last_tick_ms = now_ms;
        return false;
    }
    let elapsed_ms = now_ms.saturating_sub(area.last_tick_ms);
    let secs = elapsed_ms / 1000;
    if secs <= 0 {
        return false;
    }
    area.last_tick_ms += secs * 1000;
    let next = (area.alert_value + area.change_speed * (secs as f64)).clamp(min_v, max_v);
    let changed = (next - area.alert_value).abs() > 1e-6;
    area.alert_value = next;
    changed
}

fn push_area_notify<S: PushSink + ?Sized>(sink: &mut S, area: Option<AlertArea>, remove_id: Option<i32>) {
    let mut areas = Vec::new();
    let mut removes = Vec::new();
    if let Some(a) = area {
        areas.push(a);
    }
    if let Some(id) = remove_id {
        removes.push(id);
    }
    if !areas.is_empty() || !removes.is_empty() {
        sink.push(
            AlertAreaNotify::MSG_ID,
            &AlertAreaNotify {
                AlertAreas: areas,
                RemoveAreaIds: removes,
            },
        );
    }
}

pub fn apply_exec_alert_system_action<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    callbacks: &mut ActionCallbacks<'_, S>,
    params: Option<&Map<String, Value>>,
) -> io::Result<()> {
    let Some(opt) = params.and_then(|p| p.get("Option")).and_then(|v| v.as_object()) else {
        return Ok(());
    };
    let area_id = json_int(opt.get("AreaId")).unwrap_or(0);
    if area_id <= 0 {
        return Ok(());
    }
    let op_type = json_str(opt.get("Type")).unwrap_or("");
    let (min_v, max_v) = ctx.assets.alert_area_bounds(area_id);
    let now_ms = now_millis();

    let stored_int = |key: &str| -> Option<i64> {
        ctx.progress_comp
            .as_deref()
            .and_then(|p| p.info.dynamic.ints.iter().find(|kv| kv.key == key))
            .map(|kv| kv.val)
    };
    let mut snap = if let Some(scene) = ctx.scene.as_deref() {
        scene.alert_areas.get(&area_id).cloned().unwrap_or(AlertAreaRuntime {
            area_id,
            alert_value: stored_int(&format!("alert:area:{area_id}"))
                .map_or(min_v, |v| (v as f64).clamp(min_v, max_v)),
            enabled: stored_int(&format!("alert_enable:{area_id}"))
                .is_some_and(|v| v != 0),
            last_tick_ms: now_ms,
            ..Default::default()
        })
    } else {
        AlertAreaRuntime {
            area_id,
            alert_value: stored_int(&format!("alert:area:{area_id}"))
                .map_or(min_v, |v| (v as f64).clamp(min_v, max_v)),
            enabled: stored_int(&format!("alert_enable:{area_id}"))
                .is_some_and(|v| v != 0),
            last_tick_ms: now_ms,
            ..Default::default()
        }
    };

    let mut notify_area = None;
    let mut remove_id = None;

    match op_type {
        "EnableAlertArea" => {
            let is_enable = json_bool(opt.get("IsEnable")).unwrap_or(true);
            if let Some(Value::Object(ad)) = opt.get("AutoDisableCondition") {
                match json_int(ad.get("Type")).unwrap_or(-1) {
                    0 => {
                        snap.auto_disable_dungeon_id =
                            json_int(ad.get("RelatedDungeonId")).unwrap_or(0);
                        snap.auto_disable_quest_id = 0;
                    }
                    1 => {
                        snap.auto_disable_quest_id =
                            json_int(ad.get("RelatedQuestId")).unwrap_or(0);
                        snap.auto_disable_dungeon_id = 0;
                    }
                    _ => {}
                }
            }
            if is_enable {
                snap.enabled = true;
                snap.last_tick_ms = now_ms;
                notify_area = Some(snap.to_pb());
            } else {
                snap.enabled = false;
                snap.change_speed = 0.0;
                snap.auto_disable_dungeon_id = 0;
                snap.auto_disable_quest_id = 0;
                remove_id = Some(area_id);
            }
        }
        "EnableAlertUi" => {
            snap.enable_ui = json_bool(opt.get("IsEnable")).unwrap_or(true);
            notify_area = Some(snap.to_pb());
        }
        "SetAlertUiVisible" => {
            snap.visible_ui = json_bool(opt.get("IsVisible")).unwrap_or(true);
            notify_area = Some(snap.to_pb());
        }
        "ModifyAlertValue" => {
            advance_area_ticks(&mut snap, min_v, max_v, now_ms);
            if let Some(Value::Object(set_type)) = opt.get("SetType") {
                snap.alert_value =
                    resolve_alert_value_mod(snap.alert_value, min_v, max_v, set_type)
                        .clamp(min_v, max_v);
            }
            if let Some(Value::Object(speed)) = opt.get("ChangeSpeed") {
                snap.change_speed =
                    resolve_alert_value_mod(snap.change_speed, min_v, max_v, speed);
            }
            snap.last_tick_ms = now_ms;
            notify_area = Some(snap.to_pb());
        }
        _ => return Ok(()),
    }

    if let Some(scene) = ctx.scene.as_deref_mut() {
        scene.alert_areas.insert(area_id, snap.clone());
    }
    sync_area_to_progress(ctx, &snap)?;
    push_area_notify(ctx.sink, notify_area, remove_id);

    if let Some(cb) = callbacks.on_alert_changed {
        let depth = ALERT_NOTIFY_DEPTH.with(|d| d.get());
        if depth < 4 {
            ALERT_NOTIFY_DEPTH.with(|d| d.set(depth + 1));
            let res = cb(ctx);
            ALERT_NOTIFY_DEPTH.with(|d| d.set(depth));
            res?;
        }
    }
    Ok(())
}

pub fn tick_alert_areas<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    now_ms: i64,
) -> io::Result<bool> {
    let area_ids: Vec<i32> = ctx
        .scene
        .as_deref()
        .map(|s| {
            s.alert_areas
                .iter()
                .filter(|(_, a)| a.enabled && a.change_speed != 0.0)
                .map(|(&id, _)| id)
                .collect()
        })
        .unwrap_or_default();
    if area_ids.is_empty() {
        return Ok(false);
    }

    let mut changed_snaps = Vec::new();
    for area_id in area_ids {
        let (min_v, max_v) = ctx.assets.alert_area_bounds(area_id);
        let Some(scene) = ctx.scene.as_deref_mut() else {
            break;
        };
        if let Some(area) = scene.alert_areas.get_mut(&area_id)
            && advance_area_ticks(area, min_v, max_v, now_ms)
        {
            changed_snaps.push(area.clone());
        }
    }
    if changed_snaps.is_empty() {
        return Ok(false);
    }
    for snap in &changed_snaps {
        sync_area_to_progress(ctx, snap)?;
    }
    Ok(true)
}

pub fn next_alert_wake_delay_ms(
    alert_areas: &HashMap<i32, AlertAreaRuntime>,
    now_ms: i64,
) -> Option<i64> {
    alert_areas
        .values()
        .filter(|a| a.enabled && a.change_speed != 0.0 && a.last_tick_ms > 0)
        .map(|a| (a.last_tick_ms.saturating_add(1000) - now_ms).max(0))
        .min()
}

fn disable_matching_areas<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    pred: impl Fn(&AlertAreaRuntime) -> bool,
) -> io::Result<()> {
    let matching: Vec<i32> = ctx
        .scene
        .as_deref()
        .map(|s| {
            s.alert_areas
                .iter()
                .filter(|(_, a)| a.enabled && pred(a))
                .map(|(&id, _)| id)
                .collect()
        })
        .unwrap_or_default();
    if matching.is_empty() {
        return Ok(());
    }
    let mut disabled = Vec::new();
    let mut removed_ids = Vec::new();
    if let Some(scene) = ctx.scene.as_deref_mut() {
        for id in matching {
            if let Some(a) = scene.alert_areas.get_mut(&id) {
                a.enabled = false;
                a.change_speed = 0.0;
                a.auto_disable_dungeon_id = 0;
                a.auto_disable_quest_id = 0;
                disabled.push(a.clone());
                removed_ids.push(id);
            }
        }
    }
    for snap in &disabled {
        sync_area_to_progress(ctx, snap)?;
    }
    if !removed_ids.is_empty() {
        ctx.sink.push(
            AlertAreaNotify::MSG_ID,
            &AlertAreaNotify {
                AlertAreas: Vec::new(),
                RemoveAreaIds: removed_ids,
            },
        );
    }
    Ok(())
}

pub fn auto_disable_for_quest<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    quest_id: i32,
) -> io::Result<()> {
    if quest_id <= 0 {
        return Ok(());
    }
    disable_matching_areas(ctx, |a| a.auto_disable_quest_id == quest_id)
}

pub fn auto_disable_for_dungeon<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    dungeon_id: i32,
) -> io::Result<()> {
    if dungeon_id <= 0 {
        return Ok(());
    }
    disable_matching_areas(ctx, |a| a.auto_disable_dungeon_id == dungeon_id)
}

pub fn sync_alert_areas_on_join<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) {
    let Some(scene) = ctx.scene.as_deref() else {
        return;
    };
    let areas: Vec<AlertArea> = scene
        .alert_areas
        .values()
        .filter(|a| a.enabled || a.enable_ui)
        .map(AlertAreaRuntime::to_pb)
        .collect();
    if !areas.is_empty() {
        ctx.sink.push(
            AlertAreaNotify::MSG_ID,
            &AlertAreaNotify {
                AlertAreas: areas,
                RemoveAreaIds: Vec::new(),
            },
        );
    }
}

pub fn compare_alert_value(
    assets: Option<&sonoro_gamesv_data::Assets>,
    progress: Option<&crate::component::player::PlayerProgressComponent>,
    scene: Option<&crate::scene::Scene>,
    area_id: i32,
    compare_type: &str,
    compare_obj: Option<&Map<String, Value>>,
) -> bool {
    let (min_v, max_v) = assets.map_or((0.0, 100.0), |a| a.alert_area_bounds(area_id));
    let cur = scene
        .and_then(|s| s.alert_areas.get(&area_id))
        .map(|a| a.alert_value)
        .or_else(|| {
            let key = format!("alert:area:{area_id}");
            progress
                .and_then(|p| p.info.dynamic.ints.iter().find(|kv| kv.key == key))
                .map(|kv| kv.val as f64)
        })
        .unwrap_or(min_v);

    let rhs = match compare_obj {
        Some(obj) => match json_int(obj.get("Type")).unwrap_or(0) {
            1 => max_v,
            2 => json_f64_any(obj.get("CompareValue")).unwrap_or(0.0),
            _ => min_v,
        },
        None => min_v,
    };

    let eq = (cur - rhs).abs() <= 1e-4;
    match compare_type {
        "Eq" | "Equal" => eq,
        "Ne" | "NotEqual" => !eq,
        "Gt" | "Greater" => cur > rhs + 1e-4,
        "Ge" | "GreaterOrEqual" => cur >= rhs - 1e-4,
        "Lt" | "Less" => cur < rhs - 1e-4,
        "Le" | "LessOrEqual" => cur <= rhs + 1e-4,
        _ => eq,
    }
}

pub fn is_alert_area_enabled(
    progress: Option<&crate::component::player::PlayerProgressComponent>,
    scene: Option<&crate::scene::Scene>,
    area_id: i32,
) -> bool {
    if let Some(area) = scene.and_then(|s| s.alert_areas.get(&area_id)) {
        return area.enabled;
    }
    let key = format!("alert_enable:{area_id}");
    if let Some(kv) = progress.and_then(|p| p.info.dynamic.ints.iter().find(|kv| kv.key == key)) {
        return kv.val != 0;
    }
    let _ = json_i64(None);
    false
}
