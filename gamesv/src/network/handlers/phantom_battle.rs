use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{pb_kv, quest_engine, reward};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    MulPhantomBattleMasterLevelRewardRequest, MulPhantomBattleMasterLevelRewardResponse,
    PhantomBattleBackLibraryRequest, PhantomBattleBackLibraryResponse,
    PhantomBattleBackSlotCardLibraryRequest, PhantomBattleBackSlotCardLibraryResponse,
    PhantomBattleBadgeRewardRequest, PhantomBattleBadgeRewardResponse,
    PhantomBattleBvBLoadingFinishRequest, PhantomBattleBvBLoadingFinishResponse,
    PhantomBattleBvBSettleNotify, PhantomBattleCallCardRequest, PhantomBattleCallCardResponse, PhantomBattleCardGroupAddRequest,
    PhantomBattleCardGroupAddResponse, PhantomBattleCardGroupDeleteRequest,
    PhantomBattleCardGroupDeleteResponse, PhantomBattleCardGroupInfo,
    PhantomBattleCardGroupNameRequest, PhantomBattleCardGroupNameResponse,
    PhantomBattleCardGroupUpdateRequest, PhantomBattleCardGroupUpdateResponse,
    PhantomBattleCardOutLookUpRequest, PhantomBattleCardOutLookUpResponse,
    PhantomBattleCardRewardRequest, PhantomBattleCardRewardResponse,
    PhantomBattleCardRoleSkillRequest, PhantomBattleCardRoleSkillResponse,
    PhantomBattleCardRoleTargetInfoRequest, PhantomBattleCardRoleTargetInfoResponse,
    PhantomBattleCardSelectRequest, PhantomBattleCardSelectResponse,
    PhantomBattleCardSkillRequest, PhantomBattleCardSkillResponse,
    PhantomBattleCardTargetInfoRequest, PhantomBattleCardTargetInfoResponse,
    PhantomBattleCardUnlockRequest, PhantomBattleCardUnlockResponse,
    PhantomBattleCheckCardSkillUnlockRequest, PhantomBattleCheckCardSkillUnlockResponse,
    PhantomBattleDealCardReplaceRequest, PhantomBattleDealCardReplaceResponse,
    PhantomBattleDiscardCardRequest, PhantomBattleDiscardCardResponse,
    PhantomBattleEnterSlotRequest, PhantomBattleEnterSlotResponse, PhantomBattleEvolveRequest,
    PhantomBattleEvolveResponse, PhantomBattleFighterInfo, PhantomBattleGuideRewardRequest,
    PhantomBattleGuideRewardResponse, PhantomBattleHandCardInfo, PhantomBattleLeaveSlotRequest,
    PhantomBattleLeaveSlotResponse, PhantomBattleNpcShowOverRequest,
    PhantomBattleNpcShowOverResponse, PhantomBattleReChallengeRequest,
    PhantomBattleReChallengeResponse, PhantomBattleReconstructCardRequest,
    PhantomBattleReconstructCardResponse, PhantomBattleRepeatedTaskRewardRequest,
    PhantomBattleRepeatedTaskRewardResponse, PhantomBattleRoleRewardRequest,
    PhantomBattleRoleRewardResponse, PhantomBattleRoundOverRequest,
    PhantomBattleRoundOverResponse, PhantomBattleSelectReserveCardRequest,
    PhantomBattleSelectReserveCardResponse, PhantomBattleSelectTargetRequest,
    PhantomBattleSelectTargetResponse, PhantomBattleSkillTriggerInfo,
    PhantomBattleSlotCardSkillRequest, PhantomBattleSlotCardSkillResponse,
    PhantomBattleSlotInsteadRequest, PhantomBattleSlotInsteadResponse,
    PhantomBattleStartRequest, PhantomBattleStartResponse, PhantomBattleTaskRewardRequest,
    PhantomBattleTaskRewardResponse,
};

use super::State;

fn format_ids_csv(ids: &[i32]) -> String {
    ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_ids_csv(csv: &str) -> Vec<i32> {
    csv.split(',')
        .filter_map(|tok| tok.parse::<i32>().ok())
        .collect()
}

pub fn handle_phantom_battle_card_group_add_request(
    state: &mut State,
    req: PhantomBattleCardGroupAddRequest,
) -> Result<PhantomBattleCardGroupAddResponse, String> {
    let info = &mut state.player_components.progress.info;
    let cnt_key = format!("pb_deck_cnt:{}", req.ActivityId);
    let idx = info.get_key_int(&cnt_key) as i32;
    info.set_key_int(&cnt_key, i64::from(idx + 1));
    info.set_key_str(
        &format!("pb_deck:{}:{idx}:name", req.ActivityId),
        &String::from_utf8_lossy(&req.Name),
    );
    info.set_key_str(
        &format!("pb_deck:{}:{idx}:cards", req.ActivityId),
        &format_ids_csv(&req.EquipCardIds),
    );
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhantomBattleCardGroupAddResponse {
        ErrorCode: Some(ErrorCode::Success),
        PhantomBattleCardGroupInfo: Some(PhantomBattleCardGroupInfo {
            Name: req.Name,
            EquipCardIds: req.EquipCardIds,
            CanUse: true,
            Index: idx,
            ..Default::default()
        }),
    })
}

pub fn handle_phantom_battle_card_group_update_request(
    state: &mut State,
    req: PhantomBattleCardGroupUpdateRequest,
) -> Result<PhantomBattleCardGroupUpdateResponse, String> {
    let info = &mut state.player_components.progress.info;
    let nkey = format!("pb_deck:{}:{}:name", req.ActivityId, req.Index);
    let ckey = format!("pb_deck:{}:{}:cards", req.ActivityId, req.Index);
    info.set_key_str(&ckey, &format_ids_csv(&req.EquipCardIds));
    let name = info.get_key_str(&nkey).as_bytes().to_vec();
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhantomBattleCardGroupUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
        PhantomBattleCardGroupInfo: Some(PhantomBattleCardGroupInfo {
            Name: name,
            EquipCardIds: req.EquipCardIds,
            CanUse: true,
            Index: req.Index,
            ..Default::default()
        }),
    })
}

pub fn handle_phantom_battle_card_group_name_request(
    state: &mut State,
    req: PhantomBattleCardGroupNameRequest,
) -> Result<PhantomBattleCardGroupNameResponse, String> {
    let info = &mut state.player_components.progress.info;
    let nkey = format!("pb_deck:{}:{}:name", req.ActivityId, req.Index);
    let ckey = format!("pb_deck:{}:{}:cards", req.ActivityId, req.Index);
    info.set_key_str(&nkey, &String::from_utf8_lossy(&req.Name));
    let cards = parse_ids_csv(info.get_key_str(&ckey));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhantomBattleCardGroupNameResponse {
        ErrorCode: Some(ErrorCode::Success),
        PhantomBattleCardGroupInfo: Some(PhantomBattleCardGroupInfo {
            Name: req.Name,
            EquipCardIds: cards,
            CanUse: true,
            Index: req.Index,
            ..Default::default()
        }),
    })
}

pub fn handle_phantom_battle_card_group_delete_request(
    state: &mut State,
    req: PhantomBattleCardGroupDeleteRequest,
) -> Result<PhantomBattleCardGroupDeleteResponse, String> {
    let info = &mut state.player_components.progress.info;
    let nkey = format!("pb_deck:{}:{}:name", req.ActivityId, req.Index);
    let ckey = format!("pb_deck:{}:{}:cards", req.ActivityId, req.Index);
    info.set_key_str(&nkey, "");
    info.set_key_str(&ckey, "");
    let cnt = info.get_key_int(&format!("pb_deck_cnt:{}", req.ActivityId)) as i32;
    let mut list = Vec::new();
    for i in 0..cnt {
        let csv = info.get_key_str(&format!("pb_deck:{}:{i}:cards", req.ActivityId));
        if csv.is_empty() {
            continue;
        }
        let name = info
            .get_key_str(&format!("pb_deck:{}:{i}:name", req.ActivityId))
            .as_bytes()
            .to_vec();
        list.push(PhantomBattleCardGroupInfo {
            Name: name,
            EquipCardIds: parse_ids_csv(csv),
            CanUse: true,
            Index: i,
            ..Default::default()
        });
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhantomBattleCardGroupDeleteResponse {
        ErrorCode: Some(ErrorCode::Success),
        LastUseCardGroupIndex: 0,
        PhantomBattleCardGroupInfo: list,
    })
}

pub fn handle_phantom_battle_card_unlock_request(
    state: &mut State,
    req: PhantomBattleCardUnlockRequest,
) -> Result<PhantomBattleCardUnlockResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("pb_card:{}", req.CardId),
    )
    .map_err(|e| e.to_string())?;
    Ok(PhantomBattleCardUnlockResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_card_out_look_up_request(
    state: &mut State,
    req: PhantomBattleCardOutLookUpRequest,
) -> Result<PhantomBattleCardOutLookUpResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("pb:card_outlook:{}", req.CardId),
    )
    .map_err(|e| e.to_string())?;
    Ok(PhantomBattleCardOutLookUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_check_card_skill_unlock_request(
    _state: &mut State,
    req: PhantomBattleCheckCardSkillUnlockRequest,
) -> Result<PhantomBattleCheckCardSkillUnlockResponse, String> {
    let n = req.CardConfig.len() as i32;
    Ok(PhantomBattleCheckCardSkillUnlockResponse {
        IsUnlock: true,
        CardConfig: req.CardConfig,
        ErrorCode: Some(ErrorCode::Success),
        CurNum: n,
        TargetNum: n,
    })
}

pub fn handle_phantom_battle_start_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: PhantomBattleStartRequest,
) -> Result<PhantomBattleStartResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("pb:round", 1);
    info.set_key_int("pb:lib_num", 20);
    info.set_key_int("pb:hand_num", 5);
    info.mark_claim("bvb:event:WaitForGameBegin");
    info.set_key_str("bvb:last_event", "WaitForGameBegin");
    let active_ch = info.get_key_int("pb:challenge");
    if active_ch > 0 {
        info.mark_claim(&format!("pb:challenge:{active_ch}"));
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    }
    Ok(PhantomBattleStartResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_bvb_loading_finish_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: PhantomBattleBvBLoadingFinishRequest,
) -> Result<PhantomBattleBvBLoadingFinishResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("pb:bvb_loaded", 1);
    info.mark_claim("bvb:event:WaitForGameBegin");
    info.mark_claim("bvb:event:PlayerTurnStart");
    info.set_key_str("bvb:last_event", "PlayerTurnStart");
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    }
    Ok(PhantomBattleBvBLoadingFinishResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_npc_show_over_request(
    state: &mut State,
    _req: PhantomBattleNpcShowOverRequest,
) -> Result<PhantomBattleNpcShowOverResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "pb:npc_shown",
        1,
    )
    .map_err(|e| e.to_string())?;
    Ok(PhantomBattleNpcShowOverResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_re_challenge_request(
    state: &mut State,
    req: PhantomBattleReChallengeRequest,
) -> Result<PhantomBattleReChallengeResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("pb:challenge", i64::from(req.ChallengeId));
    info.set_key_int("pb:round", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhantomBattleReChallengeResponse {
        ChallengeId: req.ChallengeId,
        CardGroupId: 0,
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_phantom_battle_round_over_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: PhantomBattleRoundOverRequest,
) -> Result<PhantomBattleRoundOverResponse, String> {
    let round = state
        .player_components
        .progress
        .info
        .add_key_int("pb:round", 1);
    let active_ch = state
        .player_components
        .progress
        .info
        .get_key_int("pb:challenge") as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let is_final = round >= 3 || active_ch > 0;
    let _ = state.conn.push_msg(
        PhantomBattleBvBSettleNotify::MSG_ID,
        &PhantomBattleBvBSettleNotify {
            NpcOriginalDamage: 10,
            NpcCurLife: if is_final { 0 } else { 10 },
            IsFinalSettle: is_final,
            IsWin: is_final,
            PlayerDeductLife: 0,
            PlayerOriginalDamage: 0,
            NpcDeductLife: 10,
            PlayerCurLife: 30,
        },
    );
    if is_final && active_ch > 0 {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_bvb_finish(&mut qctx, active_ch);
    }
    Ok(PhantomBattleRoundOverResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_enter_slot_request(
    state: &mut State,
    req: PhantomBattleEnterSlotRequest,
) -> Result<PhantomBattleEnterSlotResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("pb:slot:{}", req.SlotIndex),
        i64::from(req.CardUid),
    )
    .map_err(|e| e.to_string())?;
    Ok(PhantomBattleEnterSlotResponse {
        ErrorCode: Some(ErrorCode::Success),
        PhantomBattleFighterInfo: Some(PhantomBattleFighterInfo {
            UId: req.CardUid,
            ..Default::default()
        }),
    })
}

pub fn handle_phantom_battle_leave_slot_request(
    state: &mut State,
    req: PhantomBattleLeaveSlotRequest,
) -> Result<PhantomBattleLeaveSlotResponse, String> {
    let key = format!("pb:slot:{}", req.SlotIndex);
    let uid = state.player_components.progress.info.get_key_int(&key) as i32;
    pb_kv::persist_int(&state.fs, &mut state.player_components.progress, &key, 0)
        .map_err(|e| e.to_string())?;
    Ok(PhantomBattleLeaveSlotResponse {
        ErrorCode: Some(ErrorCode::Success),
        PhantomBattleHandCardInfo: Some(PhantomBattleHandCardInfo {
            CardUid: uid,
            CanUse: true,
            ..Default::default()
        }),
    })
}

pub fn handle_phantom_battle_slot_instead_request(
    state: &mut State,
    req: PhantomBattleSlotInsteadRequest,
) -> Result<PhantomBattleSlotInsteadResponse, String> {
    let skey = format!("pb:slot:{}", req.StartSlotIndex);
    let tkey = format!("pb:slot:{}", req.TargetSlotIndex);
    let info = &mut state.player_components.progress.info;
    let sv = info.get_key_int(&skey);
    let tv = info.get_key_int(&tkey);
    info.set_key_int(&skey, tv);
    info.set_key_int(&tkey, sv);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhantomBattleSlotInsteadResponse {
        StartSlotIndex: req.StartSlotIndex,
        TargetSlotIndex: req.TargetSlotIndex,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_evolve_request(
    state: &mut State,
    req: PhantomBattleEvolveRequest,
) -> Result<PhantomBattleEvolveResponse, String> {
    let lib_num = state
        .player_components
        .progress
        .info
        .get_key_int("pb:lib_num")
        .max(0) as i32;
    Ok(PhantomBattleEvolveResponse {
        ErrorCode: Some(ErrorCode::Success),
        PhantomBattleFighterInfo: Some(PhantomBattleFighterInfo {
            UId: req.CardUid,
            ..Default::default()
        }),
        CardLibraryNum: lib_num,
        ..Default::default()
    })
}

pub fn handle_phantom_battle_back_library_request(
    state: &mut State,
    req: PhantomBattleBackLibraryRequest,
) -> Result<PhantomBattleBackLibraryResponse, String> {
    let lib_num = state
        .player_components
        .progress
        .info
        .add_key_int("pb:lib_num", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhantomBattleBackLibraryResponse {
        CardUid: req.CardUid,
        CurCardLibraryNum: lib_num,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_back_slot_card_library_request(
    state: &mut State,
    req: PhantomBattleBackSlotCardLibraryRequest,
) -> Result<PhantomBattleBackSlotCardLibraryResponse, String> {
    let lib_num = state
        .player_components
        .progress
        .info
        .add_key_int("pb:lib_num", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhantomBattleBackSlotCardLibraryResponse {
        CardUid: req.CardUid,
        CurCardLibraryNum: lib_num,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_call_card_request(
    state: &mut State,
    events: &mut EventQueue,
    req: PhantomBattleCallCardRequest,
) -> Result<PhantomBattleCallCardResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.mark_claim("bvb:event:SelectCardsEnd");
    info.set_key_str("bvb:last_event", "SelectCardsEnd");
    if !req.SelectedCardUid.is_empty() {
        let count = req.SelectedCardUid.len() as i64;
        info.set_key_int("bvb:last_cost", count.min(8));
        info.set_key_int("bvb:can_evolve", 1);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    }
    let fighters = req
        .SelectedCardUid
        .into_iter()
        .map(|uid| PhantomBattleFighterInfo {
            UId: uid,
            ..Default::default()
        })
        .collect();
    Ok(PhantomBattleCallCardResponse {
        PhantomBattleFighterInfo: fighters,
        EffectUid: req.EffectUid,
        HeapNum: state
            .player_components
            .progress
            .info
            .get_key_int("pb:lib_num")
            .max(0) as i32,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_reconstruct_card_request(
    state: &mut State,
    req: PhantomBattleReconstructCardRequest,
) -> Result<PhantomBattleReconstructCardResponse, String> {
    Ok(PhantomBattleReconstructCardResponse {
        ErrorCode: Some(ErrorCode::Success),
        HeapNum: state
            .player_components
            .progress
            .info
            .get_key_int("pb:lib_num")
            .max(0) as i32,
        EffectUid: req.EffectUid,
        SelectedCardUid: req.SelectedCardUid,
    })
}

pub fn handle_phantom_battle_card_select_request(
    state: &mut State,
    req: PhantomBattleCardSelectRequest,
) -> Result<PhantomBattleCardSelectResponse, String> {
    let hand = req
        .CardUid
        .into_iter()
        .map(|uid| PhantomBattleHandCardInfo {
            CardUid: uid,
            CanUse: true,
            ..Default::default()
        })
        .collect();
    Ok(PhantomBattleCardSelectResponse {
        Error: Some(ErrorCode::Success),
        PhantomBattleHandCardInfos: hand,
        CurCardLibraryNum: state
            .player_components
            .progress
            .info
            .get_key_int("pb:lib_num")
            .max(0) as i32,
    })
}

pub fn handle_phantom_battle_deal_card_replace_request(
    state: &mut State,
    req: PhantomBattleDealCardReplaceRequest,
) -> Result<PhantomBattleDealCardReplaceResponse, String> {
    let hand = req
        .ReplaceCardUid
        .into_iter()
        .map(|uid| PhantomBattleHandCardInfo {
            CardUid: uid,
            CanUse: true,
            ..Default::default()
        })
        .collect();
    Ok(PhantomBattleDealCardReplaceResponse {
        CurCardLibraryNum: state
            .player_components
            .progress
            .info
            .get_key_int("pb:lib_num")
            .max(0) as i32,
        ErrorCode: Some(ErrorCode::Success),
        PhantomBattleHandCardInfos: hand,
    })
}

pub fn handle_phantom_battle_discard_card_request(
    state: &mut State,
    req: PhantomBattleDiscardCardRequest,
) -> Result<PhantomBattleDiscardCardResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(PhantomBattleDiscardCardResponse {
        DiscardCardIds: req.DiscardCardIds,
        HandCardNum: pc.get_key_int("pb:hand_num").max(0) as i32,
        ErrorCode: Some(ErrorCode::Success),
        CurCardLibraryNum: pc.get_key_int("pb:lib_num").max(0) as i32,
    })
}

pub fn handle_phantom_battle_select_reserve_card_request(
    state: &mut State,
    req: PhantomBattleSelectReserveCardRequest,
) -> Result<PhantomBattleSelectReserveCardResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(PhantomBattleSelectReserveCardResponse {
        ReserveCardUIds: req.ReserveCardUIds,
        CurCardLibraryNum: pc.get_key_int("pb:lib_num").max(0) as i32,
        ErrorCode: Some(ErrorCode::Success),
        HandCardNum: pc.get_key_int("pb:hand_num").max(0) as i32,
    })
}

pub fn handle_phantom_battle_card_skill_request(
    state: &mut State,
    req: PhantomBattleCardSkillRequest,
) -> Result<PhantomBattleCardSkillResponse, String> {
    Ok(PhantomBattleCardSkillResponse {
        CardUId: req.CardUId,
        ErrorCode: Some(ErrorCode::Success),
        LibraryCount: state
            .player_components
            .progress
            .info
            .get_key_int("pb:lib_num")
            .max(0) as i32,
        PhantomBattleSkillTriggerInfo: Some(PhantomBattleSkillTriggerInfo {
            SourceFighterUId: req.CardUId,
            SelectFighterUId: req.SelectFighterUId,
            ..Default::default()
        }),
    })
}

pub fn handle_phantom_battle_slot_card_skill_request(
    state: &mut State,
    req: PhantomBattleSlotCardSkillRequest,
) -> Result<PhantomBattleSlotCardSkillResponse, String> {
    Ok(PhantomBattleSlotCardSkillResponse {
        ErrorCode: Some(ErrorCode::Success),
        LibraryCount: state
            .player_components
            .progress
            .info
            .get_key_int("pb:lib_num")
            .max(0) as i32,
        CardUId: req.CardUId,
        SkillId: req.SkillId,
        PhantomBattleSkillTriggerInfo: Some(PhantomBattleSkillTriggerInfo {
            SourceFighterUId: req.CardUId,
            SkillId: req.SkillId,
            SelectFighterUId: req.SelectFighterUId,
            ..Default::default()
        }),
    })
}

pub fn handle_phantom_battle_card_role_skill_request(
    _state: &mut State,
    req: PhantomBattleCardRoleSkillRequest,
) -> Result<PhantomBattleCardRoleSkillResponse, String> {
    Ok(PhantomBattleCardRoleSkillResponse {
        ErrorCode: Some(ErrorCode::Success),
        PhantomBattleSkillTriggerInfo: Some(PhantomBattleSkillTriggerInfo {
            SkillId: req.SkillId,
            SelectFighterUId: req.SelectFighterUId,
            ..Default::default()
        }),
    })
}

pub fn handle_phantom_battle_card_target_info_request(
    _state: &mut State,
    req: PhantomBattleCardTargetInfoRequest,
) -> Result<PhantomBattleCardTargetInfoResponse, String> {
    let uids = if req.CardUId > 0 {
        vec![req.CardUId]
    } else {
        Vec::new()
    };
    let num = uids.len().max(1) as i32;
    Ok(PhantomBattleCardTargetInfoResponse {
        CardUId: req.CardUId,
        EffectType: 1,
        SkillId: req.SkillId,
        FighterUId: uids,
        Num: num,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_card_role_target_info_request(
    state: &mut State,
    req: PhantomBattleCardRoleTargetInfoRequest,
) -> Result<PhantomBattleCardRoleTargetInfoResponse, String> {
    let ch = state
        .player_components
        .progress
        .info
        .get_key_int("pb:challenge")
        .max(0) as i32;
    let uids = if ch > 0 { vec![ch] } else { Vec::new() };
    let num = uids.len().max(1) as i32;
    Ok(PhantomBattleCardRoleTargetInfoResponse {
        EffectType: 1,
        SkillId: req.SkillId,
        FighterUId: uids,
        Num: num,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_select_target_request(
    state: &mut State,
    req: PhantomBattleSelectTargetRequest,
) -> Result<PhantomBattleSelectTargetResponse, String> {
    let first_uid = req.FighterUId.first().copied().unwrap_or(0);
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "pb:target_uid",
        i64::from(first_uid),
    )
    .map_err(|e| e.to_string())?;
    Ok(PhantomBattleSelectTargetResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn claim_pb_reward_ids(
    state: &mut State,
    prefix: &str,
    table: &str,
    ids: &[i32],
) -> Result<(), String> {
    for &id in ids {
        if state
            .player_components
            .progress
            .info
            .mark_claim(&format!("{prefix}:{id}"))
        {
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                table,
                i64::from(id),
                false,
                true,
                1,
                &[],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())
}

pub fn handle_phantom_battle_badge_reward_request(
    state: &mut State,
    req: PhantomBattleBadgeRewardRequest,
) -> Result<PhantomBattleBadgeRewardResponse, String> {
    claim_pb_reward_ids(
        state,
        "pb_badge",
        "PhantomBattleBadgeReward",
        &req.BadgeRewardId,
    )?;
    Ok(PhantomBattleBadgeRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_card_reward_request(
    state: &mut State,
    req: PhantomBattleCardRewardRequest,
) -> Result<PhantomBattleCardRewardResponse, String> {
    claim_pb_reward_ids(
        state,
        "pb_card_rw",
        "PhantomBattleCardReward",
        &req.CardRewardId,
    )?;
    Ok(PhantomBattleCardRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_guide_reward_request(
    state: &mut State,
    _req: PhantomBattleGuideRewardRequest,
) -> Result<PhantomBattleGuideRewardResponse, String> {
    claim_pb_reward_ids(state, "pb_guide_rw", "PhantomBattleCardReward", &[1])?;
    Ok(PhantomBattleGuideRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_task_reward_request(
    state: &mut State,
    req: PhantomBattleTaskRewardRequest,
) -> Result<PhantomBattleTaskRewardResponse, String> {
    claim_pb_reward_ids(state, "pb_task", "PhantomBattleTask", &[req.TaskId])?;
    Ok(PhantomBattleTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_repeated_task_reward_request(
    state: &mut State,
    req: PhantomBattleRepeatedTaskRewardRequest,
) -> Result<PhantomBattleRepeatedTaskRewardResponse, String> {
    claim_pb_reward_ids(state, "pb_rtask", "PhantomBattleTask", &req.TaskId)?;
    Ok(PhantomBattleRepeatedTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_battle_role_reward_request(
    state: &mut State,
    req: PhantomBattleRoleRewardRequest,
) -> Result<PhantomBattleRoleRewardResponse, String> {
    claim_pb_reward_ids(
        state,
        "pb_role_rw",
        "PhantomBattleCardRole",
        &[req.PhantomBattleRoleId],
    )?;
    Ok(PhantomBattleRoleRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_mul_phantom_battle_master_level_reward_request(
    state: &mut State,
    req: MulPhantomBattleMasterLevelRewardRequest,
) -> Result<MulPhantomBattleMasterLevelRewardResponse, String> {
    claim_pb_reward_ids(
        state,
        "pb_master",
        "PhantomBattleMasterLevel",
        &req.PhantomBattleMasterLevelId,
    )?;
    Ok(MulPhantomBattleMasterLevelRewardResponse {
        PhantomBattleMasterLevelId: req.PhantomBattleMasterLevelId,
        ErrorCode: Some(ErrorCode::Success),
    })
}
