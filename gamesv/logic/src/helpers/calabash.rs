use std::collections::BTreeMap;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::condition::Condition;
use sonoro_proto::pb::{
    CalabashCfg, CalabashDevelopConditionState, CalabashDevelopInfo, CalabashMsg, CalabashMsgNotify,
};

use crate::component::player::{PlayerBasicComponent, PlayerEchoComponent};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DiscoveryOutcome {
    pub changed: bool,
    pub union_exp: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Summary {
    pub level: i32,
    pub exp: i32,
    pub next_level_up_exp: i32,
    pub level_condition: i32,
    pub cost: i32,
}

pub fn discovered_max_quality(
    assets: &Assets,
    echo_comp: &PlayerEchoComponent,
    monster_id: i32,
) -> i32 {
    let mut best = echo_comp
        .calabash_info
        .highest_quality_for_monster(monster_id);
    for echo in echo_comp.echo_map.values() {
        let Some(item) = assets.tables.phantom_item.get_data_by_id(&echo.id) else {
            continue;
        };
        if item.MonsterId == monster_id {
            best = best.max(item.QualityId);
        }
    }
    best
}

fn check_level_condition(
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    condition_id: i32,
) -> bool {
    if condition_id <= 0 {
        return true;
    }
    if let Some(cond) = assets.tables.condition.get_data_by_id(&condition_id) {
        if cond.Type == "CheckCurWorldLevel" {
            return basic_comp.info.attributes.cur_world_level
                >= param_int(&cond, "Level").unwrap_or(0);
        }
        if cond.Type == "CheckOriginWorldLevel" {
            return basic_comp.info.attributes.origin_world_level
                >= param_int(&cond, "Level").unwrap_or(0);
        }
        if cond.Type == "CheckLevel" {
            return basic_comp.info.attributes.level >= param_int(&cond, "Level").unwrap_or(1);
        }
        return true;
    }
    if let Some(group) = assets.tables.condition_group.get_data_by_id(&condition_id) {
        if group.GroupId.is_empty() {
            return true;
        }
        if group.Relation == 1 {
            return group
                .GroupId
                .iter()
                .any(|&sub_id| check_level_condition(assets, basic_comp, sub_id));
        }
        return group
            .GroupId
            .iter()
            .all(|&sub_id| check_level_condition(assets, basic_comp, sub_id));
    }
    true
}

fn param_int(cond: &Condition, key: &str) -> Option<i32> {
    let lp = cond.LimitParams.as_ref()?.as_object()?;
    let val = lp.get(key)?;
    match val {
        serde_json::Value::Number(n) => n
            .as_i64()
            .and_then(|v| i32::try_from(v).ok())
            .or_else(|| n.as_f64().map(|f| f as i32)),
        serde_json::Value::String(s) => s.parse::<i32>().ok(),
        _ => None,
    }
}

fn calabash_exp_for_condition(assets: &Assets, condition_id: i32) -> i32 {
    assets
        .tables
        .calabash_develop_condition
        .get_data_by_id(&condition_id)
        .map_or(10, |cond| cond.RewardExp.get(&10).copied().unwrap_or(0))
}

pub fn required_quality_for_condition(assets: &Assets, condition_id: i32, idx: usize) -> i32 {
    if let Some(cond) = assets
        .tables
        .calabash_develop_condition
        .get_data_by_id(&condition_id)
        && let Some(&q) = cond.ConditionParam.first()
    {
        return q;
    }
    (idx + 2) as i32
}

pub fn total_develop_exp(assets: &Assets, echo_comp: &PlayerEchoComponent) -> i32 {
    let mut total_exp = 0;
    for reward in &assets.tables.calabash_develop_reward.items {
        if !reward.IsShow {
            continue;
        }
        let max_quality = discovered_max_quality(assets, echo_comp, reward.MonsterId);
        if max_quality <= 0 {
            continue;
        }
        for (idx, &condition_id) in reward.DevelopCondition.iter().enumerate() {
            if max_quality >= required_quality_for_condition(assets, condition_id, idx) {
                total_exp += calabash_exp_for_condition(assets, condition_id);
            }
        }
    }
    total_exp
}

pub fn summarize(
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    echo_comp: &PlayerEchoComponent,
) -> Summary {
    let total_exp = total_develop_exp(assets, echo_comp);
    let mut current_level = 1;
    let mut remaining_exp = total_exp;
    let mut next_level_up_exp = 30;
    let mut level_condition = 502001;
    let mut cost = 8;

    if let Some(lv1) = assets.tables.calabash_level.get_data_by_id(&1) {
        next_level_up_exp = lv1.LevelUpExp;
        level_condition = lv1.LevelUpCondition;
        cost = lv1.Cost;
    }

    let mut check_lvl = 1;
    while let Some(cfg) = assets.tables.calabash_level.get_data_by_id(&check_lvl) {
        current_level = cfg.Level;
        next_level_up_exp = cfg.LevelUpExp;
        level_condition = cfg.LevelUpCondition;
        cost = cfg.Cost;
        if cfg.LevelUpExp <= 0 || remaining_exp < cfg.LevelUpExp {
            break;
        }
        if !check_level_condition(assets, basic_comp, cfg.LevelUpCondition) {
            break;
        }
        remaining_exp -= cfg.LevelUpExp;
        check_lvl += 1;
    }

    Summary {
        level: current_level,
        exp: remaining_exp,
        next_level_up_exp,
        level_condition,
        cost,
    }
}

pub fn current_level(
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    echo_comp: &PlayerEchoComponent,
) -> i32 {
    summarize(assets, basic_comp, echo_comp).level
}

pub fn current_cost(
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    echo_comp: &PlayerEchoComponent,
) -> i32 {
    summarize(assets, basic_comp, echo_comp).cost
}

pub fn record_echo_discovery(
    assets: &Assets,
    echo_comp: &mut PlayerEchoComponent,
    monster_id: i32,
    quality: i32,
) -> DiscoveryOutcome {
    if monster_id <= 0 || quality <= 0 {
        return DiscoveryOutcome::default();
    }
    let prev_rewarded = echo_comp
        .calabash_info
        .rewarded_quality_for_monster(monster_id);
    if !echo_comp
        .calabash_info
        .upsert_discovery(monster_id, quality, true)
    {
        return DiscoveryOutcome::default();
    }

    let mut union_exp = 0;
    if let Some(reward) = assets
        .tables
        .calabash_develop_reward
        .get_data_by_id(&monster_id)
        && reward.IsShow
    {
        for (idx, &condition_id) in reward.DevelopCondition.iter().enumerate() {
            let req_quality = required_quality_for_condition(assets, condition_id, idx);
            if quality >= req_quality
                && prev_rewarded < req_quality
                && let Some(cond) = assets
                    .tables
                    .calabash_develop_condition
                    .get_data_by_id(&condition_id)
            {
                union_exp += cond.RewardExp.get(&1).copied().unwrap_or(0);
            }
        }
    }

    DiscoveryOutcome {
        changed: true,
        union_exp,
    }
}

pub fn build_msg_notify(
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    echo_comp: &PlayerEchoComponent,
) -> CalabashMsgNotify {
    let mut develop_infos = Vec::new();
    for reward in &assets.tables.calabash_develop_reward.items {
        if !reward.IsShow {
            continue;
        }
        let max_quality = discovered_max_quality(assets, echo_comp, reward.MonsterId);
        if max_quality <= 0 {
            continue;
        }
        let mut conditions = Vec::new();
        for (idx, &condition_id) in reward.DevelopCondition.iter().enumerate() {
            if max_quality >= required_quality_for_condition(assets, condition_id, idx) {
                conditions.push(CalabashDevelopConditionState {
                    ConditionId: condition_id,
                    Rewarded: true,
                });
            }
        }
        if !conditions.is_empty() {
            develop_infos.push(CalabashDevelopInfo {
                MonsterId: reward.MonsterId,
                UnlockConditions: conditions,
            });
        }
    }

    let summary = summarize(assets, basic_comp, echo_comp);
    let levels = (1..=summary.level).collect();
    let mut catch_gain = BTreeMap::new();
    for cfg in &assets.tables.calabash_level.items {
        catch_gain.insert(cfg.Level, cfg.TempCatchGain);
    }

    CalabashMsgNotify {
        CalabashMsg: Some(CalabashMsg {
            Level: summary.level,
            Exp: summary.exp,
            UnlockedLevels: levels,
            UnlockedDevelopRewards: develop_infos,
            ..Default::default()
        }),
        CalabashCfg: Some(CalabashCfg {
            LevelUpExp: summary.next_level_up_exp,
            LevelUpCondition: summary.level_condition,
            CatchGain: catch_gain,
        }),
    }
}
