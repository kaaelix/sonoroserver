use std::collections::HashSet;
use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_proto::pb::{
    AdviceSettingNotify, BuffItem, BuffItemNotify, CalabashLevelsRewardNotify, ControlInfoNotify,
    InstDataNotify, InstEnterInfoPb, ItemPkgOpenNotify, LevelPlayInfo, LevelPlayInfoNotify,
    MapMarkInfoNotify, MapMarkShowInfo, MoonChasingTargetGetCountNotify,
    MoonChasingTrackMoonHandbookRewardNotify, PassiveSkillNotify, PlayerVarNotify,
    RoguelikeCurrencyNotify, RoleConfigInfo, RoleConfigInfoNotify, SettingNotify, SilenceNpcNotify,
};

use crate::component::player::{
    PlayerBasicComponent, PlayerCosmeticComponent, PlayerEchoComponent, PlayerInventoryComponent,
    PlayerMailComponent, PlayerMapComponent, PlayerProgressComponent, PlayerQuestComponent,
    PlayerRoleComponent,
};
use crate::helpers::quest_types::{ActivityNotifyFn, QuestContext};
use crate::helpers::{calabash, condition, energy, phantom_projector, quest_engine};
use crate::sink::PushSink;

#[allow(clippy::too_many_arguments)]
pub fn push_data<S: PushSink + ?Sized>(
    sink: &mut S,
    fs: &FileSystem,
    assets: &Assets,
    role_comp: &PlayerRoleComponent,
    echo_comp: &PlayerEchoComponent,
    cosmetic_comp: &PlayerCosmeticComponent,
    map_comp: &PlayerMapComponent,
    basic_comp: &mut PlayerBasicComponent,
    inventory_comp: &PlayerInventoryComponent,
    quest_comp: &PlayerQuestComponent,
    mail_comp: &mut PlayerMailComponent,
    progress_comp: &PlayerProgressComponent,
    activity_notify: Option<ActivityNotifyFn<S>>,
) -> io::Result<()> {
    let role_configs: Vec<RoleConfigInfo> = role_comp
        .role_map
        .iter()
        .map(|(&role_id, r)| RoleConfigInfo {
            RoleId: role_id,
            SkillBranch: r.skill_branch,
        })
        .collect();
    sink.push(
        RoleConfigInfoNotify::MSG_ID,
        &RoleConfigInfoNotify {
            RoleConfigs: role_configs,
        },
    );

    condition::push_func_open_notify(
        sink,
        assets,
        basic_comp,
        echo_comp,
        inventory_comp,
        map_comp,
        quest_comp,
        Some(role_comp),
    );

    if let Some(notify_fn) = activity_notify {
        notify_fn(
            sink,
            assets,
            progress_comp,
            basic_comp,
            echo_comp,
            inventory_comp,
            map_comp,
            quest_comp,
            Some(role_comp),
            basic_comp.player_id,
        );
    }

    sink.push(AdviceSettingNotify::MSG_ID, &AdviceSettingNotify { IsShow: false });
    sink.push(
        ControlInfoNotify::MSG_ID,
        &ControlInfoNotify {
            ForbidList: Vec::new(),
        },
    );
    sink.push(
        InstDataNotify::MSG_ID,
        &build_inst_data_notify(assets, quest_comp),
    );
    let calabash_notify = calabash::build_msg_notify(assets, basic_comp, echo_comp);
    sink.push(sonoro_proto::pb::CalabashMsgNotify::MSG_ID, &calabash_notify);
    let projector_notify = phantom_projector::build_unlock_notify(
        assets,
        &echo_comp.calabash_info,
        &cosmetic_comp.info,
    );
    sink.push(
        sonoro_proto::pb::PhantomInteractionUnlockNotify::MSG_ID,
        &projector_notify,
    );
    sink.push(
        CalabashLevelsRewardNotify::MSG_ID,
        &CalabashLevelsRewardNotify {
            RewardedLevels: echo_comp.calabash_info.rewarded_levels.clone(),
        },
    );

    sink.push(
        ItemPkgOpenNotify::MSG_ID,
        &ItemPkgOpenNotify {
            OpenPkg: (0..8).collect(),
        },
    );

    let now_sec = energy::now_seconds();
    let now_ms = now_sec * 1000;
    let buff_cds: Vec<BuffItem> = progress_comp
        .info
        .dynamic
        .ints
        .iter()
        .filter_map(|entry| {
            let rest = entry.key.strip_prefix("buff_cd:")?;
            if entry.val <= now_ms {
                return None;
            }
            let item_id = rest.parse::<i32>().ok()?;
            Some(BuffItem {
                ItemId: item_id,
                CdTime: entry.val,
            })
        })
        .collect();
    sink.push(
        BuffItemNotify::MSG_ID,
        &BuffItemNotify {
            ItemBuffList: buff_cds,
            EquipItemList: Vec::new(),
        },
    );

    energy::sync_and_save(fs, basic_comp)?;
    energy::push_energy_notify(sink, basic_comp);

    if mail_comp.info.prune_expired(now_sec) {
        mail_comp.save(fs)?;
    }

    sink.push(
        LevelPlayInfoNotify::MSG_ID,
        &build_level_play_info_notify(assets, basic_comp, quest_comp),
    );
    sink.push(
        MapMarkInfoNotify::MSG_ID,
        &build_map_mark_info_notify(assets, map_comp, basic_comp, quest_comp),
    );
    sink.push(PlayerVarNotify::MSG_ID, &PlayerVarNotify::default());
    sink.push(
        RoguelikeCurrencyNotify::MSG_ID,
        &RoguelikeCurrencyNotify::default(),
    );
    sink.push(PassiveSkillNotify::MSG_ID, &PassiveSkillNotify::default());
    sink.push(
        sonoro_proto::pb::MailInfosNotify::MSG_ID,
        &mail_comp.info.to_proto_notify(),
    );
    sink.push(SettingNotify::MSG_ID, &SettingNotify::default());
    sink.push(
        MoonChasingTrackMoonHandbookRewardNotify::MSG_ID,
        &MoonChasingTrackMoonHandbookRewardNotify::default(),
    );
    sink.push(
        MoonChasingTargetGetCountNotify::MSG_ID,
        &MoonChasingTargetGetCountNotify::default(),
    );
    sink.push(SilenceNpcNotify::MSG_ID, &SilenceNpcNotify::default());
    Ok(())
}

fn build_unlock_eval_context<'a>(
    assets: &'a Assets,
    basic_comp: &'a PlayerBasicComponent,
    quest_comp: &'a PlayerQuestComponent,
    map_comp: &'a PlayerMapComponent,
) -> condition::EvalContext<'a> {
    static EMPTY_ECHO: std::sync::OnceLock<PlayerEchoComponent> = std::sync::OnceLock::new();
    static EMPTY_INV: std::sync::OnceLock<PlayerInventoryComponent> = std::sync::OnceLock::new();
    let echo = EMPTY_ECHO.get_or_init(|| PlayerEchoComponent {
        player_id: basic_comp.player_id,
        echo_map: Default::default(),
        preset_info: Default::default(),
        calabash_info: Default::default(),
    });
    let inv = EMPTY_INV.get_or_init(|| PlayerInventoryComponent {
        player_id: basic_comp.player_id,
        info: Default::default(),
    });
    let mut eval_ctx = condition::EvalContext::new(basic_comp, echo, inv, map_comp, quest_comp);
    eval_ctx.assets = Some(assets);
    eval_ctx
}

fn eval_non_spatial_provide_block(
    eval_ctx: &condition::EvalContext<'_>,
    cond_val_opt: Option<&serde_json::Value>,
) -> bool {
    let Some(cond_val) = cond_val_opt else {
        return true;
    };
    let Some(obj) = cond_val.as_object() else {
        return true;
    };
    let Some(conds) = obj
        .get("Conditions")
        .and_then(serde_json::Value::as_array)
    else {
        return true;
    };
    let non_spatial: Vec<&serde_json::Value> = conds
        .iter()
        .filter(|c| c.get("Type").and_then(|v| v.as_str()) != Some("ReachArea"))
        .collect();
    if non_spatial.is_empty() {
        return true;
    }
    let is_or = obj.get("Type").and_then(|v| v.as_i64()) == Some(1);
    for cond in non_spatial {
        let passed = condition::eval_single_provide_condition(eval_ctx, cond);
        if is_or && passed {
            return true;
        }
        if !is_or && !passed {
            return false;
        }
    }
    !is_or
}

pub fn is_level_play_open_globally(
    eval_ctx: &condition::EvalContext<'_>,
    lp: &sonoro_gamesv_data::tables::level_play_data::LevelPlayData,
) -> bool {
    let lp_id = lp.LevelPlayId;
    if lp_id <= 0 {
        return false;
    }
    if eval_ctx.quest_comp.info.completed_level_plays.contains(&lp_id)
        || eval_ctx
            .quest_comp
            .info
            .progress
            .entered_level_plays
            .contains(&lp_id)
    {
        return true;
    }
    if lp.Data.Type == "Quest" {
        return true;
    }
    if lp.Data.Key.starts_with("玩法样例") {
        return false;
    }
    if crate::helpers::level_play::is_parkour_challenge_lp(lp_id)
        && !condition::is_parkour_unlocked(eval_ctx)
    {
        return false;
    }
    let lp_inst_or_level = if lp.Data.InstanceId != 0 {
        lp.Data.InstanceId
    } else {
        lp.Data.LevelId
    };
    let lp_map = eval_ctx
        .assets
        .map_or(lp_inst_or_level, |a| a.resolve_map_config_id(lp_inst_or_level));
    if (lp.Data.Type == "RebornBoss" || lp_map == 8)
        && !eval_ctx
            .quest_comp
            .info
            .has_finished_quest(quest_engine::PROLOGUE_QUEST_ID)
    {
        return false;
    }
    if let Some(act) = lp.Data.LevelPlayActive.as_ref()
        && act.ActiveType == 3
        && !eval_non_spatial_provide_block(eval_ctx, act.ActiveCondition.as_ref())
    {
        return false;
    }
    eval_non_spatial_provide_block(eval_ctx, lp.Data.LevelPlayOpenCondition.as_ref())
}

pub fn is_entity_blocked_by_locked_level_play(
    eval_ctx: &condition::EvalContext<'_>,
    instance_id: i32,
    cfg_id32: i32,
) -> bool {
    let Some(assets) = eval_ctx.assets else {
        return false;
    };
    if cfg_id32 <= 0 {
        return false;
    }
    let scene_map = assets.resolve_map_config_id(instance_id);
    let check_indices = [
        assets.indexes.level_play_indices_for_ref_entity(cfg_id32),
        assets.indexes.level_play_indices_for_reward_entity(cfg_id32),
    ];
    for indices in check_indices {
        for &lp_idx in indices {
            let lp = &assets.tables.level_play_data.items[lp_idx as usize];
            if lp.Data.Type == "Quest" {
                continue;
            }
            let lp_map = assets.resolve_map_config_id(if lp.Data.InstanceId != 0 {
                lp.Data.InstanceId
            } else {
                lp.Data.LevelId
            });
            if lp.Data.InstanceId != instance_id && lp.Data.LevelId != instance_id && lp_map != scene_map
            {
                continue;
            }
            if !is_level_play_open_globally(eval_ctx, lp) {
                return true;
            }
        }
    }
    false
}

pub fn push_level_play_and_map_mark_sync<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) {
    let lp_notify = build_level_play_info_notify(ctx.assets, ctx.basic_comp, ctx.quest_comp);
    let mm_notify = build_map_mark_info_notify(
        ctx.assets,
        ctx.map_comp,
        ctx.basic_comp,
        ctx.quest_comp,
    );
    ctx.sink.push(LevelPlayInfoNotify::MSG_ID, &lp_notify);
    ctx.sink.push(MapMarkInfoNotify::MSG_ID, &mm_notify);
}

pub fn build_level_play_info_notify(
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    quest_comp: &PlayerQuestComponent,
) -> LevelPlayInfoNotify {
    let empty_map = PlayerMapComponent {
        player_id: basic_comp.player_id,
        info: Default::default(),
    };
    let eval_ctx = build_unlock_eval_context(assets, basic_comp, quest_comp, &empty_map);
    let mut seen = HashSet::new();
    let mut infos = Vec::new();
    for mark in &assets.tables.map_mark.items {
        if mark.RelativeType != 1 || mark.RelativeId <= 0 {
            continue;
        }
        if mark.ObjectType != 19 && mark.ObjectType != 28 && mark.ObjectType != 29 {
            continue;
        }
        let Some(lp) = assets
            .tables
            .level_play_data
            .get_data_by_id(&mark.RelativeId)
        else {
            continue;
        };
        if !is_level_play_open_globally(&eval_ctx, &lp) {
            continue;
        }
        if !seen.insert(mark.RelativeId) {
            continue;
        }
        let done = quest_comp
            .info
            .completed_level_plays
            .contains(&mark.RelativeId);
        infos.push(LevelPlayInfo {
            Id: mark.RelativeId,
            IsFirst: !done,
            State: if done { 3 } else { 1 },
            UpdateTime: 0,
            GetRewardCount: 0,
        });
    }
    LevelPlayInfoNotify {
        LevelPlayInfo: infos,
    }
}

pub fn build_map_mark_info_notify(
    assets: &Assets,
    map_comp: &PlayerMapComponent,
    basic_comp: &PlayerBasicComponent,
    quest_comp: &PlayerQuestComponent,
) -> MapMarkInfoNotify {
    let eval_ctx = build_unlock_eval_context(assets, basic_comp, quest_comp, map_comp);
    let mut show_marks = Vec::new();
    let mut unlock_marks = Vec::new();
    let mut seen_show = HashSet::new();
    let mut seen_unlock = HashSet::new();

    for &tp_id in &map_comp.info.unlocked_teleporters {
        if seen_unlock.insert(tp_id) {
            unlock_marks.push(tp_id);
        }
        if seen_show.insert(tp_id) {
            show_marks.push(MapMarkShowInfo {
                MarkId: tp_id,
                ShowFlag: 1,
            });
        }
    }

    for mark in &assets.tables.map_mark.items {
        let is_boss_or_challenge =
            mark.ObjectType == 19 || mark.ObjectType == 28 || mark.ObjectType == 29;
        let is_teleporter = mark.ObjectType == 5 || mark.ObjectType == 6;

        if !is_teleporter {
            if mark.ShowCondition > 0 && !condition::eval_condition_group(&eval_ctx, mark.ShowCondition) {
                continue;
            }
            if mark.RelativeType == 1
                && mark.RelativeId > 0
                && let Some(lp) = assets
                    .tables
                    .level_play_data
                    .get_data_by_id(&mark.RelativeId)
                && !is_level_play_open_globally(&eval_ctx, &lp)
            {
                continue;
            }
        }

        if is_boss_or_challenge {
            if seen_show.insert(mark.MarkId) {
                show_marks.push(MapMarkShowInfo {
                    MarkId: mark.MarkId,
                    ShowFlag: 1,
                });
            }
            if seen_unlock.insert(mark.MarkId) {
                unlock_marks.push(mark.MarkId);
            }
        } else if (!is_teleporter
            && mark.CreateOnStart == 1
            && (mark.FogHide == 0 || map_comp.info.unlocked_fogs.contains(&mark.FogHide)))
            || (is_teleporter
                && !map_comp.info.unlocked_teleporters.contains(&mark.MarkId)
                && mark.FogHide != 0
                && map_comp.info.unlocked_fogs.contains(&mark.FogHide))
        {
            if seen_show.insert(mark.MarkId) {
                show_marks.push(MapMarkShowInfo {
                    MarkId: mark.MarkId,
                    ShowFlag: 1,
                });
            }
        }
    }

    MapMarkInfoNotify {
        ShowMarkIds: show_marks,
        UnlockMarkIds: unlock_marks,
        ..Default::default()
    }
}

pub fn build_inst_data_notify(
    assets: &Assets,
    quest_comp: &PlayerQuestComponent,
) -> InstDataNotify {
    let mut enter_infos = Vec::new();
    let mut seen_shares = HashSet::new();
    for &inst_id in &quest_comp.info.completed_dungeons {
        if inst_id <= 0 {
            continue;
        }
        // InstDataNotify expects InstanceEnterControl ShareId; skip dungeons without enter-count config.
        let Some(dg) = assets.tables.instance_dungeon.get_data_by_id(&inst_id) else {
            continue;
        };
        if dg.EnterControlId <= 0 || !seen_shares.insert(dg.EnterControlId) {
            continue;
        }
        enter_infos.push(InstEnterInfoPb {
            Id: dg.EnterControlId,
            ChallengedTimes: 1,
        });
    }
    InstDataNotify {
        EnterInfos: enter_infos,
    }
}

pub fn handle_reconnect_resync<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    quest_engine::resync_on_reconnect(ctx)
}

pub fn handle_condition_state_refresh<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    quest_engine::on_progression_unlock_changed(ctx)
}
