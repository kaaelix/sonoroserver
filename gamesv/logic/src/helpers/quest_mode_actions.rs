use std::collections::BTreeMap;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Map, Value};
use sonoro_proto::enums::{
    GainDataUpdateType, MowTowerRewardStatus, RoguelikeGainDataType, RoverRogueGainDataType,
};
use sonoro_proto::pb::{
    AbyssChallengeRoomInfoNotify, AbyssChallengeSubLevelNotify, AchievementCountChangeNotify,
    AchievementEntry, AchievementProgress, AchievementProgressNotify, BattleFormation,
    BossPilingInstNotify, HonamiStoryBagInfo, HonamiStoryBagInfoUpdateNotify,
    HonamiStoryBagItemInfo, HonamiStoryItemInfo, HonamiStoryItemInfoItemInfoOneof,
    HonamiStoryNormalItemInfo, HonamiStoryPosInfo, HonamiStorySafeLeaveNotify,
    MowTowerLevelsInfo, MowTowerLevelsInfoUpdateNotify, MowTowerResultNotify, RogueGainEntry,
    RoguelikeCurrencyUpdateNotify, RoguelikeGainDataUpdateNotify, RoguelikeRoleRoomNotify,
    RoguelikeRoomInfoNotify, RoguelikeSubLevelNotify, RoverRogueBlessGroupData,
    RoverRogueBlessGroupSelectNotify, RoverRogueEventNotify, RoverRogueGainEntry,
    RoverRogueLootChangeNotify, RoverRogueShopGood, RoverRogueShopSnapshot,
    RoverRogueShopUpdateNotify, SlashAndTowerLevelPlayNotify, SlashAndTowerResultNotify,
    SlashLevelPlayInfo,
};

use crate::helpers::quest_actions::{
    apply_destroy_self, record_progress_add, record_progress_claim, record_progress_int,
    resolve_spawn_anchor, transition_to_scene,
};
use crate::helpers::quest_types::{QuestContext, json_int, json_str};
use crate::helpers::{reward, trial_roles};
use crate::sink::PushSink;

fn parse_csv_ids(raw: &str) -> Vec<i32> {
    raw.split(',')
        .filter_map(|tok| tok.trim().parse::<i32>().ok())
        .filter(|&id| id > 0)
        .collect()
}

pub fn apply_client_unlock_achievement<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    params: &Map<String, Value>,
) -> io::Result<()> {
    let Some(ach_id) = json_int(params.get("Id")).filter(|&id| id > 0) else {
        return Ok(());
    };
    if ctx.assets.tables.achievement.get_data_by_id(&ach_id).is_none() {
        return Ok(());
    }
    let Some(pc) = ctx.progress_comp.as_deref_mut() else {
        return Ok(());
    };
    let now_u32 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(1, |d| d.as_secs().clamp(1, u64::from(u32::MAX)) as u32);
    let (entry_pb, finish_count, star_count) = {
        let rec = pc.info.finish_achievement(ach_id, now_u32);
        let pb = AchievementEntry {
            Id: rec.id,
            FinishTime: rec.finish_time,
            IsReceive: rec.received,
            Progress: Some(AchievementProgress {
                CurProgress: rec.cur_progress,
                TotalProgress: rec.total_progress,
            }),
        };
        let mut fc = 0i32;
        let mut sc = 0i32;
        for entry in &pc.info.achievement.entries {
            if entry.finish_time == 0 {
                continue;
            }
            fc += 1;
            if let Some(cfg) = ctx.assets.tables.achievement.get_data_by_id(&entry.id) {
                sc += cfg.Level.max(1);
            }
        }
        (pb, fc, sc)
    };
    pc.save(ctx.fs)?;
    ctx.sink.push(
        AchievementProgressNotify::MSG_ID,
        &AchievementProgressNotify {
            AchievementEntry: Some(entry_pb),
        },
    );
    ctx.sink.push(
        AchievementCountChangeNotify::MSG_ID,
        &AchievementCountChangeNotify {
            StarCount: star_count,
            FinishCount: finish_count,
        },
    );
    Ok(())
}

pub fn apply_rogue_goto_next_floor<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let floor = record_progress_add(ctx, "rogue:floor", 1)?.max(1) as i32;
    let layer = record_progress_add(ctx, "rogue:layer", 1)?.max(1) as i32;
    let max_layer = 10i32.max(layer);

    let pools = ctx.assets.rogue_room_pools();
    let pool_entry = if !pools.is_empty() {
        let idx = ((layer as usize).saturating_sub(1)) % pools.len();
        Some(&pools[idx])
    } else {
        None
    };
    let room_id = pool_entry.map_or(1, |p| p.Id);
    let music_state = pool_entry.map_or("", |p| p.RoomsMusicState.as_str());
    let room_type_id = ctx
        .assets
        .rogue_room_types()
        .iter()
        .find(|rt| !music_state.is_empty() && rt.RoomsMusicState == music_state)
        .or_else(|| ctx.assets.rogue_room_types().first())
        .map_or(1, |rt| rt.Id);

    if let Some(pc) = ctx.progress_comp.as_deref_mut() {
        pc.info.set_key_int("rogue:room", i64::from(room_id));
        pc.info.set_key_int("rogue:floor", i64::from(floor));
        pc.save(ctx.fs)?;
    }

    ctx.sink.push(
        RoguelikeRoomInfoNotify::MSG_ID,
        &RoguelikeRoomInfoNotify {
            CurLayer: layer,
            MaxLayer: max_layer,
            SkyBoxId: 0,
            RoguelikeRoomTypeId: room_type_id,
            RoguelikeRoomId: room_id,
        },
    );

    let (pos, yaw) = current_player_pos_and_yaw(ctx);
    ctx.sink.push(
        RoguelikeSubLevelNotify::MSG_ID,
        &RoguelikeSubLevelNotify {
            A: yaw,
            X: pos[0],
            Y: pos[1],
            Z: pos[2],
            LoadSubLevel: Vec::new(),
            UnLoadSubLevel: Vec::new(),
            LoadSubLevels: Vec::new(),
            UnLoadSubLevels: Vec::new(),
        },
    );
    Ok(())
}

pub fn apply_rogue_select_room<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    params: &Map<String, Value>,
) -> io::Result<()> {
    let sel_count = record_progress_add(ctx, "rogue:room_sel", 1)?.max(1) as i32;
    let room_ty = params
        .get("Config")
        .and_then(Value::as_object)
        .and_then(|c| json_str(c.get("Type")))
        .unwrap_or("Role");

    let mut candidate_ids: Vec<i32> = ctx
        .assets
        .rogue_room_pools()
        .iter()
        .filter(|p| {
            room_ty != "Role"
                || p.RoomsMusicState.contains("rest")
                || p.RoomsMusicState.contains("safe")
                || p.BehaviorTree > 0
        })
        .take(3)
        .map(|p| p.Id)
        .collect();
    if candidate_ids.is_empty() {
        candidate_ids = ctx
            .assets
            .rogue_room_types()
            .iter()
            .take(3)
            .map(|t| t.Id)
            .collect();
    }
    if candidate_ids.is_empty() {
        candidate_ids.push(1);
    }

    if let Some(pc) = ctx.progress_comp.as_deref_mut() {
        let csv = candidate_ids
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        pc.info.set_key_str("rogue:role_room_candidates", &csv);
        pc.save(ctx.fs)?;
    }

    ctx.sink.push(
        RoguelikeRoleRoomNotify::MSG_ID,
        &RoguelikeRoleRoomNotify {
            Index: sel_count,
            RoleRoomIds: candidate_ids,
        },
    );
    Ok(())
}

pub fn apply_rogue_receive_reward<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let reward_cnt = record_progress_add(ctx, "rogue:reward_cnt", 1)?.max(1) as i32;
    let (season_id, currency_val) = if let Some(pc) = ctx.progress_comp.as_deref_mut() {
        let sid = pc.info.get_key_int("rogue:season").max(1) as i32;
        let bal = pc.info.add_key_int("rogue:currency", 50).max(50) as i32;
        pc.save(ctx.fs)?;
        (sid, bal)
    } else {
        (1, 50)
    };

    let drop_id = ctx
        .assets
        .rogue_season_rewards()
        .iter()
        .find(|r| r.SeasonId == season_id && r.DropId > 0)
        .or_else(|| {
            ctx.assets
                .rogue_season_rewards()
                .iter()
                .find(|r| r.DropId > 0)
        })
        .map_or(0, |r| r.DropId);
    if drop_id > 0 {
        let mut rctx = ctx.make_reward_context();
        let _ = reward::grant_drop_package_popup(&mut rctx, drop_id, 0);
    }

    let mut currency_dict = BTreeMap::new();
    currency_dict.insert(1, currency_val);
    ctx.sink.push(
        RoguelikeCurrencyUpdateNotify::MSG_ID,
        &RoguelikeCurrencyUpdateNotify {
            CurrencyUpdateDict: currency_dict,
        },
    );
    ctx.sink.push(
        RoguelikeGainDataUpdateNotify::MSG_ID,
        &RoguelikeGainDataUpdateNotify {
            RogueGainEntry: Some(RogueGainEntry {
                Type: Some(RoguelikeGainDataType::CommonBuff),
                Index: reward_cnt,
                ConfigId: drop_id.max(1),
                IsSelect: true,
                IsNew: true,
                RestCount: 1,
                IsValid: true,
                IncId: reward_cnt,
                ..Default::default()
            }),
            ElementDict: BTreeMap::new(),
            Index: reward_cnt,
            GainDataUpdateType: Some(GainDataUpdateType::GainDataAdd),
        },
    );
    Ok(())
}

pub fn apply_dango_abyss_goto_next_floor<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let floor = record_progress_add(ctx, "abyss:floor", 1)?.max(1) as i32;
    let cur_inst = ctx.scene.as_deref().map_or(9300, |s| s.instance_id);
    let abyss_inst = ctx
        .assets
        .abyss_insts()
        .iter()
        .find(|a| a.InstId == cur_inst)
        .or_else(|| ctx.assets.abyss_insts().first())
        .cloned()
        .unwrap_or_default();

    let rooms = ctx.assets.abyss_rooms();
    let room_id = if !rooms.is_empty() {
        rooms[((floor as usize).saturating_sub(1)) % rooms.len()].Id
    } else {
        2100 + floor
    };
    let (gain_box, max_box, cur_score) = if let Some(pc) = ctx.progress_comp.as_deref_mut() {
        pc.info.set_key_int("abyss:room", i64::from(room_id));
        let gb = pc.info.get_key_int("abyss:gain_box").max(0) as i32;
        let mb = pc.info.get_key_int("abyss:reward_box").max(0) as i32;
        let sc = pc.info.get_key_int("abyss:score").max(0) as i32;
        pc.save(ctx.fs)?;
        (gb, mb, sc)
    } else {
        (0, 0, 0)
    };

    let (pos, yaw) = current_player_pos_and_yaw(ctx);
    ctx.sink.push(
        AbyssChallengeSubLevelNotify::MSG_ID,
        &AbyssChallengeSubLevelNotify {
            X: pos[0],
            Y: pos[1],
            Z: pos[2],
            A: yaw,
            RoomId: room_id,
            LoadSubLevels: Vec::new(),
            UnLoadSubLevels: Vec::new(),
        },
    );
    ctx.sink.push(
        AbyssChallengeRoomInfoNotify::MSG_ID,
        &AbyssChallengeRoomInfoNotify {
            MaxScore: abyss_inst.TotalScore.max(300),
            GainBox: gain_box,
            MaxFloor: 10.max(floor),
            RoomId: room_id,
            CurScore: cur_score,
            ChallengeId: abyss_inst.Id.max(1),
            CurFloor: floor,
            LikeRecord: Vec::new(),
            RoleEquipMap: BTreeMap::new(),
            RouteId: abyss_inst.RouteId,
            MaxBox: max_box,
        },
    );
    Ok(())
}

pub fn apply_dango_abyss_create_reward_box<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let box_cnt = record_progress_add(ctx, "abyss:reward_box", 1)?.max(1) as i32;
    let template_id = 221_800_015 + ((box_cnt - 1) % 3);
    let tpl_key = template_id.to_string();
    let (loc, rot) = resolve_spawn_anchor(ctx, 0);
    if let Some(scene) = ctx.scene.as_deref_mut()
        && let Some(nid) = crate::commands::entity::spawn_synthetic_template_entity(
            scene,
            ctx.fs,
            ctx.assets,
            ctx.sink,
            ctx.basic_comp,
            &tpl_key,
            i64::from(template_id),
            loc,
            rot,
        )
    {
        scene.summoned_entities.entry(tpl_key).or_default().push(nid);
    }
    push_abyss_room_info_notify(ctx);
    Ok(())
}

pub fn apply_dango_abyss_receive_reward<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let reward_id = ctx
        .assets
        .find_template_config("221800015")
        .and_then(|t| t.ComponentsData.RewardComponent.as_ref().and_then(|rc| rc.RewardId))
        .or_else(|| {
            ctx.assets
                .find_template_config("branch2.3_221_Treasure030")
                .and_then(|t| t.ComponentsData.RewardComponent.as_ref().and_then(|rc| rc.RewardId))
        })
        .unwrap_or(20_200_009);

    if reward_id > 0 {
        let mut rctx = ctx.make_reward_context();
        let _ = reward::grant_table_reward(&mut rctx, "Reward", i64::from(reward_id), false, true, 0, &[], None);
    }
    record_progress_add(ctx, "abyss:reward_cnt", 1)?;
    record_progress_add(ctx, "abyss:gain_box", 1)?;
    apply_destroy_self(ctx)?;
    push_abyss_room_info_notify(ctx);
    Ok(())
}

pub fn apply_mow_tower_goto_next_floor<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let floor = record_progress_add(ctx, "mow:floor", 1)?.max(1) as i32;
    let cur_inst = ctx.scene.as_deref().map_or(0, |s| s.instance_id);
    let saved_id = ctx
        .progress_comp
        .as_deref()
        .map_or(0, |pc| pc.info.get_key_int("mow:level_id") as i32);

    let level_opt = ctx
        .assets
        .mow_tower_levels()
        .iter()
        .find(|m| {
            (saved_id > 0 && m.Id == saved_id) || (cur_inst > 0 && m.InstIds.contains(&cur_inst))
        })
        .cloned();

    let Some(level) = level_opt else {
        return switch_team_to_second_roles(ctx);
    };
    let next_inst = resolve_next_half_inst(&level.InstIds, cur_inst, floor);
    if let Some(target_inst) = next_inst {
        switch_team_to_second_roles(ctx)?;
        if target_inst != cur_inst {
            transition_to_scene(ctx, target_inst, None, None)?;
        }
    } else {
        ctx.sink.push(
            MowTowerResultNotify::MSG_ID,
            &MowTowerResultNotify {
                LevelsId: level.Id,
                FirstTimerScore: 500,
                SecondTimerScore: 500,
                FirstKilMonsterScore: level.PassScore.max(500),
                SecondKilMonsterScore: level.PassScore.max(500),
                HighScore: (level.PassScore * 2).max(1000),
            },
        );
    }

    let first_roles = current_formation_role_ids(ctx);
    let second_roles = saved_second_roles(ctx);
    ctx.sink.push(
        MowTowerLevelsInfoUpdateNotify::MSG_ID,
        &MowTowerLevelsInfoUpdateNotify {
            MowTowerLevelsInfos: vec![MowTowerLevelsInfo {
                BabelTowerLevelId: level.Id,
                UnlockTime: 1,
                IsUnlock: true,
                FirstScore: level.PassScore.max(500),
                SecondScore: if next_inst.is_none() {
                    level.PassScore.max(500)
                } else {
                    0
                },
                LevelRewardStatus: BTreeMap::from([(1, MowTowerRewardStatus::CanReward)]),
                HardLevelBuffs: Vec::new(),
                FirstRoleSelection: first_roles,
                SecondRoleSelection: second_roles,
            }],
        },
    );
    Ok(())
}

pub fn apply_slash_and_tower_goto_next_floor<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let floor = record_progress_add(ctx, "slash:floor", 1)?.max(1) as i32;
    let cur_inst = ctx.scene.as_deref().map_or(0, |s| s.instance_id);
    let saved_id = ctx
        .progress_comp
        .as_deref()
        .map_or(0, |pc| pc.info.get_key_int("slash:level_id") as i32);

    let cfg_opt = ctx
        .assets
        .slash_and_tower_cfgs()
        .iter()
        .find(|c| {
            (saved_id > 0 && c.Id == saved_id) || (cur_inst > 0 && c.InstIds.contains(&cur_inst))
        })
        .cloned();

    let Some(cfg) = cfg_opt else {
        return switch_team_to_second_roles(ctx);
    };
    let next_inst = resolve_next_half_inst(&cfg.InstIds, cur_inst, floor);
    if let Some(target_inst) = next_inst {
        switch_team_to_second_roles(ctx)?;
        if target_inst != cur_inst {
            transition_to_scene(ctx, target_inst, None, None)?;
        }
    } else {
        let first_battle = BattleFormation {
            SelectRoles: current_formation_role_ids(ctx),
            BuffSelect: 0,
            SkillBranchIds: Vec::new(),
        };
        let second_battle = BattleFormation {
            SelectRoles: saved_second_roles(ctx),
            BuffSelect: 0,
            SkillBranchIds: saved_skill_branches(ctx),
        };
        ctx.sink.push(
            SlashAndTowerResultNotify::MSG_ID,
            &SlashAndTowerResultNotify {
                LevelsId: cfg.Id,
                FirstRoundScore: cfg.PassScore.max(500),
                SecondRoundScore: cfg.PassScore.max(500),
                FirstKillMonsterScore: cfg.PassScore.max(500),
                SecondKillMonsterScore: cfg.PassScore.max(500),
                FirstFormation: Some(first_battle),
                SecondFormation: Some(second_battle),
            },
        );
    }

    ctx.sink.push(
        SlashAndTowerLevelPlayNotify::MSG_ID,
        &SlashAndTowerLevelPlayNotify {
            SlashLevelPlayInfos: vec![SlashLevelPlayInfo {
                Id: cfg.Id,
                IsLocked: false,
                FirstScore: cfg.PassScore.max(500),
                SecondScore: if next_inst.is_none() {
                    cfg.PassScore.max(500)
                } else {
                    0
                },
                FirstBattle: None,
                SecondBattle: None,
                IsPassed: next_inst.is_none(),
                IsEasyPass: false,
            }],
        },
    );
    Ok(())
}

pub fn apply_boss_piling_enter_next_inst<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let step = record_progress_add(ctx, "bp:inst_step", 1)?.max(1) as i32;
    let cur_inst = ctx.scene.as_deref().map_or(0, |s| s.instance_id);
    let saved_id = ctx
        .progress_comp
        .as_deref()
        .map_or(0, |pc| pc.info.get_key_int("bp:level_id") as i32);

    let level_opt = ctx
        .assets
        .boss_piling_levels()
        .iter()
        .find(|b| {
            (saved_id > 0 && b.Id == saved_id) || (cur_inst > 0 && b.InstIds.contains(&cur_inst))
        })
        .cloned();

    let Some(level) = level_opt else {
        return Ok(());
    };
    if let Some(target_inst) = resolve_next_half_inst(&level.InstIds, cur_inst, step)
        && target_inst != cur_inst
    {
        transition_to_scene(ctx, target_inst, None, None)?;
    }

    let mut buffs = BTreeMap::new();
    for &bid in &level.BuffGroup {
        buffs.insert(bid, 1);
    }
    ctx.sink.push(
        BossPilingInstNotify::MSG_ID,
        &BossPilingInstNotify {
            BossPilingLevelId: level.Id,
            BossPilingBuffs: buffs,
        },
    );
    Ok(())
}

pub fn switch_team_to_second_roles<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    record_progress_add(ctx, "team:fight_change", 1)?;
    let second_roles = saved_second_roles(ctx);
    if second_roles.is_empty() {
        return Ok(());
    }
    let (Some(scene), Some(role_comp)) = (ctx.scene.as_deref_mut(), ctx.role_comp.as_deref_mut())
    else {
        return Ok(());
    };
    let desired_cur = second_roles[0];
    if let Err(err) = trial_roles::sync_formation_entities(
        scene,
        ctx.fs,
        ctx.assets,
        ctx.sink,
        ctx.events,
        role_comp,
        ctx.weapon_comp,
        ctx.echo_comp,
        &second_roles,
        desired_cur,
    ) {
        tracing::warn!("ChangeFightTeam sync_formation_entities failed: {err}");
    }
    Ok(())
}

pub fn apply_honami_story_interact_pickup<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    let pickup_idx = record_progress_add(ctx, "honami:pickup", 1)?.max(1) as i32;
    let items = ctx.assets.honami_items();
    let (act_id, item_id) = items
        .iter()
        .find(|it| it.ItemType == 1)
        .or_else(|| items.first())
        .map_or((106_200_001, 87_301_001), |it| (it.ActivityId, it.Id));

    let backpacks = ctx.assets.honami_backpacks();
    let bp = backpacks
        .iter()
        .find(|b| b.Type == 0)
        .or_else(|| backpacks.first())
        .cloned()
        .unwrap_or_default();

    if let Some(pc) = ctx.progress_comp.as_deref_mut() {
        pc.info.add_key_int(&format!("honami:item:{item_id}"), 1);
        pc.info.mark_claim(&format!("honami_col:{item_id}"));
        pc.save(ctx.fs)?;
    }

    ctx.sink.push(
        HonamiStoryBagInfoUpdateNotify::MSG_ID,
        &HonamiStoryBagInfoUpdateNotify {
            HonamiStoryBagInfos: vec![HonamiStoryBagInfo {
                BagConfigId: bp.Id.max(1),
                Width: bp.Wide.max(7),
                BagSize: bp.Size.first().copied().unwrap_or(385),
                HonamiStoryBagItemInfos: vec![HonamiStoryBagItemInfo {
                    HonamiStoryItemInfo: Some(HonamiStoryItemInfo {
                        ItemInfo: Some(HonamiStoryItemInfoItemInfoOneof::HonamiStoryNormalItemInfo(
                            HonamiStoryNormalItemInfo {},
                        )),
                        IncrId: pickup_idx,
                        ItemId: item_id,
                        FuncValue: act_id,
                    }),
                    HonamiStoryPosInfo: Some(HonamiStoryPosInfo {
                        IsCross: false,
                        Posotion: 0,
                    }),
                }],
            }],
        },
    );
    apply_destroy_self(ctx)?;
    Ok(())
}

pub fn apply_honami_story_activate_safe_evacuation<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    record_progress_int(ctx, "honami:safe_leave", 1, false)?;
    let act_id = ctx
        .progress_comp
        .as_deref()
        .map(|pc| pc.info.get_key_int("honami:act_id") as i32)
        .filter(|&id| id > 0)
        .or_else(|| ctx.assets.honami_items().first().map(|it| it.ActivityId))
        .unwrap_or(106_200_001);
    ctx.sink.push(
        HonamiStorySafeLeaveNotify::MSG_ID,
        &HonamiStorySafeLeaveNotify {
            ActivityId: act_id,
            CanSafeLeave: true,
        },
    );
    Ok(())
}

pub fn apply_rover_rogue_activate<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    params: &Map<String, Value>,
) -> io::Result<()> {
    let activate_seq = record_progress_add(ctx, "rover_rogue:activate", 1)?.max(1) as i32;
    let interact_ty = json_str(params.get("InteractType")).unwrap_or("Loot");
    record_progress_claim(ctx, &format!("rover_rogue:active_ty:{interact_ty}"))?;

    match interact_ty {
        "Shop" => {
            let goods: Vec<RoverRogueShopGood> = ctx
                .assets
                .rover_rogue_goods()
                .iter()
                .take(4)
                .enumerate()
                .map(|(i, g)| RoverRogueShopGood {
                    IncId: (i as i32) + 1,
                    ShopItemId: g.Id,
                    RuntimeParam: g.Param,
                    FinalPrice: g.Price.saturating_sub(g.Discount).max(0),
                    Price: g.Price,
                    IsBought: false,
                    ShowItemId: g.ShowItemId,
                })
                .collect();
            let refresh_cnt = ctx
                .progress_comp
                .as_deref()
                .map_or(0, |pc| pc.info.get_key_int("rover_rogue:shop_refresh") as i32);
            ctx.sink.push(
                RoverRogueShopUpdateNotify::MSG_ID,
                &RoverRogueShopUpdateNotify {
                    Snapshot: Some(RoverRogueShopSnapshot {
                        ShopConfigId: 1,
                        Goods: goods,
                        BoughtIncIds: Vec::new(),
                        RefreshCount: refresh_cnt,
                        NextRefreshCost: 50,
                        MaxRefreshCount: 5,
                    }),
                },
            );
        }
        "BlessGroup" => {
            let groups: Vec<RoverRogueBlessGroupData> = ctx
                .assets
                .rover_rogue_bless_groups()
                .iter()
                .take(3)
                .map(|bg| RoverRogueBlessGroupData {
                    BlessGroupId: bg.Id,
                    BlessIdList: vec![bg.BlessRoleId],
                })
                .collect();
            ctx.sink.push(
                RoverRogueBlessGroupSelectNotify::MSG_ID,
                &RoverRogueBlessGroupSelectNotify {
                    IsToken: false,
                    BlessGroupData: groups,
                },
            );
        }
        "Event" => {
            let choices = ctx
                .assets
                .rover_rogue_event_groups()
                .first()
                .map(|eg| eg.ChoicePool.iter().copied().take(eg.Num.max(2) as usize).collect())
                .unwrap_or_else(|| vec![1001, 1002]);
            ctx.sink.push(
                RoverRogueEventNotify::MSG_ID,
                &RoverRogueEventNotify {
                    EventIncId: activate_seq,
                    ChoiceList: choices,
                },
            );
        }
        _ => {
            let loot_id = ctx
                .assets
                .rover_rogue_loots()
                .iter()
                .nth((activate_seq as usize).saturating_sub(1))
                .or_else(|| ctx.assets.rover_rogue_loots().first())
                .map_or(1_000_000_001, |l| l.Id);
            if let Some(pc) = ctx.progress_comp.as_deref_mut() {
                pc.info.mark_claim(&format!("rover_rogue:loot:{loot_id}"));
                pc.save(ctx.fs)?;
            }
            ctx.sink.push(
                RoverRogueLootChangeNotify::MSG_ID,
                &RoverRogueLootChangeNotify {
                    EnableUse: true,
                    LootData: vec![RoverRogueGainEntry {
                        Type: Some(RoverRogueGainDataType::RoverRogueGainLootItem),
                        ConfigId: loot_id,
                        IncId: activate_seq,
                        ItemRemainingRooms: 1,
                        ..Default::default()
                    }],
                },
            );
        }
    }
    Ok(())
}

fn push_abyss_room_info_notify<S: PushSink + ?Sized>(ctx: &mut QuestContext<'_, S>) {
    let pc_opt = ctx.progress_comp.as_deref();
    let cur_inst = ctx.scene.as_deref().map_or(9300, |s| s.instance_id);
    let abyss_inst = ctx
        .assets
        .abyss_insts()
        .iter()
        .find(|a| a.InstId == cur_inst)
        .or_else(|| ctx.assets.abyss_insts().first())
        .cloned()
        .unwrap_or_default();
    let room_id = pc_opt.map_or(2101, |pc| pc.info.get_key_int("abyss:room").max(2101) as i32);
    let floor = pc_opt.map_or(1, |pc| pc.info.get_key_int("abyss:floor").max(1) as i32);
    let gain_box = pc_opt.map_or(0, |pc| pc.info.get_key_int("abyss:gain_box").max(0) as i32);
    let max_box = pc_opt.map_or(0, |pc| pc.info.get_key_int("abyss:reward_box").max(0) as i32);
    let cur_score = pc_opt.map_or(0, |pc| pc.info.get_key_int("abyss:score").max(0) as i32);
    ctx.sink.push(
        AbyssChallengeRoomInfoNotify::MSG_ID,
        &AbyssChallengeRoomInfoNotify {
            MaxScore: abyss_inst.TotalScore.max(300),
            GainBox: gain_box,
            MaxFloor: 10.max(floor),
            RoomId: room_id,
            CurScore: cur_score,
            ChallengeId: abyss_inst.Id.max(1),
            CurFloor: floor,
            LikeRecord: Vec::new(),
            RoleEquipMap: BTreeMap::new(),
            RouteId: abyss_inst.RouteId,
            MaxBox: max_box,
        },
    );
}

fn current_player_pos_and_yaw<S: PushSink + ?Sized>(ctx: &QuestContext<'_, S>) -> ([f32; 3], f32) {
    let Some(scene) = ctx.scene.as_deref() else {
        return ([0.0; 3], 0.0);
    };
    let nid = scene.current_role_entity_id();
    if let Some(pos) = scene.entity(nid).and_then(|s| s.position.as_ref()) {
        return (pos.location, pos.rotation[2]);
    }
    if let Some(p) = scene.instance.players.first() {
        return (p.location, p.rotation[2]);
    }
    ([0.0; 3], 0.0)
}

fn resolve_next_half_inst(inst_ids: &[i32], cur_inst: i32, step: i32) -> Option<i32> {
    if inst_ids.len() <= 1 {
        return None;
    }
    if let Some(pos) = inst_ids.iter().position(|&id| id == cur_inst) {
        return inst_ids.get(pos + 1).copied().filter(|&id| id > 0);
    }
    if step <= 1 {
        return inst_ids.get(1).copied().filter(|&id| id > 0);
    }
    None
}

fn saved_second_roles<S: PushSink + ?Sized>(ctx: &QuestContext<'_, S>) -> Vec<i32> {
    ctx.progress_comp
        .as_deref()
        .map(|pc| parse_csv_ids(pc.info.get_key_str("inst:second_roles")))
        .unwrap_or_default()
}

fn saved_skill_branches<S: PushSink + ?Sized>(ctx: &QuestContext<'_, S>) -> Vec<i32> {
    ctx.progress_comp
        .as_deref()
        .map(|pc| parse_csv_ids(pc.info.get_key_str("inst:skill_branches")))
        .unwrap_or_default()
}

fn current_formation_role_ids<S: PushSink + ?Sized>(ctx: &QuestContext<'_, S>) -> Vec<i32> {
    let Some(scene) = ctx.scene.as_deref() else {
        return Vec::new();
    };
    let idx = scene.formation_info.cur_formation as usize;
    scene
        .formation_info
        .formations
        .get(idx)
        .map(|f| f.roles.iter().flatten().map(|r| r.role_id).collect())
        .unwrap_or_default()
}
