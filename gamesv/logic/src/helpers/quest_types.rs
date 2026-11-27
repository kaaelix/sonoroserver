use serde_json::{Map, Value};
use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::lazy_node_store::NodeEntry;
use sonoro_proto::pb::VarDefinePb;

use crate::component::player::{
    PlayerBasicComponent, PlayerEchoComponent, PlayerGuideComponent, PlayerInventoryComponent,
    PlayerMailComponent, PlayerMapComponent, PlayerProgressComponent, PlayerQuestComponent,
    PlayerRoleComponent, PlayerSceneComponent, PlayerWeaponComponent,
};
use crate::event_queue::EventQueue;
use crate::helpers::condition::{self, EvalContext};
use crate::helpers::reward::RewardContext;
use crate::scene::Scene;
use crate::sink::PushSink;

#[derive(Debug, Clone)]
pub struct DeferredBtVar {
    pub owner_id: i32,
    pub var_name: String,
    pub var_define: VarDefinePb,
}

pub type ActivityNotifyFn<S> = fn(
    &mut S,
    &Assets,
    &PlayerProgressComponent,
    &PlayerBasicComponent,
    &PlayerEchoComponent,
    &PlayerInventoryComponent,
    &PlayerMapComponent,
    &PlayerQuestComponent,
    Option<&PlayerRoleComponent>,
    i32,
);

pub struct QuestContext<'a, S: PushSink + ?Sized> {
    pub assets: &'a Assets,
    pub fs: &'a FileSystem,
    pub sink: &'a mut S,
    pub scene: Option<&'a mut Scene>,
    pub scene_comp: Option<&'a mut PlayerSceneComponent>,
    pub events: &'a mut EventQueue,
    pub basic_comp: &'a mut PlayerBasicComponent,
    pub inventory_comp: &'a mut PlayerInventoryComponent,
    pub weapon_comp: &'a mut PlayerWeaponComponent,
    pub echo_comp: &'a mut PlayerEchoComponent,
    pub map_comp: &'a mut PlayerMapComponent,
    pub quest_comp: &'a mut PlayerQuestComponent,
    pub role_comp: Option<&'a mut PlayerRoleComponent>,
    pub guide_comp: Option<&'a mut PlayerGuideComponent>,
    pub mail_comp: Option<&'a mut PlayerMailComponent>,
    pub progress_comp: Option<&'a mut PlayerProgressComponent>,
    pub activity_notify: Option<ActivityNotifyFn<S>>,
}

impl<'a, S: PushSink + ?Sized> QuestContext<'a, S> {
    pub fn make_eval_context(&self) -> EvalContext<'_> {
        self.make_eval_context_for_quest(0)
    }

    pub fn make_eval_context_for_role(&self, target_role_id: i32) -> EvalContext<'_> {
        let mut ctx = self.make_eval_context_for_quest(0);
        ctx.target_role_id = target_role_id;
        ctx
    }

    pub fn make_eval_context_for_quest(&self, owner_quest_id: i32) -> EvalContext<'_> {
        EvalContext {
            assets: Some(self.assets),
            basic_comp: self.basic_comp,
            echo_comp: self.echo_comp,
            inventory_comp: self.inventory_comp,
            map_comp: self.map_comp,
            quest_comp: self.quest_comp,
            role_comp: self.role_comp.as_deref(),
            weapon_comp: Some(&*self.weapon_comp),
            guide_comp: self.guide_comp.as_deref(),
            progress_comp: self.progress_comp.as_deref(),
            scene: self.scene.as_deref(),
            owner_quest_id,
            target_role_id: 0,
            self_entity_config_id: 0,
        }
    }

    pub fn make_reward_context(&mut self) -> RewardContext<'_, S> {
        RewardContext {
            fs: self.fs,
            assets: self.assets,
            sink: self.sink,
            basic_comp: self.basic_comp,
            inventory_comp: self.inventory_comp,
            weapon_comp: self.weapon_comp,
            echo_comp: self.echo_comp,
            progress_comp: self.progress_comp.as_deref_mut(),
        }
    }
}

pub type StartFlowFn<S> = fn(
    &mut QuestContext<'_, S>,
    quest_id: i32,
    node_id: i32,
    params: &Map<String, Value>,
    persist: bool,
    is_async: Option<bool>,
) -> std::io::Result<()>;

pub type StartFlowStateFn<S> = fn(
    &mut QuestContext<'_, S>,
    quest_id: i32,
    node_id: i32,
    flow_name: &str,
    flow_id: i32,
    state_id: i32,
    update_node_if_busy: bool,
) -> std::io::Result<()>;

pub type VoidCallbackFn<S> = fn(&mut QuestContext<'_, S>) -> std::io::Result<()>;
pub type IdCallbackFn<S> = fn(&mut QuestContext<'_, S>, id: i32) -> std::io::Result<()>;
pub type PairIdCallbackFn<S> = fn(&mut QuestContext<'_, S>, a: i32, b: i32) -> std::io::Result<()>;
pub type StateCallbackFn<S> = fn(&mut QuestContext<'_, S>, id: i32, text: &str) -> std::io::Result<()>;
pub type LevelPlayCallbackFn<S> =
    fn(&mut QuestContext<'_, S>, lp_id: i32, enable: bool) -> std::io::Result<()>;

pub struct ActionCallbacks<'a, S: PushSink + ?Sized> {
    pub bt_type: sonoro_proto::enums::BtType,
    pub is_flow_action: bool,
    pub defer_flow_transition: bool,
    pub deferred_bt_vars: Option<&'a mut Vec<DeferredBtVar>>,
    pub start_flow: Option<StartFlowFn<S>>,
    pub start_flow_state: Option<StartFlowStateFn<S>>,
    pub complete_guide: Option<IdCallbackFn<S>>,
    pub on_entity_state_changed: Option<StateCallbackFn<S>>,
    pub on_phone_message: Option<PairIdCallbackFn<S>>,
    pub on_npc_mail: Option<IdCallbackFn<S>>,
    pub on_time_changed: Option<PairIdCallbackFn<S>>,
    pub on_weather_changed: Option<IdCallbackFn<S>>,
    pub on_item_gained: Option<IdCallbackFn<S>>,
    pub on_level_play_enabled: Option<LevelPlayCallbackFn<S>>,
    pub on_accept_focus_wait: Option<VoidCallbackFn<S>>,
    pub on_finish_flow_state: Option<VoidCallbackFn<S>>,
    pub on_destroy_quest: Option<IdCallbackFn<S>>,
    pub on_finish_condition: Option<PairIdCallbackFn<S>>,
    pub on_trace_current_quest: Option<IdCallbackFn<S>>,
    pub on_var_changed: Option<StateCallbackFn<S>>,
}

impl<S: PushSink + ?Sized> Default for ActionCallbacks<'_, S> {
    fn default() -> Self {
        Self {
            bt_type: sonoro_proto::enums::BtType::BtTypeQuest,
            is_flow_action: false,
            defer_flow_transition: false,
            deferred_bt_vars: None,
            start_flow: None,
            start_flow_state: None,
            complete_guide: None,
            on_entity_state_changed: None,
            on_phone_message: None,
            on_npc_mail: None,
            on_time_changed: None,
            on_weather_changed: None,
            on_item_gained: None,
            on_level_play_enabled: None,
            on_accept_focus_wait: None,
            on_finish_flow_state: None,
            on_destroy_quest: None,
            on_finish_condition: None,
            on_trace_current_quest: None,
            on_var_changed: None,
        }
    }
}

pub fn json_str(v_opt: Option<&Value>) -> Option<&str> {
    v_opt.and_then(|v| v.as_str())
}

pub fn json_i64(v_opt: Option<&Value>) -> Option<i64> {
    let v = v_opt?;
    match v {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Some(i)
            } else {
                let f = n.as_f64()?;
                if f.is_finite() && f >= (i64::MIN as f64) && f < 9_223_372_036_854_775_808.0 {
                    Some(f as i64)
                } else {
                    None
                }
            }
        }
        Value::String(s) => s.parse::<i64>().ok(),
        Value::Bool(b) => Some(i64::from(*b)),
        _ => None,
    }
}

pub fn json_int(v_opt: Option<&Value>) -> Option<i32> {
    let n = json_i64(v_opt)?;
    i32::try_from(n).ok()
}

pub fn json_float(v_opt: Option<&Value>) -> Option<f32> {
    let f = v_opt?.as_f64()?;
    if f.is_finite() {
        Some(f as f32)
    } else {
        None
    }
}

pub fn json_bool(v_opt: Option<&Value>) -> Option<bool> {
    let v = v_opt?;
    match v {
        Value::Bool(b) => Some(*b),
        Value::Number(n) => Some(n.as_i64().unwrap_or(0) != 0),
        _ => None,
    }
}

pub fn resolve_var_name(obj: &Map<String, Value>) -> Option<&str> {
    if let Some(n) = json_str(obj.get("Name"))
        && !n.is_empty()
    {
        return Some(n);
    }
    if let Some(k) = json_str(obj.get("Keyword"))
        && !k.is_empty()
    {
        return Some(k);
    }
    None
}

pub fn scoped_var_key_with_owner(
    scene: Option<&Scene>,
    owner_id: i32,
    obj: &Map<String, Value>,
    var_name: &str,
) -> Option<String> {
    let src = json_str(obj.get("Source")).unwrap_or("");
    if src == "Global" {
        return None;
    }
    let ref_id = json_i64(obj.get("RefId")).unwrap_or(0);
    if ref_id > 0 {
        let ref_type = json_str(obj.get("RefType")).unwrap_or("Entity");
        return Some(format!("{ref_type}:{ref_id}:{var_name}"));
    }
    if owner_id > 0 {
        return Some(format!("Self:{owner_id}:{var_name}"));
    }
    if let Some(sc) = scene {
        if sc.active_flow_owner_entity_id != 0 {
            return Some(format!(
                "EntSelf:{}:{var_name}",
                sc.active_flow_owner_entity_id
            ));
        }
        if sc.active_flow_id > 0 {
            return Some(format!("FlowSelf:{}:{var_name}", sc.active_flow_id));
        }
    }
    None
}

pub fn scoped_var_key(obj: &Map<String, Value>, var_name: &str) -> Option<String> {
    scoped_var_key_with_owner(None, 0, obj, var_name)
}

pub fn resolve_var_int<S: PushSink + ?Sized>(
    ctx: &QuestContext<'_, S>,
    var_opt: Option<&Value>,
) -> i32 {
    resolve_var_int_for_owner(ctx, 0, var_opt)
}

pub fn resolve_var_int_for_owner<S: PushSink + ?Sized>(
    ctx: &QuestContext<'_, S>,
    owner_id: i32,
    var_opt: Option<&Value>,
) -> i32 {
    let Some(Value::Object(obj)) = var_opt else {
        return 0;
    };
    let src = json_str(obj.get("Source")).unwrap_or("Constant");
    if src == "Constant" {
        return json_int(obj.get("Value")).unwrap_or(0);
    }
    let Some(name) = resolve_var_name(obj) else {
        return 0;
    };
    let is_bool = matches!(json_str(obj.get("Type")), Some("Bool" | "Boolean"));
    if src == "Global"
        && let Some(pc) = ctx.progress_comp.as_deref()
    {
        let gkey = format!("var:{name}");
        for entry in &pc.info.dynamic.ints {
            if entry.key == gkey {
                return entry.val.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
            }
        }
    }
    if let Some(sk) = scoped_var_key_with_owner(ctx.scene.as_deref(), owner_id, obj, name)
        && ctx.quest_comp.info.has_var(&sk)
    {
        if is_bool {
            return i32::from(ctx.quest_comp.info.get_var_bool(&sk));
        }
        return ctx.quest_comp.info.get_var_int(&sk);
    }
    if ctx.quest_comp.info.has_var(name) {
        if is_bool {
            return i32::from(ctx.quest_comp.info.get_var_bool(name));
        }
        return ctx.quest_comp.info.get_var_int(name);
    }
    if let Some(init_val) = condition::lookup_entity_config_var(&ctx.make_eval_context(), obj, name)
    {
        return match init_val {
            Value::Number(n) => n
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .unwrap_or(0),
            Value::Bool(b) => i32::from(b),
            _ => 0,
        };
    }
    0
}

pub fn parse_int_list(val_opt: Option<&Value>) -> Vec<i32> {
    let Some(Value::Array(arr)) = val_opt else {
        return Vec::new();
    };
    arr.iter().filter_map(|v| json_int(Some(v))).collect()
}

pub fn parse_str_list(val_opt: Option<&Value>) -> Vec<String> {
    let Some(Value::Array(arr)) = val_opt else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|v| v.as_str().filter(|s| !s.is_empty()).map(ToString::to_string))
        .collect()
}

pub fn repeater_cooldown_key(tree_id: i32, node_id: i32) -> i64 {
    (i64::from(tree_id) << 32) | (i64::from(node_id) & 0xffff_ffff)
}

pub fn wait_action_delay_ms(act_val: &Value) -> i64 {
    let Some(obj) = act_val.as_object() else {
        return 0;
    };
    if json_str(obj.get("Name")) != Some("Wait") {
        return 0;
    }
    let Some(Value::Object(params)) = obj.get("Params") else {
        return 0;
    };
    let Some(secs) = json_float(params.get("Time")) else {
        return 0;
    };
    if secs <= 0.0 {
        return 0;
    }
    ((secs * 1000.0) as i64).max(200)
}

pub fn action_list_wait_delay_ms(actions_opt: Option<&Value>) -> i64 {
    let Some(Value::Array(items)) = actions_opt else {
        return 0;
    };
    items.iter().map(wait_action_delay_ms).sum()
}

pub fn subtree_wait_delay_ms(nodes: &[NodeEntry], parent_id: i32) -> i64 {
    let mut total_ms = 0i64;
    for child in nodes {
        if child.parent_id != parent_id {
            continue;
        }
        if let Value::Object(obj) = &child.data {
            total_ms += action_list_wait_delay_ms(obj.get("Actions"));
            total_ms += action_list_wait_delay_ms(obj.get("FinishActions"));
            if let Some(single) = obj.get("Action") {
                total_ms += wait_action_delay_ms(single);
            }
        }
        total_ms += subtree_wait_delay_ms(nodes, child.id);
    }
    total_ms
}
