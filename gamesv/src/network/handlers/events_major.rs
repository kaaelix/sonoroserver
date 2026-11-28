use sonoro_gamesv_data::tables::entity_components::extra::ExtraEntry;
use sonoro_gamesv_logic::coop_service::CoopService;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::level_play::LevelPlayEventKind;
use sonoro_gamesv_logic::helpers::{energy, level_play, pb_kv, reward};
use sonoro_proto::enums::{
    ErrorCode, FloroRanchPlayTaskType, JokerActionType, JokerGuessActor, JokerGuessSkillOp,
    RacingBetsRankStatus,
};
use sonoro_proto::pb::{
    roleInfo, DangoBrawlBagOperateRequest, DangoBrawlBagOperateResponse,
    DangoBrawlEventRefreshRequest,
    DangoBrawlEventRefreshResponse, DangoBrawlEventSelectRequest, DangoBrawlEventSelectResponse,
    DangoBrawlEventSlotInfo, DangoBrawlFightOverRequest, DangoBrawlFightOverResponse,
    DangoBrawlScoreRewardRequest, DangoBrawlScoreRewardResponse, DangoBrawlSettleRequest,
    DangoBrawlSettleResponse, DangoBrawlShopBuyRequest, DangoBrawlShopBuyResponse,
    DangoBrawlShopComposeRequest, DangoBrawlShopComposeResponse, DangoBrawlShopLockRequest,
    DangoBrawlShopLockResponse, DangoBrawlShopRefreshRequest, DangoBrawlShopRefreshResponse,
    DangoBrawlShopSellRequest, DangoBrawlShopSellResponse, DangoBrawlShopInfo,
    DangoBrawlStartFightRequest, DangoBrawlStartFightResponse, DangoBrawlTaskRewardRequest,
    DangoBrawlTaskRewardResponse, DangoMonopolyBoardRewardRequest, DangoMonopolyBoardRewardResponse,
    DangoMonopolyDiceRequest, DangoMonopolyDiceResponse, DangoMonopolyGridRewardRequest,
    DangoMonopolyGridRewardResponse, DangoMonopolyNextBoardRequest,
    DangoMonopolyNextBoardResponse, DangoMonopolyTaskRewardRequest, DangoMonopolyTaskRewardResponse,
    DollDeliveryRequest, DollDeliveryResponse, DollGarbInsertCoinRequest,
    DollGarbInsertCoinResponse, DollGarbStartRequest, DollGarbStartResponse,
    DollGrabItemInfo, DollGrabPauseEndRequest, DollGrabPauseEndResponse,
    DollGrabPauseStartRequest, DollGrabPauseStartResponse, DollGrabRequest, DollGrabResponse,
    DollGrabStopRequest, DollGrabStopResponse, DollGrabTimeBallRequest, DollGrabTimeBallResponse,
    FRUnitActions, FloroRanchAnimationRequest, FloroRanchAnimationResponse,
    FloroRanchCloseTaskRequest, FloroRanchCloseTaskResponse, FloroRanchDayStart,
    FloroRanchEventChoiceRequest, FloroRanchEventChoiceResponse,
    FloroRanchExecuteSkillPlayRequest, FloroRanchExecuteSkillPlayResponse,
    FloroRanchFirstEnterRequest, FloroRanchFirstEnterResponse, FloroRanchOutRequest,
    FloroRanchOutResponse, FloroRanchGacha, FloroRanchPlayGachaRefreshRequest, FloroRanchPlayGachaRefreshResponse,
    FloroRanchPlayGachaRequest, FloroRanchPlayGachaResponse, FloroRanchPlayInfo,
    FloroRanchPlayNextDayRequest, FloroRanchPlayNextDayResponse,
    FloroRanchPlayRefreshShopRequest, FloroRanchPlayRefreshShopResponse,
    FloroRanchPlayRemoveUnitRequest, FloroRanchPlayRemoveUnitResponse,
    FloroRanchPlaySelectCardGroupRequest, FloroRanchPlaySelectCardGroupResponse,
    FloroRanchPlayShopBuyRequest, FloroRanchPlayShopBuyResponse, FloroRanchPlayTask,
    FloroRanchPlayTributeRequest, FloroRanchPlayTributeResponse, FloroRanchPlayUnit,
    FloroRanchPlayUnlimitedModeRequest, FloroRanchPlayUnlimitedModeResponse,
    FloroRanchRewardMilestoneRequest, FloroRanchRewardMilestoneResponse,
    FloroRanchRewardTaskRequest, FloroRanchRewardTaskResponse, FloroRanchPlaySettleData,
    FloroRanchSettleDataRequest, FloroRanchSettleDataResponse, FloroRanchSettleRequest,
    FloroRanchSettleResponse, FloroRanchStageStart, FloroRanchStartPlayRequest,
    FloroRanchStartPlayResponse, FloroRanchShop, FloroRanchUnlockTechPointRequest,
    FloroRanchUnlockTechPointResponse, HonamiStoryActivateTalentRequest,
    HonamiStoryActivateTalentResponse, HonamiStoryActivityInfo, HonamiStoryActivityInfoRequest,
    HonamiStoryActivityInfoResponse, HonamiStoryAreaSecretRewardRequest,
    HonamiStoryAreaSecretRewardResponse, HonamiStoryBackpack, HonamiStoryBagOperateRequest,
    HonamiStoryBagOperateResponse, HonamiStoryDiscardItemRequest, HonamiStoryDiscardItemResponse,
    HonamiStoryEquipRoleRequest, HonamiStoryEquipRoleResponse, HonamiStoryInstLeaveRequest,
    HonamiStoryInstLeaveResponse, HonamiStoryItemCollectionRequest,
    HonamiStoryItemCollectionResponse, HonamiStoryLifeSupportUpRequest,
    HonamiStoryLifeSupportUpResponse, HonamiStoryLimitTaskRewardRequest,
    HonamiStoryLimitTaskRewardResponse, HonamiStoryLockItemRequest,
    HonamiStoryLockItemResponse, HonamiStoryMascotRewardRequest,
    HonamiStoryMascotRewardResponse, HonamiStoryPickUpItemRequest,
    HonamiStoryPickUpItemResponse, HonamiStoryPlayerBagInfo, HonamiStoryResidentTaskRewardRequest,
    HonamiStoryResidentTaskRewardResponse, HonamiStoryRoleData, HonamiStoryScoreRewardRequest,
    HonamiStoryScoreRewardResponse, HonamiStorySellItemRequest, HonamiStorySellItemResponse,
    HonamiStoryTalentInfo, HonamiStoryUnlockSlotRequest, HonamiStoryUnlockSlotResponse,
    HonamiStoryWeaponDressRequest, HonamiStoryWeaponDressResponse, JokerGuessDrawCardAction,
    JokerGuessDrawCardRequest, JokerGuessDrawCardResponse, JokerGuessInitGame, JokerGuessOp,
    JokerGuessPlayCardAction, JokerGuessPlayCardRequest, JokerGuessPlayCardResponse,
    JokerGuessRewardRequest, JokerGuessRewardResponse, JokerGuessRoundInfo,
    JokerGuessSkillExecAction, JokerGuessStartNewRequest, JokerGuessStartNewResponse,
    JokerGuessUseSkillRequest, JokerGuessUseSkillResponse, KurotatoChestRewardSelectRequest,
    KurotatoChestRewardSelectResponse, KurotatoEndShopStepRequest,
    KurotatoEndShopStepResponse, KurotatoEndSpecialWaveCombatRequest,
    KurotatoEndSpecialWaveCombatResponse, KurotatoGoldAddRequest, KurotatoGoldAddResponse,
    KurotatoLevelRecordDeleteRequest, KurotatoLevelRecordDeleteResponse,
    KurotatoLimitTaskReceiveRequest, KurotatoLimitTaskReceiveResponse,
    KurotatoReChallengeRequest, KurotatoReChallengeResponse, KurotatoRefineWeaponRequest,
    KurotatoRefineWeaponResponse, KurotatoResTaskReceiveRequest,
    KurotatoResTaskReceiveResponse, KurotatoRoleRecordRequest, KurotatoRoleRecordResponse,
    KurotatoScoreTaskReceiveRequest, KurotatoScoreTaskReceiveResponse,
    KurotatoSellWeaponRequest, KurotatoSellWeaponResponse, KurotatoSettlementInfo,
    KurotatoSettlementRequest, KurotatoSettlementResponse, KurotatoShopPanelPbData,
    KurotatoShopRewardLockRequest,
    KurotatoShopRewardLockResponse, KurotatoShopRewardRefreshRequest,
    KurotatoShopRewardRefreshResponse, KurotatoShopRewardSelectRequest,
    KurotatoShopRewardSelectResponse, KurotatoStepAdvanceRequest, KurotatoStepAdvanceResponse,
    KurotatoSyncEntityDealDamagePush, KurotatoUpgradeRewardRefreshRequest,
    KurotatoUpgradeRewardRefreshResponse, KurotatoUpgradeRewardSelectRequest,
    KurotatoUpgradeRewardSelectResponse, PinballBonusFullWinRequest, PinballBonusFullWinResponse,
    PinballFriendRankRequest, PinballFriendRankResponse, PinballInGameKickBallRequest,
    PinballInGameKickBallResponse, PinballInGameSyncPayloadRequest,
    PinballInGameSyncPayloadResponse, PinballRankData, PinballQuitRequest,
    PinballQuitResponse, PinballRoleLevelUpRequest, PinballRoleLevelUpResponse,
    PinballSetPayloadRequest, PinballSetPayloadResponse, PinballTaskRewardRequest,
    PinballTaskRewardResponse, PinballTeamDieRequest, PinballTeamDieResponse,
    PinballTeamReviveRequest, PinballTeamReviveResponse, PinballWeaponDegradeRequest,
    PinballWeaponDegradeResponse, PinballWeaponWearRequest, PinballWeaponWearResponse,
    RacingBetMatchActionRequest, RacingBetMatchActionResponse, RacingBetMatchInfoRequest,
    RacingBetMatchInfoResponse, RacingBetMatchRoundActionRequest,
    RacingBetMatchRoundActionResponse, RacingBetRankPlayerInfo, RacingBetRankSelfData, RacingBetsBulletScreenRequest,
    RacingBetsBulletScreenResponse, RacingBetsCloseSettleMenuRequest,
    RacingBetsCloseSettleMenuResponse, RacingBetsDangoInfo, RacingBetsGearRefundRequest,
    RacingBetsGearRefundResponse, RacingBetsGearRequest, RacingBetsGearResponse,
    RacingBetsGroupMatchesInfo, RacingBetsInstanceDangoInfo, RacingBetsMatchInfoRequest,
    RacingBetsMatchInfoResponse, RacingBetPlayerInfo, RacingBetsRankRequest,
    RacingBetsRankResponse, RacingBetsRepeatedTaskRewardRequest,
    RacingBetsRepeatedTaskRewardResponse, RacingBetsMatchRoundInfo, RacingBetsTaskRewardRequest,
    RacingBetsTaskRewardResponse, RacingBetsUpdateOddsRequest, RacingBetsUpdateOddsResponse,
    RegressBonusRequest, RegressBonusResponse, RegressBuyBonusLvRequest,
    RegressBuyBonusLvResponse, RegressDisposableRewardRequest, RegressDisposableRewardResponse,
    RegressQuestionOpenRequest, RegressQuestionOpenResponse, RegressRepeatedTaskRequest,
    RegressRepeatedTaskResponse, RegressSetCurUseTrialRoleRequest,
    RegressSetCurUseTrialRoleResponse, RegressTaskRequest, RegressTaskResponse,
    RegressTrialRoleLvUpRequest, RegressTrialRoleLvUpResponse,
    RhythmChangePartnerRoleRequest, RhythmChangePartnerRoleResponse, RhythmMainLineInfoRequest,
    RhythmMainLineInfoResponse, RhythmActivityPb, RhythmPlayerRankingInfo, RhythmRankListRequest,
    RhythmRankListResponse, RhythmRankListSelfDataRequest, RhythmRankListSelfDataResponse,
    RhythmSetRedDotRequest, RhythmSetRedDotResponse, RhythmSettleRequest, RhythmSettleResponse,
    RhythmTaskOneKeyRewardRequest, RhythmTaskOneKeyRewardResponse, RollBlockExitGamePlayRequest,
    RollBlockExitGamePlayResponse, RollBlockGamePlayReadyRequest, RollBlockGamePlayReadyResponse,
    RollBlockInputRequest, RollBlockInputResponse, RollBlockMovementFinishRequest,
    RollBlockMovementFinishResponse, RollBlockResetGamePlayRequest,
    RollBlockResetGamePlayResponse, RollBlockSwitchControlBlockRequest,
    RollBlockSwitchControlBlockResponse, RollBlockToggleHintRequest,
    RollBlockToggleHintResponse, SpringFestivalAtmosphereRewardRequest,
    SpringFestivalAtmosphereRewardResponse, SpringFestivalScoreRewardRequest,
    SpringFestivalScoreRewardResponse, SpringFestivalTaskRewardRequest,
    SpringFestivalTaskRewardResponse, SpringSignDrawRewardRequest, SpringSignDrawRewardResponse,
    SpringSignDrawRoleRequest, SpringSignDrawRoleResponse, SpringSignPhotoRewardRequest,
    SpringSignPhotoRewardResponse, SpringSignSkinRewardRequest, SpringSignSkinRewardResponse,
    OneTowerDefencePassInfo, ResultView, RewardGainingPanelInfo,
    SurvivorLastInfo, SurvivorsLastInfoRequest, SurvivorsLastInfoResponse,
    SurvivorsLockRequest, SurvivorsLockResponse, SurvivorsNormalAwardRequest,
    SurvivorsNormalAwardResponse, SurvivorsOpDoneRequest, SurvivorsOpDoneResponse,
    SurvivorsResultRequest, SurvivorsResultResponse, SurvivorsScoreAwardRequest,
    SurvivorsScoreAwardResponse, SurvivorsStepAdvanceRequest, SurvivorsStepAdvanceResponse,
    SurvivorsTalentLevelUpRequest, SurvivorsTalentLevelUpResponse, SurvivorsWeaponInfoRequest,
    SurvivorsWeaponInfoResponse, TowerDefenceChallengeQuitRequest,
    TowerDefenceChallengeQuitResponse, TowerDefenceInstanceRewardRequest,
    TowerDefenceInstanceRewardResponse, TowerDefencePassInfo, TowerDefenceRandListRequest, TowerDefenceRandListResponse, TowerDefenceRankShowNameRequest,
    TowerDefenceRankShowNameResponse, TowerDefenceScoreRewardRequest,
    TowerDefenceScoreRewardResponse, TowerDefenceSelfPassRequest, TowerDefenceSelfPassResponse,
    TrapDefenseAuxiliaryBranchRequest, TrapDefenseAuxiliaryBranchResponse,
    TrapDefenseAuxiliaryLevelUpRequest, TrapDefenseAuxiliaryLevelUpResponse,
    TrapDefenseAuxiliaryResetRequest, TrapDefenseAuxiliaryResetResponse,
    TrapDefenseBuildingBranchRequest, TrapDefenseBuildingBranchResponse,
    TrapDefenseBuildingLevelUpRequest, TrapDefenseBuildingLevelUpResponse,
    TrapDefenseBuildingResetRequest, TrapDefenseBuildingResetResponse,
    TrapDefenseBuyShopProductRequest, TrapDefenseBuyShopProductResponse,
    TrapDefenseChallengeQuitRequest, TrapDefenseChallengeQuitResponse,
    TrapDefenseCurChallengeInfoRequest, TrapDefenseCurChallengeInfoResponse,
    TrapDefenseDevelopResetRequest, TrapDefenseDevelopResetResponse,
    TrapDefenseGetRewardRequest, TrapDefenseGetRewardResponse,
    TrapDefenseGetSpecialRewardRequest, TrapDefenseGetSpecialRewardResponse,
    TrapDefensePreviewFinishRequest, TrapDefensePreviewFinishResponse,
    TrapDefenseRefreshGainingGroupRequest, TrapDefenseRefreshGainingGroupResponse,
    TrapDefenseRefreshShopRequest, TrapDefenseRefreshShopResponse,
    TrapDefenseSelectGainingGroupRequest, TrapDefenseSelectGainingGroupResponse,
    TrapDefenseShopPanelInfo, TrapDefenseSlotUpdateRequest, TrapDefenseSlotUpdateResponse,
    TrapDefenseTechUnlockRequest, TrapDefenseTechUnlockResponse, TrapDefenseUseItemRequest,
    TrapDefenseUseItemResponse,
};

use super::super::connection::drain_event_queue_best_effort;
use super::State;

fn claim_id_list(
    state: &mut State,
    prefix: &str,
    table: &str,
    ids: &[i32],
) -> Result<(), String> {
    for &id in ids {
        let key = format!("{prefix}:{id}");
        if state.player_components.progress.info.mark_claim(&key) {
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                table,
                i64::from(id),
                false,
                true,
                19500,
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

fn claim_one(state: &mut State, prefix: &str, table: &str, id: i32) -> Result<(), String> {
    claim_id_list(state, prefix, table, &[id])
}

// =========================================================================
// Dango Brawl & Dango Monopoly
// =========================================================================

pub fn handle_dango_brawl_bag_operate_request(
    state: &mut State,
    req: DangoBrawlBagOperateRequest,
) -> Result<DangoBrawlBagOperateResponse, String> {
    pb_kv::save_msg(
        &state.fs,
        &mut state.player_components.progress,
        "db:bag_ops",
        &req,
    )
    .map_err(|e| e.to_string())?;
    Ok(DangoBrawlBagOperateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_brawl_event_refresh_request(
    state: &mut State,
    req: DangoBrawlEventRefreshRequest,
) -> Result<DangoBrawlEventRefreshResponse, String> {
    let key = format!("db_evt_ref:{}", req.SlotIndex);
    let cnt = state
        .player_components
        .progress
        .info
        .add_key_int(&key, 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DangoBrawlEventRefreshResponse {
        Slot: Some(DangoBrawlEventSlotInfo {
            EventConfigId: 1,
            RefreshedCount: cnt,
        }),
        SlotIndex: req.SlotIndex,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_brawl_event_select_request(
    state: &mut State,
    req: DangoBrawlEventSelectRequest,
) -> Result<DangoBrawlEventSelectResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "db_evt_slot",
        i64::from(req.SlotIndex),
    )
    .map_err(|e| e.to_string())?;
    Ok(DangoBrawlEventSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
        ActiveEventIds: vec![req.SlotIndex],
    })
}

pub fn handle_dango_brawl_start_fight_request(
    state: &mut State,
    _req: DangoBrawlStartFightRequest,
) -> Result<DangoBrawlStartFightResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("db_fight_cnt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DangoBrawlStartFightResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_brawl_fight_over_request(
    state: &mut State,
    _req: DangoBrawlFightOverRequest,
) -> Result<DangoBrawlFightOverResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("db_round", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DangoBrawlFightOverResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_brawl_settle_request(
    state: &mut State,
    _req: DangoBrawlSettleRequest,
) -> Result<DangoBrawlSettleResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "db_round",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(DangoBrawlSettleResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_brawl_score_reward_request(
    state: &mut State,
    req: DangoBrawlScoreRewardRequest,
) -> Result<DangoBrawlScoreRewardResponse, String> {
    claim_id_list(state, "db:score", "DangoBrawlScoreReward", &req.ScoreIds)?;
    Ok(DangoBrawlScoreRewardResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_brawl_task_reward_request(
    state: &mut State,
    req: DangoBrawlTaskRewardRequest,
) -> Result<DangoBrawlTaskRewardResponse, String> {
    claim_id_list(state, "db:task", "DangoBrawlReward", &req.TaskIds)?;
    Ok(DangoBrawlTaskRewardResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_brawl_shop_buy_request(
    state: &mut State,
    req: DangoBrawlShopBuyRequest,
) -> Result<DangoBrawlShopBuyResponse, String> {
    let key = format!("db:buy:{}", req.SlotUid);
    state.player_components.progress.info.add_key_int(&key, 1);
    state
        .player_components
        .progress
        .info
        .set_key_int("db:buy_bench", i64::from(req.ToBench));
    if let Some(slot) = &req.Slot {
        pb_kv::save_msg(
            &state.fs,
            &mut state.player_components.progress,
            &format!("db:slot:{}", req.SlotUid),
            slot,
        )
        .map_err(|e| e.to_string())?;
    } else {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(DangoBrawlShopBuyResponse {
        ErrorCode: Some(ErrorCode::Success),
        UseServerBagUpdate: true,
    })
}

pub fn handle_dango_brawl_shop_compose_request(
    state: &mut State,
    _req: DangoBrawlShopComposeRequest,
) -> Result<DangoBrawlShopComposeResponse, String> {
    let uid = state
        .player_components
        .progress
        .info
        .add_key_int("db_uid", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DangoBrawlShopComposeResponse {
        ErrorCode: Some(ErrorCode::Success),
        NewItemUid: uid,
    })
}

pub fn handle_dango_brawl_shop_lock_request(
    state: &mut State,
    req: DangoBrawlShopLockRequest,
) -> Result<DangoBrawlShopLockResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("db:lock:{}", req.SlotUid),
        i64::from(req.Locked),
    )
    .map_err(|e| e.to_string())?;
    Ok(DangoBrawlShopLockResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_brawl_shop_refresh_request(
    state: &mut State,
    _req: DangoBrawlShopRefreshRequest,
) -> Result<DangoBrawlShopRefreshResponse, String> {
    let cnt = state
        .player_components
        .progress
        .info
        .add_key_int("db_shop_ref", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DangoBrawlShopRefreshResponse {
        Shop: Some(DangoBrawlShopInfo {
            NextRefreshCost: 1,
            ManualRefreshCount: cnt,
            ..Default::default()
        }),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_brawl_shop_sell_request(
    state: &mut State,
    req: DangoBrawlShopSellRequest,
) -> Result<DangoBrawlShopSellResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("db:sold:{}", req.ItemUid),
    )
    .map_err(|e| e.to_string())?;
    Ok(DangoBrawlShopSellResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_monopoly_dice_request(
    state: &mut State,
    req: DangoMonopolyDiceRequest,
) -> Result<DangoMonopolyDiceResponse, String> {
    let now_s = energy::now_seconds().max(1);
    let roll = ((now_s % 6) + 1) as i32;
    let grid_key = format!("dmono_grid:{}", req.ActivityId);
    let new_grid = state
        .player_components
        .progress
        .info
        .add_key_int(&grid_key, i64::from(roll));
    state
        .player_components
        .progress
        .info
        .set_key_int("dm:grid", new_grid);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DangoMonopolyDiceResponse {
        DiceNum: roll,
        MoveGridNum: roll,
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_dango_monopoly_grid_reward_request(
    state: &mut State,
    req: DangoMonopolyGridRewardRequest,
) -> Result<DangoMonopolyGridRewardResponse, String> {
    let cur_grid = state
        .player_components
        .progress
        .info
        .get_key_int(&format!("dmono_grid:{}", req.ActivityId))
        .max(state.player_components.progress.info.get_key_int("dm:grid")) as i32;
    let claim_key = format!("dmono:grid_rw:{}", cur_grid.max(1));
    let had_claim = state.player_components.progress.info.has_claim(&claim_key);
    claim_one(state, "dmono:grid_rw", "DangoMonopolyGrid", cur_grid.max(1))?;
    Ok(DangoMonopolyGridRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        HasReward: !had_claim,
    })
}

pub fn handle_dango_monopoly_next_board_request(
    state: &mut State,
    req: DangoMonopolyNextBoardRequest,
) -> Result<DangoMonopolyNextBoardResponse, String> {
    let info = &mut state.player_components.progress.info;
    let cur_board = info
        .get_key_int(&format!("dmono_board:{}", req.ActivityId))
        .max(info.get_key_int("dm:board"))
        .max(1);
    let next_board = (cur_board + 1) as i32;
    info.set_key_int(&format!("dmono_board:{}", req.ActivityId), i64::from(next_board));
    info.set_key_int("dm:board", i64::from(next_board));
    info.set_key_int(&format!("dmono_grid:{}", req.ActivityId), 1);
    info.set_key_int("dm:grid", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DangoMonopolyNextBoardResponse {
        ErrorCode: Some(ErrorCode::Success),
        BoardId: next_board,
        ..Default::default()
    })
}

pub fn handle_dango_monopoly_board_reward_request(
    state: &mut State,
    req: DangoMonopolyBoardRewardRequest,
) -> Result<DangoMonopolyBoardRewardResponse, String> {
    claim_id_list(state, "dmono:board", "DangoMonopolyBoard", &req.BoardIds)?;
    Ok(DangoMonopolyBoardRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_dango_monopoly_task_reward_request(
    state: &mut State,
    req: DangoMonopolyTaskRewardRequest,
) -> Result<DangoMonopolyTaskRewardResponse, String> {
    claim_id_list(state, "dango_mono:task", "DangoMonopolyTask", &req.TaskIds)?;
    Ok(DangoMonopolyTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

// =========================================================================
// Doll Grabber & Delivery
// =========================================================================

fn resolve_doll_machine_cfg_and_item(
    state: &State,
    entity_inc_id: i64,
    uni_id: i32,
) -> (i32, i32) {
    let inst_id = state.scene.as_ref().map_or(8, |s| s.instance_id);
    let cfg_id = state
        .scene
        .as_ref()
        .and_then(|s| s.find_config_id_by_net_id(entity_inc_id))
        .unwrap_or(entity_inc_id as i32);
    let Some(le) = state
        .assets
        .find_level_entity(inst_id, i64::from(cfg_id))
        .or_else(|| state.assets.find_level_entity_any_map(i64::from(cfg_id)))
    else {
        return (cfg_id, uni_id);
    };
    let mut comps = le.ComponentsData.clone();
    if let Some(tpl) = state.assets.find_template_config(&le.BlueprintType) {
        tpl.ComponentsData.merge_into(&mut comps);
    }
    for entry in &comps.extra {
        if let ExtraEntry::DollGrabMachineComponent(dm) = entry {
            for v in &dm.Items {
                let entry_uni = v
                    .get("Id")
                    .and_then(|x| x.as_i64())
                    .and_then(|x| i32::try_from(x).ok())
                    .unwrap_or(0);
                let entry_item = v
                    .get("ItemId")
                    .and_then(|x| x.as_i64())
                    .and_then(|x| i32::try_from(x).ok())
                    .unwrap_or(0);
                if entry_uni == uni_id && entry_item > 0 {
                    return (cfg_id, entry_item);
                }
            }
        }
    }
    (cfg_id, uni_id)
}

fn process_doll_grab(
    state: &mut State,
    entity_inc_id: i64,
    uni_id: i32,
    is_time_ball: bool,
) -> Result<Vec<DollGrabItemInfo>, String> {
    let (cfg_id, item_id) = resolve_doll_machine_cfg_and_item(state, entity_inc_id, uni_id);
    let doll_cfg = state.assets.tables.doll_grab_items_csv.get_data_by_id(&item_id);
    let max_cnt = doll_cfg.as_ref().map(|d| d.MaxGetCount).unwrap_or(0);
    let cur_cnt = state
        .player_components
        .progress
        .info
        .get_key_int(&format!("doll:cnt:{item_id}")) as i32;
    let drop_id = doll_cfg.as_ref().map(|d| d.DropId).unwrap_or(0);
    let binding_item_id = doll_cfg.as_ref().map(|d| d.BindingItemId).unwrap_or(0);
    let inf_score = doll_cfg.as_ref().map(|d| d.InfiniteModeScore).unwrap_or(0);

    let mut grab_items = Vec::new();
    if max_cnt <= 0 || cur_cnt < max_cnt {
        state
            .player_components
            .progress
            .info
            .add_key_int(&format!("doll:cnt:{item_id}"), 1);
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("doll:uni:{cfg_id}:{uni_id}"), 1);
        if drop_id > 0 {
            let preview = reward::preview_drop_items(&state.assets, drop_id);
            let mut rctx = state.reward_context();
            let _ = reward::grant_drop_package(&mut rctx, drop_id, 0)
                .map_err(|e| e.to_string())?;
            for g in preview {
                grab_items.push(DollGrabItemInfo {
                    ItemId: g.item_id,
                    ItemCount: g.count,
                });
            }
        }
        if grab_items.is_empty() && binding_item_id > 0 {
            grab_items.push(DollGrabItemInfo {
                ItemId: binding_item_id,
                ItemCount: 1,
            });
        }
    }
    if grab_items.is_empty() {
        grab_items.push(DollGrabItemInfo {
            ItemId: item_id,
            ItemCount: 1,
        });
    }

    let delta_score = if inf_score > 0 {
        inf_score
    } else if is_time_ball {
        15
    } else {
        10
    };
    let info = &mut state.player_components.progress.info;
    let sess = info.add_key_int(&format!("doll:cur:{cfg_id}"), i64::from(delta_score)) as i32;
    let acc = info.add_key_int(&format!("doll:acc:{cfg_id}"), i64::from(delta_score)) as i32;
    let high = sess.max(info.get_key_int(&format!("doll:high:{cfg_id}")) as i32);
    info.set_key_int(&format!("doll:high:{cfg_id}"), i64::from(high));
    info.set_key_int(&format!("doll:fin:{cfg_id}"), 1);
    info.mark_claim(&format!("doll:fin:{cfg_id}"));
    info.add_key_int("doll_grab:cur", i64::from(delta_score));
    info.add_key_int("doll_grab:total", i64::from(delta_score));

    if let Some(scene) = state.scene.as_mut()
        && let Some(ent) = scene.entity_mut(entity_inc_id)
        && let Some((can_items, hi, ac)) = &mut ent.world_item.doll_machine
    {
        can_items.retain(|&id| id != uni_id);
        *hi = (*hi).max(high);
        *ac = acc;
    }

    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    super::world_puzzle::flush_quest_condition_changed(state);
    Ok(grab_items)
}

pub fn handle_doll_delivery_request(
    state: &mut State,
    req: DollDeliveryRequest,
) -> Result<DollDeliveryResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("doll:delivery:{}", req.EntityCfgId),
    )
    .map_err(|e| e.to_string())?;

    let mut belong_type: Option<String> = None;
    let mut delivery_rewards: Vec<(i32, i32)> = Vec::new();
    let inst_id = state.scene.as_ref().map_or(8, |s| s.instance_id);
    if let Some(le) = state
        .assets
        .find_level_entity(inst_id, i64::from(req.EntityCfgId))
        .or_else(|| state.assets.find_level_entity_any_map(i64::from(req.EntityCfgId)))
    {
        let mut comps = le.ComponentsData.clone();
        if let Some(tpl) = state.assets.find_template_config(&le.BlueprintType) {
            tpl.ComponentsData.merge_into(&mut comps);
        }
        for entry in &comps.extra {
            if let ExtraEntry::DollGrabShowcaseComponent(sc) = entry {
                if let Some(bt) = &sc.BelongType
                    && !bt.is_empty()
                {
                    belong_type = Some(bt.clone());
                }
                for rw in &sc.DeliveryRewards {
                    let cnt = rw
                        .get("AchievedDollCount")
                        .and_then(|v| v.as_i64())
                        .and_then(|v| i32::try_from(v).ok())
                        .unwrap_or(0);
                    let drop_id = rw
                        .get("DropId")
                        .and_then(|v| v.as_i64())
                        .and_then(|v| i32::try_from(v).ok())
                        .unwrap_or(0);
                    if cnt > 0 && drop_id > 0 {
                        delivery_rewards.push((cnt, drop_id));
                    }
                }
            }
        }
    }

    let mut all_delivered = Vec::new();
    let mut newly_delivered = Vec::new();
    let mut first_match = None;
    for di in &state.assets.tables.doll_grab_items_csv.items {
        if di.BindingItemId <= 0 && di.InfiniteModeScore <= 0 {
            continue;
        }
        if let Some(bt) = &belong_type
            && di.ShowcaseBelongType != *bt
        {
            continue;
        }
        if first_match.is_none() {
            first_match = Some(di.Id);
        }
        let already = state
            .player_components
            .progress
            .info
            .get_key_int(&format!("doll:delivered:{}", di.Id))
            > 0;
        let caught = state
            .player_components
            .progress
            .info
            .get_key_int(&format!("doll:cnt:{}", di.Id))
            > 0;
        let owned = di.BindingItemId > 0
            && state
                .player_components
                .inventory
                .info
                .normal_item_count(di.BindingItemId)
                > 0;
        if already || caught || owned {
            all_delivered.push(di.Id);
            if !already {
                newly_delivered.push(di.Id);
                state
                    .player_components
                    .progress
                    .info
                    .set_key_int(&format!("doll:delivered:{}", di.Id), 1);
                state
                    .player_components
                    .progress
                    .info
                    .mark_claim(&format!("doll:item:{}:{}", req.EntityCfgId, di.Id));
            }
        }
    }
    if all_delivered.is_empty()
        && let Some(fid) = first_match
    {
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("doll:delivered:{fid}"), 1);
        state
            .player_components
            .progress
            .info
            .mark_claim(&format!("doll:item:{}:{}", req.EntityCfgId, fid));
        all_delivered.push(fid);
        newly_delivered.push(fid);
    }

    let combo_count = state
        .assets
        .doll_grab_combos
        .iter()
        .filter(|combo| {
            combo.MainItemId > 0
                && all_delivered.contains(&combo.MainItemId)
                && combo
                    .PartItemIds
                    .iter()
                    .all(|pid| all_delivered.contains(pid))
        })
        .count() as i32;
    state
        .player_components
        .progress
        .info
        .set_key_int(&format!("doll:combo:{}", req.EntityCfgId), i64::from(combo_count));

    let mut drop_ids = Vec::new();
    let achieved_milestone = if combo_count > 0 {
        combo_count.max(all_delivered.len() as i32)
    } else {
        all_delivered.len() as i32
    };
    for (need_cnt, drop_id) in delivery_rewards {
        let key = format!("doll:drw:{}:{need_cnt}", req.EntityCfgId);
        if achieved_milestone >= need_cnt
            && state.player_components.progress.info.mark_claim(&key)
        {
            let mut rctx = state.reward_context();
            let _ = reward::grant_drop_package(&mut rctx, drop_id, 0);
            drop_ids.push(drop_id);
        }
    }

    if let Some(scene) = state.scene.as_mut()
        && let Some(nid) = scene.find_entity_net_id_by_config_id(req.EntityCfgId)
        && let Some(ent) = scene.entity_mut(nid)
    {
        ent.world_item.doll_showcase_items = Some(all_delivered.clone());
    }

    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if !newly_delivered.is_empty() || !drop_ids.is_empty() {
        let _ = state.conn.push_msg(
            sonoro_proto::pb::DollDeliveryNotify::MSG_ID,
            &sonoro_proto::pb::DollDeliveryNotify {
                DropIds: drop_ids,
                ItemIds: newly_delivered,
            },
        );
    }
    super::world_puzzle::flush_quest_condition_changed(state);
    Ok(DollDeliveryResponse {
        ItemIds: all_delivered,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_doll_garb_insert_coin_request(
    state: &mut State,
    req: DollGarbInsertCoinRequest,
) -> Result<DollGarbInsertCoinResponse, String> {
    let cfg_id = state
        .scene
        .as_ref()
        .and_then(|s| s.find_config_id_by_net_id(req.EntityIncId))
        .unwrap_or(req.EntityIncId as i32);
    let info = &mut state.player_components.progress.info;
    info.add_key_int(&format!("doll:coins:{}", req.EntityIncId), 1);
    info.set_key_int(&format!("doll:cur:{cfg_id}"), 0);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DollGarbInsertCoinResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_doll_garb_start_request(
    state: &mut State,
    req: DollGarbStartRequest,
) -> Result<DollGarbStartResponse, String> {
    let cfg_id = state
        .scene
        .as_ref()
        .and_then(|s| s.find_config_id_by_net_id(req.EntityIncId))
        .unwrap_or(req.EntityIncId as i32);
    let info = &mut state.player_components.progress.info;
    info.set_key_int("doll_grab:cur", 0);
    info.set_key_int(&format!("doll:cur:{cfg_id}"), 0);
    info.set_key_int(&format!("doll:in:{cfg_id}"), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DollGarbStartResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_doll_grab_pause_start_request(
    state: &mut State,
    req: DollGrabPauseStartRequest,
) -> Result<DollGrabPauseStartResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("doll:paused:{}", req.EntityIncId),
        1,
    )
    .map_err(|e| e.to_string())?;
    Ok(DollGrabPauseStartResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_doll_grab_pause_end_request(
    state: &mut State,
    req: DollGrabPauseEndRequest,
) -> Result<DollGrabPauseEndResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("doll:paused:{}", req.EntityIncId),
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(DollGrabPauseEndResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_doll_grab_request(
    state: &mut State,
    req: DollGrabRequest,
) -> Result<DollGrabResponse, String> {
    let grab_items = process_doll_grab(state, req.EntityIncId, req.ItemUniId, false)?;
    Ok(DollGrabResponse {
        ErrorCode: Some(ErrorCode::Success),
        GrabItemInfos: grab_items,
    })
}

pub fn handle_doll_grab_time_ball_request(
    state: &mut State,
    req: DollGrabTimeBallRequest,
) -> Result<DollGrabTimeBallResponse, String> {
    let grab_items = process_doll_grab(state, req.EntityIncId, req.ItemUniId, true)?;
    Ok(DollGrabTimeBallResponse {
        ErrorCode: Some(ErrorCode::Success),
        GrabItemInfos: grab_items,
    })
}

pub fn handle_doll_grab_stop_request(
    state: &mut State,
    req: DollGrabStopRequest,
) -> Result<DollGrabStopResponse, String> {
    let cfg_id = state
        .scene
        .as_ref()
        .and_then(|s| s.find_config_id_by_net_id(req.EntityIncId))
        .unwrap_or(req.EntityIncId as i32);
    let info = &mut state.player_components.progress.info;
    let sess = (info.get_key_int(&format!("doll:cur:{cfg_id}")) as i32)
        .max(info.get_key_int("doll_grab:cur") as i32);
    let total = (info.get_key_int(&format!("doll:acc:{cfg_id}")) as i32)
        .max(info.get_key_int("doll_grab:total") as i32);
    let high = sess
        .max(info.get_key_int(&format!("doll:high:{cfg_id}")) as i32)
        .max(info.get_key_int("doll_grab:high") as i32);
    info.set_key_int(&format!("doll:high:{cfg_id}"), i64::from(high));
    info.set_key_int("doll_grab:high", i64::from(high));
    info.set_key_int(&format!("doll:fin:{cfg_id}"), 1);
    info.set_key_int(&format!("doll:in:{cfg_id}"), 0);
    info.mark_claim(&format!("doll:fin:{cfg_id}"));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    super::world_puzzle::flush_quest_condition_changed(state);
    Ok(DollGrabStopResponse {
        HighScore: high,
        AccumulatedScore: total,
        SessionScore: sess,
        ErrorCode: Some(ErrorCode::Success),
    })
}

// =========================================================================
// Floro Ranch
// =========================================================================

pub fn handle_floro_ranch_first_enter_request(
    state: &mut State,
    _req: FloroRanchFirstEnterRequest,
) -> Result<FloroRanchFirstEnterResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "fr_first_enter",
        1,
    )
    .map_err(|e| e.to_string())?;
    Ok(FloroRanchFirstEnterResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_floro_ranch_animation_request(
    state: &mut State,
    req: FloroRanchAnimationRequest,
) -> Result<FloroRanchAnimationResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("fr:anim:{}", req.ActivityId),
    )
    .map_err(|e| e.to_string())?;
    Ok(FloroRanchAnimationResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_floro_ranch_start_play_request(
    state: &mut State,
    req: FloroRanchStartPlayRequest,
) -> Result<FloroRanchStartPlayResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("fr:day", 1);
    info.set_key_int("fr:stage", 1);
    info.set_key_int("fr:coin", 100);
    info.set_key_int("fr:diamond", 10);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FloroRanchStartPlayResponse {
        ErrorCode: Some(ErrorCode::Success),
        PlayInfo: Some(FloroRanchPlayInfo {
            SubInsId: req.SubInsId,
            CurCoin: 100,
            CurDiamond: 10,
            SkillId: req.SkillId,
            Stage: 1,
            Day: 1,
            CurStageDayNum: 1,
            TotalDay: 1,
            RemainDay: 5,
            Races: req.Races,
            ..Default::default()
        }),
        ..Default::default()
    })
}

fn floro_settle_tasks() -> Vec<FloroRanchPlayTask> {
    vec![FloroRanchPlayTask {
        TaskType: Some(FloroRanchPlayTaskType::DaySettleActions),
        UnitActions: Some(FRUnitActions::default()),
        ..Default::default()
    }]
}

pub fn handle_floro_ranch_play_next_day_request(
    state: &mut State,
    _req: FloroRanchPlayNextDayRequest,
) -> Result<FloroRanchPlayNextDayResponse, String> {
    let info = &mut state.player_components.progress.info;
    let cur_day = info.add_key_int("fr:day", 1) as i32;
    info.add_key_int("fr:coin", 50);
    let cur_stage = info.get_key_int("fr:stage").max(1) as i32;
    let is_unlim = info.get_key_int("fr:unlimited") > 0;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FloroRanchPlayNextDayResponse {
        ErrorCode: Some(ErrorCode::Success),
        Task: vec![FloroRanchPlayTask {
            TaskType: Some(FloroRanchPlayTaskType::DayStart),
            DayStart: Some(FloroRanchDayStart {
                CurDay: cur_day,
                CurStage: cur_stage,
                IsUnlimited: is_unlim,
                TotalDay: cur_day,
                RemainDay: if is_unlim { 99 } else { (6 - cur_day).max(0) },
                ..Default::default()
            }),
            ..Default::default()
        }],
    })
}

pub fn handle_floro_ranch_play_tribute_request(
    state: &mut State,
    _req: FloroRanchPlayTributeRequest,
) -> Result<FloroRanchPlayTributeResponse, String> {
    let info = &mut state.player_components.progress.info;
    let stage = info.add_key_int("fr:stage", 1) as i32;
    let coin = info.get_key_int("fr:coin");
    let diamond = info.get_key_int("fr:diamond");
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FloroRanchPlayTributeResponse {
        Settle: false,
        Task: vec![FloroRanchPlayTask {
            TaskType: Some(FloroRanchPlayTaskType::StageStart),
            StageStart: Some(FloroRanchStageStart {
                Target: 100 * i64::from(stage),
                DayCount: 5,
                Reward: 50,
            }),
            ..Default::default()
        }],
        CurCoin: coin,
        CurDiamond: diamond,
        RemainDay: 5,
        CurStage: stage,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_floro_ranch_play_unlimited_mode_request(
    state: &mut State,
    _req: FloroRanchPlayUnlimitedModeRequest,
) -> Result<FloroRanchPlayUnlimitedModeResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("fr:unlimited", 1);
    let cur_day = info.get_key_int("fr:day").max(1) as i32;
    let cur_stage = info.get_key_int("fr:stage").max(1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FloroRanchPlayUnlimitedModeResponse {
        Task: vec![FloroRanchPlayTask {
            TaskType: Some(FloroRanchPlayTaskType::DayStart),
            DayStart: Some(FloroRanchDayStart {
                CurDay: cur_day,
                CurStage: cur_stage,
                IsUnlimited: true,
                TotalDay: cur_day,
                RemainDay: 99,
                ..Default::default()
            }),
            ..Default::default()
        }],
        ErrorCode: Some(ErrorCode::Success),
        CurStage: cur_stage,
    })
}

pub fn handle_floro_ranch_play_gacha_request(
    state: &mut State,
    req: FloroRanchPlayGachaRequest,
) -> Result<FloroRanchPlayGachaResponse, String> {
    let unit_inc = state
        .player_components
        .progress
        .info
        .add_key_int("fr:unit_inc", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let unit = FloroRanchPlayUnit {
        PlayIncId: unit_inc,
        Id: req.CardId,
        StartStage: 1,
        StartDay: 1,
        ..Default::default()
    };
    Ok(FloroRanchPlayGachaResponse {
        ErrorCode: Some(ErrorCode::Success),
        GachaResult: vec![unit.clone()],
        Unit: Some(unit),
    })
}

pub fn handle_floro_ranch_play_gacha_refresh_request(
    state: &mut State,
    req: FloroRanchPlayGachaRefreshRequest,
) -> Result<FloroRanchPlayGachaRefreshResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(FloroRanchPlayGachaRefreshResponse {
        CurDiamond: pc.get_key_int("fr:diamond"),
        CurCoin: pc.get_key_int("fr:coin"),
        ErrorCode: Some(ErrorCode::Success),
        Task: Some(FloroRanchGacha {
            AllowedRefresh: true,
            IncId: req.IncId,
            FreeTimes: 1,
            ..Default::default()
        }),
    })
}

pub fn handle_floro_ranch_play_refresh_shop_request(
    state: &mut State,
    req: FloroRanchPlayRefreshShopRequest,
) -> Result<FloroRanchPlayRefreshShopResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(FloroRanchPlayRefreshShopResponse {
        Shop: Some(FloroRanchShop {
            AllowedRefresh: true,
            IncId: req.IncId,
            FreeTimes: 1,
            ..Default::default()
        }),
        CurDiamond: pc.get_key_int("fr:diamond"),
        CurCoin: pc.get_key_int("fr:coin"),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_floro_ranch_play_shop_buy_request(
    state: &mut State,
    req: FloroRanchPlayShopBuyRequest,
) -> Result<FloroRanchPlayShopBuyResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(FloroRanchPlayShopBuyResponse {
        Shop: Some(FloroRanchShop {
            AllowedRefresh: true,
            IncId: req.IncId,
            FreeTimes: 1,
            ..Default::default()
        }),
        CurCoin: pc.get_key_int("fr:coin"),
        CurDiamond: pc.get_key_int("fr:diamond"),
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_floro_ranch_play_select_card_group_request(
    state: &mut State,
    req: FloroRanchPlaySelectCardGroupRequest,
) -> Result<FloroRanchPlaySelectCardGroupResponse, String> {
    pb_kv::save_msg(
        &state.fs,
        &mut state.player_components.progress,
        "fr:card_group",
        &req,
    )
    .map_err(|e| e.to_string())?;
    let units = req
        .CardId
        .iter()
        .enumerate()
        .map(|(idx, &cid)| FloroRanchPlayUnit {
            Id: cid,
            PlayIncId: (idx + 1) as i32,
            ..Default::default()
        })
        .collect();
    Ok(FloroRanchPlaySelectCardGroupResponse {
        NewPhantoms: units,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_floro_ranch_play_remove_unit_request(
    state: &mut State,
    req: FloroRanchPlayRemoveUnitRequest,
) -> Result<FloroRanchPlayRemoveUnitResponse, String> {
    let coin = state
        .player_components
        .progress
        .info
        .add_key_int("fr:coin", 5);
    let diamond = state
        .player_components
        .progress
        .info
        .get_key_int("fr:diamond");
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("fr:rm_unit:{}:{}", req.SubInsId, req.UnitIncId),
    )
    .map_err(|e| e.to_string())?;
    Ok(FloroRanchPlayRemoveUnitResponse {
        Task: floro_settle_tasks(),
        ErrorCode: Some(ErrorCode::Success),
        CurDiamond: diamond,
        CurCoin: coin,
    })
}

pub fn handle_floro_ranch_close_task_request(
    state: &mut State,
    req: FloroRanchCloseTaskRequest,
) -> Result<FloroRanchCloseTaskResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("fr:close_task:{}:{}", req.SubInsId, req.TaskIncId),
    )
    .map_err(|e| e.to_string())?;
    Ok(FloroRanchCloseTaskResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_floro_ranch_event_choice_request(
    state: &mut State,
    req: FloroRanchEventChoiceRequest,
) -> Result<FloroRanchEventChoiceResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("fr:evt_choice:{}:{}", req.SubInsId, req.IncId),
        i64::from(req.Choice),
    )
    .map_err(|e| e.to_string())?;
    Ok(FloroRanchEventChoiceResponse {
        ErrorCode: Some(ErrorCode::Success),
        Task: floro_settle_tasks(),
    })
}

pub fn handle_floro_ranch_execute_skill_play_request(
    state: &mut State,
    req: FloroRanchExecuteSkillPlayRequest,
) -> Result<FloroRanchExecuteSkillPlayResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("fr:skill:{}:{}", req.SubInsId, req.IncId), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FloroRanchExecuteSkillPlayResponse {
        ErrorCode: Some(ErrorCode::Success),
        Task: floro_settle_tasks(),
    })
}

pub fn handle_floro_ranch_out_request(
    state: &mut State,
    _req: FloroRanchOutRequest,
) -> Result<FloroRanchOutResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "fr:cur_sub_ins",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(FloroRanchOutResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_floro_ranch_unlock_tech_point_request(
    state: &mut State,
    req: FloroRanchUnlockTechPointRequest,
) -> Result<FloroRanchUnlockTechPointResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("fr:tech:{}", req.TechPoint),
    )
    .map_err(|e| e.to_string())?;
    Ok(FloroRanchUnlockTechPointResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_floro_ranch_reward_milestone_request(
    state: &mut State,
    req: FloroRanchRewardMilestoneRequest,
) -> Result<FloroRanchRewardMilestoneResponse, String> {
    claim_id_list(state, "fr_ms", "FloroRanchReward", &req.MilestoneIds)?;
    Ok(FloroRanchRewardMilestoneResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_floro_ranch_reward_task_request(
    state: &mut State,
    req: FloroRanchRewardTaskRequest,
) -> Result<FloroRanchRewardTaskResponse, String> {
    claim_id_list(state, "floro:task", "FloroRanchTask", &req.TaskIds)?;
    Ok(FloroRanchRewardTaskResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_floro_ranch_settle_data_request(
    state: &mut State,
    _req: FloroRanchSettleDataRequest,
) -> Result<FloroRanchSettleDataResponse, String> {
    let pc = &state.player_components.progress.info;
    let coin = pc.get_key_int("fr:coin");
    let days = pc.get_key_int("fr:day").max(1) as i32;
    Ok(FloroRanchSettleDataResponse {
        SettleData: Some(FloroRanchPlaySettleData {
            TotalDays: days,
            TotalCoin: coin,
            Win: true,
            ..Default::default()
        }),
        ErrorCode: Some(ErrorCode::Success),
        ShowMvp: true,
    })
}

pub fn handle_floro_ranch_settle_request(
    state: &mut State,
    _req: FloroRanchSettleRequest,
) -> Result<FloroRanchSettleResponse, String> {
    let pc = &state.player_components.progress.info;
    let coin = pc.get_key_int("fr:coin");
    let days = pc.get_key_int("fr:day").max(1) as i32;
    Ok(FloroRanchSettleResponse {
        TotalCoin: coin,
        MaxDayCoin: coin,
        TotalDays: days,
        Win: true,
        ErrorCode: Some(ErrorCode::Success),
        SettleData: Some(FloroRanchPlaySettleData {
            TotalDays: days,
            TotalCoin: coin,
            Win: true,
            ..Default::default()
        }),
        ..Default::default()
    })
}

// =========================================================================
// Honami Story
// =========================================================================

pub fn handle_honami_story_activity_info_request(
    state: &mut State,
    req: HonamiStoryActivityInfoRequest,
) -> Result<HonamiStoryActivityInfoResponse, String> {
    let pc = &state.player_components.progress.info;
    let ls_lv = pc.get_key_int("honami:ls_lv").max(1) as i32;
    let mut activated_talents = Vec::new();
    let mut talent_infos = Vec::new();
    for c in &pc.dynamic.claims {
        if let Some(rest) = c.strip_prefix("honami:talent:")
            && let Ok(tid) = rest.parse::<i32>()
        {
            activated_talents.push(tid);
            talent_infos.push(HonamiStoryTalentInfo {
                TalentId: tid,
                Status: 2,
            });
        }
    }
    let mut unlocked_weapons = Vec::new();
    let mut role_equip_list = Vec::new();
    for rack_idx in 0..3 {
        let dressed_weapon = pc
            .get_key_int(&format!(
                "honami:weapon_dress:{}:{rack_idx}",
                req.ActivityId
            ))
            .max(0) as i32;
        if dressed_weapon > 0 {
            unlocked_weapons.push(dressed_weapon);
        }
        role_equip_list.push(HonamiStoryRoleData {
            RoleId: rack_idx + 1,
            DressWeapon: dressed_weapon,
            ..Default::default()
        });
    }
    Ok(HonamiStoryActivityInfoResponse {
        HonamiStoryActivityInfo: Some(HonamiStoryActivityInfo {
            ActivatedTalentId: activated_talents,
            HonamiStoryTalentInfos: talent_infos,
            LifeSupportLevel: ls_lv,
            HonamiStoryPlayerBagInfo: Some(HonamiStoryPlayerBagInfo {
                Warehouse: Some(HonamiStoryBackpack {
                    BackpackId: 1,
                    Width: 7,
                    Capacity: 385,
                    ..Default::default()
                }),
                EquipRack: Some(HonamiStoryBackpack {
                    BackpackId: 2,
                    Width: 7,
                    Capacity: 63,
                    ..Default::default()
                }),
                RoleEquipList: role_equip_list,
                UnlockedWeaponIds: unlocked_weapons,
            }),
            ..Default::default()
        }),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_activate_talent_request(
    state: &mut State,
    req: HonamiStoryActivateTalentRequest,
) -> Result<HonamiStoryActivateTalentResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("honami:talent:{}", req.TalentId),
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStoryActivateTalentResponse {
        ErrorCode: Some(ErrorCode::Success),
        TalentId: req.TalentId,
    })
}

pub fn handle_honami_story_area_secret_reward_request(
    state: &mut State,
    req: HonamiStoryAreaSecretRewardRequest,
) -> Result<HonamiStoryAreaSecretRewardResponse, String> {
    claim_one(state, "honami_sec", "HonamiStoryArea", req.AreaSecretId)?;
    Ok(HonamiStoryAreaSecretRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_bag_operate_request(
    state: &mut State,
    req: HonamiStoryBagOperateRequest,
) -> Result<HonamiStoryBagOperateResponse, String> {
    pb_kv::save_msg(
        &state.fs,
        &mut state.player_components.progress,
        "honami:bag_op",
        &req,
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStoryBagOperateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_discard_item_request(
    state: &mut State,
    req: HonamiStoryDiscardItemRequest,
) -> Result<HonamiStoryDiscardItemResponse, String> {
    pb_kv::persist_opt_msg(
        &state.fs,
        &mut state.player_components.progress,
        "honami:discard",
        req.HonamiStoryBagUpdateContext.as_ref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStoryDiscardItemResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_pick_up_item_request(
    state: &mut State,
    req: HonamiStoryPickUpItemRequest,
) -> Result<HonamiStoryPickUpItemResponse, String> {
    pb_kv::persist_opt_msg(
        &state.fs,
        &mut state.player_components.progress,
        "honami:pickup",
        req.HonamiStoryBagUpdateInfo.as_ref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStoryPickUpItemResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_lock_item_request(
    state: &mut State,
    req: HonamiStoryLockItemRequest,
) -> Result<HonamiStoryLockItemResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("honami:lock:{}:{}", req.BackPackId, req.ItemUid),
        i64::from(req.IsLock),
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStoryLockItemResponse {
        ItemUid: req.ItemUid,
        IsLock: req.IsLock,
        BackPackId: req.BackPackId,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_sell_item_request(
    state: &mut State,
    req: HonamiStorySellItemRequest,
) -> Result<HonamiStorySellItemResponse, String> {
    pb_kv::save_msg(
        &state.fs,
        &mut state.player_components.progress,
        &format!("honami:sell:{}", req.ActivityId),
        &req,
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStorySellItemResponse {
        ActivityId: req.ActivityId,
        HonamiStoryItemUidInfos: req.HonamiStoryItemUidInfos,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_equip_role_request(
    state: &mut State,
    req: HonamiStoryEquipRoleRequest,
) -> Result<HonamiStoryEquipRoleResponse, String> {
    pb_kv::save_msg(
        &state.fs,
        &mut state.player_components.progress,
        &format!("honami:equip:{}", req.ActivityId),
        &req,
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStoryEquipRoleResponse {
        RoleId: req.RoleId,
        ActivityId: req.ActivityId,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_inst_leave_request(
    state: &mut State,
    _req: HonamiStoryInstLeaveRequest,
) -> Result<HonamiStoryInstLeaveResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "honami:in_inst",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStoryInstLeaveResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_item_collection_request(
    state: &mut State,
    req: HonamiStoryItemCollectionRequest,
) -> Result<HonamiStoryItemCollectionResponse, String> {
    claim_id_list(
        state,
        "honami:col",
        "HonamiStoryItemCollection",
        &req.HonamiStoryItemCollectionId,
    )?;
    Ok(HonamiStoryItemCollectionResponse {
        ActivityId: req.ActivityId,
        HonamiStoryItemCollectionId: req.HonamiStoryItemCollectionId,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_life_support_up_request(
    state: &mut State,
    req: HonamiStoryLifeSupportUpRequest,
) -> Result<HonamiStoryLifeSupportUpResponse, String> {
    let info = &mut state.player_components.progress.info;
    let cur = info.get_key_int("honami:life").max(info.get_key_int("honami:ls_lv")).max(1);
    let lv = (cur + 1) as i32;
    info.set_key_int("honami:life", i64::from(lv));
    info.set_key_int("honami:ls_lv", i64::from(lv));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(HonamiStoryLifeSupportUpResponse {
        ActivityId: req.ActivityId,
        LifeSupportLevel: lv,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_limit_task_reward_request(
    state: &mut State,
    req: HonamiStoryLimitTaskRewardRequest,
) -> Result<HonamiStoryLimitTaskRewardResponse, String> {
    claim_id_list(state, "honami:lim", "HonamiStoryLimitTask", &req.LimitTaskIds)?;
    if req.LimitTaskId != 0 {
        claim_one(state, "honami:lim", "HonamiStoryLimitTask", req.LimitTaskId)?;
    }
    Ok(HonamiStoryLimitTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_resident_task_reward_request(
    state: &mut State,
    req: HonamiStoryResidentTaskRewardRequest,
) -> Result<HonamiStoryResidentTaskRewardResponse, String> {
    claim_id_list(
        state,
        "honami:perm",
        "HonamiStoryResidentTask",
        &req.ResidentTaskIds,
    )?;
    Ok(HonamiStoryResidentTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_mascot_reward_request(
    state: &mut State,
    req: HonamiStoryMascotRewardRequest,
) -> Result<HonamiStoryMascotRewardResponse, String> {
    claim_one(
        state,
        "honami_mascot",
        "HonamiStoryMascot",
        req.HonamiStoryMascotId,
    )?;
    Ok(HonamiStoryMascotRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_score_reward_request(
    state: &mut State,
    req: HonamiStoryScoreRewardRequest,
) -> Result<HonamiStoryScoreRewardResponse, String> {
    claim_id_list(
        state,
        "honami:score",
        "HonamiStoryScoreReward",
        &req.ScoreRewardId,
    )?;
    Ok(HonamiStoryScoreRewardResponse {
        ScoreRewardId: req.ScoreRewardId,
        ActivityId: req.ActivityId,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_honami_story_unlock_slot_request(
    state: &mut State,
    req: HonamiStoryUnlockSlotRequest,
) -> Result<HonamiStoryUnlockSlotResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("honami:slot:{}:{}", req.RackIndex, req.SlotId),
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStoryUnlockSlotResponse {
        ErrorCode: Some(ErrorCode::Success),
        RackIndex: req.RackIndex,
        SlotId: req.SlotId,
    })
}

pub fn handle_honami_story_weapon_dress_request(
    state: &mut State,
    req: HonamiStoryWeaponDressRequest,
) -> Result<HonamiStoryWeaponDressResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("honami:weapon_dress:{}:{}", req.ActivityId, req.RackIndex),
        i64::from(req.WeaponItemId),
    )
    .map_err(|e| e.to_string())?;
    Ok(HonamiStoryWeaponDressResponse {
        ErrorCode: Some(ErrorCode::Success),
        RackIndex: req.RackIndex,
        WeaponItemId: req.WeaponItemId,
    })
}

// =========================================================================
// JokerGuess, Kurotato & Pinball
// =========================================================================

pub fn handle_joker_guess_start_new_request(
    state: &mut State,
    req: JokerGuessStartNewRequest,
) -> Result<JokerGuessStartNewResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "joker:cur_level",
        i64::from(req.LevelId),
    )
    .map_err(|e| e.to_string())?;
    Ok(JokerGuessStartNewResponse {
        ErrorCode: Some(ErrorCode::Success),
        InitData: Some(JokerGuessInitGame {
            FirstActor: Some(JokerGuessActor::Who),
            ..Default::default()
        }),
        Cmd: vec![JokerGuessOp {
            JokerGuessLevelId: req.LevelId,
            OpType: Some(JokerActionType::EnterNewRound),
            NewRoundAction: Some(JokerGuessRoundInfo {
                RoundNum: 1,
                ChangeResult: 0,
            }),
            ..Default::default()
        }],
    })
}

pub fn handle_joker_guess_draw_card_request(
    state: &mut State,
    req: JokerGuessDrawCardRequest,
) -> Result<JokerGuessDrawCardResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "joker:drawn_card",
        i64::from(req.TargetCardId),
    )
    .map_err(|e| e.to_string())?;
    let lv = state
        .player_components
        .progress
        .info
        .get_key_int("joker:cur_level")
        .max(0) as i32;
    Ok(JokerGuessDrawCardResponse {
        DrawCardResult: req.TargetCardId,
        ErrorCode: Some(ErrorCode::Success),
        Cmd: vec![JokerGuessOp {
            JokerGuessLevelId: lv,
            OpType: Some(JokerActionType::DrawCard),
            DrawAction: Some(JokerGuessDrawCardAction {
                Drawer: Some(JokerGuessActor::Who),
                DrawCard: req.TargetCardId,
                ..Default::default()
            }),
            ..Default::default()
        }],
    })
}

pub fn handle_joker_guess_play_card_request(
    state: &mut State,
    req: JokerGuessPlayCardRequest,
) -> Result<JokerGuessPlayCardResponse, String> {
    pb_kv::persist_opt_msg(
        &state.fs,
        &mut state.player_components.progress,
        "joker:last_play",
        req.PlayCard.as_ref(),
    )
    .map_err(|e| e.to_string())?;
    let lv = state
        .player_components
        .progress
        .info
        .get_key_int("joker:cur_level")
        .max(0) as i32;
    Ok(JokerGuessPlayCardResponse {
        ErrorCode: Some(ErrorCode::Success),
        Cmd: vec![JokerGuessOp {
            JokerGuessLevelId: lv,
            OpType: Some(JokerActionType::PlayCard),
            PlayAction: Some(JokerGuessPlayCardAction {
                PlayActor: Some(JokerGuessActor::Who),
                PlayCard: req.PlayCard,
            }),
            ..Default::default()
        }],
    })
}

pub fn handle_joker_guess_use_skill_request(
    state: &mut State,
    req: JokerGuessUseSkillRequest,
) -> Result<JokerGuessUseSkillResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "joker:last_skill",
        i64::from(req.SkillId),
    )
    .map_err(|e| e.to_string())?;
    let lv = state
        .player_components
        .progress
        .info
        .get_key_int("joker:cur_level")
        .max(0) as i32;
    Ok(JokerGuessUseSkillResponse {
        ErrorCode: Some(ErrorCode::Success),
        Cmd: vec![JokerGuessOp {
            JokerGuessLevelId: lv,
            OpType: Some(JokerActionType::ExecSkill),
            SkillExecAction: Some(JokerGuessSkillExecAction {
                JokerSkillId: req.SkillId,
                Owner: Some(JokerGuessActor::Who),
                Result: Some(req.Op.unwrap_or(JokerGuessSkillOp::Use)),
                ..Default::default()
            }),
            ..Default::default()
        }],
    })
}

pub fn handle_joker_guess_reward_request(
    state: &mut State,
    req: JokerGuessRewardRequest,
) -> Result<JokerGuessRewardResponse, String> {
    claim_one(state, "joker_rw", "JokerGuessLevel", req.LevelId)?;
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("guess_joker:done:{}", req.LevelId),
    )
    .map_err(|e| e.to_string())?;
    Ok(JokerGuessRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_sync_entity_deal_damage_push(
    state: &mut State,
    push: KurotatoSyncEntityDealDamagePush,
) -> Result<(), String> {
    let mut dirty = false;
    let info = &mut state.player_components.progress.info;
    for (k, v) in push.EntityDealDamages {
        info.add_key_int(&format!("kurotato:dmg:{k}"), i64::from(v));
        info.add_key_int("kurotato:dmg_total", i64::from(v));
        dirty = true;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn handle_kurotato_chest_reward_select_request(
    state: &mut State,
    req: KurotatoChestRewardSelectRequest,
) -> Result<KurotatoChestRewardSelectResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "kurotato:chest_sold",
        i64::from(req.WillSold),
    )
    .map_err(|e| e.to_string())?;
    Ok(KurotatoChestRewardSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_end_shop_step_request(
    state: &mut State,
    req: KurotatoEndShopStepRequest,
) -> Result<KurotatoEndShopStepResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "kurotato:endless",
        i64::from(req.IsEndLess),
    )
    .map_err(|e| e.to_string())?;
    Ok(KurotatoEndShopStepResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_end_special_wave_combat_request(
    state: &mut State,
    _req: KurotatoEndSpecialWaveCombatRequest,
) -> Result<KurotatoEndSpecialWaveCombatResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        "kurotato:spec_wave",
    )
    .map_err(|e| e.to_string())?;
    Ok(KurotatoEndSpecialWaveCombatResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_gold_add_request(
    state: &mut State,
    req: KurotatoGoldAddRequest,
) -> Result<KurotatoGoldAddResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("kurotato:gold", i64::from(req.GoldCount));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(KurotatoGoldAddResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_level_record_delete_request(
    state: &mut State,
    req: KurotatoLevelRecordDeleteRequest,
) -> Result<KurotatoLevelRecordDeleteResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("kurotato:lvl:{}", req.LevelId),
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(KurotatoLevelRecordDeleteResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_limit_task_receive_request(
    state: &mut State,
    req: KurotatoLimitTaskReceiveRequest,
) -> Result<KurotatoLimitTaskReceiveResponse, String> {
    claim_id_list(state, "kurotato:lim", "KurotatoLimitAward", &req.TaskIds)?;
    Ok(KurotatoLimitTaskReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_res_task_receive_request(
    state: &mut State,
    req: KurotatoResTaskReceiveRequest,
) -> Result<KurotatoResTaskReceiveResponse, String> {
    claim_id_list(state, "kurotato:res", "KurotatoResAward", &req.TaskIds)?;
    Ok(KurotatoResTaskReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_score_task_receive_request(
    state: &mut State,
    req: KurotatoScoreTaskReceiveRequest,
) -> Result<KurotatoScoreTaskReceiveResponse, String> {
    claim_id_list(state, "kurotato_st", "KurotatoScoreAward", &req.TaskIds)?;
    Ok(KurotatoScoreTaskReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_re_challenge_request(
    state: &mut State,
    req: KurotatoReChallengeRequest,
) -> Result<KurotatoReChallengeResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "kurotato:rechallenge_cur",
        i64::from(req.IsFromCurWave),
    )
    .map_err(|e| e.to_string())?;
    Ok(KurotatoReChallengeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_refine_weapon_request(
    state: &mut State,
    req: KurotatoRefineWeaponRequest,
) -> Result<KurotatoRefineWeaponResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("kurotato:refine_wp:{}", req.WeaponIncId), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(KurotatoRefineWeaponResponse {
        ErrorCode: Some(ErrorCode::Success),
        PanelPbData: Some(Default::default()),
    })
}

pub fn handle_kurotato_sell_weapon_request(
    state: &mut State,
    req: KurotatoSellWeaponRequest,
) -> Result<KurotatoSellWeaponResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.mark_claim(&format!("kurotato:sell_wp:{}", req.WeaponIncId));
    info.add_key_int("kurotato:gold", 10);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(KurotatoSellWeaponResponse {
        ErrorCode: Some(ErrorCode::Success),
        PanelPbData: Some(Default::default()),
    })
}

pub fn handle_kurotato_role_record_request(
    state: &mut State,
    _req: KurotatoRoleRecordRequest,
) -> Result<KurotatoRoleRecordResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        "kurotato:role_rec",
    )
    .map_err(|e| e.to_string())?;
    Ok(KurotatoRoleRecordResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_settlement_request(
    state: &mut State,
    _req: KurotatoSettlementRequest,
) -> Result<KurotatoSettlementResponse, String> {
    let kills = state
        .player_components
        .progress
        .info
        .get_key_int("simple_combat:kills") as i32;
    Ok(KurotatoSettlementResponse {
        ErrorCode: Some(ErrorCode::Success),
        KurotatoSettlementInfo: Some(KurotatoSettlementInfo {
            IsPass: true,
            CumulativeKills: kills,
            ..Default::default()
        }),
    })
}

pub fn handle_kurotato_shop_reward_lock_request(
    state: &mut State,
    req: KurotatoShopRewardLockRequest,
) -> Result<KurotatoShopRewardLockResponse, String> {
    let key = format!("kurotato:shop_lock:{}", req.SelectionId);
    let prev = state.player_components.progress.info.get_key_int(&key);
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &key,
        if prev == 0 { 1 } else { 0 },
    )
    .map_err(|e| e.to_string())?;
    Ok(KurotatoShopRewardLockResponse {
        ErrorCode: Some(ErrorCode::Success),
        PanelPbData: Some(Default::default()),
    })
}

pub fn handle_kurotato_shop_reward_refresh_request(
    state: &mut State,
    _req: KurotatoShopRewardRefreshRequest,
) -> Result<KurotatoShopRewardRefreshResponse, String> {
    let cnt = state
        .player_components
        .progress
        .info
        .add_key_int("kurotato:shop_ref", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(KurotatoShopRewardRefreshResponse {
        ErrorCode: Some(ErrorCode::Success),
        PanelPbData: Some(KurotatoShopPanelPbData {
            RefreshCost: cnt + 1,
            ..Default::default()
        }),
    })
}

pub fn handle_kurotato_shop_reward_select_request(
    state: &mut State,
    req: KurotatoShopRewardSelectRequest,
) -> Result<KurotatoShopRewardSelectResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("kurotato:shop_sel:{}", req.SelectionId),
    )
    .map_err(|e| e.to_string())?;
    Ok(KurotatoShopRewardSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
        PanelPbData: Some(Default::default()),
    })
}

pub fn handle_kurotato_step_advance_request(
    state: &mut State,
    _req: KurotatoStepAdvanceRequest,
) -> Result<KurotatoStepAdvanceResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("kurotato:step", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    {
        let mut qctx = state.quest_context(&mut events);
        if let Some(mut lp_ctx) = level_play::LevelPlayContext::from_quest_ctx(&mut qctx) {
            let _ = level_play::on_condition_event(
                &mut lp_ctx,
                LevelPlayEventKind::FinishKurotato,
                0,
                0,
            );
        }
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(KurotatoStepAdvanceResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kurotato_upgrade_reward_refresh_request(
    state: &mut State,
    _req: KurotatoUpgradeRewardRefreshRequest,
) -> Result<KurotatoUpgradeRewardRefreshResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("kurotato:upg_ref", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(KurotatoUpgradeRewardRefreshResponse {
        ErrorCode: Some(ErrorCode::Success),
        PanelPbData: Some(Default::default()),
    })
}

pub fn handle_kurotato_upgrade_reward_select_request(
    state: &mut State,
    req: KurotatoUpgradeRewardSelectRequest,
) -> Result<KurotatoUpgradeRewardSelectResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("kurotato:upg_sel:{}", req.SelectionId),
    )
    .map_err(|e| e.to_string())?;
    Ok(KurotatoUpgradeRewardSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_bonus_full_win_request(
    state: &mut State,
    _req: PinballBonusFullWinRequest,
) -> Result<PinballBonusFullWinResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        "pinball:full_win",
    )
    .map_err(|e| e.to_string())?;
    Ok(PinballBonusFullWinResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_friend_rank_request(
    state: &mut State,
    _req: PinballFriendRankRequest,
) -> Result<PinballFriendRankResponse, String> {
    let mut rank_data: Vec<PinballRankData> = CoopService::collect_server_player_snapshots(
        &state.fs,
        state.coop_service.as_deref(),
        state.player_id,
    )
    .into_iter()
    .filter(|p| p.progress.get_key_int("pinball:kicks") > 0)
    .map(|p| PinballRankData {
        Name: p.name.into_bytes(),
        TowerLevel: p.progress.get_key_int("pinball:tower").max(1) as i32,
        ..Default::default()
    })
    .collect();
    rank_data.sort_by_key(|a| std::cmp::Reverse(a.TowerLevel));
    Ok(PinballFriendRankResponse {
        ErrorCode: Some(ErrorCode::Success),
        RankData: rank_data,
        PlayerSelf: Some(PinballRankData {
            Name: state
                .player_components
                .basic
                .info
                .attributes
                .name
                .as_bytes()
                .to_vec(),
            TowerLevel: state
                .player_components
                .progress
                .info
                .get_key_int("pinball:tower")
                .max(1) as i32,
            ..Default::default()
        }),
    })
}

pub fn handle_pinball_in_game_kick_ball_request(
    state: &mut State,
    _req: PinballInGameKickBallRequest,
) -> Result<PinballInGameKickBallResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("pinball:kicks", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PinballInGameKickBallResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_in_game_sync_payload_request(
    state: &mut State,
    req: PinballInGameSyncPayloadRequest,
) -> Result<PinballInGameSyncPayloadResponse, String> {
    pb_kv::persist_opt_msg(
        &state.fs,
        &mut state.player_components.progress,
        "pinball:payload",
        req.Payload.as_ref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(PinballInGameSyncPayloadResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_quit_request(
    state: &mut State,
    req: PinballQuitRequest,
) -> Result<PinballQuitResponse, String> {
    pb_kv::persist_opt_msg(
        &state.fs,
        &mut state.player_components.progress,
        "pinball:last_quit",
        req.Payload.as_ref(),
    )
    .map_err(|e| e.to_string())?;
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "pinball:in_game",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(PinballQuitResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_role_level_up_request(
    state: &mut State,
    req: PinballRoleLevelUpRequest,
) -> Result<PinballRoleLevelUpResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("pinball:role_lv:{}", req.RoleId), i64::from(req.AddLv));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PinballRoleLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_set_payload_request(
    state: &mut State,
    req: PinballSetPayloadRequest,
) -> Result<PinballSetPayloadResponse, String> {
    pb_kv::persist_opt_msg(
        &state.fs,
        &mut state.player_components.progress,
        "pinball:payload",
        req.Payload.as_ref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(PinballSetPayloadResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_task_reward_request(
    state: &mut State,
    req: PinballTaskRewardRequest,
) -> Result<PinballTaskRewardResponse, String> {
    claim_id_list(state, "pinball:task", "PinballTask", &req.TaskIds)?;
    Ok(PinballTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_team_die_request(
    state: &mut State,
    _req: PinballTeamDieRequest,
) -> Result<PinballTeamDieResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("pinball:deaths", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PinballTeamDieResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_team_revive_request(
    state: &mut State,
    _req: PinballTeamReviveRequest,
) -> Result<PinballTeamReviveResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("pinball:revives", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PinballTeamReviveResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_weapon_degrade_request(
    state: &mut State,
    req: PinballWeaponDegradeRequest,
) -> Result<PinballWeaponDegradeResponse, String> {
    let mut dirty = false;
    for wid in req.WeaponIncIds {
        state
            .player_components
            .progress
            .info
            .add_key_int(&format!("pinball:wp_deg:{wid}"), 1);
        dirty = true;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(PinballWeaponDegradeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pinball_weapon_wear_request(
    state: &mut State,
    req: PinballWeaponWearRequest,
) -> Result<PinballWeaponWearResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("pinball:role_wp:{}", req.RoleId),
        i64::from(req.WeaponInc),
    )
    .map_err(|e| e.to_string())?;
    Ok(PinballWeaponWearResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

// =========================================================================
// RacingBets, Regress, Rhythm & RollBlock
// =========================================================================

pub fn handle_racing_bet_match_info_request(
    state: &mut State,
    req: RacingBetMatchInfoRequest,
) -> Result<RacingBetMatchInfoResponse, String> {
    let list = (1..=6)
        .map(|did| RacingBetsInstanceDangoInfo {
            DangoId: did,
            CurPoint: state
                .player_components
                .progress
                .info
                .get_key_int(&format!("rb:bet:{}:{did}", req.LegMatchId)) as i32,
            ..Default::default()
        })
        .collect();
    Ok(RacingBetMatchInfoResponse {
        DangoInfo: list,
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn racing_bet_ensure_cash(info: &mut sonoro_gamesv_fs::progress_info::ProgressInfo) -> i32 {
    if !info.has_claim("rb:cash_init") {
        info.mark_claim("rb:cash_init");
        if info.get_key_int("rb:cash") <= 0 {
            info.set_key_int("rb:cash", 1000);
        }
    }
    info.get_key_int("rb:cash").max(0) as i32
}

pub fn handle_racing_bet_match_action_request(
    state: &mut State,
    req: RacingBetMatchActionRequest,
) -> Result<RacingBetMatchActionResponse, String> {
    let info = &mut state.player_components.progress.info;
    let _ = racing_bet_ensure_cash(info);
    let round_key = format!("rb:round:{}:{}", req.ActivityId, req.LegMatchId);
    if info.get_key_int(&round_key) <= 0 {
        info.set_key_int(&round_key, 1);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RacingBetMatchActionResponse {
        ActivityId: req.ActivityId,
        LegMatchId: req.LegMatchId,
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_racing_bet_match_round_action_request(
    state: &mut State,
    req: RacingBetMatchRoundActionRequest,
) -> Result<RacingBetMatchRoundActionResponse, String> {
    let info = &mut state.player_components.progress.info;
    let cash = racing_bet_ensure_cash(info);
    let round = req.Order.max(1);
    let is_final = round >= 3;
    info.set_key_int(
        &format!("rb:round:{}:{}", req.ActivityId, req.LegMatchId),
        i64::from(round),
    );
    let settle_claim = format!("rb:settled:{}:{}", req.ActivityId, req.LegMatchId);
    if is_final && info.mark_claim(&settle_claim) {
        let winner = ((req.LegMatchId.abs() % 6) + 1) as i32;
        info.set_key_int(&format!("rb:winner:{}", req.LegMatchId), i64::from(winner));
        let bet = info.get_key_int(&format!("rb:bet:{}:{}", req.LegMatchId, winner)).max(0);
        if bet > 0 {
            let odds = i64::from(200 + winner * 20);
            let payout = (bet * odds) / 100;
            info.set_key_int("rb:cash", i64::from(cash) + payout);
            info.add_key_int("rb:hits", 1);
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RacingBetMatchRoundActionResponse {
        ErrorCode: Some(ErrorCode::Success),
        Rounds: Some(RacingBetsMatchRoundInfo {
            RoundRank: round,
            IsFinalRound: is_final,
            ..Default::default()
        }),
        IsResultRound: is_final,
        ..Default::default()
    })
}

pub fn handle_racing_bets_bullet_screen_request(
    state: &mut State,
    req: RacingBetsBulletScreenRequest,
) -> Result<RacingBetsBulletScreenResponse, String> {
    state.player_components.progress.info.add_key_int(
        &format!("rb:bullet:{}:{}", req.LegMatchId, req.BulletScreenId),
        1,
    );
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RacingBetsBulletScreenResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_racing_bets_close_settle_menu_request(
    state: &mut State,
    req: RacingBetsCloseSettleMenuRequest,
) -> Result<RacingBetsCloseSettleMenuResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rb:close_settle:{}:{}", req.ActivityId, req.LegMatchId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RacingBetsCloseSettleMenuResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_racing_bets_gear_request(
    state: &mut State,
    req: RacingBetsGearRequest,
) -> Result<RacingBetsGearResponse, String> {
    let info = &mut state.player_components.progress.info;
    let mut cash = racing_bet_ensure_cash(info);
    if req.Cash <= 0 || req.Cash > cash {
        return Ok(RacingBetsGearResponse {
            ErrorCode: Some(if req.Cash <= 0 {
                ErrorCode::RequestParamError
            } else {
                ErrorCode::ErrItemNotEnough
            }),
            PlayerInfo: Some(RacingBetPlayerInfo {
                CurCash: cash,
                TotalCash: cash,
                ..Default::default()
            }),
        });
    }
    cash -= req.Cash;
    info.set_key_int("rb:cash", i64::from(cash));
    let bet_key = format!("rb:bet:{}:{}", req.LegMatchId, req.DangoId);
    info.add_key_int(&bet_key, i64::from(req.Cash));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RacingBetsGearResponse {
        ErrorCode: Some(ErrorCode::Success),
        PlayerInfo: Some(RacingBetPlayerInfo {
            CurCash: cash,
            TotalCash: cash,
            ..Default::default()
        }),
    })
}

pub fn handle_racing_bets_gear_refund_request(
    state: &mut State,
    req: RacingBetsGearRefundRequest,
) -> Result<RacingBetsGearRefundResponse, String> {
    let info = &mut state.player_components.progress.info;
    let mut cash = racing_bet_ensure_cash(info);
    let bet_key = format!("rb:bet:{}:{}", req.LegMatchId, req.DangoId);
    let prev_bet = info.get_key_int(&bet_key).max(0) as i32;
    if prev_bet > 0 {
        cash = cash.saturating_add(prev_bet);
        info.set_key_int("rb:cash", i64::from(cash));
        info.set_key_int(&bet_key, 0);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RacingBetsGearRefundResponse {
        ErrorCode: Some(ErrorCode::Success),
        PlayerInfo: Some(RacingBetPlayerInfo {
            CurCash: cash,
            TotalCash: cash,
            ..Default::default()
        }),
    })
}

pub fn handle_racing_bets_match_info_request(
    state: &mut State,
    req: RacingBetsMatchInfoRequest,
) -> Result<RacingBetsMatchInfoResponse, String> {
    let _cash = racing_bet_ensure_cash(&mut state.player_components.progress.info);
    Ok(RacingBetsMatchInfoResponse {
        MatchInfos: vec![RacingBetsGroupMatchesInfo {
            GroupMatchId: 1,
            Dangos: (1..=6).collect(),
            ..Default::default()
        }],
        ErrorCode: Some(ErrorCode::Success),
        ActivityId: req.ActivityId,
        ..Default::default()
    })
}

pub fn handle_racing_bets_rank_request(
    state: &mut State,
    _req: RacingBetsRankRequest,
) -> Result<RacingBetsRankResponse, String> {
    let cash = state
        .player_components
        .progress
        .info
        .get_key_int("rb:cash")
        .max(1000) as i32;
    let mut players: Vec<RacingBetRankPlayerInfo> = CoopService::collect_server_player_snapshots(
        &state.fs,
        state.coop_service.as_deref(),
        state.player_id,
    )
    .into_iter()
    .map(|p| {
        let p_cash = p.progress.get_key_int("rb:cash").max(1000) as i32;
        RacingBetRankPlayerInfo {
            PlayerId: p.player_id,
            HeadIcon: p.head_id,
            Name: p.name.into_bytes(),
            Cash: p_cash,
            HitNum: p.progress.get_key_int("rb:hits").max(0) as i32,
            Rank: 0,
            RankNameless: false,
        }
    })
    .collect();
    players.sort_by_key(|a| std::cmp::Reverse(a.Cash));
    for (idx, p) in players.iter_mut().enumerate() {
        p.Rank = (idx + 1) as i32;
    }
    Ok(RacingBetsRankResponse {
        ErrorCode: Some(ErrorCode::Success),
        Status: Some(RacingBetsRankStatus::Rank),
        PlayerInfo: players,
        SelfData: Some(RacingBetRankSelfData {
            HeadIcon: state.player_components.basic.info.attributes.head_photo,
            Name: state
                .player_components
                .basic
                .info
                .attributes
                .name
                .as_bytes()
                .to_vec(),
            Cash: cash,
            HitNum: state
                .player_components
                .progress
                .info
                .get_key_int("rb:hits")
                .max(0) as i32,
        }),
        ..Default::default()
    })
}

pub fn handle_racing_bets_update_odds_request(
    state: &mut State,
    req: RacingBetsUpdateOddsRequest,
) -> Result<RacingBetsUpdateOddsResponse, String> {
    let dango_infos = (1..=6)
        .map(|did| RacingBetsDangoInfo {
            Id: did,
            CurPoint: state
                .player_components
                .progress
                .info
                .get_key_int(&format!("rb:bet:{}:{did}", req.LegMatchId)) as i32,
            Odds: 200 + did * 20,
        })
        .collect();
    Ok(RacingBetsUpdateOddsResponse {
        DangoInfo: dango_infos,
        OddsVersion: b"1".to_vec(),
        ActivityId: req.ActivityId,
        LegMatchId: req.LegMatchId,
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_racing_bets_task_reward_request(
    state: &mut State,
    req: RacingBetsTaskRewardRequest,
) -> Result<RacingBetsTaskRewardResponse, String> {
    claim_one(state, "rb_task", "RacingBetsTask", req.TaskId)?;
    Ok(RacingBetsTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_racing_bets_repeated_task_reward_request(
    state: &mut State,
    req: RacingBetsRepeatedTaskRewardRequest,
) -> Result<RacingBetsRepeatedTaskRewardResponse, String> {
    claim_id_list(state, "rb_rtask", "RacingBetsTask", &req.TaskId)?;
    Ok(RacingBetsRepeatedTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_regress_bonus_request(
    state: &mut State,
    req: RegressBonusRequest,
) -> Result<RegressBonusResponse, String> {
    claim_id_list(state, "reg_bonus", "RegressBonus", &req.Ids)?;
    Ok(RegressBonusResponse {
        ErrorCode: Some(ErrorCode::Success),
        Ids: req.Ids,
        ..Default::default()
    })
}

pub fn handle_regress_buy_bonus_lv_request(
    state: &mut State,
    req: RegressBuyBonusLvRequest,
) -> Result<RegressBuyBonusLvResponse, String> {
    let lv = state
        .player_components
        .progress
        .info
        .add_key_int("regress:bonus_lv", i64::from(req.BuyLv)) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RegressBuyBonusLvResponse {
        ErrorCode: Some(ErrorCode::Success),
        BonusNum: lv,
    })
}

pub fn handle_regress_disposable_reward_request(
    state: &mut State,
    _req: RegressDisposableRewardRequest,
) -> Result<RegressDisposableRewardResponse, String> {
    claim_one(state, "reg_disp", "RegressDisposableReward", 1)?;
    Ok(RegressDisposableRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_regress_question_open_request(
    state: &mut State,
    req: RegressQuestionOpenRequest,
) -> Result<RegressQuestionOpenResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("reg_q:{}", req.QuestionId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RegressQuestionOpenResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_regress_task_request(
    state: &mut State,
    req: RegressTaskRequest,
) -> Result<RegressTaskResponse, String> {
    claim_one(state, "regress:task", "RegressTask", req.TaskId)?;
    Ok(RegressTaskResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_regress_repeated_task_request(
    state: &mut State,
    req: RegressRepeatedTaskRequest,
) -> Result<RegressRepeatedTaskResponse, String> {
    claim_id_list(state, "regress:task", "RegressTask", &req.TaskId)?;
    Ok(RegressRepeatedTaskResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_regress_set_cur_use_trial_role_request(
    state: &mut State,
    req: RegressSetCurUseTrialRoleRequest,
) -> Result<RegressSetCurUseTrialRoleResponse, String> {
    let prev = state
        .player_components
        .progress
        .info
        .get_key_int("reg:trial_role") as i32;
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "reg:trial_role",
        i64::from(req.TrialRoleId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RegressSetCurUseTrialRoleResponse {
        Code: Some(ErrorCode::Success),
        ReplacedRoleId: prev,
        RoleId: req.TrialRoleId,
        ReplacedCurRole: prev != 0,
        ..Default::default()
    })
}

pub fn handle_regress_trial_role_lv_up_request(
    state: &mut State,
    req: RegressTrialRoleLvUpRequest,
) -> Result<RegressTrialRoleLvUpResponse, String> {
    let lv = state
        .player_components
        .progress
        .info
        .add_key_int(&format!("reg_role_lv:{}", req.TrialRoleId), 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RegressTrialRoleLvUpResponse {
        RoleId: req.TrialRoleId,
        CurUseRoleInfo: Some(roleInfo {
            RoleId: req.TrialRoleId,
            Level: lv,
            ..Default::default()
        }),
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_rhythm_change_partner_role_request(
    state: &mut State,
    req: RhythmChangePartnerRoleRequest,
) -> Result<RhythmChangePartnerRoleResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rhythm:role",
        i64::from(req.RhythmRoleId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RhythmChangePartnerRoleResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rhythm_main_line_info_request(
    state: &mut State,
    _req: RhythmMainLineInfoRequest,
) -> Result<RhythmMainLineInfoResponse, String> {
    let role_id = state
        .player_components
        .progress
        .info
        .get_key_int("rhythm:role") as i32;
    Ok(RhythmMainLineInfoResponse {
        RhythmMainLinePb: Some(RhythmActivityPb {
            RhythmRoleId: role_id,
            ..Default::default()
        }),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rhythm_rank_list_request(
    state: &mut State,
    _req: RhythmRankListRequest,
) -> Result<RhythmRankListResponse, String> {
    let friends = CoopService::collect_server_player_snapshots(
        &state.fs,
        state.coop_service.as_deref(),
        state.player_id,
    )
    .into_iter()
    .map(|p| RhythmPlayerRankingInfo {
        PlayerId: p.player_id,
        Name: p.name.into_bytes(),
        SubLevelInfo: Vec::new(),
    })
    .collect();
    Ok(RhythmRankListResponse {
        SelfData: Some(RhythmPlayerRankingInfo {
            PlayerId: state.player_id,
            Name: state
                .player_components
                .basic
                .info
                .attributes
                .name
                .as_bytes()
                .to_vec(),
            ..Default::default()
        }),
        ErrorCode: Some(ErrorCode::Success),
        FriendData: friends,
    })
}

pub fn handle_rhythm_rank_list_self_data_request(
    state: &mut State,
    _req: RhythmRankListSelfDataRequest,
) -> Result<RhythmRankListSelfDataResponse, String> {
    Ok(RhythmRankListSelfDataResponse {
        SelfData: Some(RhythmPlayerRankingInfo {
            PlayerId: state.player_id,
            Name: state
                .player_components
                .basic
                .info
                .attributes
                .name
                .as_bytes()
                .to_vec(),
            ..Default::default()
        }),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rhythm_set_red_dot_request(
    state: &mut State,
    req: RhythmSetRedDotRequest,
) -> Result<RhythmSetRedDotResponse, String> {
    pb_kv::persist_opt_msg(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rhythm:reddot:{}", req.ActivityId),
        req.RedDot.as_ref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(RhythmSetRedDotResponse {
        ErrorCode: Some(ErrorCode::Success),
        RedDot: req.RedDot,
    })
}

pub fn handle_rhythm_settle_request(
    state: &mut State,
    req: RhythmSettleRequest,
) -> Result<RhythmSettleResponse, String> {
    let score = req.Payload.as_ref().map(|p| p.Score).unwrap_or(0);
    let key = format!("rhythm:best:{}", req.SubLevelId);
    let prev = state.player_components.progress.info.get_key_int(&key) as i32;
    let best_flag = score > prev;
    let best = score.max(prev);
    let mut dirty = false;
    if best_flag {
        state
            .player_components
            .progress
            .info
            .set_key_int(&key, i64::from(best));
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("rhythm:score:{}", req.SubLevelId), i64::from(best));
        dirty = true;
    }
    if let Some(p) = &req.Payload
        && let Some(r) = p.Rank
    {
        let rank_val = r as i64;
        if rank_val > 0 {
            let rkey = format!("rhythm:rank:{}", req.SubLevelId);
            let prev_rank = state.player_components.progress.info.get_key_int(&rkey);
            if prev_rank <= 0 || rank_val < prev_rank {
                state
                    .player_components
                    .progress
                    .info
                    .set_key_int(&rkey, rank_val);
                dirty = true;
            }
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(RhythmSettleResponse {
        BestScore: best,
        BestScoreFlag: best_flag,
        ErrCode: Some(ErrorCode::Success),
        Payload: req.Payload,
    })
}

pub fn handle_rhythm_task_one_key_reward_request(
    state: &mut State,
    req: RhythmTaskOneKeyRewardRequest,
) -> Result<RhythmTaskOneKeyRewardResponse, String> {
    claim_id_list(state, "rhythm:task", "RhythmTask", &req.TaskId)?;
    Ok(RhythmTaskOneKeyRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_roll_block_exit_game_play_request(
    state: &mut State,
    req: RollBlockExitGamePlayRequest,
) -> Result<RollBlockExitGamePlayResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rb_blk:active:{}", req.IncId),
        0,
    )
    .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    let code = {
        let mut qctx = state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::roll_block::handle_exit(&mut qctx, req.IncId)
            .map_err(|e| e.to_string())?
    };
    drain_event_queue_best_effort(&mut events, state);
    Ok(RollBlockExitGamePlayResponse { Code: Some(code) })
}

pub fn handle_roll_block_game_play_ready_request(
    state: &mut State,
    req: RollBlockGamePlayReadyRequest,
) -> Result<RollBlockGamePlayReadyResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rb_blk:active:{}", req.IncId),
        1,
    )
    .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    let code = {
        let mut qctx = state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::roll_block::handle_ready(&mut qctx, req.IncId)
    };
    drain_event_queue_best_effort(&mut events, state);
    Ok(RollBlockGamePlayReadyResponse { Code: Some(code) })
}

pub fn handle_roll_block_input_request(
    state: &mut State,
    req: RollBlockInputRequest,
) -> Result<RollBlockInputResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("rb_blk:moves:{}", req.IncId), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    let code = {
        let mut qctx = state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::roll_block::handle_input(&mut qctx, req.IncId, req.Input)
    };
    drain_event_queue_best_effort(&mut events, state);
    Ok(RollBlockInputResponse { Code: Some(code) })
}

pub fn handle_roll_block_movement_finish_request(
    state: &mut State,
    req: RollBlockMovementFinishRequest,
) -> Result<RollBlockMovementFinishResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rb_blk:entity:{}", req.IncId),
        req.EntityId,
    )
    .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    let code = {
        let mut qctx = state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::roll_block::handle_movement_finish(
            &mut qctx,
            req.IncId,
            req.EntityId,
        )
        .map_err(|e| e.to_string())?
    };
    drain_event_queue_best_effort(&mut events, state);
    Ok(RollBlockMovementFinishResponse { Code: Some(code) })
}

pub fn handle_roll_block_reset_game_play_request(
    state: &mut State,
    req: RollBlockResetGamePlayRequest,
) -> Result<RollBlockResetGamePlayResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rb_blk:moves:{}", req.IncId),
        0,
    )
    .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    let code = {
        let mut qctx = state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::roll_block::handle_reset(&mut qctx, req.IncId)
    };
    drain_event_queue_best_effort(&mut events, state);
    Ok(RollBlockResetGamePlayResponse { Code: Some(code) })
}

pub fn handle_roll_block_switch_control_block_request(
    state: &mut State,
    req: RollBlockSwitchControlBlockRequest,
) -> Result<RollBlockSwitchControlBlockResponse, String> {
    let mut events = EventQueue::new();
    let (prev, next) = {
        let mut qctx = state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::roll_block::handle_switch_control_block(
            &mut qctx,
            req.IncId,
        )
    };
    drain_event_queue_best_effort(&mut events, state);
    Ok(RollBlockSwitchControlBlockResponse {
        PreMainControlBlockEntityId: prev,
        CurMainControlBlockEntityId: next,
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_roll_block_toggle_hint_request(
    state: &mut State,
    req: RollBlockToggleHintRequest,
) -> Result<RollBlockToggleHintResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rb_blk:hint:{}", req.IncId),
        i64::from(req.IsOpen),
    )
    .map_err(|e| e.to_string())?;
    let code = sonoro_gamesv_logic::helpers::roll_block::handle_toggle_hint(
        state.scene.as_mut(),
        req.IncId,
        req.IsOpen,
    );
    Ok(RollBlockToggleHintResponse { Code: Some(code) })
}

// =========================================================================
// Spring Festival, Survivors, TowerDefence & TrapDefense
// =========================================================================

pub fn handle_spring_festival_atmosphere_reward_request(
    state: &mut State,
    req: SpringFestivalAtmosphereRewardRequest,
) -> Result<SpringFestivalAtmosphereRewardResponse, String> {
    claim_id_list(state, "sf_atm", "SpringFestivalReward", &req.LevelIds)?;
    if !req.LevelIds.is_empty() {
        state
            .player_components
            .progress
            .info
            .add_key_int("spring:atmosphere", req.LevelIds.len() as i64);
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(SpringFestivalAtmosphereRewardResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_spring_festival_score_reward_request(
    state: &mut State,
    req: SpringFestivalScoreRewardRequest,
) -> Result<SpringFestivalScoreRewardResponse, String> {
    claim_id_list(state, "sf_score", "SpringFestivalReward", &req.ScoreIds)?;
    Ok(SpringFestivalScoreRewardResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_spring_festival_task_reward_request(
    state: &mut State,
    req: SpringFestivalTaskRewardRequest,
) -> Result<SpringFestivalTaskRewardResponse, String> {
    claim_id_list(state, "sf_task", "SpringFestivalReward", &req.TaskIds)?;
    Ok(SpringFestivalTaskRewardResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_spring_sign_draw_reward_request(
    state: &mut State,
    req: SpringSignDrawRewardRequest,
) -> Result<SpringSignDrawRewardResponse, String> {
    claim_one(state, "spring_sign", "SpringReward", req.Id)?;
    Ok(SpringSignDrawRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_spring_sign_draw_role_request(
    state: &mut State,
    _req: SpringSignDrawRoleRequest,
) -> Result<SpringSignDrawRoleResponse, String> {
    let draw_id = state
        .player_components
        .progress
        .info
        .add_key_int("ssign:draw_idx", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(SpringSignDrawRoleResponse {
        DrawId: draw_id,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_spring_sign_photo_reward_request(
    state: &mut State,
    _req: SpringSignPhotoRewardRequest,
) -> Result<SpringSignPhotoRewardResponse, String> {
    claim_one(state, "ssign_photo", "SpringReward", 1)?;
    Ok(SpringSignPhotoRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_spring_sign_skin_reward_request(
    state: &mut State,
    _req: SpringSignSkinRewardRequest,
) -> Result<SpringSignSkinRewardResponse, String> {
    claim_one(state, "spring_sign:skin", "SpringReward", 1)?;
    Ok(SpringSignSkinRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_survivors_last_info_request(
    state: &mut State,
    _req: SurvivorsLastInfoRequest,
) -> Result<SurvivorsLastInfoResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(SurvivorsLastInfoResponse {
        SurvivorLastInfo: Some(SurvivorLastInfo {
            LevelId: pc.get_key_int("surv:level") as i32,
            WaveId: pc.get_key_int("surv:wave").max(1) as i32,
            MaxWaveId: pc.get_key_int("surv:max_wave").max(1) as i32,
            ..Default::default()
        }),
    })
}

pub fn handle_survivors_lock_request(
    state: &mut State,
    req: SurvivorsLockRequest,
) -> Result<SurvivorsLockResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("surv:lock:{}", req.ClientSelectId),
        i64::from(req.IsLock),
    )
    .map_err(|e| e.to_string())?;
    Ok(SurvivorsLockResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_survivors_normal_award_request(
    state: &mut State,
    req: SurvivorsNormalAwardRequest,
) -> Result<SurvivorsNormalAwardResponse, String> {
    claim_id_list(state, "surv:task", "SurvivorsTask", &req.Configs)?;
    Ok(SurvivorsNormalAwardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_survivors_score_award_request(
    state: &mut State,
    req: SurvivorsScoreAwardRequest,
) -> Result<SurvivorsScoreAwardResponse, String> {
    claim_id_list(state, "surv_score", "SurvivorsScoreReward", &req.Configs)?;
    Ok(SurvivorsScoreAwardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_survivors_op_done_request(
    state: &mut State,
    req: SurvivorsOpDoneRequest,
) -> Result<SurvivorsOpDoneResponse, String> {
    let key = format!("surv:op:{}", req.IncId);
    pb_kv::save_msg(&state.fs, &mut state.player_components.progress, &key, &req)
        .map_err(|e| e.to_string())?;
    Ok(SurvivorsOpDoneResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_survivors_step_advance_request(
    state: &mut State,
    _req: SurvivorsStepAdvanceRequest,
) -> Result<SurvivorsStepAdvanceResponse, String> {
    let info = &mut state.player_components.progress.info;
    let wave = info.add_key_int("surv:wave", 1);
    let max_w = wave.max(info.get_key_int("surv:max_wave"));
    info.set_key_int("surv:max_wave", max_w);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    {
        let mut qctx = state.quest_context(&mut events);
        if let Some(mut lp_ctx) = level_play::LevelPlayContext::from_quest_ctx(&mut qctx) {
            let _ = level_play::on_condition_event(
                &mut lp_ctx,
                LevelPlayEventKind::FinishSurvivorsRouge,
                0,
                wave as i32,
            );
        }
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(SurvivorsStepAdvanceResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_survivors_result_request(
    state: &mut State,
    _req: SurvivorsResultRequest,
) -> Result<SurvivorsResultResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(SurvivorsResultResponse {
        ErrCode: Some(ErrorCode::Success),
        ResultView: Some(ResultView {
            LevelId: pc.get_key_int("surv:level") as i32,
            WaveId: pc.get_key_int("surv:wave").max(1) as i32,
            IsFinishCond: true,
            KillMonsterCount: pc.get_key_int("simple_combat:kills") as i32,
            ..Default::default()
        }),
    })
}

pub fn handle_survivors_talent_level_up_request(
    state: &mut State,
    req: SurvivorsTalentLevelUpRequest,
) -> Result<SurvivorsTalentLevelUpResponse, String> {
    let lv = state
        .player_components
        .progress
        .info
        .add_key_int(&format!("surv:talent:{}", req.SkillId), 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(SurvivorsTalentLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
        Level: lv,
    })
}

pub fn handle_survivors_weapon_info_request(
    state: &mut State,
    _req: SurvivorsWeaponInfoRequest,
) -> Result<SurvivorsWeaponInfoResponse, String> {
    let mut kills = std::collections::BTreeMap::new();
    for entry in &state.player_components.progress.info.dynamic.ints {
        if let Some(rest) = entry.key.strip_prefix("surv:wp_kill:")
            && let Ok(wid) = rest.parse::<i32>()
        {
            kills.insert(wid, entry.val as i32);
        }
    }
    Ok(SurvivorsWeaponInfoResponse {
        WeaponKillDict: kills,
    })
}

pub fn handle_tower_defence_challenge_quit_request(
    state: &mut State,
    _req: TowerDefenceChallengeQuitRequest,
) -> Result<TowerDefenceChallengeQuitResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "td:in_challenge",
        0,
    )
    .map_err(|e| e.to_string())?;
    let score = state
        .player_components
        .progress
        .info
        .get_key_int("td:last_score")
        .max(1000) as i32;
    Ok(TowerDefenceChallengeQuitResponse {
        ErrorCode: Some(ErrorCode::Success),
        Score: score,
    })
}

pub fn handle_tower_defence_instance_reward_request(
    state: &mut State,
    req: TowerDefenceInstanceRewardRequest,
) -> Result<TowerDefenceInstanceRewardResponse, String> {
    claim_id_list(state, "td:inst", "TowerDefenceReward", &req.Ids)?;
    Ok(TowerDefenceInstanceRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_tower_defence_score_reward_request(
    state: &mut State,
    req: TowerDefenceScoreRewardRequest,
) -> Result<TowerDefenceScoreRewardResponse, String> {
    claim_id_list(state, "td:score", "TowerDefenceReward", &req.Ids)?;
    Ok(TowerDefenceScoreRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_tower_defence_rand_list_request(
    state: &mut State,
    _req: TowerDefenceRandListRequest,
) -> Result<TowerDefenceRandListResponse, String> {
    Ok(TowerDefenceRandListResponse {
        ShowName: state
            .player_components
            .progress
            .info
            .get_key_int("td:show_name")
            != 0,
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_tower_defence_rank_show_name_request(
    state: &mut State,
    req: TowerDefenceRankShowNameRequest,
) -> Result<TowerDefenceRankShowNameResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "td:show_name",
        i64::from(req.ShowName),
    )
    .map_err(|e| e.to_string())?;
    Ok(TowerDefenceRankShowNameResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_tower_defence_self_pass_request(
    state: &mut State,
    req: TowerDefenceSelfPassRequest,
) -> Result<TowerDefenceSelfPassResponse, String> {
    let pc = &state.player_components.progress.info;
    let score = pc.get_key_int("td:last_score").max(0) as i32;
    let pass_time = pc.get_key_int("td:best_time").max(0) as i32;
    let list = if score > 0 || req.Id != 0 {
        vec![OneTowerDefencePassInfo {
            ChallengeId: req.Id,
            IsSingle: true,
            PassInfo: Some(TowerDefencePassInfo {
                Score: score,
                PassTime: pass_time,
                ..Default::default()
            }),
        }]
    } else {
        Vec::new()
    };
    Ok(TowerDefenceSelfPassResponse {
        SelfPassData: list,
    })
}

pub fn handle_trap_defense_auxiliary_branch_request(
    state: &mut State,
    req: TrapDefenseAuxiliaryBranchRequest,
) -> Result<TrapDefenseAuxiliaryBranchResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("trap:aux_br:{}", req.Id),
        i64::from(req.Branch),
    )
    .map_err(|e| e.to_string())?;
    Ok(TrapDefenseAuxiliaryBranchResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_auxiliary_level_up_request(
    state: &mut State,
    req: TrapDefenseAuxiliaryLevelUpRequest,
) -> Result<TrapDefenseAuxiliaryLevelUpResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("trap:aux_lv:{}", req.Id), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(TrapDefenseAuxiliaryLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_auxiliary_reset_request(
    state: &mut State,
    req: TrapDefenseAuxiliaryResetRequest,
) -> Result<TrapDefenseAuxiliaryResetResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("trap:aux_lv:{}", req.Id),
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(TrapDefenseAuxiliaryResetResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_building_branch_request(
    state: &mut State,
    req: TrapDefenseBuildingBranchRequest,
) -> Result<TrapDefenseBuildingBranchResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("trap:bld_br:{}", req.Id),
        i64::from(req.Branch),
    )
    .map_err(|e| e.to_string())?;
    Ok(TrapDefenseBuildingBranchResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_building_level_up_request(
    state: &mut State,
    req: TrapDefenseBuildingLevelUpRequest,
) -> Result<TrapDefenseBuildingLevelUpResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("trap:bld_lv:{}", req.Id), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(TrapDefenseBuildingLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_building_reset_request(
    state: &mut State,
    req: TrapDefenseBuildingResetRequest,
) -> Result<TrapDefenseBuildingResetResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("trap:bld_lv:{}", req.Id),
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(TrapDefenseBuildingResetResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_buy_shop_product_request(
    state: &mut State,
    req: TrapDefenseBuyShopProductRequest,
) -> Result<TrapDefenseBuyShopProductResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.add_key_int(
        &format!("trap:shop_buy:{}", req.ItemConfigId),
        i64::from(req.BuyCount.max(1)),
    );
    let rem = (5 - info.get_key_int("trap:shop_ref")).max(0) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(TrapDefenseBuyShopProductResponse {
        ErrorCode: Some(ErrorCode::Success),
        ShopInfo: Some(TrapDefenseShopPanelInfo {
            TotalRefreshTimes: 5,
            RemainRefreshTimes: rem,
            ..Default::default()
        }),
    })
}

pub fn handle_trap_defense_challenge_quit_request(
    state: &mut State,
    req: TrapDefenseChallengeQuitRequest,
) -> Result<TrapDefenseChallengeQuitResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "trap:in_challenge",
        if req.Settle { 0 } else { 2 },
    )
    .map_err(|e| e.to_string())?;
    Ok(TrapDefenseChallengeQuitResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_cur_challenge_info_request(
    state: &mut State,
    _req: TrapDefenseCurChallengeInfoRequest,
) -> Result<TrapDefenseCurChallengeInfoResponse, String> {
    let targets = state
        .player_components
        .progress
        .info
        .dynamic
        .claims
        .iter()
        .filter_map(|c| c.strip_prefix("trap:target:")?.parse::<i32>().ok())
        .collect();
    Ok(TrapDefenseCurChallengeInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        Target: targets,
    })
}

pub fn handle_trap_defense_develop_reset_request(
    state: &mut State,
    req: TrapDefenseDevelopResetRequest,
) -> Result<TrapDefenseDevelopResetResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("trap:reset_cnt:{}", req.ActivityId), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(TrapDefenseDevelopResetResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_get_reward_request(
    state: &mut State,
    req: TrapDefenseGetRewardRequest,
) -> Result<TrapDefenseGetRewardResponse, String> {
    claim_id_list(state, "trapdef:rw", "TrapDefenseReward", &req.Ids)?;
    Ok(TrapDefenseGetRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_get_special_reward_request(
    state: &mut State,
    req: TrapDefenseGetSpecialRewardRequest,
) -> Result<TrapDefenseGetSpecialRewardResponse, String> {
    claim_id_list(state, "trap_sp_rw", "TrapDefenseSpecialReward", &req.Ids)?;
    Ok(TrapDefenseGetSpecialRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_preview_finish_request(
    state: &mut State,
    _req: TrapDefensePreviewFinishRequest,
) -> Result<TrapDefensePreviewFinishResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        "trap:preview_done",
    )
    .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    {
        let mut qctx = state.quest_context(&mut events);
        if let Some(mut lp_ctx) = level_play::LevelPlayContext::from_quest_ctx(&mut qctx) {
            let _ = level_play::on_condition_event(
                &mut lp_ctx,
                LevelPlayEventKind::FinishTrapDefense,
                0,
                0,
            );
        }
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(TrapDefensePreviewFinishResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_refresh_gaining_group_request(
    state: &mut State,
    _req: TrapDefenseRefreshGainingGroupRequest,
) -> Result<TrapDefenseRefreshGainingGroupResponse, String> {
    let ref_cnt = state
        .player_components
        .progress
        .info
        .add_key_int("trap:gain_ref", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(TrapDefenseRefreshGainingGroupResponse {
        NewRewardInfo: Some(RewardGainingPanelInfo {
            TotalRefreshTimes: 3,
            RemainRefreshTimes: (3 - ref_cnt).max(0) as i32,
            ..Default::default()
        }),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_refresh_shop_request(
    state: &mut State,
    _req: TrapDefenseRefreshShopRequest,
) -> Result<TrapDefenseRefreshShopResponse, String> {
    let ref_cnt = state
        .player_components
        .progress
        .info
        .add_key_int("trap:shop_ref", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(TrapDefenseRefreshShopResponse {
        NewShopInfo: Some(TrapDefenseShopPanelInfo {
            TotalRefreshTimes: 5,
            RemainRefreshTimes: (5 - ref_cnt).max(0) as i32,
            ..Default::default()
        }),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_select_gaining_group_request(
    state: &mut State,
    req: TrapDefenseSelectGainingGroupRequest,
) -> Result<TrapDefenseSelectGainingGroupResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "trap:gain_grp",
        i64::from(req.GainingGroupId),
    )
    .map_err(|e| e.to_string())?;
    Ok(TrapDefenseSelectGainingGroupResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_slot_update_request(
    state: &mut State,
    req: TrapDefenseSlotUpdateRequest,
) -> Result<TrapDefenseSlotUpdateResponse, String> {
    pb_kv::save_msg(
        &state.fs,
        &mut state.player_components.progress,
        "trap:slots",
        &req,
    )
    .map_err(|e| e.to_string())?;
    Ok(TrapDefenseSlotUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_tech_unlock_request(
    state: &mut State,
    req: TrapDefenseTechUnlockRequest,
) -> Result<TrapDefenseTechUnlockResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("trap:tech:{}", req.Id),
    )
    .map_err(|e| e.to_string())?;
    Ok(TrapDefenseTechUnlockResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trap_defense_use_item_request(
    state: &mut State,
    req: TrapDefenseUseItemRequest,
) -> Result<TrapDefenseUseItemResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("trap:use_item:{}", req.ItemConfigId), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(TrapDefenseUseItemResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
