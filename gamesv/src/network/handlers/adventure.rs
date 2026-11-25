use std::collections::BTreeMap;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::special_item_incr::TRIAL_INCR_MIN;
use sonoro_gamesv_logic::component::player::{
    PlayerBasicComponent, PlayerEchoComponent, PlayerMapComponent, PlayerProgressComponent,
    PlayerQuestComponent, PlayerRoleComponent, PlayerWeaponComponent,
};
use sonoro_gamesv_logic::helpers::{calabash, quest_engine, reward};
use sonoro_proto::enums::{AdventreTaskState, DetectionType, ErrorCode, SlientFirstAwardState};
use sonoro_proto::pb::{
    AddChapterResultRequest, AddChapterResultResponse, AdventreTask, AdventureDetectionConfig,
    AdventureItemData, AdventureManualData, AdventureManualDataRequest,
    AdventureManualDataResponse, AdventureManualRequest, AdventureManualResponse,
    AdventureRewardData, ChapterResultRewardRequest, ChapterResultRewardResponse, DetectionTarget,
    DetectionUnlock, GetAdventureRewardRequest, GetAdventureRewardResponse,
    GetChapterRewardRequest, GetChapterRewardResponse, MulGetAdventureRewardRequest,
    MulGetAdventureRewardResponse, NormalMonsterManualInfoRequest,
    NormalMonsterManualInfoResponse, ReceivedSilentFirstAwardRequest,
    ReceivedSilentFirstAwardResponse, WeeklyFrameworkInfo, WeeklyFrameworkInfoRequest,
    WeeklyFrameworkInfoResponse, WeeklyFrameworkScoreRewardRequest,
    WeeklyFrameworkScoreRewardResponse,
};

use super::State;

struct AdventureSnapshot<'a> {
    assets: &'a Assets,
    basic_comp: &'a PlayerBasicComponent,
    role_comp: &'a PlayerRoleComponent,
    weapon_comp: &'a PlayerWeaponComponent,
    echo_comp: &'a PlayerEchoComponent,
    map_comp: &'a PlayerMapComponent,
    quest_comp: &'a PlayerQuestComponent,
    progress_comp: &'a PlayerProgressComponent,
}

impl<'a> AdventureSnapshot<'a> {
    fn from_state(state: &'a State) -> Self {
        Self {
            assets: &state.assets,
            basic_comp: &state.player_components.basic,
            role_comp: &state.player_components.role,
            weapon_comp: &state.player_components.weapon,
            echo_comp: &state.player_components.echo,
            map_comp: &state.player_components.map,
            quest_comp: &state.player_components.quest,
            progress_comp: &state.player_components.progress,
        }
    }
}

fn count_roles_at_least_level(snap: &AdventureSnapshot<'_>, min_level: i32) -> i32 {
    snap.role_comp
        .role_map
        .iter()
        .filter(|&(&role_id, role)| {
            !snap.role_comp.is_trial_role(role_id)
                && (1000..2000).contains(&role_id)
                && role.level >= min_level
        })
        .count() as i32
}

fn count_weapons_at_least_level(snap: &AdventureSnapshot<'_>, min_level: i32) -> i32 {
    snap.weapon_comp
        .weapon_map
        .iter()
        .filter(|&(&incr_id, w)| incr_id < TRIAL_INCR_MIN && w.level >= min_level)
        .count() as i32
}

fn count_weapons_at_least_quality(snap: &AdventureSnapshot<'_>, min_quality: i32) -> i32 {
    snap.weapon_comp
        .weapon_map
        .iter()
        .filter(|&(&incr_id, w)| {
            incr_id < TRIAL_INCR_MIN
                && snap
                    .assets
                    .tables
                    .weapon_conf
                    .get_data_by_id(&w.id)
                    .is_some_and(|cfg| cfg.QualityId >= min_quality)
        })
        .count() as i32
}

fn count_roles_with_skill_level(
    snap: &AdventureSnapshot<'_>,
    min_skills: i32,
    min_level: i32,
) -> i32 {
    snap.role_comp
        .role_map
        .iter()
        .filter(|&(&role_id, role)| {
            if snap.role_comp.is_trial_role(role_id) || !(1000..2000).contains(&role_id) {
                return false;
            }
            let skills_met = role.skills.iter().filter(|&&(_, lv)| lv >= min_level).count() as i32;
            skills_met >= min_skills
        })
        .count() as i32
}

fn count_equipped_echoes(
    snap: &AdventureSnapshot<'_>,
    min_quality: i32,
    min_fetter_match: i32,
) -> i32 {
    let mut best_role_equipped = 0i32;
    for (&role_id, role) in &snap.role_comp.role_map {
        if snap.role_comp.is_trial_role(role_id) || !(1000..2000).contains(&role_id) {
            continue;
        }
        let mut equipped_count = 0i32;
        let mut fetter_ids = [0i32; 5];
        let mut fetter_counts = [0i32; 5];
        let mut fetter_len = 0usize;
        let mut has_2pc = false;

        for &(_, incr_id) in &role.phantom {
            if incr_id <= 0 {
                continue;
            }
            let Some(echo) = snap.echo_comp.echo_map.get(&incr_id) else {
                continue;
            };
            let Some(cfg) = snap.assets.tables.phantom_item.get_data_by_id(&echo.id) else {
                continue;
            };
            if cfg.QualityId < min_quality {
                continue;
            }
            equipped_count += 1;
            if echo.fetter_group_id > 0 {
                if let Some(pos) = fetter_ids[..fetter_len]
                    .iter()
                    .position(|&id| id == echo.fetter_group_id)
                {
                    fetter_counts[pos] += 1;
                    if fetter_counts[pos] >= 2 {
                        has_2pc = true;
                    }
                } else if fetter_len < fetter_ids.len() {
                    fetter_ids[fetter_len] = echo.fetter_group_id;
                    fetter_counts[fetter_len] = 1;
                    fetter_len += 1;
                }
            }
        }

        if (min_fetter_match <= 0 || has_2pc) && equipped_count > best_role_equipped {
            best_role_equipped = equipped_count;
        }
    }
    best_role_equipped
}

fn count_echoes(snap: &AdventureSnapshot<'_>, min_level: i32, min_quality: i32) -> i32 {
    snap.echo_comp
        .echo_map
        .iter()
        .filter(|&(&incr_id, echo)| {
            incr_id < TRIAL_INCR_MIN
                && echo.level >= min_level
                && snap
                    .assets
                    .tables
                    .phantom_item
                    .get_data_by_id(&echo.id)
                    .is_some_and(|cfg| cfg.QualityId >= min_quality)
        })
        .count() as i32
}

fn count_tuned_echoes(snap: &AdventureSnapshot<'_>) -> i32 {
    snap.echo_comp
        .echo_map
        .iter()
        .filter(|&(&incr_id, echo)| incr_id < TRIAL_INCR_MIN && !echo.sub_prop.is_empty())
        .count() as i32
}

fn count_discovered_echoes(snap: &AdventureSnapshot<'_>, min_quality: i32) -> i32 {
    snap.assets
        .tables
        .calabash_develop_reward
        .items
        .iter()
        .filter(|r| {
            r.IsShow
                && calabash::discovered_max_quality(snap.assets, snap.echo_comp, r.MonsterId)
                    >= min_quality
        })
        .count() as i32
}

fn has_teleporter(snap: &AdventureSnapshot<'_>, mark_id: i32) -> bool {
    snap.map_comp.info.unlocked_teleporters.contains(&mark_id)
}

fn count_teleporters_by_object_type(snap: &AdventureSnapshot<'_>, target_type: i32) -> i32 {
    snap.map_comp
        .info
        .unlocked_teleporters
        .iter()
        .filter(|&&tp_id| {
            snap.assets
                .tables
                .map_mark
                .get_data_by_id(&tp_id)
                .is_some_and(|mark| mark.ObjectType == target_type)
        })
        .count() as i32
}

fn has_completed_mark_relative(snap: &AdventureSnapshot<'_>, mark_id: i32) -> bool {
    let Some(mark) = snap.assets.tables.map_mark.get_data_by_id(&mark_id) else {
        return false;
    };
    mark.RelativeId > 0
        && snap
            .quest_comp
            .info
            .completed_level_plays
            .contains(&mark.RelativeId)
}

fn count_tower_stars(snap: &AdventureSnapshot<'_>, min_difficulty: i32) -> i32 {
    snap.progress_comp
        .info
        .challenge
        .tower_floors
        .iter()
        .filter(|fl| fl.difficulty >= min_difficulty)
        .map(|fl| fl.star)
        .sum()
}

fn evaluate_task_progress(snap: &AdventureSnapshot<'_>, task_id: i32) -> i32 {
    let quest_info = &snap.quest_comp.info;
    match task_id {
        // Chapter 1
        10010001 => count_roles_at_least_level(snap, 20),
        10010002 => count_weapons_at_least_quality(snap, 3),
        10010003 => count_weapons_at_least_level(snap, 20),
        10010004 => i32::from(has_teleporter(snap, 300404)),
        10010005 => i32::from(quest_info.has_finished_quest(139000029)),
        10010007 => (count_equipped_echoes(snap, 0, 0) / 3).min(1),

        // Chapter 2
        10020001 => count_roles_at_least_level(snap, 30),
        10020002 => count_weapons_at_least_level(snap, 30),
        10020003 => i32::from(has_teleporter(snap, 300601)),
        10020004 => i32::from(quest_info.has_finished_quest(139000030)),
        10020005 => i32::from(!quest_info.opened_chests.is_empty() || count_echoes(snap, 1, 1) > 0),
        10020006 => count_teleporters_by_object_type(snap, 5),
        10020007 => count_discovered_echoes(snap, 2),
        10020009 => {
            i32::from(has_completed_mark_relative(snap, 5021) || !quest_info.completed_level_plays.is_empty())
        }

        // Chapter 3
        10030001 => count_roles_at_least_level(snap, 30),
        10030002 => i32::from(has_teleporter(snap, 3005)),
        10030003 => count_echoes(snap, 10, 1),
        10030004 => i32::from(has_teleporter(snap, 300901)),
        10030005 => i32::from(quest_info.has_finished_quest(139000031)),
        10030006 => count_teleporters_by_object_type(snap, 8),
        10030007 => quest_info.completed_level_plays.len() as i32,
        10030008 => i32::from(!quest_info.completed_dungeons.is_empty()),
        10030009 => i32::from(count_equipped_echoes(snap, 1, 1) >= 2),

        // Chapter 4
        10040001 => count_roles_at_least_level(snap, 50),
        10040002 => count_weapons_at_least_level(snap, 50),
        10040003 => count_echoes(snap, 10, 1),
        10040004 => i32::from(has_teleporter(snap, 300701)),
        10040005 => count_echoes(snap, 0, 1),
        10040006 => {
            i32::from(has_completed_mark_relative(snap, 5001) || !quest_info.completed_level_plays.is_empty())
        }
        10040007 => quest_info.completed_dungeons.len() as i32,
        10040008 => i32::from(has_teleporter(snap, 3003)),
        10040009 => count_discovered_echoes(snap, 3),
        10040010 => count_roles_with_skill_level(snap, 1, 3),

        // Chapter 5
        10050001 => count_roles_at_least_level(snap, 50),
        10050002 => count_weapons_at_least_level(snap, 50),
        10050003 => count_echoes(snap, 15, 1),
        10050004 => count_roles_with_skill_level(snap, 2, 3),
        10050005 => i32::from(
            snap.basic_comp.info.attributes.cur_world_level >= 2
                || snap.map_comp.info.unlocked_fogs.len() >= 3,
        ),
        10050006 => count_discovered_echoes(snap, 4),
        10050007 => {
            i32::from(has_completed_mark_relative(snap, 1101) || quest_info.completed_level_plays.len() >= 2)
        }
        10050008 => i32::from(quest_info.completed_dungeons.len() >= 2),
        10050009 => count_echoes(snap, 0, 4),

        // Chapter 6
        10060001 => count_roles_at_least_level(snap, 60),
        10060002 => count_weapons_at_least_level(snap, 60),
        10060003 => count_echoes(snap, 20, 1),
        10060004 => count_echoes(snap, 0, 4),
        10060005 => count_discovered_echoes(snap, 4),
        10060006 => i32::from(snap.basic_comp.info.attributes.level >= 30),
        10060007 => quest_info.completed_level_plays.len() as i32,
        10060008 => count_roles_with_skill_level(snap, 2, 3),
        10060009 => {
            i32::from(has_completed_mark_relative(snap, 1102) || quest_info.completed_level_plays.len() >= 3)
        }

        // Chapter 7
        10070001 => count_roles_at_least_level(snap, 70),
        10070002 => count_echoes(snap, 25, 1),
        10070003 => count_echoes(snap, 0, 5),
        10070004 => i32::from(quest_info.has_finished_quest(125000124)),
        10070005 => calabash::current_level(snap.assets, snap.basic_comp, snap.echo_comp),
        10070006 => i32::from(
            quest_info.opened_chests.len() >= 10 || snap.map_comp.info.unlocked_fogs.len() >= 6,
        ),
        10070008 => {
            i32::from(has_completed_mark_relative(snap, 1403) || quest_info.completed_level_plays.len() >= 4)
        }

        // Chapter 8
        10080001 => count_roles_at_least_level(snap, 70),
        10080002 => count_roles_with_skill_level(snap, 3, 4),
        10080003 => count_echoes(snap, 25, 1),
        10080004 => i32::from(quest_info.has_finished_quest(145000004)),
        10080005 => calabash::current_level(snap.assets, snap.basic_comp, snap.echo_comp),
        10080006 => i32::from(
            quest_info.opened_chests.len() >= 20 || snap.map_comp.info.unlocked_fogs.len() >= 9,
        ),
        10080008 => {
            i32::from(has_completed_mark_relative(snap, 1503) || quest_info.completed_level_plays.len() >= 5)
        }

        // Chapter 9
        10090001 => count_roles_at_least_level(snap, 80),
        10090002 => count_roles_with_skill_level(snap, 3, 6),
        10090003 => count_tuned_echoes(snap) * 10,
        10090004 => i32::from(quest_info.has_finished_quest(114000027)),
        10090005 => calabash::current_level(snap.assets, snap.basic_comp, snap.echo_comp),
        10090006 => i32::from(
            quest_info.opened_chests.len() >= 30 || snap.map_comp.info.unlocked_fogs.len() >= 12,
        ),
        10090007 => {
            i32::from(has_completed_mark_relative(snap, 1603) || quest_info.completed_level_plays.len() >= 6)
        }
        10090008 => {
            i32::from(has_completed_mark_relative(snap, 1703) || quest_info.completed_level_plays.len() >= 7)
        }

        // Tower of Adversity
        10050010 => i32::from(
            !snap.progress_comp.info.challenge.tower_floors.is_empty()
                || quest_info.completed_dungeons.len() >= 3,
        ),
        10060010 => i32::from(
            snap.progress_comp.info.challenge.tower_floors.len() >= 3
                || quest_info.completed_dungeons.len() >= 4,
        ),
        10070007 => i32::from(
            count_tower_stars(snap, 0) >= 6
                || snap.progress_comp.info.challenge.tower_floors.len() >= 6,
        ),
        10080007 => i32::from(count_tower_stars(snap, 2) >= 6 || count_tower_stars(snap, 0) >= 12),
        _ => 0,
    }
}

fn append_reward_data_for_drop(
    assets: &Assets,
    rewards: &mut Vec<AdventureRewardData>,
    drop_id: i32,
) {
    if drop_id <= 0 || rewards.iter().any(|existing| existing.DropId == drop_id) {
        return;
    }
    let preview = reward::preview_drop_items(assets, drop_id);
    if preview.is_empty() {
        return;
    }
    let items = preview
        .into_iter()
        .map(|it| AdventureItemData {
            ItemId: it.item_id,
            ItemNum: it.count,
        })
        .collect();
    rewards.push(AdventureRewardData {
        DropId: drop_id,
        Items: items,
    });
}

fn build_manual_data(snap: &AdventureSnapshot<'_>) -> AdventureManualData {
    let mut tasks = Vec::new();
    let mut finished_per_chapter = [0i32; 16];

    for task in &snap.assets.tables.adventure_task.items {
        let progress = evaluate_task_progress(snap, task.Id).clamp(0, task.NeedProgress);
        let is_done = progress >= task.NeedProgress;
        if is_done && (1..finished_per_chapter.len() as i32).contains(&task.ChapterId) {
            finished_per_chapter[task.ChapterId as usize] += 1;
        }
        let ckey = format!("adv_task:{}", task.Id);
        let is_claimed = snap.progress_comp.info.has_claim(&ckey);
        let state = if is_claimed {
            AdventreTaskState::Received
        } else if is_done {
            AdventreTaskState::Finish
        } else {
            AdventreTaskState::UnFinish
        };
        tasks.push(AdventreTask {
            Id: task.Id,
            State: Some(state),
            AdventreProgress: progress,
        });
    }

    let mut unlock_chapters = vec![1i32];
    let mut now_chapter = 1i32;
    let mut check_chap = 2i32;
    while let Some(chap) = snap
        .assets
        .tables
        .adventure_task_chapter
        .get_data_by_id(&check_chap)
    {
        let prev_idx = (check_chap - 1) as usize;
        if prev_idx >= finished_per_chapter.len()
            || finished_per_chapter[prev_idx] < chap.LevelUnlockCount
        {
            break;
        }
        unlock_chapters.push(chap.Id);
        now_chapter = chap.Id;
        check_chap += 1;
    }

    let visible_tasks = tasks
        .into_iter()
        .filter(|t| {
            snap.assets
                .tables
                .adventure_task
                .get_data_by_id(&t.Id)
                .is_some_and(|cfg| unlock_chapters.contains(&cfg.ChapterId))
        })
        .collect();

    let mut reward_chapters = Vec::new();
    let mut received_chapter = 0i32;
    for chap in &snap.assets.tables.adventure_task_chapter.items {
        let rkey = format!("adv_chap:{}", chap.Id);
        if snap.progress_comp.info.has_claim(&rkey) {
            reward_chapters.push(chap.Id);
            if chap.Id > received_chapter {
                received_chapter = chap.Id;
            }
        }
    }

    AdventureManualData {
        AdventreTask: visible_tasks,
        NowChapter: now_chapter,
        ReceivedChapter: received_chapter,
        UnlockChapters: unlock_chapters,
        RewardChapters: reward_chapters,
    }
}

pub fn handle_adventure_manual_request(
    state: &mut State,
    _req: AdventureManualRequest,
) -> Result<AdventureManualResponse, String> {
    let snap = AdventureSnapshot::from_state(state);
    let mut targets = Vec::new();
    let mut rewards = Vec::new();
    let mut monster_unlocks = Vec::new();
    let mut dungeon_unlocks = Vec::new();
    let mut silent_unlocks = Vec::new();
    let mut dungeon_configs = Vec::new();
    let mut silent_configs = Vec::new();
    let mut silent_first_award = BTreeMap::new();

    for task in &snap.assets.tables.adventure_task.items {
        append_reward_data_for_drop(snap.assets, &mut rewards, task.DropIds);
    }
    for chap in &snap.assets.tables.adventure_task_chapter.items {
        append_reward_data_for_drop(snap.assets, &mut rewards, chap.DropIds);
    }

    for md in &snap.assets.tables.monster_detection.items {
        monster_unlocks.push(md.Id);
        targets.push(DetectionTarget {
            Id: md.Id,
            Type: Some(DetectionType::NormalMonster),
            UnlockState: true,
            DetectionId: md.Id,
            ..Default::default()
        });
    }
    for dd in &snap.assets.tables.dungeon_detection.items {
        dungeon_unlocks.push(dd.Id);
        targets.push(DetectionTarget {
            Id: dd.DungeonId,
            Type: Some(DetectionType::Dungeon),
            UnlockState: true,
            DetectionId: dd.Id,
            ..Default::default()
        });
        dungeon_configs.push(AdventureDetectionConfig {
            Id: dd.Id,
            EffectBeginTime: dd.BeginTimeStamp,
            EffectEndTime: 0,
        });
    }
    for sd in &snap.assets.tables.silent_area_detection.items {
        silent_unlocks.push(sd.Id);
        targets.push(DetectionTarget {
            Id: sd.Id,
            Type: Some(DetectionType::SilentArea),
            UnlockState: true,
            DetectionId: sd.Id,
            ..Default::default()
        });
        silent_configs.push(AdventureDetectionConfig {
            Id: sd.Id,
            EffectBeginTime: sd.BeginTimeStamp,
            EffectEndTime: 0,
        });
        let done = has_completed_mark_relative(&snap, sd.MarkId);
        silent_first_award.insert(
            sd.Id,
            if done {
                SlientFirstAwardState::IsFinish
            } else {
                SlientFirstAwardState::NotFinish
            },
        );
    }

    Ok(AdventureManualResponse {
        ErrorCode: Some(ErrorCode::Success),
        AdventureManualData: Some(build_manual_data(&snap)),
        DetectionTarget: targets,
        AdventureRewardData: rewards,
        DetectionUnlocks: Some(DetectionUnlock {
            MonsterDetectionIds: monster_unlocks,
            DungeonDetectionIds: dungeon_unlocks,
            SilentAreaDetectionIds: silent_unlocks,
        }),
        SlientFirstAwardMap: silent_first_award,
        SilenceAreaConfigs: silent_configs,
        DungeonDetections: dungeon_configs,
        ..Default::default()
    })
}

pub fn handle_adventure_manual_data_request(
    state: &mut State,
    _req: AdventureManualDataRequest,
) -> Result<AdventureManualDataResponse, String> {
    let snap = AdventureSnapshot::from_state(state);
    Ok(AdventureManualDataResponse {
        ErrorCode: Some(ErrorCode::Success),
        AdventureManualData: Some(build_manual_data(&snap)),
    })
}

pub fn handle_weekly_framework_info_request(
    state: &mut State,
    _req: WeeklyFrameworkInfoRequest,
) -> Result<WeeklyFrameworkInfoResponse, String> {
    let now_ms = quest_engine::tree::now_ms_pub();
    let progress_comp = &state.player_components.progress;
    let mut score_tasks: Vec<i32> = [1000, 2000, 3000, 4000, 5000, 6000]
        .into_iter()
        .filter(|tid| !progress_comp.info.has_claim(&format!("weekly_fw:{tid}")))
        .collect();
    if score_tasks.is_empty() {
        score_tasks = vec![1000, 2000, 3000, 4000, 5000, 6000];
    }
    Ok(WeeklyFrameworkInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        FrameworkInfo: Some(WeeklyFrameworkInfo {
            ConfigId: 1,
            BeginTime: now_ms - 86_400_000,
            EndTime: now_ms + 604_800_000,
            ScoreTasks: score_tasks,
            WorldLevel: state.player_components.basic.info.attributes.cur_world_level.max(1),
            ..Default::default()
        }),
    })
}

fn claim_single_adventure_task(state: &mut State, task_id: i32) -> Result<bool, String> {
    let k = format!("adv_task:{task_id}");
    if !state.player_components.progress.info.mark_claim(&k) {
        return Ok(false);
    }

    if let Some(cfg) = state
        .assets
        .tables
        .adventure_task
        .get_data_by_id(&task_id)
        .map(|r| r.into_owned())
    {
        if cfg.DropIds > 0 {
            let mut rctx = state.reward_context();
            reward::grant_drop_package_popup(&mut rctx, cfg.DropIds, 19500)
                .map_err(|e| e.to_string())?;
        }
    } else {
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "AdventureTaskV2",
            i64::from(task_id),
            false,
            true,
            19500,
            &[
                reward::RewardItem {
                    item_id: 3,
                    count: 40,
                },
                reward::RewardItem {
                    item_id: 1,
                    count: 100,
                },
            ],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(true)
}

pub fn handle_get_adventure_reward_request(
    state: &mut State,
    req: GetAdventureRewardRequest,
) -> Result<GetAdventureRewardResponse, String> {
    if claim_single_adventure_task(state, req.Id)? {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(GetAdventureRewardResponse {
        Id: req.Id,
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_mul_get_adventure_reward_request(
    state: &mut State,
    req: MulGetAdventureRewardRequest,
) -> Result<MulGetAdventureRewardResponse, String> {
    let mut claimed = Vec::new();
    let mut dirty = false;

    for task_id in req.Id {
        if claim_single_adventure_task(state, task_id)? {
            dirty = true;
            claimed.push(task_id);
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    Ok(MulGetAdventureRewardResponse {
        Code: Some(ErrorCode::Success),
        Id: claimed,
    })
}

pub fn handle_get_chapter_reward_request(
    state: &mut State,
    req: GetChapterRewardRequest,
) -> Result<GetChapterRewardResponse, String> {
    let chap_id = req.Chapter;
    let k = format!("adv_chap:{chap_id}");
    if state.player_components.progress.info.mark_claim(&k) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        if let Some(cfg) = state
            .assets
            .tables
            .adventure_task_chapter
            .get_data_by_id(&chap_id)
            .map(|r| r.into_owned())
        {
            if cfg.DropIds > 0 {
                let mut rctx = state.reward_context();
                reward::grant_drop_package_popup(&mut rctx, cfg.DropIds, 19500)
                    .map_err(|e| e.to_string())?;
            }
        } else {
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "AdventureTaskChapterV2",
                i64::from(chap_id),
                false,
                true,
                19500,
                &[reward::RewardItem {
                    item_id: 3,
                    count: 100,
                }],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(GetChapterRewardResponse {
        Chapter: chap_id,
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_add_chapter_result_request(
    state: &mut State,
    req: AddChapterResultRequest,
) -> Result<AddChapterResultResponse, String> {
    let k = format!("chap_res:{}:{}:{}", req.ActivityId, req.ChapterId, req.ResultId);
    let _ = state.player_components.progress.info.mark_claim(&k);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(AddChapterResultResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_chapter_result_reward_request(
    state: &mut State,
    req: ChapterResultRewardRequest,
) -> Result<ChapterResultRewardResponse, String> {
    let k = format!(
        "chap_res_clm:{}:{}:{}",
        req.ActivityId, req.ChapterId, req.ResultId
    );
    if state.player_components.progress.info.mark_claim(&k) {
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
                    count: 50,
                },
                reward::RewardItem {
                    item_id: 2,
                    count: 10000,
                },
            ],
            19500,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(ChapterResultRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_received_silent_first_award_request(
    state: &mut State,
    req: ReceivedSilentFirstAwardRequest,
) -> Result<ReceivedSilentFirstAwardResponse, String> {
    let k = format!("silent_first:{}", req.Id);
    let mut item_map = BTreeMap::new();
    if state.player_components.progress.info.mark_claim(&k) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut granted = Vec::new();
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "SilentAreaDetection",
            i64::from(req.Id),
            false,
            true,
            1,
            &[
                reward::RewardItem {
                    item_id: 3,
                    count: 40,
                },
                reward::RewardItem {
                    item_id: 2,
                    count: 10000,
                },
            ],
            Some(&mut granted),
        )
        .map_err(|e| e.to_string())?;
        for it in granted {
            item_map.insert(it.item_id, it.count);
        }
    }
    Ok(ReceivedSilentFirstAwardResponse {
        ErrorCode: Some(ErrorCode::Success),
        ItemMap: item_map,
        ..Default::default()
    })
}

pub fn handle_normal_monster_manual_info_request(
    state: &mut State,
    req: NormalMonsterManualInfoRequest,
) -> Result<NormalMonsterManualInfoResponse, String> {
    let targets = state
        .assets
        .tables
        .monster_detection
        .items
        .iter()
        .filter(|md| req.DetectionId <= 0 || md.Id == req.DetectionId)
        .map(|md| DetectionTarget {
            Id: md.Id,
            Type: Some(DetectionType::NormalMonster),
            UnlockState: true,
            DetectionId: md.Id,
            ..Default::default()
        })
        .collect();
    Ok(NormalMonsterManualInfoResponse {
        Code: Some(ErrorCode::Success),
        DetectionTarget: targets,
    })
}

pub fn handle_weekly_framework_score_reward_request(
    state: &mut State,
    req: WeeklyFrameworkScoreRewardRequest,
) -> Result<WeeklyFrameworkScoreRewardResponse, String> {
    let mut dirty = false;
    for tid in req.TaskIds {
        let k = format!("weekly_fw:{tid}");
        if !state.player_components.progress.info.mark_claim(&k) {
            continue;
        }
        dirty = true;
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "WeeklyFrameAward",
            i64::from(tid),
            false,
            true,
            19500,
            &[
                reward::RewardItem {
                    item_id: 3,
                    count: 80,
                },
                reward::RewardItem {
                    item_id: 2,
                    count: 20000,
                },
            ],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(WeeklyFrameworkScoreRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
