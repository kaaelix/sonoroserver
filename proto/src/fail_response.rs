// Generated fail_response trait implementations matching Zig handlers.zig:failResponse.
#![allow(clippy::field_reassign_with_default)]
pub trait FailResponse: Default {
    fn fail_response() -> Self {
        Self::default()
    }
}

impl FailResponse for crate::pb::ANStartResponse {}
impl FailResponse for crate::pb::AbyssChallengeGotoNextRoomResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssChallengeQuitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssChallengeUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssLikeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssLittleRoleLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssPluginPutResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssPluginSynthesisResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssRankListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssReviveCharacterResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AbyssSelfPassResponse {}
impl FailResponse for crate::pb::AbyssShowNameResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AcceptEntrustResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AcceptFocusWaitQuestResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AccessPasserbyNpcSpawnerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AccessPathConfigResponse {}
impl FailResponse for crate::pb::AccessPathTimeServerConfigResponse {}
impl FailResponse for crate::pb::AchievementFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AchievementInfoResponse {}
impl FailResponse for crate::pb::AchievementReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AchievementUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActionErrorCodeNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActionFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActionOperationScreenSuccessResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActionResetPosPrePerformanceEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActivateBuffResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActiveDragonPoolResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActivityFirstReadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActivityPayShopBuyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActivityPreOpenResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActivityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActivityResultRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ActorVisibleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AddChapterResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AddDecalResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AddPhantomEquipGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AddReviveRegionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AddTemporaryTeleportResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AddTreasureBoxSlotResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AddVisionEquipGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AdventureManualDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AdventureManualResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AdviceCreateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AdviceDeleteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AdviceModifyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AdviceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AdviceSetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AdviceVoteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AgreeJoinResultNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AgreeJoinResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AiBlackboardCdResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AiBlackboardsResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AiControlSwitchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AiHateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AiInformationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AllMsgResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AnimalDestroyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AnimalDieResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AnimalDropResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AnimationGameplayTagResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AnimationStateChangedResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AnimationStateInitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ApplyBuffS2cResponsePush {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ApplyGameplayEffectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ApplyJoinWorldResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ApplyRechallengeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ApplySceneClientEntityIncIdResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = 1;
        r
    }
}
impl FailResponse for crate::pb::ApplyVisionGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ApplyerEnterSceneNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AreaTerminalPinOperationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ArtemisNodeUpdateResponse {}
impl FailResponse for crate::pb::AtomicProcessAddBuffResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AtomicProcessRemoveBuffResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AtomicProcessTeleportResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AttachEntityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AttributeChangedResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AttributeInitResponse {}
impl FailResponse for crate::pb::AvignonRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AvoidBlockClientInitDoneResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AvoidBlockSubmitOverNodeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AwakeDynamicPortalResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::AwakeItemVehicleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BabelTowerDailyTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BabelTowerRankListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BabelTowerSelfPassResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BabelTowerSettlementResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BabelTowerShowNameResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BabelTowerTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BatchMaterialReplaceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BatchSynthesisItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BathChangeCameraResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BathPhotoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BattlePassLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BattlePassRecurringTakeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BattlePassResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BattlePassTakeAllRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BattlePassTakeRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BattlePassTaskResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BattlePassTaskTakeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BattleStateChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BeControlledDestroyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BeControlledEntityHoverResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BeControlledResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BeControlledShowResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BeControlledThrowResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BeamCastHitPlayerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BirthDayCardRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BirthDayRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BirthdayInitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BirthdayShowSetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BlackCoastThemeActiveRewardResponse {}
impl FailResponse for crate::pb::BlackCoastThemeTaskRewardResponse {}
impl FailResponse for crate::pb::BlackSwordChallengeInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BlackSwordChallengeMarkItemPanelInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BlockListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BlockPlayerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BoneVisibleChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BossPilingRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BossPilingSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BossRushBatchClaimRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BossRushBuffChooseResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BossRushChooseBuffResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BossRushClaimRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BossRushLevelScoreRewardResponse {}
impl FailResponse for crate::pb::BossRushSettlementResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BossRushTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BrochureRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BtForcedOccupationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BtGiveUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BtRollbackResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BtSetTimerInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BtnStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BuffEffectExecuteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BuffEffectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BuffItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BuffStackCountResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BuildingLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::BuildingUnLockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ButtonIsEnableResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CalabashBatchRefiningResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CalabashLevelRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CalabashMsgResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CalabashSkinDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CalabashSkinTakeOnResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CameraSettingModifyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CameraSettingQueryResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CancelMatchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CancelQuestFocusModeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CaptureBatchEntityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CaptureEntityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CaughtResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CdKeyVerifyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CertificateLevelRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangeCardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangeDataLayerFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangeEntityRoleResponse {}
impl FailResponse for crate::pb::ChangeEntityStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangeHeadPhotoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangeOrnamentResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangePhantomEquipGroupNameResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangePlayerTitleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangeStateConfirmResponse {}
impl FailResponse for crate::pb::ChangeStateResponse {}
impl FailResponse for crate::pb::ChangeSummonEntityStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangeVehicleRideSharingResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangeVisionGroupNameResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChangeWeatherResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChannelChatResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChapterResultRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CharacterAttachResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CharacterDeathResponse {}
impl FailResponse for crate::pb::CharacterDetachResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChatMutePlayerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ChatResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CheatInputResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CheckGearResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CheckPublicResourceVersionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CheckUseSoundBoxSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CircumFluenceTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ClientBasicInfoResponse {}
impl FailResponse for crate::pb::ClientCurrentRoleReportResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ClientPullResourcePackageResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CombatDataMaxResponse {}
impl FailResponse for crate::pb::CombatMaxCaseMessageResponse {}
impl FailResponse for crate::pb::CombatMessagePostInfoResponse {}
impl FailResponse for crate::pb::CombatMessagePrefixInfoResponse {}
impl FailResponse for crate::pb::CombatSendPackResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CombineEntitiesResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CommitBoardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CommonOrganResetTimeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CommonTouchUiSettingUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ConditionGroupStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ConfirmQuestResourceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ConnectorCanConnectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ConnectorConnectEntityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ConstructTrapResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ConsumptiveActivityInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ConsumptiveActivityScoreResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ConsumptiveRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CookFoodResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CookFormulaResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CookingDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CoopRoleRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CoopSpRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CorniceChallengeTransResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CorniceMeetingRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CountryExploreScoreInfoResponse {}
impl FailResponse for crate::pb::CreatStoryCharacterResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CreateBulletResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CreateCharacterResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::CreatureDetailResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DErrorResult {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DailyAdventurePtRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DailyAdventureTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DailyQuestChangePreferResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DailyQuestGetRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DamageExecuteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlBagOperateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlEventRefreshResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlEventSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlFightOverResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlShopBuyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlShopComposeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlShopLockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlShopRefreshResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlShopSellResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlStartFightResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoBrawlTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoMonopolyBoardRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoMonopolyDiceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoMonopolyGridRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoMonopolyNextBoardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DangoMonopolyTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DarkCoastDeliveryResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DarkCoastInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DeadeyeShootHitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DebugFightErrInfoNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DeconstructTrapResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DeletePhantomEquipGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DeleteStoryCharacterResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DeleteVisionEquipGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DestroyBulletResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DetectionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DirectTrainGetPlayerIdResponse {}
impl FailResponse for crate::pb::DirectTrainInfoResponse {}
impl FailResponse for crate::pb::DiscardWeaponResponse {}
impl FailResponse for crate::pb::DissolveCombineRelationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DollDeliveryResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DollGarbInsertCoinResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DollGarbStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DollGrabPauseEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DollGrabPauseStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DollGrabResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DollGrabStopResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DollGrabTimeBallResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DollSmallMapInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DragonPoolConfResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DropCatchInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DropCatchLevelFinishResponse {}
impl FailResponse for crate::pb::DropCatchLevelRewardResponse {}
impl FailResponse for crate::pb::DrownEndTeleportResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::DrownResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EdgeRunnerFunctionPreUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EdgeRunnerScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EdgeRunnerTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ElevatorStateChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EnableDamageRecordResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EncircleChallengeCompleteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EncircleChallengeStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EndLessHistoryResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EndSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EnergySyncResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EnrichmentAreaResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EnterAreaResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EnterBigWorldInstResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EnterGameResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EnterInstResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EnterMatchInstResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EnterViewDirectionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityAccessCasketResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityAccessRangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityActiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityAdsorbResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityBattleInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityBeamReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityBlackboardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityBuffProducerOperateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityBuffProducerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityCameraAlertResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityDynamicInteractResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityEnterVehicleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityFollowTrackResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityInteractResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityIsVisibleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityOnLandedResponse {}
impl FailResponse for crate::pb::EntityPatrolAddTagResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityPatrolChangeDirResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityPatrolStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityPatrolStopResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityPosAbnormalResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityPositionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityRandomInteractResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntitySendEventResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityStaticHookMoveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EntityWalkingPatternFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EquipSkinTakeOnResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EquipSkinUnLoadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EquipTakeOnResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::EquipWeaponSkinResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExchangeRewardInfoResponse {}
impl FailResponse for crate::pb::ExchangeRewardResponse {}
impl FailResponse for crate::pb::ExecuteQteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExhibitionItemSaveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExitViewDirectionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExploreActivityTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExploreEntityTraceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExploreProgressResponse {}
impl FailResponse for crate::pb::ExploreProgressRewardResponse {}
impl FailResponse for crate::pb::ExploreScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExploreSkillActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExploreSkillCustomResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExploreSkillPullGiantResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ExploreSkillRouletteSetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FarmGoldLevelDiffResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FarmGoldLevelPlayRepeatedResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FarmGoldLevelPlayResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FarmGoldPointRepeatedResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FarmGoldPointResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FarmGoldResultNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FarmGoldResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FastReturnDungeonResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FavoriteMusicMotorCycleUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FeiXuePreheatInfoRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FinishDownloadQuestResourceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingActivityLimitRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingActivityMilestoneRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingCabinPutResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingCageInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingCompleteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingDataResponse {}
impl FailResponse for crate::pb::FishingEntrustAcceptResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingEntrustHandInResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingEntrustRefreshResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingEntrustTraceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingExitResponse {}
impl FailResponse for crate::pb::FishingHandInResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingIllustratedRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingInfoResponse {}
impl FailResponse for crate::pb::FishingPointInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingQuickSellPreviewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingQuickSellResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingSailingResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingSellPreviewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingSellResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingShipFixResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingShipSkinChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingSkillBaitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingSkillBombResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingSkillTeleportResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingTechLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FishingTeleportToBoatResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FixToolResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlagChallengeRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchAnimationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchCloseTaskResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchEventChoiceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchExecuteSkillPlayResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchFirstEnterResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchOutResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayEventChoiceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayGachaRefreshResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayGachaResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayNextDayResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayRefreshGachaResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayRefreshShopResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayRemoveUnitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlaySelectCardGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayShopBuyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayTributeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchPlayUnlimitedModeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchRewardMilestoneResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchRewardTaskResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchSettleDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchStartPlayResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchUnlockTechPointResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FloroRanchWeeklyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlowActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlowEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlowRestartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlowSeqSetPlayerPosResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlowTeleportToPlotPosResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlowerPollutionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlySkinAllUnLoadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlySkinUnLoadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlySkinWearAllRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FlySkinWearResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FollowTrackResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FoodProcessResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ForgeFormulaUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ForgeInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ForgeItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FormationAttrResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FormationChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FormationModifyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FormationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FormationSwitchCurrentResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FormationSwitchRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FoundationCoreOperateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FoundationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FragileChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FriendAllResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FriendApplyHandleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FriendApplySendResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FriendDeleteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FriendRecentlyTeamResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FriendRemarkResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FsmConditionPassResponse {}
impl FailResponse for crate::pb::FsmPlayMontageResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FsmStateBehaviorResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FunPlayChallengeAwardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FuncShowResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::FurnitureDiySaveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GaSwitchCommonEnemyProCampResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GachaAccumulateInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GachaAccumulateRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GachaInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GachaPoolDetailResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GachaResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GachaUsePoolResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GameStatsShowResponse {}
impl FailResponse for crate::pb::GameplayCueResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GameplayTagResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GatewayEchoResponse {}
impl FailResponse for crate::pb::GatherActivityRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetAdventureRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetChapterRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetDetectionLabelInfoResponse {}
impl FailResponse for crate::pb::GetDetectionTargetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetFormationDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetInstExchangeRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetLevelPlayRewardsResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetMusicInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetRewardTreasureBoxResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetTraceQuestYuXueModeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GetYuXueModeInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GiantDeliveryItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GiantMaxLevelDeliveryItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GiantStableValueActivityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GiftPackPreviewResponse {}
impl FailResponse for crate::pb::GivebackInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GivebackRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GmEntityFsmGroupInfoResponse {}
impl FailResponse for crate::pb::GmLevelActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GolemCrackUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GravityFlipInteractResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GravityGearResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GravityHookLockPointChangeGravityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GuideFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::GuideInfoResponse {}
impl FailResponse for crate::pb::GuideTriggerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::H5CircumFluenceActivityReadResponse {}
impl FailResponse for crate::pb::H5ViewActivityReadResponse {}
impl FailResponse for crate::pb::HandInItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HandInMingSuResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HardnessModeChangedResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HarvestActivityLevelDiffResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HarvestActivityLevelResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HarvestActivityPointResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HarvestActivityResultNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HarvestActivityResultResponse {}
impl FailResponse for crate::pb::HeartbeatResponse {}
impl FailResponse for crate::pb::HitEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HoldHandResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryActivateTalentResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryActivityInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryAreaSecretRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryBagOperateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryDiscardItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryEquipRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryInstLeaveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryItemCollectionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryLifeSupportUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryLimitTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryLockItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryMascotRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryPickUpItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryResidentTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStorySellItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryUnlockSlotResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HonamiStoryWeaponDressResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HookLockBatchCollectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HookLockPointExitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HookLockPointFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::HookLockPointResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::IllustratedInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::IllustratedReadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::IllustratedRedDotResponse {}
impl FailResponse for crate::pb::IllustratedUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfluenceInfoResponse {}
impl FailResponse for crate::pb::InfluenceRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrArchiveReadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrArchiveTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrFireNoticeResponse {}
impl FailResponse for crate::pb::InfrInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrManualCancelTraceRoadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrManualSwitchTraceRoadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrPhoneTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrRoadBuildResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrThemeActivityAwardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrV2InfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrV2LevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrV2ManualCancelTraceTreeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrV2ManualSwitchTraceTreeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrV2ScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrV2TaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InfrV2TreeBuildResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InitHonamiActivityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InitRangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InputSettingResponse {}
impl FailResponse for crate::pb::InputSettingUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InspirationUnlockChoiceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InstEntityTeleportResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InstEntranceDetailResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InstTeleportToResetPointResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InteractiveUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InterruptSkillInDelayResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InvalidItemResponse {}
impl FailResponse for crate::pb::InvestResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::InviteRechallengeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ItemDecomposePreviewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ItemDecomposeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ItemDeprecateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ItemDisuseResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ItemExchangeInfoResponse {}
impl FailResponse for crate::pb::ItemExchangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ItemGiftUseResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ItemLockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ItemUseResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::JoinSceneNotifyConfirmResponse {}
impl FailResponse for crate::pb::JokerGuessDrawCardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::JokerGuessPlayCardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::JokerGuessRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::JokerGuessStartNewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::JokerGuessUseSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KcpConvResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KickMatchTeamPlayerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KickWorldTeamResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoChestRewardSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoEndShopStepResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoEndSpecialWaveCombatResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoGoldAddResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoLevelRecordDeleteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoLimitTaskReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoReChallengeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoRefineWeaponResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoResTaskReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoRoleRecordResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoScoreTaskReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoSellWeaponResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoSettlementResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoShopRewardLockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoShopRewardRefreshResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoShopRewardSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoStepAdvanceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoUpgradeRewardRefreshResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::KurotatoUpgradeRewardSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LandingDamageResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LanguageSettingUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LastBigSceneMiniMapInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LeaveInstResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LeaveMatchTeamResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LeaveWorldTeamResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LevelPlayFastReturnPosResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LevelPlayOpenResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LevelPlayRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LevelPlayStateListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LevelPlayStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LevelPlaySwitchSportModeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LevelPlayVarAsyncResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LevelPlayVarResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LevelSequenceFrameEventActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LifePointChallengeStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LineCrossChallengeStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LinkageCheckInRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LinkageRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LivenessResponse {}
impl FailResponse for crate::pb::LivenessTakeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LivenessTaskTakeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LoadCalabashSkinInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LoadEquipInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LoadingConfigResponse {}
impl FailResponse for crate::pb::LoadingLevelAreaResponse {}
impl FailResponse for crate::pb::LobbyListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LobbyQueryPlayersResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LogicStateInitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LoginResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LogoutNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LongShanScoreRewardResponse {}
impl FailResponse for crate::pb::LongShanTaskRewardResponse {}
impl FailResponse for crate::pb::LoopTeleportVolumeActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LordGymBeginResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LordGymInfoResponse {}
impl FailResponse for crate::pb::LordGymReadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LowMemoryChangeSceneFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::LstBattleRecordResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MailBindInfoResponse {}
impl FailResponse for crate::pb::MailBindResponse {}
impl FailResponse for crate::pb::MailBindRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MailDeleteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MailFavoriteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MailGetAttachmentResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MailReadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MainPhantomRecommendResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapCancelTraceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapMarkInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapMarkInfoUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapMarkResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapRemoveAllMarkResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapRemoveMarkResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapReplaceMarkResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapTraceInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapTraceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapTravelFullRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapTravelLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapTravelSoarRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapTravelTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MapUnlockFieldInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MatchChangePlayerUiStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MatchChangeReadyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MatchChangeRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MatchConfirmResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MatchThreeEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MatchThreeStarResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MaterialReplaceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MaterialResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MessageDecodeFailNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MingSuHandRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MiniWriteLetterSaveContentResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MixDrinkRoleInviteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MixDrinkRoleRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MixDrinkSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MixDrinkTopicSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MobileButtonSettingUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ModifyBulletParamsResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ModifyNameResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ModifySignatureResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MonsterBoomResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MonsterDetectionInfoResponse {}
impl FailResponse for crate::pb::MonsterDrownResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MonsterPatrolPointReachedResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MontagePlayResponse {}
impl FailResponse for crate::pb::MonthCardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MoonPhaseRandomResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MoonPhaseRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MoralePosResponse {}
impl FailResponse for crate::pb::MoraleTreasureBoxTraceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorChangeOutlookResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorCreateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorDecalMotorIpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorDecalTaskResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorDevelopTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorDiyInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightEnterSubLevelResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightKillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightLastSaveDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightOneKeyRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightPlayerDeadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightRankListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightRankListSelfDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightRefreshCollectionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightRefreshNextWaveGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightSelectBuffGateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightSelectCollectionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorFightTalentUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorLevelOneKeyRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorLinkTaskResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorOutlookCreatePresetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorOutlookDeletePresetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorOutlookEditPresetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorOutlookInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorParkourLevelRewardResponse {}
impl FailResponse for crate::pb::MotorSliderResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorTaskOneKeyRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorTechLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorTechTreeSwitchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorUseSceneResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MotorUseSkinResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MovePlacementResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MoveSceneItemResetPositionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MowTowerLevelsRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MowTowerSettlementResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulAchievementReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulCalabashLevelRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulCorniceMeetingRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulExploreScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulFishingIllustratedRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulGetAdventureRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulMapTravelSoarRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulMapTravelTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulPhantomBattleMasterLevelRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulRogueWeeklyRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MulRoguelikeWriteCatRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MultiExploreScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::MutiplayerTeamRefreshNearListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NavigationFindPathResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NetTestDataResponse {}
impl FailResponse for crate::pb::NewBieCourseRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewHandInItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewJourneyResponse {}
impl FailResponse for crate::pb::NewPlayerSupportRewardTaskResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewPlayerSupportSetCurUseTrialRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewPlayerSupportSetCurUseTrialRoleV2Response {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewPlayerSupportTrialRoleLvUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewPlayerSupportTrialRoleLvUpV2Response {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTowerClearRecordResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTowerCycleStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTowerLastCycleInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTowerMedalInfoResponse {}
impl FailResponse for crate::pb::NewTowerRecommendResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTowerRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTowerRoleEnergyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTowerSeasonRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTowerSeasonScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTowerSeasonTaskDataUpdateResponse {}
impl FailResponse for crate::pb::NewTowerUpdateRecordResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewTrialRoleGetNightmarePhantomInstInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewbieAdventureChapterDropResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewbieAdventureRoleRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewbieAdventureTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewbieCarnivalAwardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewbieCarnivalSwitchRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewbieCourseV2RewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NewbieFetchScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NormalItemResponse {}
impl FailResponse for crate::pb::NormalMonsterManualInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::NpcTraceFailedResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::OccupyRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::OnlineMotorActivityLevelResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::OpenSystemBoardResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::OrderApplyBuffResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::OrderRemoveBuffByTagsResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::OrderRemoveBuffResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::OutputDebugInfoResponse {}
impl FailResponse for crate::pb::ParkourChallengeResponse {}
impl FailResponse for crate::pb::ParkourChallengeTakeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ParkourChallengeTransResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PartComponentInitResponse {}
impl FailResponse for crate::pb::PartUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PassPortalResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PassiveSkillAddResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PassiveSkillRemoveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PayGiftInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PayGiftResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PayInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PayItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PayShopBuyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PayShopDirectBuyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PayShopInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PayShopItemUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PayShopUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PbGetRoleListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PbOverRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PbUpLevelRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PbUpLevelSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhBaPlanBatchOperResponse {}
impl FailResponse for crate::pb::PhBaPlanFindPlanResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhBaPlanSaveUsePlanResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhBaPlanSetFiveStarSwitchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhBaPlanSetPlanStatusResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhBaPlanUpdatePlanResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhBaPlanUsePlanResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomAttrRecommendResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomAutoPutResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBatchDirectRefiningResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBatchOperResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBatchPolishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBatchRefiningResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleAreaSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleBackLibraryResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleBackSlotCardLibraryResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleBadgeRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleBvBLoadingFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCallCardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardGroupAddResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardGroupDeleteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardGroupNameResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardGroupUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardOutLookUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardRoleSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardRoleTargetInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardSelectResponse {}
impl FailResponse for crate::pb::PhantomBattleCardSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardTargetInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCardUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleChallengeUpdateNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleCheckCardSkillUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleDealCardReplaceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleDebugInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleDiscardCardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleEnterSlotResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleEvolveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleGuideRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleLeaveSlotResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleMasterLevelRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleNpcShowOverResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattlePhantomCardSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleReChallengeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleReconstructCardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleRepeatedTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleRoleRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleRoundOverResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleSelectReserveCardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleSelectTargetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleSkillEffectFailNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleSlotCardSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleSlotInsteadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomBattleTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomCollectReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomEquipGroupInfosResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomFetterRecommendResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomFuncValueBatchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomIdentifyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomInteractionEquipResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomInteractionSkinChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomItemResponse {}
impl FailResponse for crate::pb::PhantomLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomManageConfigResponse {}
impl FailResponse for crate::pb::PhantomManageConfigUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomMusicCollectUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomMusicInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomMusicSaveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomMusicSwitchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomMusicUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomPolishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomPutOnResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomRecommendResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomRefiningResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomReturnPreviewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomReturnResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomSettingBatchUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomSettingResponse {}
impl FailResponse for crate::pb::PhantomSettingUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomSkinChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomVicePolishAckResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhantomVicePolishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhotoFightResultNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhotoFightSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhotoFightTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PhotoMemoryResponse {}
impl FailResponse for crate::pb::PhotoMemoryRewardResponse {}
impl FailResponse for crate::pb::PickUpDropItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PilotThrowOperationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballBonusFullWinResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballFriendRankResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballInGameKickBallResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballInGameSyncPayloadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballQuitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballRoleLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballSetPayloadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballTeamDieResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballTeamReviveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballWeaponDegradeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PinballWeaponWearResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlaceItemOnBoardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayPointStateAsyncResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerAccessCasketResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerAccessEffectAreaResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerAccessRangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerBasicInfoGetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerCreatePsnSessionResponse {}
impl FailResponse for crate::pb::PlayerCreateXboxSessionResponse {}
impl FailResponse for crate::pb::PlayerEnterPollutionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerGameTokenResponse {}
impl FailResponse for crate::pb::PlayerHeadDataResponse {}
impl FailResponse for crate::pb::PlayerHeadResponse {}
impl FailResponse for crate::pb::PlayerMotionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerPsnBlockListResponse {}
impl FailResponse for crate::pb::PlayerPsnInfoGetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerRoleVoiceSetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerTitleDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerTitleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerVoiceLanguageResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerVoxelEnvResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PlayerXboxBlockListResponse {}
impl FailResponse for crate::pb::PlayerXboxInfoGetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PolluteGridCellResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PreheatSignRewardResponse {}
impl FailResponse for crate::pb::PreheatSignSurveyInfoResponse {}
impl FailResponse for crate::pb::PressGameResponse {}
impl FailResponse for crate::pb::PressResponse {}
impl FailResponse for crate::pb::PrivateChatDataResponse {}
impl FailResponse for crate::pb::PrivateChatHistoryResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PrivateChatOperateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PrivateChatResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ProgressBarFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ProtoKeyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PsnSettingResponse {}
impl FailResponse for crate::pb::PsnSettingUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PullingFoundationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::PutVisionGroupToTopResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QingXiaoTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QueryOnlinePlayersAreaResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QueryRecallItemBagResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestBranchResponse {}
impl FailResponse for crate::pb::QuestBranchSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestBranchUpdateNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestFastReturnPosResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestFinishActionEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestFinishActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestNpcMoveOverResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestReadyForGiveUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestRedDotResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestReviewDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestReviewInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuestTreeInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::QuickHackSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RTimeStopAnimResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RTimeStopInstResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RTimeStopResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetLegMatchResultNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetMatchActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetMatchInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetMatchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetMatchRoundActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetMatchRoundResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsBulletScreenResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsCloseSettleMenuResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsGearRefundResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsGearResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsLegMatchInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsMatchInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsPlayerInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsRankResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsRepeatedTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsTaskResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RacingBetsUpdateOddsResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ReadCardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ReadDisplayInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RealmBetweenFullRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RealmBetweenLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RealmBetweenMotorRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RealmBetweenTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RecallExploreSkillRouletteSetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RecallItemUseResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RecallQuestActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RecallQuestFinishActionEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RecallQuestFinishActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RecallQuestNpcMoveOverResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RecallQuestReadyForGiveUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RecallTeleportToMarkIdResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ReceiveAreaStageRewardAsyncResponse {}
impl FailResponse for crate::pb::ReceiveRechallengeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ReceivedSilentFirstAwardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ReconnectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RegressBonusResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RegressBuyBonusLvResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RegressDisposableRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RegressQuestionOpenResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RegressRepeatedTaskResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RegressSetCurUseTrialRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RegressTaskResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RegressTrialRoleLvUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoteBatchCollectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveBuffByIdS2cResponsePush {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveBuffS2cResponsePush {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveGameplayEffectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveInstanceTimerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveMapMarkResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveReviveRegionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveRideSharingPassengerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveSummonEntityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveTemporaryTeleportResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RemoveTreasureBoxSlotResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RenameFormationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RenjuExitMatchedActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RenjuResetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ReorderFormationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ReportPlayerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ResetBabelTowerLevelResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ResonantChainOptionalLimitResponse {}
impl FailResponse for crate::pb::ResonantChainUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ReviveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RhythmChangePartnerRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RhythmMainLineInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RhythmRankListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RhythmRankListSelfDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RhythmSetRedDotResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RhythmSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RhythmTaskOneKeyRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RiskHarvestBuffCountRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RiskHarvestBuffUnlockRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RiskHarvestInstRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RiskHarvestScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RiskHarvestSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RiskHarvestStarRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoadBookFullRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoadBookLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoadBookMotorRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoadBookTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RobotRolePropResponse {}
impl FailResponse for crate::pb::RogueArchiveListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueArchiveSaveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueBlackFlowerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueInstExtraInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResChangeFormationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResEndingRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResGotoNextRoomResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResGridEventResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResIllustrationAwardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResInstResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResInstStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResLastInstInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResMoveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResOpDoneResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResSwitchFormationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResTalentSkillLevelResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueResTaskAwardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueTowerTrialGiveUpTempArchiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueTowerTrialReChallengeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueTowerTrialResultNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueTowerTrialResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueTowerTrialSelectBossResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueTowerTrialStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWeeklyArtifactInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWeeklyArtifactSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWeeklyBlackFlowerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWeeklyGotoNextRoomResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWeeklyLastInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWeeklyOptionSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWeeklyResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWeeklyRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWeeklyStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWhiteCatLimitedRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RogueWhiteCatRecordResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeBossInstStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeChooseDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeChooseDataResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeGiveUpGainResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeGotoNextRoomResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeLastInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikePopularEntriesChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikePopularEntriesInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeQuitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeRefreshGainResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeRoleInstStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeRoleRoomSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeSeasonDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeSeasonRewardReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeTalentInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeTalentLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeTokenReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeTrialRoleInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeWriteCatBossRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeWriteCatLevelPlayRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoguelikeWriteCatRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleActivateSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleBreakThroughViewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleDevelopConfigResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleElementChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleFavorAcceptQuestResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleFavorListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleFavorTalkScoreResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleFavorUnLockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleGiftRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleIllustratedInfoResponse {}
impl FailResponse for crate::pb::RoleLevelUpViewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleMotionUnLockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleOperateSelfBgmResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RolePropResponse {}
impl FailResponse for crate::pb::RoleQuestAcceptResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleQuestPointStateResponse {}
impl FailResponse for crate::pb::RoleSexChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleShowListUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleSkillBranchModifyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleSkillLevelUpViewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleSkillQuickLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleSkillViewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleSkinChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleSkinResponse {}
impl FailResponse for crate::pb::RoleSkinTrialSettleNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTagChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerAttributeQueryResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerChallengeSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerConsumeItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerPhantomAddAttrsResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerRankResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerRankRewardReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerRewardTaskReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerSaveDeleteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerScoreRewardReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerStageRetryResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerStageSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTowerStepAdvanceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTrainResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleTrialSettleNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleUpdateDevelopTargetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleVisionMainPhantomResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleVisionRecommendAttrResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoleVisionRecommendDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RollBlockExitGamePlayResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RollBlockGamePlayReadyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RollBlockInputResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RollBlockMovementFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RollBlockResetGamePlayResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RollBlockSwitchControlBlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RollBlockToggleHintResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueBlessGroupSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueChooseDataResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueEventSelectResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueGiveUpGainResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueGotoNextRoomCommitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueInsListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueLootChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueLootInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueQuitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueRefreshGainResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueShopBuyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueShopRefreshResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueTalentInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueTalentUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RoverRogueTaskRewardTakeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RunTestResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::RunTurntableActivityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneGroupDataAndCurFormationEntityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneItemMoveSplineResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneItemMoveTargetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneItemSequenceFrameEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneItemSequenceFrameStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneItemSetPosSenderResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneItemStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneItemStopResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneLoadingFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneModeChangeFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneSubLevelsResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneTestNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SceneTraceResponse {}
impl FailResponse for crate::pb::ScheduleNpcUpdatePosResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ScheduleRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ScratchCardActivityInfoResponse {}
impl FailResponse for crate::pb::ScratchCardRewardResponse {}
impl FailResponse for crate::pb::SeamlessTeleportResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SeamlessTeleportVehicleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SecFBRoundEndFlowReportResponse {}
impl FailResponse for crate::pb::SecFBRoundStartFlowResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SecGetReportData2FlowResponse {}
impl FailResponse for crate::pb::SecRoleFightFlowResponse {}
impl FailResponse for crate::pb::SecWorldInfoFlowResponse {}
impl FailResponse for crate::pb::SecWorldSEndFlowResponse {}
impl FailResponse for crate::pb::SecWorldStartFlowResponse {}
impl FailResponse for crate::pb::SelectBabelActivityDeTermResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SelectDetectionTargetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SelfPortraitStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SendEquipSkinResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SendMovieModeRideSharingResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ServerPlayStationPlayOnlyStateResponse {}
impl FailResponse for crate::pb::SetAlertUiVisibleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetBaoziStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetBtObjYuXueModeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetClientGlobalVarResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetFanNumberOfTurnsResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetFanStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetFocusModeDeterCondResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetFocusModeDeterConditionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetInitTagResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetMatchTeamMatchFlagResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetPlayerYuXueModeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetQuestFocusModeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SetQuestResourceStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SheriffZoneInfoResponse {}
impl FailResponse for crate::pb::ShopBuyNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShopBuyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShopInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShopUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShortMessageChatCharmChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShortMessageChatThemeChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShortMessageInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShortMessageReadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShortMessageReceivedResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShortMessageReplyResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShortMessageShowChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShortMessageUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ShowInvalidItemResponse {}
impl FailResponse for crate::pb::SignActivityFreeDropResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SignActivityResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SimpleCombatEntityDieResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SimpleTrackReportAsyncResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SimpleTrackReportResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SingleInstRechallengeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SitChairResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SkinRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SlashAndTowerInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SlashAndTowerRecommendResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SlashAndTowerResetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SlashAndTowerSaveRecordResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SlashAndTowerScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SneakFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SneakResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SneakTimeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SolarSchoolBatchSyncResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SolarSchoolLoadResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SolarSchoolOneKeyGalleryRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SolarSchoolOneKeyRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SolarSchoolReachEndingResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SolarSchoolSaveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SolarSchoolSetFlowResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SolarSchoolTaskProgressRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SpecialItemEquipResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SpecialItemUnEquipResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SplineMoveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SplinePositionUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SpringFestivalAtmosphereRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SpringFestivalScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SpringFestivalTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SpringSignDrawRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SpringSignDrawRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SpringSignPhotoRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SpringSignSkinRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::StartMatchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::StartRecallResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::StopMoveSplineResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::StorageInfoResponse {}
impl FailResponse for crate::pb::StorageInfoUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SubmitAwakeAndLoadEntityNodeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SubmitNodeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SubmitPhotoNodeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SubmitWalkingPatternPointResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::Summon2Response {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::Summon3Response {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SummonRandomResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SummonResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SurvivorsLastInfoResponse {}
impl FailResponse for crate::pb::SurvivorsLockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SurvivorsNormalAwardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SurvivorsOpDoneResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SurvivorsResultResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SurvivorsScoreAwardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SurvivorsStepAdvanceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SurvivorsTalentLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SurvivorsWeaponInfoResponse {}
impl FailResponse for crate::pb::SwitchBattleModeResponse {}
impl FailResponse for crate::pb::SwitchCharacterStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SwitchLogicStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SwitchRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SynthesisFormulaUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SynthesisInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SynthesisItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SynthesisLevelRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SysErrorNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::SysInfoNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TargetGearHitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeamChallengeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeamMatchAcceptInviteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeamMatchInviteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeamMatchInviteRetToHostNotify {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeamParkourRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeamRecommendResponse {}
impl FailResponse for crate::pb::TeleportDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeleportDungeonActionResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeleportFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeleportFlowEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeleportToTargetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TeleportTransferResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TetrisFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TetrisUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ThemePersonalScoreResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ThemeWorldScoreResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ThirdPartyShareResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ThroughTrainFinishViewResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ThroughTrainResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ThrowDamageResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TimeCheckResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TimePointActivityBatchRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TimePointActivityRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TimelineTraceControlResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TimelineTraceExitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TimelineTraceStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TimerEndResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TopPhantomEquipGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TotalTopUpRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TotalTopUpScoreInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ToughCalcExtraRatioChangeResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerApplyFloorDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerDefenceChallengeQuitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerDefenceInstanceRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerDefenceRandListResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerDefenceRankShowNameResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerDefenceScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerDefenceSelfPassResponse {}
impl FailResponse for crate::pb::TowerFormationRecommendResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerGuideActivityInfoResponse {}
impl FailResponse for crate::pb::TowerGuideActivityRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerResetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerResponse {}
impl FailResponse for crate::pb::TowerRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TowerSeasonUpdateResponse {}
impl FailResponse for crate::pb::TowerStartResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TraceEntityPosResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TraceMoonPhaseRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TraceQuestResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TraceRecallQuestResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrackMoonActivityRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrackMoonBuildFlowResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrackMoonBuildingDataResponse {}
impl FailResponse for crate::pb::TrackMoonEntrustDataResponse {}
impl FailResponse for crate::pb::TrackMoonIllustratedResponse {}
impl FailResponse for crate::pb::TrackMoonRoleDataResponse {}
impl FailResponse for crate::pb::TrackMoonSettleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrackMoonTargetRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseAuxiliaryBranchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseAuxiliaryLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseAuxiliaryResetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseBuildingBranchResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseBuildingLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseBuildingResetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseBuyShopProductResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseChallengeQuitResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseCurChallengeInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseDevelopResetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseGetRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseGetSpecialRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefensePreviewFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseQuickEquipResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseRefreshGainingGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseRefreshShopResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseSelectGainingGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseSlotUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseTechUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrapDefenseUseItemResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrialRoleRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TrialRoleSkinRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TriggerExitSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TsAnimNotifyStateAbsoluteTimeStopResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TurntableCompleteResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TutorialInfoResponse {}
impl FailResponse for crate::pb::TutorialReceiveResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::TutorialUnlockResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UiGamePlayResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorId = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UnOpenedAreaPullbackResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UnblockPlayerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UnlockInstEntranceResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UnlockRoleSkinListResponse {}
impl FailResponse for crate::pb::UpdateAchievementInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UpdateFightRoleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UpdateFormationResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UpdatePhantomEquipGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UpdatePlayStationBlockAccountResponse {}
impl FailResponse for crate::pb::UpdatePlayerPassiveSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UpdateSceneDateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UpdateSceneGroupDataResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UpdateSceneItemBlackboardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UpdateVoxelEnvResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UseDetectionSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UsePhantomEquipGroupResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UseSkillFailResponse {}
impl FailResponse for crate::pb::UseSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::UseSoundBoxSkillResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VacuumCleanerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ValidTimeItemResponse {}
impl FailResponse for crate::pb::VehicleChangePassengerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VehicleEnterSharingResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VehicleFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VehicleManipulateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VehiclePlatformGetOnResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VehicleRemovePassengerResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VehicleResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VfxNpcPatrolFinishResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.Code = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VisionEquipGroupInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::VisionExploreSkillSetResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeaponBreachResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeaponItemResponse {}
impl FailResponse for crate::pb::WeaponLevelUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeaponResonUpResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeaponSkinResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeatherControlInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeatherControlInfoWithoutCheckAsyncResponse {}
impl FailResponse for crate::pb::WeatherControlResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeatherSwitchInfoResponse {}
impl FailResponse for crate::pb::WebSignResponse {}
impl FailResponse for crate::pb::WeekCardInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeekCardRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeeklyFrameworkInfoResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WeeklyFrameworkScoreRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WellStreamingFinisResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WorldBlackboardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WorldEnterPermissionsResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WorldLevelDownResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WorldLevelRegainResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WuWuKujiGachaResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WuWuPackageRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::WuWuTaskRewardResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::XboxSettingResponse {}
impl FailResponse for crate::pb::XboxSettingUpdateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrorCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
impl FailResponse for crate::pb::ZitherPlayStateResponse {
    fn fail_response() -> Self {
        let mut r = Self::default();
        r.ErrCode = Some(crate::enums::ErrorCode::InternalError);
        r
    }
}
