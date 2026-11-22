using System;
using System.Collections.Generic;
using System.Security.Cryptography;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

/// <summary>
/// Explicit typed C# handlers replacing the legacy Tier-3 reflection auto-responder.
/// Compiles in seconds inside AkiLogicService so RPC/reward/menu changes do not require rebuilding gamesv.
/// </summary>
public static class ExtraHandlers
{
    private static T Parse<T>(byte[] payload) where T : IMessage<T>, new()
    {
        var msg = new T();
        if (payload.Length > 0) msg.MergeFrom(payload);
        return msg;
    }

    private static AkiResult OkResp<TResp>(EMessageId respId, TResp resp) where TResp : IMessage
        => AkiResult.Ok(respId, resp);

    private static string GetOrCreateGmSignToken(int playerId)
    {
        if (playerId <= 0) return string.Empty;
        var existing = SqliteStore.GetKv(playerId, "gm", "sign_token");
        if (!string.IsNullOrEmpty(existing) && existing.Length >= 16)
        {
            return existing;
        }
        var token = Convert.ToHexString(RandomNumberGenerator.GetBytes(12)).ToLowerInvariant();
        SqliteStore.SetKv(playerId, "gm", "sign_token", token);
        return token;
    }

    private static AkiResult GrantClaimReward(int playerId, string claimKey, EMessageId respId, IMessage resp, params (int ItemId, int Count)[] grants)
    {
        var result = AkiResult.Ok(respId, resp);
        if (playerId > 0)
        {
            var claims = SqliteStore.GetActivityClaims(playerId);
            if (!claims.Contains(claimKey))
            {
                SqliteStore.SetActivityClaim(playerId, claimKey, "1");
                result.WithGrants(grants);
            }
        }
        return result;
    }

    public static void RegisterAll(Dictionary<ushort, Func<int, byte[], AkiResult>> map)
    {
        // Formation & Role / Phantom Recommend
        map[(ushort)EMessageId.FormationRequest] = (_, _) => OkResp(EMessageId.FormationResponse, new FormationResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.FormationModifyRequest] = (pid, payload) =>
        {
            var req = Parse<FormationModifyRequest>(payload);
            if (pid > 0) SqliteStore.SetKv(pid, "formation", "modify_id", req.Id.ToString());
            return OkResp(EMessageId.FormationModifyResponse, new FormationModifyResponse { ErrorCode = ErrorCode.Success, Formation = new Formation { Id = req.Id } });
        };
        map[(ushort)EMessageId.FormationChangeRequest] = (pid, payload) =>
        {
            var req = Parse<FormationChangeRequest>(payload);
            if (pid > 0) SqliteStore.SetKv(pid, "formation", "current_id", req.FormationId.ToString());
            return OkResp(EMessageId.FormationChangeResponse, new FormationChangeResponse { ErrorCode = ErrorCode.Success, FormationId = req.FormationId });
        };
        map[(ushort)EMessageId.FormationSwitchCurrentRequest] = (pid, payload) =>
        {
            var req = Parse<FormationSwitchCurrentRequest>(payload);
            if (pid > 0) SqliteStore.SetKv(pid, "formation", "current_id", req.Id.ToString());
            return OkResp(EMessageId.FormationSwitchCurrentResponse, new FormationSwitchCurrentResponse { ErrorCode = ErrorCode.Success, CurrentId = req.Id });
        };
        map[(ushort)EMessageId.FormationSwitchRoleRequest] = (_, payload) =>
        {
            var req = Parse<FormationSwitchRoleRequest>(payload);
            return OkResp(EMessageId.FormationSwitchRoleResponse, new FormationSwitchRoleResponse { ErrorCode = ErrorCode.Success, RoleIndex = req.RoleIndex });
        };
        map[(ushort)EMessageId.MainPhantomRecommendRequest] = (_, _) => OkResp(EMessageId.MainPhantomRecommendResponse, new MainPhantomRecommendResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.PhantomRecommendRequest] = (_, payload) =>
        {
            var req = Parse<PhantomRecommendRequest>(payload);
            return OkResp(EMessageId.PhantomRecommendResponse, new PhantomRecommendResponse { ErrorCode = ErrorCode.Success, RoleId = req.RoleId });
        };
        map[(ushort)EMessageId.PhantomReturnRequest] = (_, _) => OkResp(EMessageId.PhantomReturnResponse, new PhantomReturnResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.PhantomReturnPreviewRequest] = (_, _) => OkResp(EMessageId.PhantomReturnPreviewResponse, new PhantomReturnPreviewResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.UpdatePhantomEquipGroupRequest] = (_, _) => OkResp(EMessageId.UpdatePhantomEquipGroupResponse, new UpdatePhantomEquipGroupResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.UpdatePlayerPassiveSkillRequest] = (_, _) => OkResp(EMessageId.UpdatePlayerPassiveSkillResponse, new UpdatePlayerPassiveSkillResponse { ErrCode = ErrorCode.Success });
        map[(ushort)EMessageId.BuffEffectExecuteRequest] = (_, _) => OkResp(EMessageId.BuffEffectExecuteResponse, new BuffEffectExecuteResponse { Code = ErrorCode.Success });
        map[(ushort)EMessageId.MontagePlayRequest] = (_, payload) =>
        {
            var req = Parse<MontagePlayRequest>(payload);
            return OkResp(EMessageId.MontagePlayResponse, new MontagePlayResponse { MontageIndex = req.MontageIndex, Error = new DErrorResult() });
        };
        map[(ushort)EMessageId.RoleFavorTalkScoreRequest] = (_, _) => OkResp(EMessageId.RoleFavorTalkScoreResponse, new RoleFavorTalkScoreResponse { ErrCode = ErrorCode.Success, IsComplete = true });
        map[(ushort)EMessageId.RoleQuestPointStateRequest] = (_, _) => OkResp(EMessageId.RoleQuestPointStateResponse, new RoleQuestPointStateResponse { CurrPoint = 0, MaxPoint = 100 });
        map[(ushort)EMessageId.CreatStoryCharacterRequest] = (_, _) => OkResp(EMessageId.CreatStoryCharacterResponse, new CreatStoryCharacterResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.DeleteStoryCharacterRequest] = (_, _) => OkResp(EMessageId.DeleteStoryCharacterResponse, new DeleteStoryCharacterResponse { ErrorCode = ErrorCode.Success });

        // World, Combat Init, Explore Skills, DragonPool & Items
        map[(ushort)EMessageId.ActiveDragonPoolRequest] = (pid, payload) =>
        {
            var req = Parse<ActiveDragonPoolRequest>(payload);
            if (pid > 0 && req.DragonPoolId > 0) SqliteStore.SetActivityClaim(pid, $"dragon_pool:{req.DragonPoolId}", "1");
            return OkResp(EMessageId.ActiveDragonPoolResponse, new ActiveDragonPoolResponse { ErrorCode = ErrorCode.Success, DragonPoolId = req.DragonPoolId, ActiveStatus = 1 });
        };
        map[(ushort)EMessageId.SpecialItemEquipRequest] = (pid, _) =>
        {
            if (pid > 0) SqliteStore.SetKv(pid, "special_item", "equipped", "1");
            return OkResp(EMessageId.SpecialItemEquipResponse, new SpecialItemEquipResponse { ErrorCode = ErrorCode.Success });
        };
        map[(ushort)EMessageId.SpecialItemUnEquipRequest] = (pid, _) =>
        {
            if (pid > 0) SqliteStore.SetKv(pid, "special_item", "equipped", "0");
            return OkResp(EMessageId.SpecialItemUnEquipResponse, new SpecialItemUnEquipResponse { ErrorCode = ErrorCode.Success });
        };
        map[(ushort)EMessageId.ItemDisuseRequest] = (_, _) => OkResp(EMessageId.ItemDisuseResponse, new ItemDisuseResponse { ErrCode = ErrorCode.Success });
        map[(ushort)EMessageId.UnlockInstEntranceRequest] = (_, _) => OkResp(EMessageId.UnlockInstEntranceResponse, new UnlockInstEntranceResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.AddReviveRegionRequest] = (_, _) => OkResp(EMessageId.AddReviveRegionResponse, new AddReviveRegionResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.RemoveReviveRegionRequest] = (_, _) => OkResp(EMessageId.RemoveReviveRegionResponse, new RemoveReviveRegionResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.AttributeInitRequest] = (_, payload) =>
        {
            var req = Parse<AttributeInitRequest>(payload);
            return OkResp(EMessageId.AttributeInitResponse, new AttributeInitResponse { Success = true, Id = req.Id });
        };
        map[(ushort)EMessageId.PartComponentInitRequest] = (_, _) => OkResp(EMessageId.PartComponentInitResponse, new PartComponentInitResponse());
        map[(ushort)EMessageId.GameplayTagRequest] = (_, _) => OkResp(EMessageId.GameplayTagResponse, new GameplayTagResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.CreatureDetailRequest] = (_, payload) =>
        {
            var req = Parse<CreatureDetailRequest>(payload);
            return OkResp(EMessageId.CreatureDetailResponse, new CreatureDetailResponse { ErrorCode = ErrorCode.Success, Id = req.Id, ConditionSatisfy = true });
        };
        map[(ushort)EMessageId.TraceEntityPosRequest] = (_, _) => AkiResult.OkRaw((ushort)EMessageId.TraceEntityPosResponse, new byte[] { 0x48, 0x00, 0x2A, 0x00 });
        map[(ushort)EMessageId.ScheduleNpcUpdatePosRequest] = (_, _) => OkResp(EMessageId.ScheduleNpcUpdatePosResponse, new ScheduleNpcUpdatePosResponse { ErrCode = ErrorCode.Success });
        map[(ushort)EMessageId.SneakFinishRequest] = (_, _) => OkResp(EMessageId.SneakFinishResponse, new SneakFinishResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.PolluteGridCellRequest] = (_, _) => OkResp(EMessageId.PolluteGridCellResponse, new PolluteGridCellResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.RemoveInstanceTimerRequest] = (_, payload) =>
        {
            var req = Parse<RemoveInstanceTimerRequest>(payload);
            return OkResp(EMessageId.RemoveInstanceTimerResponse, new RemoveInstanceTimerResponse { ErrorCode = ErrorCode.Success, TimerId = req.TimerId });
        };
        map[(ushort)EMessageId.SetAlertUiVisibleRequest] = (_, _) => OkResp(EMessageId.SetAlertUiVisibleResponse, new SetAlertUiVisibleResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.CheckGearRequest] = (_, _) => OkResp(EMessageId.CheckGearResponse, new CheckGearResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.GravityGearRequest] = (_, _) => OkResp(EMessageId.GravityGearResponse, new GravityGearResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.FollowTrackRequest] = (_, _) => OkResp(EMessageId.FollowTrackResponse, new FollowTrackResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.ExploreSkillPullGiantRequest] = (_, payload) =>
        {
            var req = Parse<ExploreSkillPullGiantRequest>(payload);
            return OkResp(EMessageId.ExploreSkillPullGiantResponse, new ExploreSkillPullGiantResponse { Code = ErrorCode.Success, EntityId = req.EntityId });
        };
        map[(ushort)EMessageId.ExploreSkillCustomRequest] = (_, payload) =>
        {
            var req = Parse<ExploreSkillCustomRequest>(payload);
            return OkResp(EMessageId.ExploreSkillCustomResponse, new ExploreSkillCustomResponse { Code = ErrorCode.Success, EntityId = req.EntityId });
        };
        map[(ushort)EMessageId.UseDetectionSkillRequest] = (_, _) => OkResp(EMessageId.UseDetectionSkillResponse, new UseDetectionSkillResponse { Code = ErrorCode.Success });
        map[(ushort)EMessageId.RTimeStopAnimRequest] = (_, _) => OkResp(EMessageId.RTimeStopAnimResponse, new RTimeStopAnimResponse { ErrorCode = ErrorCode.Success });

        // YuXue Mode
        map[(ushort)EMessageId.SetPlayerYuXueModeRequest] = (pid, _) =>
        {
            if (pid > 0) SqliteStore.SetKv(pid, "yuxue", "player_mode", "1");
            return OkResp(EMessageId.SetPlayerYuXueModeResponse, new SetPlayerYuXueModeResponse { ErrorCode = ErrorCode.Success });
        };
        map[(ushort)EMessageId.SetBtObjYuXueModeRequest] = (_, _) => OkResp(EMessageId.SetBtObjYuXueModeResponse, new SetBtObjYuXueModeResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.GetYuXueModeInfoRequest] = (_, _) => OkResp(EMessageId.GetYuXueModeInfoResponse, new GetYuXueModeInfoResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.GetTraceQuestYuXueModeRequest] = (pid, _) =>
        {
            bool active = pid > 0 && SqliteStore.GetKv(pid, "yuxue", "player_mode") == "1";
            return OkResp(EMessageId.GetTraceQuestYuXueModeResponse, new GetTraceQuestYuXueModeResponse { ErrorCode = ErrorCode.Success, InYuXueMode = active });
        };

        // Activity & Minigame Handlers + SQLite Reward Claims
        map[(ushort)EMessageId.MapTravelSoarRewardRequest] = (pid, payload) =>
        {
            var req = Parse<MapTravelSoarRewardRequest>(payload);
            return GrantClaimReward(pid, $"map_travel_soar:{req.TaskId}", EMessageId.MapTravelSoarRewardResponse, new MapTravelSoarRewardResponse { ErrorCode = ErrorCode.Success }, (3, 40), (2, 10000));
        };
        map[(ushort)EMessageId.MapTravelTaskRewardRequest] = (pid, payload) =>
        {
            var req = Parse<MapTravelTaskRewardRequest>(payload);
            return GrantClaimReward(pid, $"map_travel_task:{req.TaskId}", EMessageId.MapTravelTaskRewardResponse, new MapTravelTaskRewardResponse { ErrorCode = ErrorCode.Success }, (3, 40), (2, 10000));
        };
        map[(ushort)EMessageId.RogueWeeklyRewardRequest] = (pid, payload) =>
        {
            var req = Parse<RogueWeeklyRewardRequest>(payload);
            return GrantClaimReward(pid, $"rogue_weekly_rew:{req.ActivityId}:{req.ConfigId}", EMessageId.RogueWeeklyRewardResponse, new RogueWeeklyRewardResponse { ErrorCode = ErrorCode.Success }, (3, 80), (2, 20000));
        };
        map[(ushort)EMessageId.RiskHarvestBuffCountRewardRequest] = (pid, payload) =>
        {
            var req = Parse<RiskHarvestBuffCountRewardRequest>(payload);
            return GrantClaimReward(pid, $"risk_buff_cnt:{req.Id}", EMessageId.RiskHarvestBuffCountRewardResponse, new RiskHarvestBuffCountRewardResponse { ErrorCode = ErrorCode.Success }, (3, 40), (2, 10000));
        };
        map[(ushort)EMessageId.RiskHarvestBuffUnlockRewardRequest] = (pid, payload) =>
        {
            var req = Parse<RiskHarvestBuffUnlockRewardRequest>(payload);
            return GrantClaimReward(pid, $"risk_buff_unlock:{req.Id}", EMessageId.RiskHarvestBuffUnlockRewardResponse, new RiskHarvestBuffUnlockRewardResponse { ErrorCode = ErrorCode.Success }, (3, 40), (2, 10000));
        };
        map[(ushort)EMessageId.CorniceMeetingRewardRequest] = (pid, payload) =>
        {
            var req = Parse<CorniceMeetingRewardRequest>(payload);
            return GrantClaimReward(pid, $"cornice:{req.LevelPlayId}:{req.ScoreIndex}", EMessageId.CorniceMeetingRewardResponse, new CorniceMeetingRewardResponse { ErrorCode = ErrorCode.Success }, (3, 40), (2, 10000));
        };
        map[(ushort)EMessageId.CircumFluenceTaskRewardRequest] = (pid, payload) =>
        {
            var req = Parse<CircumFluenceTaskRewardRequest>(payload);
            return GrantClaimReward(pid, $"circum:{req.ActivityId}:{(int)req.RewardType}", EMessageId.CircumFluenceTaskRewardResponse, new CircumFluenceTaskRewardResponse { ErrorCode = ErrorCode.Success }, (3, 40), (2, 10000));
        };
        map[(ushort)EMessageId.PhantomBattleMasterLevelRewardRequest] = (pid, payload) =>
        {
            var req = Parse<PhantomBattleMasterLevelRewardRequest>(payload);
            return GrantClaimReward(pid, $"pb_master:{req.PhantomBattleMasterLevelId}", EMessageId.PhantomBattleMasterLevelRewardResponse, new PhantomBattleMasterLevelRewardResponse { ErrorCode = ErrorCode.Success }, (3, 40), (2, 10000));
        };
        map[(ushort)EMessageId.RoguelikeWriteCatRewardRequest] = (pid, payload) =>
        {
            var req = Parse<RoguelikeWriteCatRewardRequest>(payload);
            return GrantClaimReward(pid, $"rogue_cat:{req.ActivityId}:{req.Index}", EMessageId.RoguelikeWriteCatRewardResponse, new RoguelikeWriteCatRewardResponse { ErrorCode = ErrorCode.Success }, (3, 40), (2, 10000));
        };
        map[(ushort)EMessageId.RoguelikeSeasonDataRequest] = (_, _) => OkResp(EMessageId.RoguelikeSeasonDataResponse, new RoguelikeSeasonDataResponse { ErrorCode = ErrorCode.Success, SeasonData = new SeasonData() });
        map[(ushort)EMessageId.RoguelikeBossInstStartRequest] = (_, _) => OkResp(EMessageId.RoguelikeBossInstStartResponse, new RoguelikeBossInstStartResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.RogueTowerTrialReChallengeRequest] = (_, _) => OkResp(EMessageId.RogueTowerTrialReChallengeResponse, new RogueTowerTrialReChallengeResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.RoverRogueTalentInfoRequest] = (_, _) => OkResp(EMessageId.RoverRogueTalentInfoResponse, new RoverRogueTalentInfoResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.DirectTrainInfoRequest] = (_, _) => OkResp(EMessageId.DirectTrainInfoResponse, new DirectTrainInfoResponse { MainActivity = new ActivityData { Type = ActivityType.ThroughTrain } });
        map[(ushort)EMessageId.ExchangeRewardInfoRequest] = (_, _) => OkResp(EMessageId.ExchangeRewardInfoResponse, new ExchangeRewardInfoResponse());
        map[(ushort)EMessageId.TrapDefenseQuickEquipRequest] = (_, _) => OkResp(EMessageId.TrapDefenseQuickEquipResponse, new TrapDefenseQuickEquipResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.MixDrinkTopicSelectRequest] = (_, _) => OkResp(EMessageId.MixDrinkTopicSelectResponse, new MixDrinkTopicSelectResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.FarmGoldLevelPlayRequest] = (_, _) => OkResp(EMessageId.FarmGoldLevelPlayResponse, new FarmGoldLevelPlayResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.FarmGoldPointRequest] = (_, _) => OkResp(EMessageId.FarmGoldPointResponse, new FarmGoldPointResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.FloroRanchWeeklyRequest] = (_, _) => OkResp(EMessageId.FloroRanchWeeklyResponse, new FloroRanchWeeklyResponse { ErrorCode = ErrorCode.Success, PlayerInfo = new FloroRanchPlayerInfo() });
        map[(ushort)EMessageId.FloroRanchPlayEventChoiceRequest] = (_, _) => OkResp(EMessageId.FloroRanchPlayEventChoiceResponse, new FloroRanchPlayEventChoiceResponse { ErrorCode = ErrorCode.Success, Update = new FloroRanchPlayInfo() });
        map[(ushort)EMessageId.FloroRanchPlayRefreshGachaRequest] = (_, _) => OkResp(EMessageId.FloroRanchPlayRefreshGachaResponse, new FloroRanchPlayRefreshGachaResponse { ErrorCode = ErrorCode.Success, Gacha = new FloroRanchGacha() });
        map[(ushort)EMessageId.RacingBetMatchRoundRequest] = (_, _) => OkResp(EMessageId.RacingBetMatchRoundResponse, new RacingBetMatchRoundResponse { ErrorCode = ErrorCode.Success, Round = new RacingBetsMatchRound() });
        map[(ushort)EMessageId.RacingBetsPlayerInfoRequest] = (_, _) => OkResp(EMessageId.RacingBetsPlayerInfoResponse, new RacingBetsPlayerInfoResponse { ErrorCode = ErrorCode.Success, PlayerInfo = new RacingBetPlayerInfo() });
        map[(ushort)EMessageId.RacingBetsTaskRequest] = (_, _) => OkResp(EMessageId.RacingBetsTaskResponse, new RacingBetsTaskResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.RacingBetsLegMatchInfoRequest] = (_, _) => OkResp(EMessageId.RacingBetsLegMatchInfoResponse, new RacingBetsLegMatchInfoResponse { ErrorCode = ErrorCode.Success, LegInfo = new RacingBetsLegMatchesInfo() });
        map[(ushort)EMessageId.RacingBetMatchRequest] = (_, payload) =>
        {
            var req = Parse<RacingBetMatchRequest>(payload);
            return OkResp(EMessageId.RacingBetMatchResponse, new RacingBetMatchResponse { ErrorCode = ErrorCode.Success, ActivityId = req.ActivityId, LegMatchId = req.LegMatchId });
        };
        map[(ushort)EMessageId.PhantomBattleDebugInfoRequest] = (_, _) => OkResp(EMessageId.PhantomBattleDebugInfoResponse, new PhantomBattleDebugInfoResponse { ErrorCode = ErrorCode.Success });
        map[(ushort)EMessageId.PhantomBattleAreaSkillRequest] = (_, _) => OkResp(EMessageId.PhantomBattleAreaSkillResponse, new PhantomBattleAreaSkillResponse { ErrorCode = ErrorCode.Success, PhantomBattleSkillTriggerInfos = new PhantomBattleSkillTriggerInfo() });
        map[(ushort)EMessageId.PhantomBattlePhantomCardSkillRequest] = (_, payload) =>
        {
            var req = Parse<PhantomBattlePhantomCardSkillRequest>(payload);
            return OkResp(EMessageId.PhantomBattlePhantomCardSkillResponse, new PhantomBattlePhantomCardSkillResponse { ErrorCode = ErrorCode.Success, CardUId = req.CardUid });
        };

        // Network, Platform, Debug, System
        map[(ushort)EMessageId.CheckPublicResourceVersionRequest] = (_, _) => OkResp(EMessageId.CheckPublicResourceVersionResponse, new CheckPublicResourceVersionResponse { ErrorId = ErrorCode.Success });
        map[(ushort)EMessageId.PlayerGameTokenRequest] = (pid, _) => OkResp(EMessageId.PlayerGameTokenResponse, new PlayerGameTokenResponse { GameToken = GetOrCreateGmSignToken(pid) });
        map[(ushort)EMessageId.MutiplayerTeamRefreshNearListRequest] = (_, _) => OkResp(EMessageId.MutiplayerTeamRefreshNearListResponse, new MutiplayerTeamRefreshNearListResponse { ErrorCode = ErrorCode.Success, NextRefreshTime = 5 });
        map[(ushort)EMessageId.PlayerPsnBlockListRequest] = (_, _) => OkResp(EMessageId.PlayerPsnBlockListResponse, new PlayerPsnBlockListResponse());
        map[(ushort)EMessageId.PlayerCreateXboxSessionRequest] = (_, _) => OkResp(EMessageId.PlayerCreateXboxSessionResponse, new PlayerCreateXboxSessionResponse());
        map[(ushort)EMessageId.PsnSettingRequest] = (_, _) => OkResp(EMessageId.PsnSettingResponse, new PsnSettingResponse());
        map[(ushort)EMessageId.GatewayEchoRequest] = (_, payload) =>
        {
            var req = Parse<GatewayEchoRequest>(payload);
            return OkResp(EMessageId.GatewayEchoResponse, new GatewayEchoResponse { Content = req.Content ?? string.Empty });
        };
        map[(ushort)EMessageId.NetTestDataRequest] = (_, _) => OkResp(EMessageId.NetTestDataResponse, new NetTestDataResponse());
        map[(ushort)EMessageId.PressRequest] = (_, _) => OkResp(EMessageId.PressResponse, new PressResponse { Payload = ByteString.Empty });
        map[(ushort)EMessageId.PressGameRequest] = (_, _) => OkResp(EMessageId.PressGameResponse, new PressGameResponse { Payload = ByteString.Empty });
        map[(ushort)EMessageId.EnableDamageRecordRequest] = (_, _) => AkiResult.OkRaw((ushort)EMessageId.EnableDamageRecordResponse, Array.Empty<byte>());
        map[(ushort)EMessageId.CombatMaxCaseMessageRequest] = (_, _) => AkiResult.OkRaw((ushort)EMessageId.CombatMaxCaseMessageResponse, Array.Empty<byte>());
        map[(ushort)EMessageId.CheatInputRequest] = (_, _) => AkiResult.OkRaw((ushort)EMessageId.CheatInputResponse, Array.Empty<byte>());
        map[(ushort)EMessageId.RunTestRequest] = (_, _) => OkResp(EMessageId.RunTestResponse, new RunTestResponse { Code = ErrorCode.Success });
        map[(ushort)EMessageId.GmLevelActionRequest] = (_, _) => OkResp(EMessageId.GmLevelActionResponse, new GmLevelActionResponse { Code = ErrorCode.Success });
        map[(ushort)EMessageId.KcpConvRequest] = (_, payload) =>
        {
            var req = Parse<KcpConvRequest>(payload);
            return OkResp(EMessageId.KcpConvResponse, new KcpConvResponse
            {
                Code = ErrorCode.Success,
                Conv = 1,
                CrcCheck = req.DisableCrc ? 0 : 1,
                KcpMtu = 1400,
            });
        };
    }
}
