// Auto-generated from proto/src/pb.zig. Do not edit by hand.
#![allow(non_camel_case_types, dead_code, clippy::all)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DoubleDropFrom {
    UnDefineDouble = 0,
    DoubleActivity = 1,
    FromRegress = 2,
}

impl Default for DoubleDropFrom {
    fn default() -> Self {
        Self::UnDefineDouble
    }
}

impl crate::wire::PbEnum for DoubleDropFrom {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::UnDefineDouble,
            1 => Self::DoubleActivity,
            2 => Self::FromRegress,
            _ => Self::UnDefineDouble,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DoubleDropFrom {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ErrorCode {
    Success = 0,
    RequestParamError = 1,
    InternalError = 2,
    UnKnownError = 3,
    ServerNotReady = 4,
    ServerFullLoad = 5,
    InvalidUserId = 6,
    InvalidToken = 7,
    InvalidRequest = 8,
    GmFail = 9,
    GmParamError = 10,
    GmException = 11,
    NotEnableGm = 12,
    NotElegantName = 13,
    ContainsDirtyWord = 14,
    DirtyWordServerError = 15,
    DirtyWordServerReturnEmpty = 16,
    DirtyWordCheckException = 17,
    ComponentNotExist = 18,
    ServerFullLoadGame = 19,
    ServerFullLoadGate = 20,
    PlayerLoggingInFlag = 21,
    GateLoginException = 22,
    GateLoginSeverSelectErr = 23,
    GateTokenAccessErr = 24,
    GateTokenAccessException = 25,
    GateLoginUserIdErr = 26,
    GateLoginNodeIdErr = 27,
    GateLoginCreateCharacterErr = 28,
    GateCreateCharacterException = 29,
    GateEnterGameAddressNotFound = 30,
    GateEnterGameException = 31,
    GateEnterGameCreatePlayerErr = 32,
    GateEnterGameAddPlayerErr = 33,
    GameGateNodeNotFound = 34,
    GamePlayerAdminExist = 35,
    GameReloginGateNodeNotFound = 36,
    GameReloginPlayerNotFound = 37,
    ServerNotOpen = 38,
    ServerMaintenance = 39,
    InvalidLoginType = 40,
    InvalidGateway = 41,
    SdkserverError = 42,
    TokenNotAuthrized = 43,
    HadBan = 44,
    NotInUserIdWhiteList = 45,
    NoHealthyGamesvr = 46,
    NoHealthyGateway = 47,
    GarFailed = 48,
    GarSdkCheckFail = 49,
    GarNoneUserInfo = 50,
    GarQueryUserInfoError = 51,
    GarNoRegion = 52,
    InternalExceptionCode = 53,
    DecodeExceptionCode = 54,
    EncodeExceptionCode = 55,
    InvalidRequestExceptionCode = 56,
    MessageOutOfLimitExceptionCode = 57,
    MessageNoHandler = 58,
    EncryptionNoCreate = 59,
    DecryptFail = 60,
    PlayerNotInTheScene = 61,
    NonReentrantExceptionCode = 62,
    PlayerLoggedOut = 63,
    MsgFunctionClose = 64,
    SeqNoError = 65,
    InvalidMessageType = 66,
    InvalidMessageHeader = 67,
    InvalidSeqNo = 68,
    InvalidMessageId = 69,
    ProtobufDecodeFailed = 70,
    ErrProtoSeedCheck = 71,
    MessageCouldNotBeRouted = 72,
    ErrPlayerLogined = 73,
    ClosedRegister = 100000,
    RegisterOutOfLimit = 100001,
    HaveNoCharacter = 100002,
    InvalidCharacterName = 100003,
    CreateCharacterFailed = 100004,
    CreateCharacterDuplicateKey = 100005,
    PlayerAlreadyLogin = 100006,
    PlayerLoggingIn = 100007,
    ErrLoginGwreconnecting = 100008,
    LoginRetry = 100009,
    QueryPlayerDataFailed = 100010,
    CheckPlayerDataFailed = 100011,
    CheckPlayerDataFailedDebug = 100012,
    LogoutUnknownError = 100013,
    AccountLoggedInElsewhere = 100014,
    AccountIsBlocked = 100015,
    DataOverflow = 100016,
    AccountBeKick = 100017,
    AppVersionNotMatch = 100018,
    LauncherVersionIsTooLow = 100019,
    ResourceVersionIsTooLow = 100020,
    CloseConnection = 100021,
    ErrAcquirePlayerLockFailed = 100022,
    ErrPlayerLoggingOut = 100023,
    MessageChecksumFailed = 100024,
    LoginTimeout = 100025,
    ErrWeaponDefault = 200000,
    ErrWeaponLevelLimit = 200001,
    ErrWeaponBreachLimit = 200002,
    ErrWeaponConsumeInvalid = 200003,
    ErrWeaponPkgFull = 200004,
    ErrRoleNoConfig = 200005,
    ErrRoleIsActive = 200006,
    ErrRoleNotActive = 200007,
    ErrRoleOverNotEnough = 200008,
    ErrRoleLevelNotEnough = 200009,
    ErrRoleException = 200010,
    ErrRoleNotExchange = 200011,
    ErrRoleResonNotActive = 200012,
    ErrRoleResonIsActive = 200013,
    ErrRoleConfigNotRight = 200014,
    ErrRoleLevelMax = 200015,
    ErrRolePerResonNotActive = 200016,
    ErrRoleConditionNotFind = 200017,
    ErrRoleConditionNoEnough = 200018,
    ErrRoleInvalidNameLength = 200019,
    ErrRoleExpInvalid = 200020,
    ErrRoleActiveNeedNoEnough = 200021,
    ErrRoleResonMaxLevel = 200022,
    ErrRoleProtoError = 200023,
    ErrRoleItemListEmpty = 200024,
    ErrRoleItemListCountOutRange = 200025,
    ErrRoleItemExpError = 200026,
    ErrRolePhantPosError = 200027,
    ErrRolePhantSameError = 200028,
    ErrRolePhantEmptyError = 200029,
    ErrRoleItemListNoEnough = 200030,
    ErrRoleGetSkillByIdFailed = 200031,
    ErrRoleFavorLevelNotEnough = 200032,
    ErrRolSkillNodeType = 200033,
    ErrRolSkillNodeTypeActive = 200034,
    ErrRolSkillNodeTypeUlock = 200035,
    ErrRolSkillPointsNotEnough = 200036,
    ErrTrialRoleExist = 200037,
    ErrTrialRoleNotExist = 200038,
    ErrTrialRoleRegionDataExist = 200039,
    ErrTrialRoleBtObjDataExist = 200040,
    ErrTrialRoleRegionExist = 200041,
    ErrTrialRoleRegionNotExist = 200042,
    ErrLoadEquipDefault = 200043,
    ErrLoadEquipInvalidPos = 200044,
    ErrLoadEquipInvalidRole = 200045,
    ErrLoadEquipRoleConfig = 200046,
    ErrPhantomIdNotExist = 200047,
    ErrPhantomNotExist = 200048,
    ErrPhantomLvupMax = 200049,
    ErrPhantomLvupMismatchItemId = 200050,
    ErrPhantomLvupNoItem = 200051,
    ErrPhantomLvupLimit = 200052,
    ErrPhantomItemType = 200053,
    ErrPhantomInvalidPos = 200054,
    ErrPhantomConfigNotFound = 200055,
    ErrPhantomItemNotExist = 200056,
    ErrPhantomPropNotExist = 200057,
    ErrPhantomQaulityNotExist = 200058,
    ErrPhantomBreachNotExist = 200059,
    ErrPhantomLevelNotEnough = 200060,
    ErrPhantomExpItemNotExist = 200061,
    ErrPhantomSubPropRandomErr = 200062,
    ErrPhantomSubPropNotEnough = 200063,
    ErrPhantomSubPropGenDupicate = 200064,
    ErrPhantomSubStrengthenPropNotExist = 200065,
    ErrPhantomLevelConfigNotExist = 200066,
    ErrPhantomLevelUpConsumeItemNotEnough = 200067,
    ErrPhantomLevelUpMaterialLock = 200068,
    ErrPhantomLevelUpConsumeItemErr = 200069,
    ErrPhantomLevelUpRepeatItem = 200070,
    ErrPhantomMainPropNotExist = 200071,
    ErrPhantomGrowthNotExist = 200072,
    ErrPhantomBreachItemCount = 200073,
    ErrPhantomBreachRepeatItem = 200074,
    ErrPhantomDecomposeEquiped = 200075,
    ErrPhantomDecomposeFail = 200076,
    ErrPhantomBreachBindItem = 200077,
    ErrPhantomBreachErrItem = 200078,
    ErrPhantomRecommendNoData = 200079,
    ErrPhantomCannotTakeOff = 200080,
    ErrPhantomCannotReplace = 200081,
    ErrVisionSkillFavoriteTypeLimit = 200082,
    ErrVisionSkillFavoriteCountLimit = 200083,
    ErrVisionSkillCfgNotFound = 200084,
    ErrVisionSkillNotFound = 200085,
    ErrVisionSkillLevelUpMax = 200086,
    ErrVisionSkillLevelUpLimit = 200087,
    ErrVisionSkillSlotNotFound = 200088,
    ErrVisionSkillEquipTypeLimit = 200089,
    ErrVisionSkillUnEquipLimit = 200090,
    ErrVisionSkillGemCfgNotFound = 200091,
    ErrVisionSkillEquipLimit = 200092,
    ErrVisionSkillGemLimit = 200093,
    ErrVisionSkillOperFail = 200094,
    ErrVisionSkillSlotEquipLimit = 200095,
    ErrExploreSkillRouletteRepeat = 200096,
    ErrItemCfgNotFound = 200097,
    ErrItemNotFound = 200098,
    ErrItemNotEnough = 200099,
    ErrItemDecomposeLimit = 200100,
    ErrItemUseLevelLimit = 200101,
    ErrItemLockLimit = 200102,
    ErrItemInvalidParams = 200103,
    ErrItemDecomposeFail = 200104,
    ErrItemUseFail = 200105,
    ErrExchangeRewardCostItemNotEnough = 200106,
    ExchangeRewardSuccess = 200107,
    ErrPkgCapacityNotEnough = 200108,
    ErrGiftOptionalCount = 200109,
    ErrGiftOptionalNotExists = 200110,
    ErrGiftNotExists = 200111,
    ErrItemCount = 200112,
    ErrItemIdNotContain = 200113,
    ErrItemTypeNotContain = 200114,
    ErrCalabashMaxLevel = 200115,
    ErrCalabashConfig = 200116,
    ErrCalabashLevelUp = 200117,
    ErrCalabashExp = 200118,
    ErrCalabashDevelopNoReward = 200119,
    ErrCalabashMonsterNotFound = 200120,
    PropRewardTips = 200121,
    ErrEnergyMaxCharge = 200122,
    ErrStateCanotTeleport = 200123,
    ErrStateCannotEnterInst = 200124,
    ErrStateCannotOnline = 200125,
    ErrStateCannotChangeFormation = 200126,
    ErrReportPlayerCountLimit = 200127,
    ErrReportPlayerReasonNotFound = 200128,
    ErrReportMessageLengthLimit = 200129,
    ErrCookingToolFixed = 200130,
    ErrCookingFormulaNotFound = 200131,
    ErrCookingCount = 200132,
    ErrCookingProcessNotFound = 200133,
    ErrCookingLevelNotFound = 200134,
    ErrCookingLevelLimt = 200135,
    ErrCookingInteractiveNotFound = 200136,
    ErrCookingFuncNotOpen = 200137,
    ErrChallengeNotFound = 200138,
    ErrChallengeNoTeam = 200139,
    ErrChallengeTeamLimit = 200140,
    ErrChallengeTeamMemLimit = 200141,
    ErrChallengeChangeFormation = 200142,
    ErrChallengeFunNotOpen = 200143,
    ErrChallengeSeasonUpdate = 200144,
    ErrChallengeLockRoleLimit = 200145,
    ErrChallengeRoleLocked = 200146,
    ErrChallengeNoRoleAlive = 200147,
    ErrChallengeFormationEmpty = 200148,
    ErrCycleChallengeNoRoleAlive = 200149,
    ErrCycleChallengeFormationEmpty = 200150,
    ErrInfluenceLocked = 200151,
    ErrInfluenceRewardNotFound = 200152,
    ErrInfluenceConfigNotFound = 200153,
    ErrReputationLimit = 200154,
    ErrInfluenceRewardFailed = 200155,
    ErrInfluenceFunNotOpen = 200156,
    ErrForgeFuncNotOpen = 200157,
    ErrForgeCountLimit = 200158,
    ErrForgeLocked = 200159,
    ErrForgeConfigNotFound = 200160,
    ErrForgeUnlocked = 200161,
    ErrSynthesisFuncNotOpen = 200162,
    ErrSynthesisConfigNotFound = 200163,
    ErrSynthesisCountLimit = 200164,
    ErrSynthesisLocked = 200165,
    ErrSynthesisLevelNotFound = 200166,
    ErrSynthesisLevelLimit = 200167,
    ErrSynthesisCannotUnlock = 200168,
    ErrSynthesisUnlocked = 200169,
    ErrTrialRoleCannotMatch = 200170,
    ErrPhantomFormationTeleport = 200171,
    ErrPhantomFormationEnterInst = 200172,
    ErrPhantomFormationMultiPlay = 200173,
    ErrPhantomFormationAdvice = 200174,
    ErrPhantomFormationChangeFormation = 200175,
    ErrPhantomFormationRepeat = 200176,
    ErrPhantomFormationChangeFailed = 200177,
    ErrRoleChangeRoleCreateFailed = 200178,
    ErrRoleChangeRoleUpdateCreateFailed = 200179,
    ErrRoleChangeRoleNotUnlock = 200180,
    ErrRoleChangeMultiPlay = 200181,
    ErrRoleChangeInst = 200182,
    ErrRoleChangeElementFunc = 200183,
    ErrPhantomFormationCannotJoin = 200184,
    ErrPhantomFormationHost = 200185,
    ErrRoleChangeShowAllRole = 200186,
    ErrInteractBoardEntityNotFound = 200187,
    ErrInteractBoardRange = 200188,
    ErrInteractBoardSystemNotFound = 200189,
    ErrInteractBoardEntityConfig = 200190,
    ErrInteractEntranceNotFound = 200191,
    ErrInteractEntranceNotMatch = 200192,
    ErrItemMaxUseCount = 200193,
    ErrFuncNotExist = 200194,
    ErrPhantomChangeInBattle = 200195,
    ErrItemCanNotDestroy = 200196,
    ErrPhantomEquipSourceCost = 200197,
    ErrPhantomEquipTargetCost = 200198,
    ErrPhantomEquipDuplicate = 200199,
    ErrPhantomAutoEquipFromOther = 200200,
    ErrPhantomConsumeItemCount = 200201,
    ErrPhantomConsumeItemDuplicate = 200202,
    ErrPhantomConsumeItemIncrDuplicate = 200203,
    ErrPhantomConsumeItem = 200204,
    ErrPhantomConsumeNoExp = 200205,
    ErrPhantomBreachPos = 200206,
    ErrPhantomBreachSuspend = 200207,
    ErrPhantomBreachQuality = 200208,
    ErrPhantomBreachExp = 200209,
    ErrPhantomBreachConsumeItem = 200210,
    ErrPhantomBreachNoSuspend = 200211,
    ErrPhantomSpecialSkillRole = 200212,
    ErrPhantomNotEquip = 200213,
    ErrPhantomSpecilSkillPos = 200214,
    ErrPhantomSubPropPlanConfig = 200215,
    ErrPhantomMainPropGenFail = 200216,
    ErrLivenessFuncNotOpen = 200217,
    ErrLivenessTaskNotFound = 200218,
    ErrLivenessTaskDataNotFound = 200219,
    ErrLivenessTaskNotFinish = 200220,
    ErrLivenessTaskRewarded = 200221,
    ErrLivenessRewardNotFound = 200222,
    ErrLivenessGoalNotReach = 200223,
    ErrLivenessRewardParam = 200224,
    ErrLivenessTaskRewardParam = 200225,
    ErrWeaponLevelUpComsumeCount = 200226,
    ErrWeaponConsumeSelf = 200227,
    ErrWeaponConsumeItemNotFound = 200228,
    ErrWeaponConsumeItemIdNotFound = 200229,
    ErrWeaponLocked = 200230,
    ErrWeaponConsumeDuplicate = 200231,
    ErrWeaponEquiped = 200232,
    ErrWeaponLevelUpItemDuplicate = 200233,
    ErrWeaponLevelUpNoExp = 200234,
    ErrWeaponLevelUpLevel = 200235,
    ErrPhantomMainPropNotMatch = 200236,
    ErrPhantomSubPropNotMatch = 200237,
    ErrPhantomEquiped = 200238,
    ErrAdviceNotInit = 200239,
    ErrTowerChallengeNotOpen = 200240,
    ErrTowerNotInChallenge = 200241,
    ErrTowerConfigNotFound = 200242,
    ErrTowerChallengeNotInOpenTime = 200243,
    ErrTowerInChallenge = 200244,
    ErrTowerFormationCount = 200245,
    ErrTowerFormationRoleDuplicate = 200246,
    ErrTowerRoleCost = 200247,
    ErrTowerDifficultyNotClear = 200248,
    ErrTowerFloorNotClear = 200249,
    ErrTowerAreaNotClear = 200250,
    ErrTowerRecommendNotSettle = 200251,
    ErrTowerRewardNotFound = 200252,
    ErrTowerNoReward = 200253,
    ErrTowerRewarded = 200254,
    ErrTowerRewardTarget = 200255,
    ErrTowerSeasonUpdate = 200256,
    ErrLordGymConfigNotFound = 200257,
    ErrLordGymNotInPlay = 200258,
    ErrLordGymLock = 200259,
    ErrLordGymBtTreeNotFound = 200260,
    ErrRoleSexFuncNotOpen = 200261,
    ErrPhantomSubPropLocked = 200262,
    ErrPhantomIdentifyNoCost = 200263,
    ErrGiftPackType = 200264,
    ErrGiftPackUseLimit = 200265,
    ErrCdKeyNotEnable = 200266,
    ErrCdKeyRequestCount = 200267,
    ErrCdKeyRequestErr = 200268,
    ErrCdKeyRequestDataErr = 200269,
    ErrCdKeyException = 200270,
    ErrCdKeyProcessCount = 200271,
    ErrCdKeyNotFound = 200272,
    ErrCdKeyBatchNotFound = 200273,
    ErrCdKeyNotInValidTime = 200274,
    ErrCdKeyBatchMaxCount = 200275,
    ErrCdKeyEachPlayerMaxCount = 200276,
    ErrCdKeyGroupCount = 200277,
    ErrCdKeyCondition = 200278,
    ErrCdKeyAddCountFail = 200279,
    ErrCdKeyLength = 200280,
    ErrCdKeyCharacter = 200281,
    ErrGiftPackRandomErr = 200282,
    ErrReconnectUserWhiteList = 200283,
    ErrReconnectChannelWhiteList = 200284,
    ErrReconnectIpInvalid = 200285,
    ErrReconnectIpWhiteList = 200286,
    ErrCdKeyExpire = 200287,
    ErrWeaponResonLevelLimit = 200288,
    ErrWeaponConfigNotFound = 200289,
    ErrWeaponResonConfigNotFound = 200290,
    ErrWeaponResonConsumeItem = 200291,
    ErrWeaponResonConsumeGold = 200292,
    ErrDestroyItemDuplicate = 200293,
    ErrDestroyWeapon = 200294,
    ErrCannotDestroyItem = 200295,
    ErrCannotDestroyPhantom = 200296,
    ErrCannotDestroyWeaponForm = 200297,
    ErrCannotDestroyItemUnknown = 200298,
    ErrWeaponConsumeQuality = 200299,
    ErrPhantomSkinChangeCd = 200300,
    ErrPhantomSkinUnlock = 200301,
    ErrPhantomSkinMatch = 200302,
    ErrLoginGameTainted = 200303,
    ErrCookLimitCount = 200304,
    ErrCookLimitTime = 200305,
    ErrForgeLimitCount = 200306,
    ErrForgeLimitTime = 200307,
    ErrSynthesisLimitCount = 200308,
    ErrSynthesisLimitTime = 200309,
    ErrLoginIpBan = 200310,
    ErrLoginDeviceBan = 200311,
    ErrRoleNameEmpty = 200312,
    ErrAdviceLength = 200313,
    ErrPhantomRefiningCount = 200314,
    ErrPhantomRefiningScore = 200315,
    ErrPhantomRefiningTotalScore = 200316,
    ErrPhantomRefiningDeveloped = 200317,
    ErrPhotoMemoryCollectConfig = 200318,
    ErrPhotoMemoryFuncNotOpen = 200319,
    ErrPhotoMemoryCollectLock = 200320,
    ErrPhotoMemoryCollectRewarded = 200321,
    ErrRoleCount = 200322,
    ErrCookFormulaUnlocked = 200323,
    ErrForgeFormulaUnlocked = 200324,
    ErrSynthesisFormulaUnlocked = 200325,
    ErrCookFormulaBuyCount = 200326,
    ErrForgeFormulaBuyCount = 200327,
    ErrSynthesisFormulaBuyCount = 200328,
    ErrCdKeyDailyVerifyCount = 200329,
    ErrLongShanTaskNotFound = 200330,
    ErrLongShanActivityClosed = 200331,
    ErrLongShanTaskNotAccept = 200332,
    ErrLongShanTaskNotComplete = 200333,
    ErrLongShanTaskRewarded = 200334,
    ErrTowerDefenceRewardParamErr = 200335,
    ErrTowerDefenceInstanceNotFound = 200336,
    ErrTowerDefenceActivityNotOpen = 200337,
    ErrTowerDefenceActivityDataNotFound = 200338,
    ErrTowerDefenceInstDataNotFound = 200339,
    ErrTowerDefenceInstRewarded = 200340,
    ErrTowerDefenceInstScoreNotEnough = 200341,
    ErrTowerDefenceScoreRewarded = 200342,
    ErrTowerDefenceScoreRewardNotEnough = 200343,
    ErrTowerDefenceScoreRewardNotFound = 200344,
    ErrTowerDefenceInstBuffNotEnable = 200345,
    ErrTowerDefencePhantomDuplicate = 200346,
    ErrTowerDefencePhantomNotSelect = 200347,
    ErrorTowerDefenceInstNotOpen = 200348,
    ErrorTowerDefenceInstCondition = 200349,
    ErrNameModifyCd = 200350,
    ErrNameVerifying = 200351,
    ErrTimePointRewardActivityConfigNotFound = 200352,
    ErrTimePointRewardActivityNotOpen = 200353,
    ErrTimePointRewardActivityRewarded = 200354,
    ErrTimePointRewardActivityTime = 200355,
    ErrCdkeyVerifying = 200356,
    ErrorTowerDefenceInstLocked = 200357,
    ErrTrackMoonEntrustLocked = 200358,
    ErrTrackMoonRoleLocked = 200359,
    ErrTrackMoonBuildLocked = 200360,
    ErrTrackMoonBuildEntity = 200361,
    ErrCopyUserRequestErr = 200362,
    ErrCopyUserInserting = 200363,
    ErrCopyUserErr = 200364,
    ErrCopyUserDataErr = 200365,
    ErrCopyUserInsertErr = 200366,
    ErrCopyUserInsertFailed = 200367,
    ErrRiskHarvestBuffGroupNotFound = 200368,
    ErrRiskHarvestActivityClosePlay = 200369,
    ErrRiskHarvestModeChangeClosePlay = 200370,
    ErrRiskHarvestLeaveClosePlay = 200371,
    ErrRiskHarvestInstNotFound = 200372,
    ErrRiskHarvestActivityNotOpen = 200373,
    ErrRiskHarvestInstDataNotFound = 200374,
    ErrRiskHarvestInstRewarded = 200375,
    ErrRiskHarvestInstNotPass = 200376,
    ErrRiskHarvestScoreRewardNotFound = 200377,
    ErrRiskHarvestScoreRewarded = 200378,
    ErrRiskHarvestScoreNotEnough = 200379,
    ErrRiskHarvestRoleTrial = 200380,
    ErrRiskHarvestMatching = 200381,
    ErrRiskHarvestNotDefaultWorld = 200382,
    ErrRiskHarvestMultiMode = 200383,
    ErrRiskHarvestInstLocked = 200384,
    ErrRiskHarvestInstTeleportEntityNotFound = 200385,
    ErrRiskHarvestInstOpen = 200386,
    ErrRiskHarvestPlayOpenFailed = 200387,
    ErrRiskHarvestPlayDataNotFound = 200388,
    ErrRiskHarvestBuffNoReward = 200389,
    ErrRiskHarvestBuffRewarded = 200390,
    ErrRiskHarvestBuffLocked = 200391,
    ErrRiskHarvestBuffCountRewardNotFound = 200392,
    ErrRiskHarvestBuffCountRewarded = 200393,
    ErrRiskHarvestBuffCountNotEnough = 200394,
    ErrRiskHarvestInstScoreNotEnough = 200395,
    ErrItemDisuseLimit = 200396,
    ErrItemDisuseFunc = 200397,
    ErrPhantomRefiningMaxCount = 200398,
    ErrPhantomRefiningDulplicate = 200399,
    ErrPhantomNotNormal = 200400,
    ErrGetSelfPsnOnlineId = 200401,
    ErrGetPsnUserPlayerErr = 200402,
    ErrTowerDefenceHostLeave = 200403,
    ErrRiskHarvestNotInInst = 200404,
    ErrEnterInstTypeErr = 200405,
    ErrInputSettingCount = 200406,
    ErrInputSettingDeviceType = 200407,
    ErrInputSettingActionCount = 200408,
    ErrInputSettingAxisCount = 200409,
    ErrInputCombinationActionCount = 200410,
    ErrInputCombinationAxisCount = 200411,
    ErrInputSettingActionName = 200412,
    ErrInputSettingAxisName = 200413,
    ErrInputCombinationActionName = 200414,
    ErrInputCombinationAxisName = 200415,
    ErrInputActionKeyNameLength = 200416,
    ErrInputActionKeyLength = 200417,
    ErrInputAxisKeyNameLength = 200418,
    ErrInputAxisKeyLength = 200419,
    ErrInputCombinationActionKeyNameLength = 200420,
    ErrInputCombinationActionKeyLength = 200421,
    ErrInputCombinationActionKeyListLength = 200422,
    ErrInputCombinationAxisKeyNameLength = 200423,
    ErrInputCombinationAxisKeyLength = 200424,
    ErrInputCombinationAxisKeyListLength = 200425,
    ErrInputDeviceSubTypeLength = 200426,
    ErrPsnaccountBlocked = 200427,
    ErrRoleNameInvalid = 200428,
    ErrMailBindRewardNotBind = 200429,
    ErrMailBindRewarded = 200430,
    ErrInputSettingNull = 200431,
    ErrMultiInstExchangeCountErr = 200432,
    ErrMultiInstExchangeFirstPass = 200433,
    ErrMultiInstExchangeActivity = 200434,
    ErrMultiInstExchangeFuncNotOpen = 200435,
    ErrMultiInstExchangeTypeErr = 200436,
    ErrMultiInstExchangeLevelTypeErr = 200437,
    ErrRoleSkinLocked = 200438,
    ErrRoleSkinConfig = 200439,
    ErrRoleSkinNotMatch = 200440,
    ErrRoleSkinWeaponNotSuit = 200441,
    ErrSdkLoginResetByPeer = 200442,
    ErrSdkLoginHttpRequestException = 200443,
    ErrSdkLoginTaskTimeout = 200444,
    ErrSdkLoginTaskCanceled = 200445,
    ErrSexChangeCd = 200446,
    ErrSexChangeLogout = 200447,
    ErrSexChangeTrialActive = 200448,
    ErrLobbyListRequestLimit = 200449,
    ErrLobbyQueryPlayerRequestLimit = 200450,
    ErrPlayerBasicInfoRequestLimit = 200451,
    ErrBlockPlayerRequestLimit = 200452,
    ErrPsnPlayerInfoRequestLimit = 200453,
    ErrFishingPosInvalidX = 200454,
    ErrFishingPosInvalidY = 200455,
    ErrFishingPosOverlap = 200456,
    ErrFishingParamLengthCabinLeft = 200457,
    ErrFishingParamLengthCabinRight = 200458,
    ErrFishingCabinNotOpen = 200459,
    ErrFishingCabinNotFound = 200460,
    ErrFishingCountNotMatch = 200461,
    ErrFishingDuplicate = 200462,
    ErrFishingRemoveItemErr = 200463,
    ErrFishingItemNotFound = 200464,
    ErrFishingCountNotMatchRequest = 200465,
    ErrFishingCountNotMatchCabin = 200466,
    ErrFishingHandInNotMatch = 200467,
    ErrFishingQuickSellNoItem = 200468,
    ErrFishingNoQuickSell = 200469,
    ErrFishingQuickSellConfig = 200470,
    ErrFishingQuickSellNotFilled = 200471,
    ErrFishingQuickSellItemErr = 200472,
    ErrFishingNoSellItem = 200473,
    ErrFishingCanNotSell = 200474,
    ErrFishingItemConfigNotFound = 200475,
    ErrFishingSellItemDuplicate = 200476,
    ErrFishingPriceCalFailed = 200477,
    ErrFishingSellCount = 200478,
    ErrFishingPointNotOpen = 200479,
    ErrFishingPointConfig = 200480,
    ErrFishingPointCount = 200481,
    ErrFishingTempCabinMax = 200482,
    ErrFishingPointGenFail = 200483,
    ErrFishingTechLevel = 200484,
    ErrFishingPreNodeLock = 200485,
    ErrFishingTechLock = 200486,
    ErrFishingTechConfig = 200487,
    ErrFishingLevelMax = 200488,
    ErrFishingSkinConfig = 200489,
    ErrFishingPortConfig = 200490,
    ErrFishingEntrustConfig = 200491,
    ErrFishingEntrustNotAccepted = 200492,
    ErrFishingEntrustNotFound = 200493,
    ErrFishingEntrustDestination = 200494,
    ErrFishingEntrustItemNotEnough = 200495,
    ErrFishingEntrustItemTotalNotMatch = 200496,
    ErrFishingEntrustDumplicate = 200497,
    ErrFishingEntrustEmptyItem = 200498,
    ErrFishingEntrustItemNotMatch = 200499,
    ErrFishingEntrustRefreshInitial = 200500,
    ErrFishingEntrustRefreshPrice = 200501,
    ErrFishingEntrustNotAcceptable = 200502,
    ErrFishingNotInBigWorld = 200503,
    ErrFishingMultiMode = 200504,
    ErrFishingNotOwner = 200505,
    ErrFishingSkinLock = 200506,
    ErrFishingPortLock = 200507,
    ErrFishingCageLock = 200508,
    ErrFishingCageConfig = 200509,
    ErrFishingHandInConfig = 200510,
    ErrFishingHandInCountNotMatch = 200511,
    ErrFishingTempPointData = 200512,
    ErrFishingTempPointConfig = 200513,
    ErrFishingBombItem = 200514,
    ErrFishingBombNoPoint = 200515,
    ErrFishingBaitArea = 200516,
    ErrFishingNotSailing = 200517,
    ErrFishingInPort = 200518,
    ErrFishingInShip = 200519,
    ErrFishingIllustratedRewardNotFound = 200520,
    ErrFishingIllustratedRewarded = 200521,
    ErrFishingIllustratedCondition = 200522,
    ErrFishingTechOutputNotFound = 200523,
    ErrFishingShipNotFound = 200524,
    ErrFishingPortTech = 200525,
    ErrFishingRemoveFuncNotOpen = 200526,
    ErrFishingIllustratedFuncNotOpen = 200527,
    ErrFishingSellFuncNotOpen = 200528,
    ErrFishingTechFuncNotOpen = 200529,
    ErrFishingEntrustHandInQuickSell = 200530,
    ErrActivityPreOpenLock = 200531,
    ErrActivityShowLock = 200532,
    ErrShopGoodsVisibleCondition = 200533,
    ErrShopGoodsDisableCondition = 200534,
    ErrFishingEntrustRefreshFail = 200535,
    ErrPlayerDataVersion = 200536,
    ErrBrokenCircuitRejected = 200537,
    ErrRateLimiterRejected = 200538,
    ErrTimeoutRejected = 200539,
    ErrLoginEnvironment = 200540,
    ErrLoginUserEmpty = 200541,
    ErrOldGameNodeLogoutFail = 200542,
    ErrOldGameNodeLogoutOffline = 200543,
    ErrReloginBranchNameNotMatch = 200544,
    ErrReLoginFightDataInConsistent = 200545,
    ErrCreatePlayerData = 200546,
    ErrReLoginPlayerLoggingOut = 200547,
    ErrAccountLoggedInElsewhere = 200548,
    ErrTowerChallengeTeleportLocked = 200551,
    ErrTowerDefenceGroupConfig = 200552,
    ErrTowerDefencePreInstNotPass = 200553,
    ErrTowerDefenceRankCd = 200554,
    ErrFishingFixItemNotEnough = 200549,
    ErrFishingEntrustUpdateItemNotEnough = 200550,
    ErrTowerDefenceGroupActivityC = 200555,
    ErrTowerDefenceGroupActivity = 200556,
    ErrAbyssComNotFound = 200557,
    ErrAbyssInstConfig = 200558,
    ErrAbyssRoomConfig = 200559,
    ErrAbyssEnterCtx = 200560,
    ErrAbyssActivityNotOpen = 200561,
    ErrAbyssRewardConfig = 200562,
    ErrAbyssRewardNotOpen = 200563,
    ErrAbyssRewardClosed = 200564,
    ErrAbyssRewarded = 200565,
    ErrAbyssRewardCondition = 200566,
    ErrAbyssLastActionNotFinish = 200567,
    ErrAbyssNextRoomNotFound = 200568,
    ErrAbyssRoleNotEquip = 200569,
    ErrAbyssEquipRoleNotExist = 200570,
    ErrAbyssEquipRoleDuplicate = 200571,
    ErrAbyssRoomNoBox = 200572,
    ErrAbyssRewardPlayerCount = 200573,
    ErrAbyssRewardBoxEntityIncId = 200574,
    ErrAbyssBoxRewarded = 200575,
    ErrAbyssBoxNotBelong = 200576,
    ErrAbyssBoxConfigNotFound = 200577,
    ErrAbyssBoxDropFailed = 200578,
    ErrAbyssChallengeLocked = 200579,
    ErrAbyssChallengeNoConsume = 200580,
    ErrAbyssChallengeUnlockItem = 200581,
    ErrAbyssLikePlayerNotInScene = 200582,
    ErrAbyssLikeDuplicate = 200583,
    ErrAbyssRoleItemNotFound = 200584,
    ErrAbyssRoleConfig = 200585,
    ErrAbyssRoleLevelConfig = 200586,
    ErrAbyssRoleMaxPluginCount = 200587,
    ErrAbyssRoleLevelMaxPlugin = 200588,
    ErrAbyssPutOnDuplicate = 200589,
    ErrAbyssEquipOldRoleFailed = 200590,
    ErrAbyssSysthesisMaxCount = 200591,
    ErrAbyssSysthesisItemDuplicate = 200592,
    ErrAbyssPluginItemNotFound = 200593,
    ErrAbyssPluginItemLocked = 200594,
    ErrAbyssPluginConfigNotFound = 200595,
    ErrAbyssPluginEquipped = 200596,
    ErrAbyssQuialityConfig = 200597,
    ErrAbyssSysthesisConsumeItemDuplicate = 200598,
    ErrAbyssRankListCd = 200599,
    ErrAbyssRoleUpLevel = 200600,
    ErrAbyssRoleUpItem = 200601,
    ErrAbyssRoleUpNoComsume = 200602,
    ErrAbyssPreChallengeNotPass = 200603,
    ErrAbyssSlotConfig = 200604,
    ErrAbyssSlotNotMatch = 200605,
    ErrAbyssInstUnlockTime = 200606,
    ErrAbyssSettled = 200607,
    ErrFormationTrailGender = 200608,
    ErrLongShanStageLocked = 200609,
    ErrAbyssRewardIdRepeated = 200610,
    ErrAbyssRewardCount = 200611,
    ErrOtherPlayerCondition = 200612,
    ErrAbyssItemSkillBelongTo = 200613,
    ErrLongShanScoreRewardCount = 200614,
    ErrLongShanScoreRewardDuplicate = 200615,
    ErrLongShanScoreRewardConfig = 200616,
    ErrLongShanScoreRewardActivity = 200617,
    ErrLongShanScoreRewarded = 200618,
    ErrLongShanScoreNotEnough = 200619,
    ErrAbyssDuplicatePassive = 200620,
    ErrActivityLifePointChallengeNotFound = 200621,
    ErrActivityLifePointNotOpen = 200622,
    ErrActivityLifePointPreLocked = 200623,
    ErrPhantomSettingRuleDuplicate = 200624,
    ErrPhantomSettingRuleNotFound = 200625,
    ErrPhantomSettingEmpty = 200626,
    ErrPhantomSettingType = 200627,
    ErrPhantomSettingNameLength = 200628,
    ErrPhantomSettingNull = 200629,
    ErrPhantomSettingIndexRange = 200630,
    ErrPhantomRuleIndexRange = 200631,
    ErrPhantomRuleCount = 200632,
    ErrPhantomBathchOperCount = 200633,
    ErrPhantomBathchType = 200634,
    ErrMobileSettingRequestParam = 200635,
    ErrMobileSettingParamDuplicate = 200636,
    ErrCommonUiSettingRequestParam = 200637,
    ErrCommonUiSettingParamDuplicate = 200638,
    ErrCommonUiSettingConfig = 200639,
    ErrActivityNotOpenTip = 200640,
    ErrLineCrossChallengeNotFound = 200641,
    ErrActivitylineCrossNotOpen = 200642,
    ErrActivitylineCrossPreLocked = 200643,
    ErrPhantomSettingFuncNotOpen = 200644,
    ErrTowerSeasonConfig = 200645,
    ErrAccountDeactivation = 200646,
    ErrPlayerAccountDeactivation = 200647,
    ErrPlayerSkinChange = 200648,
    ErrTrapDefenseNotMulti = 200649,
    ErrNotInTrapDefenseInst = 200650,
    ErrBossRushTaskNoConfig = 200651,
    ErrBossRushRequestActivityNotMatch = 200652,
    ErrPhantomBatchPolishCount = 200653,
    ErrPhantomBatchPolishDuplicate = 200654,
    ErrPhantomBatchPolishPropNotMatch = 200655,
    ErrPhantomBatchPolishNoValidItem = 200656,
    ErrLoginEmptyToken = 200657,
    ErrPhantomVicePolishLimit = 200658,
    ErrPhantomVicePolishParam = 200659,
    ErrPhantomVicePolishItemPropLimit = 200660,
    ErrPhantomVicePolishItemLevelLimit = 200661,
    ErrPhantomVicePolishParamDuplicate = 200662,
    ErrPhantomVicePolishPropGen = 200663,
    ErrPhantomVicePolishNoneProp = 200664,
    ErrSkinRewardNotFound = 200665,
    ErrSkinRewardState = 200666,
    ErrTowerForbidRechallenge = 200667,
    ErrInputSettingActoinType = 200668,
    ErrCreateCharacterGender = 200669,
    ErrSkillBranchRoleValid = 200670,
    ErrSkillBranchNotValid = 200671,
    ErrTrailRoleSkillBranchNotValid = 200672,
    ErrRoleDevConfigVersion = 200673,
    ErrRoleDevConfigVersionTime = 200674,
    ErrRoleDevConfigNoUpdate = 200675,
    ErrMotorDevelopRequestParam = 200676,
    ErrMotorDevelopParamDuplicate = 200677,
    ErrMotorDevelopTaskNotFound = 200678,
    ErrMotorDevelopActivityNotOpen = 200679,
    ErrMotorDevelopTaskNotComplete = 200680,
    ErrPhantomVicePolishNoAck = 200681,
    ErrClientStorageSystemCount = 200682,
    ErrClientStorageSystemDuplicate = 200683,
    ErrClientStorageSystem = 200684,
    ErrClientStorageStringLength = 200685,
    ErrClientStorageType = 200686,
    ErrClientStorageCapacity = 200687,
    ErrClientStorageSystemTypeErr = 200688,
    ErrXboxRegionNotFound = 200689,
    ErrNotXboxChannelId = 200690,
    ErrXboxRegionSet = 200691,
    ErrXboxRegionSetException = 200692,
    ErrXboxRegionGetException = 200693,
    ErrXboxRegionGetFailed = 200694,
    ErrQuestBranchPageNotFound = 200695,
    ErrQuestBranchQuestState = 200696,
    ErrQuestBranchComplete = 200697,
    ErrQuestBranchNotComplete = 200698,
    ErrFightFormationNameChangeCd = 200699,
    ErrFightFormationNameLength = 200700,
    ErrRoleSkillLevel = 200701,
    ErrQuestBranchConfig = 200702,
    ErrConditionStateRequestParam = 200703,
    ErrConditionStateRequestModule = 200704,
    ErrPolishPhantomStackKey = 200705,
    ErrSplitPhantomStackKey = 200706,
    ErrConsumePhantomStackKey = 200707,
    ErrBatchPolishPhantomStackKey = 200708,
    ErrPhantomMaxStackCount = 200709,
    ErrReturnPhantomStackKey = 200710,
    ErrRefiningPhantomStackKey = 200711,
    ErrPhantomPkgFull = 200712,
    ErrMapDefault = 300000,
    ErrMapMarkNumLimit = 300001,
    ErrMapNoFogConfig = 300002,
    ErrMapFogAlreadyUnlock = 300003,
    ErrFormationEmpty = 300004,
    ErrFormationUnknown = 300005,
    ErrFormationDead = 300006,
    ErrFormationRoleRepeat = 300007,
    ErrFormationRoleNotActive = 300008,
    ErrFormationRoleIndexOut = 300009,
    ErrFormationRoleCountOut = 300010,
    ErrFightFormationRoleNotExist = 300011,
    ErrFightFormationRoleIdNotMatch = 300012,
    ErrFightFormationRoleCountNotMatch = 300013,
    ErrFightFormationRoleCareerNotMatch = 300014,
    ErrFightFormationRoleElementNotMatch = 300015,
    ErrFightFormationCannotTrial = 300016,
    ErrFightFormationTrialRoleNotMatch = 300017,
    ErrFormationOverSize = 300018,
    ErrSwitchRoleIsDead = 300019,
    ErrUpdateFormationCurRoleIsDead = 300020,
    ErrUpdateFormationRoleIdsIsNull = 300021,
    ErrFormationIdOutOfRange = 300022,
    ErrCanNotCancelCurFormation = 300023,
    ErrCurRoleNotInFormationRoleIds = 300024,
    ErrUpateFormationNotInSingleWorld = 300025,
    ErrSwitchRoleTypeSignleWorld = 300026,
    ErrSwitchRoleTypeMultiWorld = 300027,
    ErrSwitchRoleTypeFbInstance = 300028,
    ErrSwitchRoleTypeUndefine = 300029,
    ErrSingWorldCanNotUpdateFightRoles = 300030,
    ErrUpdateFightRolesIsNull = 300031,
    ErrUpdateFightRolesCurIdNotExist = 300032,
    ErrInStroyCharacterCanNotSwitchRole = 300033,
    ErrSwitchRoleNotInFightRoles = 300034,
    ErrCanNotSwitchRepeat = 300035,
    ErrSwitchRoleEntityIdNotExist = 300036,
    ErrSwitchRoleEntityNotExist = 300037,
    ErrSitchRoleEntityIsDead = 300038,
    ErrorTeamOperaFail = 300039,
    ErrorPlayerAlreadyHaveTeam = 300040,
    ErrorTeamInviteContentInvalid = 300041,
    ErrorPlayerInBanTime = 300042,
    ErrorPlayerInInviteCd = 300043,
    ErrorPlayerAlreadyInTeam = 300044,
    ErrorKickOutPermissionNotEnough = 300045,
    ErrorTeamIsFull = 300046,
    ErrorTeamServiceNotReady = 300047,
    ErrorTeamPlayerJoinRepeat = 300048,
    ErrorPlayerNotInTeam = 300049,
    ErrorInvitePlayerNotExist = 300050,
    ErrorKickPlayerNotInTeam = 300051,
    ErrorDismissPermissionNotEnough = 300052,
    ErrorTeamRoleIdNotActive = 300053,
    ErrorTeamRoleIdRepeat = 300054,
    ErrorJoinOtherWorldOtherNotExist = 300055,
    ErrorJoinOtherWorldOtherNotInScene = 300056,
    ErrorJoinOtherWorldSceneNotExist = 300057,
    ErrorTeamNotExist = 300058,
    ErrRewardCfgNotFound = 300059,
    ErrTeleportIdNotExist = 300060,
    ErrTeleportIdNotActivate = 300061,
    ErrTeleportCreatureIdNotExist = 300062,
    ErrTeleportIdAlreadyActivate = 300063,
    ErrTeleportGmGetPlayerFailed = 300064,
    ErrTeleportGmGetCreatureGenCfgFailed = 300065,
    ErrTgmNotExitst = 300066,
    ErrTgmNotPlayer = 300067,
    ErrTgmNotGenCfg = 300068,
    ErrTgmInsId = 300069,
    ErrTeleportEntityNotExist = 300070,
    ErrTeleportComponentNotExist = 300071,
    ErrTeleportComponentNotMatch = 300072,
    ErrAreaEnterRepeated = 300073,
    ErrAreaIdNotExist = 300074,
    ErrAreaIdNoNeedRecord = 300075,
    ErrPlayerIsNotDead = 300076,
    ErrPlayerCanNotRevive = 300077,
    ErrPlayerReviveCountReachMax = 300078,
    ErrPlayerReviveDelayNotReach = 300079,
    ErrAutoReviveNotRequest = 300080,
    ErrReviveRegionExisted = 300081,
    ErrReviveRegionNotExisted = 300082,
    ErrReviveRegionConfigNotExist = 300083,
    ErrCanNotUseItemRevive = 300084,
    ErrIsMatching = 300085,
    ErrNotInMatcing = 300086,
    ErrMatchPoolNotExist = 300087,
    ErrNotFindMatchResult = 300088,
    ErrConfirmResultRepeat = 300089,
    ErrAlreadyHaveFbTeam = 300090,
    ErrFbTeamNotExist = 300091,
    ErrPlayerNotInFbTeam = 300092,
    ErrHostCanNotReady = 300093,
    ErrChangeReadyRepeat = 300094,
    ErrFbTeamHaveSameRole = 300095,
    ErrReadyStateCanNotChangeRole = 300096,
    ErrChangeSameRole = 300097,
    ErrNotHaveKickPermission = 300098,
    ErrBeKickNotInFbTeam = 300099,
    ErrNotHaveFightPermission = 300100,
    ErrFbTeamNotAllReady = 300101,
    ErrFbInstIdNotExist = 300102,
    ErrFbMatchRoleNotMatch = 300103,
    ErrSingleInstCanNotMatch = 300104,
    ErrWaitOtherEnterSceneForbidMatch = 300105,
    ErrIsEnteringOtherSceneForbidMatch = 300106,
    InstPlayBtObjNotFound = 300107,
    InstPlayNotSuccess = 300108,
    InstPlayAlreadyGetReward = 300109,
    InstPlayExchangeRewardFail = 300110,
    InstPlaySetterRepeat = 300111,
    InstEntranceNotUnlock = 300112,
    InstEntranceNotOpen = 300113,
    EnterInstLevelNotEnough = 300114,
    EnterInstWorldLevelNotEnough = 300115,
    EnterInstQuestNotEnough = 300116,
    ErrForbidEnterInstInMatch = 300117,
    ErrForbidEnterInstInEnteringOtherWorld = 300118,
    ErrForbidEnterInstInWaitingOtherEnterWorld = 300119,
    ErrEnterInstTypeNotMatch = 300120,
    ErrNotHaveGetRewardCount = 300121,
    ErrInMatching = 300122,
    ErrNotInMatching = 300123,
    ErrNotFindValidMatchServer = 300124,
    ErrNotFindMatchServerPrx = 300125,
    ErrNotHaveMatchTeamInfo = 300126,
    ErrAlreadyConfirmMatchResult = 300127,
    ErrMatchTeamNotInReadyState = 300128,
    ErrMatchRoleNotActive = 300129,
    ErrMatchReadyRepeat = 300130,
    ErrMatchPlayerNotReady = 300131,
    ErrMatchNotHostCanNotKick = 300132,
    ErrMatchNotHostCanNotSetMatching = 300133,
    ErrSetMatchFlagRepeat = 300134,
    ErrPlayerNotInMatchTeam = 300135,
    ErrGetMatchPoolFail = 300136,
    ErrPlayerInMatchPool = 300137,
    ErrPlayerNotInMatchPool = 300138,
    ErrPlayerInMatchTeamCanNotCancel = 300139,
    ErrPlayerIsConfirmResult = 300140,
    ErrNotFindMatchTeam = 300141,
    ErrPlayerIsReadyCanNotChangeRole = 300142,
    ErrNotHostCanNotSetMultRoles = 300143,
    ErrCanNotSetRepeatRole = 300144,
    ErrPlayerNotReadyCanNotCancel = 300145,
    ErrRoleRepeatCanNotReady = 300146,
    ErrBeKickNotInMatchTeam = 300147,
    ErrNotHostCanNotKick = 300148,
    ErrNotHostCanNotSetTeamState = 300149,
    ErrTeamMatchingCanNotStartInst = 300150,
    ErrMatchTeamHavePlayerNotReady = 300151,
    ErrNotHostCanNotEnterInst = 300152,
    ErrMatchTeamIsNotEnterInstState = 300153,
    ErrMatchInstIdNotExist = 300154,
    ErrSingleInstanceCanNotMatch = 300155,
    ErrOnlineStateCanNotMatch = 300156,
    ErrTeamHaveSameRoleCanNotBegin = 300157,
    ErrNotJoinChatChannel = 300158,
    ErrChatChannelNotFound = 300159,
    ErrChatChannelTypeNotMatch = 300160,
    ErrChatContentTooLong = 300161,
    ErrFightRoleIsAllDied = 300162,
    ErrLoadingSceneIdNotMatch = 300163,
    ErrLoadingPlayerNotInScene = 300164,
    ErrPlayerIsSceneLoadingCanNotBeKick = 300165,
    ErrTeamPlayerIsSceneLoadingCanNotDissolve = 300166,
    ErrIsSceneLoadingCanNotDissolve = 300167,
    ErrSceneLoadingCanNotEnterInst = 300168,
    ErrActivateResetPointNotEntity = 300169,
    ErrHostIsLoadingScene = 300170,
    ErrHostIsLoadingSceneCanNotApply = 300171,
    ErrIsLoadingSceneCanNotAcceptApply = 300172,
    ErrNotFindHostWorldScene = 300173,
    ErrCanNotRepeatCreateNeedSaveScene = 300174,
    DeadStateCanNotAgreeOherEnter = 300175,
    HostIsDeadStateCanNotEnter = 300176,
    ErrSceneIsLoadingCanNotLeave = 300177,
    ErrInstCanNotReChallenge = 300178,
    ErrInstMemberNotEnoughCanNotReChallenge = 300179,
    ErrInstHavePlayerLeaveCanNotReChallenge = 300180,
    ErrInstHavePlayerNotDeadCanNotReChallenge = 300181,
    ErrInstNotSettleCanNotReChallenge = 300182,
    ErrInstCanNotRepetApplyRechallenge = 300183,
    ErrInstCanNotRepetReceiveRechallenge = 300184,
    ErrInstOwnerCanIniviteRechallenge = 300185,
    ErrInstOwnerCanNotReceiveRechallenge = 300186,
    ErrPlayerIsLogoutCanNotCreateScene = 300187,
    ErrPlayerIsCreatingScene = 300188,
    ErrPlayerCreateSceneFail = 300189,
    ErrBigWorldCanNotReset = 300190,
    ErrMultiGameModeCanNotReset = 300191,
    ErrIsEnterSceneApplyingCanNotDoRepeate = 300192,
    ErrIsQueryLobbyFriendDetailCanNotDoRepeate = 300193,
    ErrIsQueryLobbyPlayerDetailCanNotDoRepeate = 300194,
    ErrPlayerIsLoadingCanNotDoTeleport = 300195,
    ErrPlayerIsTeleportCanNotDoTeleport = 300196,
    ErrTeleportPositionIllegal = 300197,
    ErrPlayerIsLoadingCanNotRevive = 300198,
    ErrPlayerIsTeleportCanNotRevive = 300199,
    ErrPlayerIsInTeleportCanNotBeKick = 300200,
    ErrTeamPlayerIsInTeleportCanNotDissolve = 300201,
    ErrHostIsInTeleportCanNotApply = 300202,
    ErrIsInTeleportCanNotAcceptApply = 300203,
    ErrStrNotIllegal = 400000,
    ErrBasicInfoPhotoUnlocked = 400001,
    ErrBasicInfoFrameUnlocked = 400002,
    ErrCanNotGetSelfBasicInfo = 400003,
    ErrMailNotExist = 400004,
    ErrMailAlreadyRead = 400005,
    ErrNoMailCanGet = 400006,
    ErrMailNoAttachment = 400007,
    ErrMailAttachmentIsGet = 400008,
    ErrMailAttachmentNotGet = 400009,
    ErrMailNotRead = 400010,
    ErrNoMailCanDelete = 400011,
    ErrMailItemBagFull = 400012,
    ErrMailFuncNotOpen = 400013,
    ErrMailOverSize = 400014,
    ErrMailTakeLimit = 400015,
    ErrMailAttachmentItemInvalidCount = 400016,
    ErrMailAttachmentItemNoConf = 400017,
    ErrMailNoConf = 400018,
    ErrShopIdNotExit = 400019,
    ErrShopInfoExist = 400020,
    ErrShopTimeLimit = 400021,
    ErrShopMoneyId = 400022,
    ErrShopNumLimit = 400023,
    ErrShopCondLimit = 400024,
    ErrShopBankNoExit = 400025,
    ErrShopNoShow = 400026,
    ErrShopVersion = 400027,
    ErrShopIlligalParam = 400028,
    ErrDragonPoolConf = 400029,
    ErrFullLevel = 400030,
    ErrItemConf = 400031,
    ErrNotEnoughItem = 400032,
    NotMingSuTi = 400033,
    HadFinishMingSuTi = 400034,
    MingSuCallEntityFail = 400035,
    ErrDragonPoolFuncNotOpen = 400036,
    ErrWorldLevelHadDown = 400037,
    ErrWorldLevelNotDown = 400038,
    ErrWorldLevelMin = 400039,
    ErrWorldLevelCd = 400040,
    ErrIsBlockedPlayer = 400041,
    ErrIsNotBlockedPlayer = 400042,
    ErrBlockListCountMax = 400043,
    ErrYouAreBlocked = 400044,
    ErrAlreadyOnFriendList = 400045,
    ErrNotOnFriendList = 400046,
    ErrAlreadyOnFriendApplyList = 400047,
    ErrFriendApplyNotExists = 400048,
    ErrFriendListCountMax = 400049,
    ErrInitiatorFriendListCountMax = 400050,
    ErrReceiverApplyListCountMax = 400051,
    ErrCanNotFriendApplySendToSelf = 400052,
    ErrFriendApplySended = 400053,
    ErrFriendRemarkLengthLimit = 400054,
    ErrFriendApplyRequestLimit = 400055,
    ErrFriendRequestEmpty = 400056,
    ErrFriendRequestOverSize = 400057,
    ErrPayShopNotExists = 400058,
    ErrPayShopDisabled = 400059,
    ErrPayShopGoodsNotExists = 400060,
    ErrPayShopGoodsDisabled = 400061,
    ErrPayShopGoodsLocked = 400062,
    ErrPayShopGoodsOutSellTime = 400063,
    ErrPayShopGoodsBuyLimit = 400064,
    ErrPayShopDataChanged = 400065,
    ErrPayShopIllegalBuyCount = 400066,
    ErrPayShopIsDirect = 400067,
    ErrPayShopIsNotDirect = 400068,
    ErrPayShopTabDisabled = 400069,
    ErrMonthCardWithoutValidity = 400070,
    ErrMonthCardUpdateConfNotExist = 400071,
    ErrMonthCardDaysMax = 400072,
    ErrMonthCardRewardGot = 400073,
    ErrMonthCardConfNotExist = 400074,
    ErrIsNotSpecialItem = 400075,
    ErrNoEquipSpecialItem = 400076,
    ErrNoValidBattlePass = 400077,
    ErrBattlePassRewardNotFound = 400078,
    ErrBattlePassNotPaid = 400079,
    ErrBattlePassIsPaid = 400080,
    ErrBattlePassRewardLocked = 400081,
    ErrBattlePassRewardTaken = 400082,
    ErrBattlePassCanNotRepeatActive = 400083,
    BattlePassNoRecurringReward = 400084,
    ErrBattlePassIsAdvanced = 400085,
    ErrBattlePassTaskNotFound = 400086,
    ErrBattlePassTaskNotFinished = 400087,
    ErrBattlePassTaskTaken = 400088,
    ErrBattlePassExpIsFull = 400089,
    ErrAdviceNotFound = 400090,
    ErrConjunctionCanNotWord = 400091,
    ErrAdviceTextNotExists = 400092,
    ErrAdviceWordNotExists = 400093,
    ErrAdviceTemplateNotExists = 400094,
    ErrAdviceCellCalcException = 400095,
    ErrIsNotAdviceEntity = 400096,
    ErrAdviceCreateLimit = 400097,
    ErrAdviceContentCanNotEmpty = 400098,
    ErrAdviceEntityNotFount = 400099,
    ErrAdviceVoteLimit = 400100,
    ErrAdviceIsVoteUp = 400101,
    ErrAdviceIsVoteDown = 400102,
    ErrNoAdviceItem = 400103,
    ErrAdviceCreateNotOpen = 400104,
    ErrAdviceCanNotCreateByVisitor = 400105,
    ErrAdviceSetingIsShow = 400106,
    ErrAdviceSetingIsNoShow = 400107,
    ErrAdviceUpMaxValue = 400108,
    ErrAdviceDownMaxValue = 400109,
    ProtoVersionCheckFail = 400110,
    ProtoMd5CheckFail = 400111,
    ConfigVersionCheckFail = 400112,
    ConfigMd5CheckFail = 400113,
    ErrInvalidMonthCardDays = 400114,
    ErrMonthCardExtendedDaysMax = 400115,
    ErrMobileButtonNoCfg = 400116,
    ErrMoneyWrongPayCount = 400117,
    ErrMailTextSenderNotFound = 400118,
    ErrMailTextTitleNotFound = 400119,
    ErrMailTextContentNotFound = 400120,
    ErrAdviceIsNotVoteUp = 400121,
    ErrParkourChallengeNoConf = 400122,
    ErrParkourLocationNoConf = 400123,
    ErrParkourChallengeNotOpen = 400124,
    ErrParkourChallengeNoData = 400125,
    ErrParkourChallengeTaken = 400126,
    ErrParkourChallengeUnderscore = 400127,
    ErrParkourChallengeScoreNoConf = 400128,
    ErrParkourTakeFail = 400129,
    ErrShopIllegalBuyCount = 400130,
    ErrQuestErrTaskId = 500000,
    ErrQuestErrStepId = 500001,
    ErrQuestErrTaskBag = 500002,
    ErrQuestStepStatusNotCanAccept = 500003,
    ErrQuestStepStatusNotCanCommit = 500004,
    ErrQuestStepConf = 500005,
    ErrQuestStepData = 500006,
    ErrQuestCanNotAccept = 500007,
    ErrAreaQuestDelegationBoardRequest = 500008,
    ErrAreaQuestAreaIdErr = 500009,
    ErrAreaQuestExpired = 500010,
    ErrDevoteLevel = 500011,
    ErrDevoteRewardReceived = 500012,
    ErrQuestNotFinish = 500013,
    ErrDevoteId = 500014,
    ErrAreaQuestLimit = 500015,
    ErrQuestNodeNotActive = 500016,
    ErrQuestNotActiveId = 500017,
    ErrQuestNodeNotFound = 500018,
    ErrQuestComNotFound = 500019,
    ErrQuestTraceType = 500020,
    ErrQuestNotProgress = 500021,
    ErrQuestNoCombatState = 500022,
    ErrQuestNodeData = 500023,
    ErrQuestNotChildQuestNode = 500024,
    ErrQuestNotClientSubmit = 500025,
    ErrQuestAccepted = 500026,
    ErrResourceOccupation = 500027,
    ErrRequestOccupationType = 500028,
    ErrNotFoundOccupation = 500029,
    ErrNotOnlineQuestAccept = 500030,
    ErrQuestDestroy = 500031,
    ErrTreeNodeNotFind = 500032,
    ErrTreeNodeNotActive = 500033,
    ErrIsNotChildQuestNode = 500034,
    ErrChildQuestConditionCanNotSubmit = 500035,
    ErrNodeNotFindAction = 500036,
    ErrNodeActionIsFinish = 500037,
    ErrNodeActionGetItemIsNotQuestItem = 500038,
    ErrNodeActionGetItemHasNotFreeSize = 500039,
    ErrInvalidBtType = 500040,
    ErrTimerNotFind = 500041,
    ErrPreCondition = 500042,
    ErrHandIdItemData = 500043,
    ErrTreeNotFailedNode = 500044,
    ErrTreeNotFailConf = 500045,
    ErrTreeNotGiveUpConf = 500046,
    ErrTreeNotRollback = 500047,
    ErrNodeNotFindNpcId = 500048,
    ErrNotRollbackPermission = 500049,
    ErrNotRollbackRepeat = 500050,
    ErrTreeSuspend = 500051,
    ErrPlayerNotInQuestMap = 500052,
    ErrSaveNewNotRollback = 500053,
    ErrUiPlayType = 500054,
    ErrOccupationTime = 500055,
    ErrReleaseTime = 500056,
    ErrActionSetTime = 500057,
    ErrForcedOccupationResource = 500058,
    ErrAddPlayBubble = 500059,
    ErrDisableSwitchOccupation = 500060,
    ErrOpenSystemBoardResultFail = 500061,
    ErrEntityNoInhaledComponent = 500062,
    ErrEntityInhaledStrength = 500063,
    ErrDisableSwitchGender = 500064,
    ErrTapeDefault = 600000,
    ErrTapeInvalidPos = 600001,
    ErrTapeIsNotActiveRole = 600002,
    ErrTapeItemTypeFail = 600003,
    ErrTapeNotExistTapeItem = 600004,
    ErrTapeNotExistTapeConfig = 600005,
    ErrTapeNotExistTapeProps = 600006,
    ErrTapeHasTakeOnTape = 600007,
    ErrTapeHasNotTakeOnTape = 600008,
    ErrTapeNotExistTapeQualityConfig = 600009,
    ErrTapeNotExistLevelUpExpConfig = 600010,
    ErrTapeInvalidLevelUpExpValue = 600011,
    ErrTapeNotExistExpDecayRatioConfig = 600012,
    ErrTapeLevelUpEqualItem = 600013,
    ErrTapeLevelUpRepeatItem = 600014,
    ErrTapeLevelUpInvalidExpItemNum = 600015,
    ErrTapeLevelUpInvalidExpRate = 600016,
    ErrTapeLevelUpInvalidAddExp = 600017,
    ErrTapeLevelUpMaxLevel = 600018,
    ErrTapeLevelUpConsumeItemNotEnough = 600019,
    ErrTapeLevelUpMaterialLock = 600020,
    ErrTapeTransferEqualItem = 600021,
    ErrTapeTransferQualityNotEqual = 600022,
    ErrTapeTransferSuitNotEqual = 600023,
    ErrTapeTransferMaterialLock = 600024,
    ErrTapeTransferMaterialEquipped = 600025,
    ErrTapeNotExistTransferPropNumConfig = 600026,
    ErrTapeTransferPropNumIsMax = 600027,
    ErrTapeTransferRandomSubPropFail = 600028,
    ErrTapeResetTransferHasNotProp = 600029,
    ErrTapeResetTransferMaterialNotEnough = 600030,
    ErrTapeNotExistTapeExpItem = 600031,
    ErrTapeNotExistTapeExpItemConfig = 600032,
    ErrCollectEntityNotExist = 600033,
    ErrCollectInvalidEntityMainType = 600034,
    ErrRunningLevelPlayNotFind = 600035,
    ErrLevelPlayInteractionEntity = 600036,
    ErrLevelPlayNotExistByConfId = 600037,
    ErrLevelPlayNotCreate = 600038,
    ErrLevelPlayRewarded = 600039,
    ErrLevelPlayInteractionType = 600040,
    ErrLevelPlayNotPlayer = 600041,
    ErrLevelPlayNotComplete = 600042,
    ErrLevelPlayRewardFail = 600043,
    ErrLevelPlayNotWaitState = 600044,
    ErrLevelPlayAction = 600045,
    ErrLevelPlayGetRewardLimit = 600046,
    ErrFlowNotExist = 600047,
    ErrFlowHaveNotActionWait = 600048,
    ErrFlowHaveNotTalkWait = 600049,
    ErrFlowHaveNotOptionWait = 600050,
    ErrFlowInvalidOptionId = 600051,
    ErrInteractFlowCanNotPlay = 600052,
    ErrInteractInvalidFlowState = 600053,
    ErrInteractOptionOwnerIsNotFlowOwner = 600054,
    ErrInteractOptionOwnerIsNotActionOwner = 600055,
    ErrActionOwnerIsNotEntity = 600056,
    ErrActionOwnerNotFound = 600057,
    ErrActionSceneNotFound = 600058,
    ErrActionGroupNotFound = 600059,
    ErrFinishClientActionFail = 600060,
    ErrActionHostPlayerNotFound = 600061,
    ErrActionFail = 600062,
    ErrActionPlayerNotFound = 600063,
    ErrInteractMultiGameMode = 600064,
    ErrInteractAddFlowFail = 600065,
    ErrBehaviorTreeOwnerNotFound = 600066,
    ErrBehaviorTreeNotFound = 600067,
    ErrBehaviorTreePending = 600068,
    ErrBehaviorTreeTimerTypeNotFound = 600069,
    ErrBehaviorTreeStopTimerFail = 600070,
    ErrBehaviorTreeTimerCompNotFound = 600071,
    ErrInteractCd = 600072,
    ErrInteractRange = 600073,
    ErrDropPickRange = 600074,
    ErrBtTmpItemContextNotExist = 600075,
    ErrBtTmpItemBtObjNotExist = 600076,
    ErrPlayerBigWorldNotExist = 600077,
    ErrRoleEntityNotExist = 600078,
    ErrAddFlowFail = 600079,
    ErrInteracting = 600080,
    ErrInteractCollectBagFull = 600081,
    ErrBtObjIsNotInstPlay = 600082,
    ErrReviveConfigNotExist = 600083,
    ErrFinishFlowFail = 600084,
    ErrFlowActionFail = 600085,
    ErrGmSubmitChildQuestNodeMaxDepth = 600086,
    ErrGmSubmitChildQuestNodeIsNotProgress = 600087,
    ErrEntityPatrolComponentNotExist = 600088,
    ErrInteractIsNotParticipant = 600089,
    ErrVisionEntityInteractFail = 600090,
    ErrMaxDropTimes = 600091,
    ErrStateEntityMultiHang = 600092,
    ErrPlayerLoading = 600093,
    ErrPlayerTeleporting = 600094,
    ErrInteractBtPending = 600095,
    ErrInteractDead = 600096,
    ErrMultiHangEntity = 600097,
    ErrRenjuCanNotResetWhenComplete = 600098,
    ErrRenjuCanNotMove = 600099,
    ErrEntityNotFound = 600100,
    ErrSceneHostPlayerNotMatch = 600101,
    ErrVehicleComponentNotFound = 600102,
    ErrVehicleSeatNotFound = 600103,
    ErrVehicleGettingOn = 600104,
    ErrPortalEntityNotFound = 600105,
    ErrPortalCompNotFound = 600106,
    ErrPortalTeleportPosNotEqual = 600107,
    ErrPlayerNotInVehicle = 600108,
    ErrVehiclePassengerRoleExist = 600109,
    ErrCreateVehiclePassengerEntityFail = 600110,
    ErrVehiclePassengerNotFound = 600111,
    ErrCanNotMovePlacement = 600112,
    ErrGmSetLimitTeleportDungeon = 600113,
    ErrMaxSetTagIdDepth = 600114,
    ErrBeforeSetStateTagId = 600115,
    ErrSetStateTagIdLock = 600116,
    ErrDangoMonopolyActivityDataNotFound = 600117,
    ErrDangoMonopolyReqTaskRewardMax = 600118,
    ErrDangoMonopolyTaskConfigNotFound = 600119,
    ErrDangoMonopolyTaskNotCompleted = 600120,
    ErrDangoMonopolyTaskRewardHasGet = 600121,
    ErrDangoMonopolyBoardConfigNotFound = 600122,
    ErrDangoMonopolyGridRewardNotGet = 600123,
    ErrDangoMonopolyHasNotDiceItem = 600124,
    ErrDangoMonopolyHasNotGridReward = 600125,
    ErrDangoMonopolyGridConfigNotFound = 600126,
    ErrDangoMonopolyReqBoardRewardMax = 600127,
    ErrDangoMonopolyBoardNotCompleted = 600128,
    ErrDangoMonopolyBoardRewardHasGet = 600129,
    ErrDangoMonopolyActivityConfigNotFound = 600130,
    ErrDangoMonopolyBoardLock = 600131,
    ErrChangeEntityStateActionEntityNotFound = 600132,
    ErrDangoMonopolyDiceNumInvalid = 600133,
    ErrDangoMonopolyBoardCompleted = 600134,
    ErrLevelSequenceFrameEventCompNotFound = 600135,
    ErrLevelSequenceFrameEventDataNotFound = 600136,
    ErrLevelSequenceFrameEventSectionsNotFound = 600137,
    ErrActionCtxInvalid = 600138,
    ErrActionOwnerInvalid = 600139,
    ErrGlobalFixCfgNotFound = 600140,
    ErrActionGetItemIsNotQuestItem = 600141,
    ErrGlobalFixExecuteCountMax = 600142,
    ErrActionGroupCreateTooFrequently = 600143,
    ErrActionGroupParallelTooMuch = 600144,
    ErrCanNotRemoveLastTrialRole = 600145,
    ErrEntityIsNotActivateState = 600146,
    ErrRestoreTrialRoleNotInRegion = 600147,
    ErrRemoveTrialRoleNotExist = 600148,
    ErrRemoveLastTrialRole = 600149,
    ErrSaveNodeDeActiveTrialRoleEmpty = 600150,
    ErrNewHandInItemTypeInvalid = 600151,
    ErrIgameObjectHostIsNotIplayer = 600152,
    ErrSceneWorldNotExist = 700000,
    ErrPlayerNotInScene = 700001,
    ErrDropEntityNotExist = 700002,
    ErrDropComponentNotExist = 700003,
    ErrDropOwnerError = 700004,
    ErrPlayerAlreadyInScene = 700005,
    ErrSceneIdParseError = 700006,
    ErrJoinSceneIdNotExist = 700007,
    ErrSceneInviteFail = 700008,
    ErrSceneInvitePlayerNotExist = 700009,
    ErrSceneInviteTokenInvalid = 700010,
    ErrSceneInviterNotExist = 700011,
    ErrSceneInviteeIdNotMatch = 700012,
    ErrSceneTeamIsFull = 700013,
    ErrScenePlayerIsInTeam = 700014,
    ErrSceneInviteerIsInPlayeInst = 700015,
    ErrBeKickerNotInScene = 700016,
    ErrorCanNotSceneKickSelf = 700017,
    ErrCanNotKickOtherInPlayInst = 700018,
    ErrCanNotKickOtherWhoIsInPlayInst = 700019,
    ErrNoSceneKickPermission = 700020,
    ErrInviterIsInOtherScene = 700021,
    ErrInOtherSceneCanNotInvite = 700022,
    ErrSceneBackSceneFlagError = 700023,
    ErrLeaveSceneIdNotMatch = 700024,
    ErrPrewarTeamAlreadyExist = 700025,
    ErrPrewarTeamNotExist = 700026,
    ErrCreatePrewarTeamPermissionNotEnough = 700027,
    ErrPrewarTeamInvitePermissionNotEnough = 700028,
    ErrPrewarBeInviterNotInScene = 700029,
    ErrPrewarBeInviterInInstance = 700030,
    ErrPlayerAlreadyInPrewarTeam = 700031,
    ErrPlayerNotInPrewarTeam = 700032,
    ErrPlayerSetPrewarReadyStateRepeat = 700033,
    ErrPrewarTeamBeKickerNotInScene = 700034,
    ErrPrewarTeamKickPermissionNotEnough = 700035,
    ErrPrewarIniviteTooFrequently = 700036,
    ErrPrewarJoinInstanceIdNotMatch = 700037,
    ErrPrewarCaptainCanNotLeave = 700038,
    ErrPrewarReadyStateCanNotChangeRole = 700039,
    ErrScenePrewarTeamHavePlayerNotReady = 700040,
    ErrAlreadyInMultiScene = 700041,
    ErrOnlyCaptainCanDissolvePrewarTeam = 700042,
    ErrMultiChangeRoleIndexInvalid = 700043,
    ErrMultiCanNotChangeOtherRole = 700044,
    ErrMultiChangeRoleEntityNorExist = 700045,
    ErrSceneCanNotUseThisFunc = 700046,
    ErrSceneCanNotUseThisItem = 700047,
    ErrSceneFightRoleIdRepeat = 700048,
    ErrShieldAddEntityNotExist = 700049,
    ErrShieldAddShieldIdExisted = 700050,
    ErrShieldChangeEntityNotExist = 700051,
    ErrShieldChangeShieldIdNotExist = 700052,
    ErrShieldRemoveEntityNotExist = 700053,
    ErrShieldRemoveShieldIdNotExist = 700054,
    ErrHardnessModeChangedEntityNotExist = 700055,
    ErrSceneEntityNotExist = 700056,
    ErrSceneEntityNotHavePartData = 700057,
    ErrsceneEntityNotHavePartId = 700058,
    ErrChangeControlRoleRepeat = 700059,
    ErrVisionSkillCallEntityFail = 700060,
    ErrSceneDataLoadError = 700061,
    ErrCreatureDataError = 700062,
    ErrCreatureGenIsExist = 700063,
    ErrCreatureGenIsNotExist = 700064,
    ErrCreatureGenIsControlByOther = 700065,
    ErrCreatureGenNotHaveControlPerm = 700066,
    ErrCreatureReachMaxCount = 700067,
    ErrCreatureConditionNotMatch = 700068,
    ErrCreatureTimeIntervalError = 700069,
    ErrCreatureCfgNotExist = 700070,
    ErrCreatureEntityIsNotValidity = 700071,
    ErrUniqueEntityCanNotCreateTwice = 700072,
    ErrRoleNotHaveVisionSkill = 700073,
    ErrHitGearEntityNotExist = 700074,
    ErrHitGearHaveNotEntityConfig = 700075,
    ErrHitGearHaveNotGearConfig = 700076,
    ErrHitGearHaveNotGameplayConfig = 700077,
    ErrHitGearHaveNotStepConfig = 700078,
    ErrHitGearHaveEntityCommonTag = 700079,
    ErrHitGearEntityFunctionTypeFail = 700080,
    ErrHitGearAcceptStepFail = 700081,
    ErrCreateInstanceNotContainEntrance = 700082,
    ErrCreateInstanceHaveNotEntranceConfig = 700083,
    ErrCreateInstanceEntranceLock = 700084,
    ErrCreateInstanceHaveNotConfig = 700085,
    ErrCreateInstanceEnterCountNotEnough = 700086,
    ErrCreateInstanceConditionNotMatch = 700087,
    ErrEnterCountRequestHaveNotConfig = 700088,
    ErrUnlockInstanceEntranceHaveNotConfig = 700089,
    ErrUnlockInstanceEntranceNotNeedUnlock = 700090,
    ErrUnlockInstanceEntranceUnlocked = 700091,
    ErrUnlockInstanceEntranceCondiitonNotMatch = 700092,
    ErrEnterSceneGameplayRequestHaveNotConfig = 700093,
    ErrEnterSceneGameplayRequestAccepted = 700094,
    ErrEnterSceneGameplayRequestAcceptFail = 700095,
    ErrStoryCharacterCreatFail = 700096,
    ErrStoryCharacterCreatRepeat = 700097,
    ErrStoryCharacterNotExist = 700098,
    ErrCheckGearEntityNotExist = 700099,
    ErrCheckGearType = 700100,
    ErrCheckGearNotEntityConfig = 700101,
    ErrCheckGearActive = 700102,
    ErrCheckGearInactive = 700103,
    ErrTargetGearGroupEntityNotExist = 700104,
    ErrTargetGearGroupConfigNotExist = 700105,
    ErrTargetGearEntityNotExist = 700106,
    ErrTargetGearConfigNotExist = 700107,
    ErrTargetGearStartTypeIsNotHit = 700108,
    ErrTargetGearStartTypeIsNotAction = 700109,
    ErrTargetGearStarted = 700110,
    ErrTargetGearFinished = 700111,
    ErrTargetGearIsNotInCreatedConsole = 700112,
    ErrTargetGearGroupEntityIsNotAllInit = 700113,
    ErrLanternCatNotExit = 700114,
    ErrLanternCatConfNotExit = 700115,
    ErrLanternCatType = 700116,
    ErrLanternActived = 700117,
    ErrLanternTargetNotExit = 700118,
    ErrCaptureFail = 700119,
    ErrDyingFail = 700120,
    ErrThrowDamageEntityNotExit = 700121,
    ErrThrowDamageCalculateId = 700122,
    ErrThrowDamageIdNotExit = 700123,
    ErrThrowDamageRoleIdConf = 700124,
    ErrThrowDamageTypeNotExit = 700125,
    InstIdNotExist = 700126,
    ErrControlObjectEntityNotExist = 700127,
    ErrControlObjectConfigNotExist = 700128,
    ErrControlGroupConfigNotExist = 700129,
    ErrControlObjectLocked = 700130,
    ErrControlGroupLocked = 700131,
    ErrControlCanNotPutTarget = 700132,
    ErrControlTargetOccupied = 700133,
    ErrControlObjectCatching = 700134,
    ErrControlObjectNotCatching = 700135,
    ErrControlObjectOtherCatching = 700136,
    ErrEntityPositionIllegal = 700137,
    ErrTreasureBoxNot = 700138,
    ErrTreasureBoxNotInit = 700139,
    ErrTreasureBoxNotConfig = 700140,
    ErrTreasureBoxHadReward = 700141,
    ErrTreasureBoxNotInteraction = 700142,
    ErrTreasureBoxNotDropId = 700143,
    ErrTreasureBoxDropErr = 700144,
    ErrTreasureBoxNotExist = 700145,
    ErrTreasureBoxInvalidTag = 700146,
    ErrTreasureBoxHadTag = 700147,
    ErrTreasureBoxNotTag = 700148,
    ErrSneakGameNotOpen = 700149,
    ErrSneakFinishRepeat = 700150,
    ErrClientControlDamage = 700151,
    ErrSceneDataSaveFail = 700152,
    NotInFbInstance = 700153,
    GmerrCanNotCreateWorldInst = 700154,
    GmerrPlayerAlreadyInFbInst = 700155,
    GmerrTagetInstanceIsNotMulti = 700156,
    GmerrPlayerNotFound = 700157,
    ErrEntityFlowTooMuch = 700158,
    GmErrIsWalkable = 700159,
    GmErrIsNotWalkable = 700160,
    GmErrNoNavmesh = 700161,
    ErrBigWorldInstIdNotExist = 700162,
    ErrInstIdNotBigWorld = 700163,
    ErrInInstanceNotSwitchBigWorld = 700164,
    ErrAlreadyInThisBigWorld = 700165,
    ErrNoPermissionGetTreasureBox = 700166,
    ErrCreateBigWorldRepeat = 700167,
    DebugErrInstIdNotExist = 700168,
    ErrSceneAiStopped = 700169,
    ErrGlobalEntityConfigNotExist = 700170,
    ErrSceneFixedConfigNotExist = 700171,
    ErrSceneFixedEntityNotFound = 700172,
    ErrSceneGlobalEntityNotFount = 700173,
    ErrEntityNotHaveVarComponent = 700174,
    ErrEntityVarNameNotExist = 700175,
    ErrEntityVarTypeError = 700176,
    ErrEntityConfigNotOffer = 700177,
    ErrConfigTypeNotGloabl = 700178,
    ErrConfigTypeNotSceneFixed = 700179,
    ErrConfigTypeNotCharacter = 700180,
    ErrEntityPosNotOffer = 700181,
    ErrSceneCellPosNotFount = 700182,
    ErrEntityCongigNotInSleep = 700183,
    ErrSummonCfgNotFound = 700184,
    ErrSummonAddEntityFail = 700185,
    ErrSummonMaxCount = 700186,
    ErrSummonMaxGenerations = 700187,
    ErrSummonEntityIdAlreadyExist = 700188,
    ErrSummonerEntityType = 700189,
    ErrEntityStatusIsNotDead = 700190,
    ErrEntityNotHaveAttributeComp = 700191,
    ErrEntityDbData = 700192,
    ErrSceneFixedEntityCreated = 700193,
    ErrInvalidAwakeEntityContext = 700194,
    ErrTriggerComponentNotExist = 700195,
    ErrTriggerComponentMaxCount = 700196,
    ErrNotSelfRole = 700197,
    ErrNoControlRights = 700198,
    ErrEntityHaveNotEntityOwner = 700199,
    ErrEntityOwnerNotMatch = 700200,
    ErrCreateSceneFixedEntitiesEmpty = 700201,
    ErrInteractComponentNotExist = 700202,
    ErrInteractOptionIndexInvalid = 700203,
    ErrOnlineInteractNoPermission = 700204,
    ErrOnlineInteractNotOpen = 700205,
    ErrAwakeEntityNoPermission = 700206,
    ErrCannotUseSkillStatus = 700207,
    ErrInteractOptionGuidInvalid = 700208,
    ErrAddInteractOptionFail = 700209,
    ErrRemoveInteractOptionFail = 700210,
    ErrInteractOptionOwnerNotFound = 700211,
    ErrSummonPlayerId = 700212,
    ErrSummonTemplateCfgNotFound = 700213,
    ErrAttributeComponent = 700214,
    ErrAnimFsmComponent = 700215,
    ErrStateComponent = 700216,
    ErrBattleComponent = 700217,
    ErrPartComponent = 700218,
    ErrAiControlComponent = 700219,
    ErrSummonsComponent = 700220,
    ErrAiBlackboardComponent = 700221,
    ErrSetVarInvalidContext = 700222,
    ErrSetVarInvalidVarRefPb = 700223,
    ErrSetVarGetRightVarDefineFail = 700224,
    ErrSetVarSetLeftVarDefineFail = 700225,
    ErrCalcVarInvalidContext = 700226,
    ErrCalcVarInvalidVarRef = 700227,
    ErrCalcVarGetVarDefineFail = 700228,
    ErrCalcVarInvalidVarType = 700229,
    ErrCalcVarInvalidOp = 700230,
    ErrCalcVarSetResultFail = 700231,
    ErrActionEntityNoExist = 700232,
    ErrActionNoInteractConfig = 700233,
    ErrActionIdNoExist = 700234,
    ErrActionBtObjNoExist = 700235,
    ErrActionNodeNoExist = 700236,
    ErrActionNoChildQuest = 700237,
    ErrActionParams = 700238,
    ErrActionNotEntityContext = 700239,
    ErrActionExecutorNotFind = 700240,
    ErrActionSessionNotFind = 700241,
    ErrActionCreateSessionIdFail = 700242,
    ErrActionPathConvertFail = 700243,
    ErrActionConfigNotFind = 700244,
    ErrActionHaveNotHandler = 700245,
    ErrActionInternalError = 700246,
    ErrActionInvalidIndex = 700247,
    ErrActionIsNotServer = 700248,
    ErrActionRemainActionNotFinish = 700249,
    ErrActionExecutorIsNotBlackbard = 700250,
    ErrContinuityActionNotFinish = 700251,
    ErrActionIsNotContinuity = 700252,
    ErrResetLocationEntityNotExist = 700253,
    ErrEntityPosAbnormalNotExists = 700254,
    ErrGmRemoveEntityNotExists = 700255,
    ErrDrownEntityNotExists = 700256,
    ErrTargetGearNotExists = 700257,
    ErrOutofBattleEntityNotExists = 700258,
    ErrOrderAddBuffEntityNotExists = 700259,
    ErrOrderRemoveBuffEntityNotExists = 700260,
    ErrActivateBuffEntityNotExists = 700261,
    ErrToughCalcExtraRatioChangeEntityNotExists = 700262,
    ErrAdsorbEntityNotExist = 700263,
    ErrAdsorbCondNotMeet = 700264,
    ErrTimelineTrackMultiGameForbid = 700265,
    ErrTimelineTraceEntityNotExists = 700266,
    ErrTimelineTraceComponentNotExists = 700267,
    ErrTimelineTraceGroupIndex = 700268,
    ErrTimelineTraceFinish = 700269,
    ErrTimelineTraceCondition = 700270,
    ErrTimelineTraceTargetEmpty = 700271,
    ErrTimelineTraceControl = 700272,
    ErrTimelineTraceFinishCondition = 700273,
    ErrTimelineTraceNotInControl = 700274,
    ErrForbidEnterInstance = 700275,
    ErrForbitEnterBigWorld = 700276,
    ErrPrefabIncIdExist = 700277,
    ErrPrefabIdExist = 700278,
    ErrPrefabNumberIsZero = 700279,
    ErrPrefabEntityIsExist = 700280,
    ErrPrefabTreasureBox = 700281,
    ErrPrefabActionCreate = 700282,
    ErrPrefabVarNoExist = 700283,
    ErrClientOnlyEntityCantCreate = 700284,
    ErrTimelineTraceActionRun = 700285,
    ErrGmActivateTeleportSceneNotExist = 700286,
    ErrGmCreateInstSceneHasExist = 700287,
    ErrVfxNpcNotExist = 700288,
    ErrVfxNpcIsNotVfxNpc = 700289,
    ErrBlackboardLimit = 700290,
    ErrBlackboardArrayLimit = 700291,
    ErrBlackboardStringLimit = 700292,
    ErrReconnectGwgetGatePlayerFailed = 800000,
    ErrGwreconnectGwinvalidPlayerState = 800001,
    ErrGwreconnectGwverifyTokenFailed = 800002,
    ErrGwreconnectGwbackOnlineAsyncFailed = 800003,
    ErrGwreconnectGwbackOnlineAsyncException = 800004,
    ErrReconnectGwclientLatestSeqNoNotHit = 800005,
    ErrGwreconnectConfirmGetPlayerFailed = 800006,
    ErrAttrChangeHandleInvalidClientAction = 800007,
    ErrThrowDamageReqGetStateComponentFailed = 800008,
    ErrThrowDamageReqEntityIsAlreadyDead = 800009,
    ErrAnimalDieRequestForceSetDieError = 800010,
    ErrCollectEntityForceSetEntityDieError = 800011,
    ErrMonsterBoomForceSetDieError = 800012,
    ErrAttrChangeReqReplaceAttrListFailed = 800013,
    ErrReconnectInvalidOperation = 800014,
    ErrReconnectGwNodeTainted = 800015,
    ErrFavorRoleNotFound = 900000,
    ErrFavorConfNotFound = 900001,
    ErrFavorQuestNotFound = 900002,
    ErrFavorLevelRewardLimit = 900003,
    ErrFavorQuestAcceptLimit = 900004,
    ErrFavorItemLocked = 900005,
    ErrFavorItemHasUnLocked = 900006,
    ErrElevatorEntityNotExit = 900007,
    ErrElevatorConfigNotExit = 900008,
    ErrElevatorLocked = 900009,
    ErrElevatorIsNotReverse = 900010,
    ErrElevatorIsNotForward = 900011,
    ErrElevatorIsNotStart = 900012,
    ErrElevatorIsNotEnd = 900013,
    ErrElevatorFloorError = 900014,
    ErrHostRefuse = 900015,
    ErrHostOffline = 900016,
    ErrHostHasOnline = 900017,
    ErrHostPlayerMax = 900018,
    ErrHostRefuseStrangers = 900019,
    ErrHostForbidJoin = 900020,
    ErrHostTemporarilyForbidJoin = 900021,
    ErrSlaveInBlockList = 900022,
    ErrExceedJoinLevelDiff = 900023,
    ErrHostNotOpenOnlineFunc = 900024,
    ErrHostInOtherPlayer = 900025,
    ErrHostInForbidOnlineQuest = 900026,
    ErrSlaveHasOnline = 900027,
    ErrSlaveInForbidOnlineQuest = 900028,
    ErrSlaveNotOpenOnlineFunc = 900029,
    ErrSlaveApplyRepeated = 900030,
    ErrSlaveTryApplySelf = 900031,
    ErrLobbyTryQuerySelf = 900032,
    ErrSlaveRequestExpired = 900033,
    ErrEnterringOtherScene = 900034,
    ErrWaitingOtherJoin = 900035,
    ErrWaitListFull = 900036,
    ErrAlreayInWaitEnterList = 900037,
    ErrHostNotInBigWorld = 900038,
    ErrPlayerNotInBigWorld = 900039,
    ErrPlayerNotInWaitList = 900040,
    ErrForbidOperaInMatching = 900041,
    ErrLobbyNotFoundPlayer = 900042,
    ErrRoleTrailCannotOnline = 900043,
    ErrHostRoleTrail = 900044,
    ErrInMatchingCanNotJoinOther = 900045,
    ErrInMatchCanNotBeApply = 900046,
    ErrInMatchCanNotAcceptApply = 900047,
    ErrSlaveInFlow = 900048,
    ErrHostInFlow = 900049,
    ErrAchievementNotClinet = 900050,
    ErrTriggerConditionNotMet = 900051,
    ErrNpcTraceNotConf = 900052,
    ErrBuffProducerConfNotFound = 900053,
    ErrBuffProducerHasDone = 900054,
    ErrBuffConsumerConfNotFound = 900055,
    ErrBuffConsumerBuffNotFound = 900056,
    ErrBuffConsumerEntityNotFound = 900057,
    ErrItemPosInvaild = 900058,
    ErrItemIdInvaild = 900059,
    ErrRouletteFuncIdInvaild = 900060,
    ErrStateIsRunning = 900061,
    ErrSceneEntityNotFind = 900062,
    ErrActionPlayersIsEmpty = 900063,
    ErrFireBulletNoLauncher = 900064,
    ErrFireBulletNoTarget = 900065,
    ErrTurntableConfigNotFound = 900066,
    ErrTurntableActivityNotOpen = 900067,
    ErrTurntableActivityIsFinish = 900068,
    ErrTurntableActivityQuestNotFinish = 900069,
    ErrTurntableActivityRoundConfigNotFound = 900070,
    ErrEnrichmentAreaIsEmpty = 900071,
    ErrEnrichmentAreaNotFind = 900072,
    ErrEnrichmentAreaInCd = 900073,
    ErrEnrichmentAreaInFog = 900074,
    ErrEntityWalkingPoint = 900075,
    ErrServerConfigReload = 900076,
    ErrAreaCheckFailed = 900077,
    ErrHostSceneBlockSplitFail = 900078,
    ErrSlaveSceneBlockSplitFail = 900079,
    ErrHookLockBatchCollectMaxCount = 900080,
    ErrHookLockBatchCollectFail = 900081,
    ErrEntityPackIdErr = 900082,
    ErrEntityInQuickHackSkill = 900083,
    ErrEntityQuickHackSkillNoAction = 900084,
    ErrVehicleInPhantomFormation = 900085,
    ErrGiantSystemNotOpen = 900086,
    ErrGiantEcoIsLock = 900087,
    ErrGiantItemInvaild = 900088,
    ErrGiantItemNotEnough = 900089,
    ErrGiantItemNotMatch = 900090,
    ErrGiantMaxLevel = 900091,
    ErrGiantMaxLevelDropLimit = 900092,
    ErrGiantNotMaxLevel = 900093,
    ErrGiantActivityReward = 900094,
    ErrGiantActivityNodeNotActive = 900095,
    ErrLoopTeleportVolumeConfig = 900096,
    ErrLoopTeleportVolumeCondition = 900097,
    ErrBuffItemConfig = 1000000,
    ErrBuffItemNotShare = 1000001,
    ErrBuffItemShareRoleId = 1000002,
    ErrBuffItemRoleIdNotExist = 1000003,
    ErrBuffItemNotEnough = 1000004,
    ErrBuffItemMultiUse = 1000005,
    ErrBuffItemCdLimit = 1000006,
    ErrBuffItemNumZero = 1000007,
    ErrBuffItemNotPlayer = 1000008,
    ErrSceneItemNotExit = 1000009,
    ErrSceneItemType = 1000010,
    ErrSceneItemOperate = 1000011,
    ErrSceneItemState = 1000012,
    ErrStateEntityNoExit = 1000013,
    ErrStateEntityNotTagComp = 1000014,
    ErrStateEntityTypeNotExit = 1000015,
    ErrStateEntityStateNotExit = 1000016,
    ErrStateEntityNotConfig = 1000017,
    ErrStateEntityStateType = 1000018,
    ErrStateEntityStateNoChange = 1000019,
    ErrStateEntitySilent = 1000020,
    ErrStateEntityComplete = 1000021,
    ErrStateEntityLock = 1000022,
    ErrStateEntityNotBorn = 1000023,
    ErrStateNameNoExit = 1000024,
    ErrStateInBorn = 1000025,
    ErrStateCondition = 1000026,
    ErrStateNotOwner = 1000027,
    ErrChangeSelfStateObjNotEntity = 1000028,
    ErrFoundationNotExists = 1000029,
    ErrTeleControlNotExists = 1000030,
    ErrFoundationNotComponent = 1000031,
    ErrFoundationActived = 1000032,
    ErrFoundationUnActived = 1000033,
    ErrFoundationNotMatch = 1000034,
    ErrFoundationNotInRange = 1000035,
    ErrFoundationNotStateId = 1000036,
    ErrGravityGearNotExists = 1000037,
    ErrGravityGearNotConfig = 1000038,
    ErrGravityGearForbidReset = 1000039,
    ErrFollowTrackEntityNoExist = 1000040,
    ErrFollowTrackNotComp = 1000041,
    ErrFollowTrackNotFoundationId = 1000042,
    ErrFollowTrackNotFoundation = 1000043,
    ErrFollowTrackActiveed = 1000044,
    ErrThrowPlayerNotExit = 1000045,
    ErrAnimalEntityNotExist = 1000046,
    ErrNotAnimalEntity = 1000047,
    ErrSneakBtObjNotExist = 1000048,
    ErrSneakNodeIdNotExist = 1000049,
    ErrSneakNotFailedNode = 1000050,
    ErrSneakNotTime = 1000051,
    ErrSneakTime = 1000052,
    ErrInSneak = 1000053,
    ErrNotInSneak = 1000054,
    ErrBeControlledEntityNotExist = 1000055,
    ErrNotBeControlledEntity = 1000056,
    ErrNotBeControlledPlayer = 1000057,
    ErrNotBeControlledNotPlayer = 1000058,
    ErrBeControlledShowEntityNotExist = 1000059,
    ErrNotBeControlledShowEntity = 1000060,
    ErrNotBeControlledShowPlayer = 1000061,
    ErrBeControlledShowNoChange = 1000062,
    ErrGravityGearCondition = 1000063,
    ErrChairEntityNoExist = 1000064,
    ErrChairSitDownErr = 1000065,
    ErrChairEntity = 1000066,
    ErrPlayerAlreadySit = 1000067,
    ErrChairNotStateConfig = 1000068,
    ErrSneakBtObjIncId = 1000069,
    ErrTimelineMove = 1000070,
    ErrBeControlledConfig = 1000071,
    ErrBeControlledThrow = 1000072,
    ErrBeControlledTimeNull = 1000073,
    ErrTriggerEnterActionEffective = 1000074,
    ErrTriggerLeaveActionEffective = 1000075,
    ErrTriggerLastActionStateError = 1000076,
    GuideGroupInfoIsNull = 1100000,
    GuideStateError = 1100001,
    GuideConfigNotFind = 1100002,
    GuideNoEnough = 1100003,
    GuideIsFinish = 1100004,
    GuidePerIsNotFinish = 1100005,
    GuideNoCondition = 1100006,
    GuideNoCurGroup = 1100007,
    GuideIsServerMonitor = 1100008,
    GuideNoPending = 1100009,
    GuideStepRepeat = 1100010,
    GuideGroupNoClient = 1100011,
    GuideGroupDoing = 1100012,
    GuideGroupIsNotRepeat = 1100013,
    GuideTutorialConfigNotFind = 1100014,
    GuideTutorialIsUnlock = 1100015,
    GuideTutorialNotUnlock = 1100016,
    GuideTutorialIsReceive = 1100017,
    GuideTutorialAwardConfigNotFind = 1100018,
    GuideTutorialAwardError = 1100019,
    GuideGroupIdNoMatch = 1100020,
    ErrRequestTypeNotExist = 1100021,
    ErrIllustratedEntryLock = 1100022,
    ErrIllustratedEntryBanUnlock = 1100023,
    ErrRequestTypeMax = 1100024,
    AchievementEntryNotExist = 1100025,
    AchievementEntryNotFinish = 1100026,
    AchievementEntryIsReceive = 1100027,
    AchievementEntryNoConfig = 1100028,
    AchievementEntryNotOpen = 1100029,
    AchievementGroupEntryNotExist = 1100030,
    AchievementGroupEntryNotFinish = 1100031,
    AchievementGroupEntryIsReceive = 1100032,
    AchievementGroupEntryNoConfig = 1100033,
    AchievementGroupEntryNotOpen = 1100034,
    SilentAreaNotConfig = 1100035,
    SilentAreaNotUnlock = 1100036,
    SilentAreaNotFinish = 1100037,
    SilentAreaReceive = 1100038,
    AchievementEntryIsFinish = 1100039,
    AchievementEntryNeedCondition = 1100040,
    AchievementSceneNotFind = 1100041,
    BirthdayIsSetting = 1100042,
    BirthdayInValid = 1100043,
    RoleShowListMaxCount = 1100044,
    RoleShowListHasRepeatId = 1100045,
    RoleShowListHasInValidId = 1100046,
    CardShowListMaxCount = 1100047,
    CardShowListHasRepeatId = 1100048,
    CardShowListHasInValidId = 1100049,
    CardRepeatSet = 1100050,
    CardIsInValidId = 1100051,
    CardIsRead = 1100052,
    RoleShowListEmpty = 1100053,
    SettingNotFind = 1100054,
    RogueRoadConfigNotFind = 1100055,
    RollRogueRoomError = 1100056,
    RollRogueBuffError = 1100057,
    GetRogueRoomIdsError = 1100058,
    GetRoguePortalEntityNotFind = 1100059,
    GetRoguePortalLocationNotFind = 1100060,
    HttpTimeout = 1100061,
    HttpResultUndefine = 1100062,
    ConvGateTimeout = 1100063,
    ProtoKeyTimeout = 1100064,
    LoginReqTimeout = 1100065,
    EnterGameTimeout = 1100066,
    ReReconvReqTimeout = 1100067,
    RecvSeqNoNotHit = 1100068,
    AchievementFuncNotOpen = 1100069,
    RoguelikeInstComponentNotFind = 1100070,
    RogueCurRoomDataIsNull = 1100071,
    LevelPlayComponentNotFind = 1100072,
    OpenLevelPlayFail = 1100073,
    CloseLevelPlayFail = 1100074,
    RogueRoomConfigNotFind = 1100075,
    RogueRoomTypeNotRight = 1100076,
    RogueRoomTypeNotConfig = 1100077,
    RogueRoomSubLevelNotFind = 1100078,
    SelectNextRoomIsValid = 1100079,
    RogueGainPackageFail = 1100080,
    RogueGainListIsNull = 1100081,
    RogueGainIdValid = 1100082,
    QulityListCountNotRight = 1100083,
    RandomResultCountNotRight = 1100084,
    GuaranteeRogueBuffInValid = 1100085,
    RoleBuffPoolNotFind = 1100086,
    NotValidBuff = 1100087,
    NotValidPhantom = 1100088,
    RandomPhantomFail = 1100089,
    NotValidRole = 1100090,
    RandomRoleFail = 1100091,
    RogueRoadNotFind = 1100092,
    ResultCountNotMatch = 1100093,
    InValidRoomCountNotMatch = 1100094,
    GuaranteeRogueRoomInValid = 1100095,
    InstIdNotMatchLevelPlayId = 1100096,
    GetRoomBornPositionFail = 1100097,
    RoguePortalDataNotClean = 1100098,
    RoguePortalRoomDataNotFind = 1100099,
    RogueSelectRoomFail = 1100100,
    RogueProgressDataIsEmpty = 1100101,
    RogueGainTypeIsValid = 1100102,
    RougeNotOpen = 1100103,
    RougeInstIdIsValid = 1100104,
    RogueRoleListCountNotRight = 1100105,
    RogueMainRoleConfigNotFind = 1100106,
    RogueGainDataDictError = 1100107,
    RogueDiscountedBuffConfigNotFind = 1100108,
    RogueDiscountedRoomTypeConfigNotFind = 1100109,
    RogueDiscountedShopConfigNotFind = 1100110,
    RogueDiscountedCalculateFail = 1100111,
    RogueMoneyNotEnough = 1100112,
    RougeShopRefreshTimeEmpyt = 1100113,
    RougeCurRoomNotFinish = 1100114,
    PlayerDataRepairErrorDebug = 1100115,
    PlayerDataRepairError = 1100116,
    CreateCharacterReqTimeout = 1100117,
    SignActivityNotOpen = 1100118,
    SignActivityNoConfig = 1100119,
    SignActivityIndexValid = 1100120,
    SignActivityNoData = 1100121,
    SignActivityStateNotRight = 1100122,
    RogueSeasonDataNull = 1100123,
    RogueSeasonConfigNotFind = 1100124,
    RogueTokenConfigNotFind = 1100125,
    RogueTokenStatusVaild = 1100126,
    RogueSeasonRewardConfigNotFind = 1100127,
    RogueSeasonRewardIsReceive = 1100128,
    RougeSeasonPointNotEnough = 1100129,
    RougeRoomDataError = 1100130,
    RogueGainDataError = 1100131,
    RogueRoleIdsError = 1100132,
    RogueRogueRoomRouteError = 1100133,
    RogueGetCurRoomLevelPlayError = 1100134,
    RogueTalentTreeConfigNotFind = 1100135,
    RogueTalentTreeConditionNotMet = 1100136,
    RogueTalentTreePerNodeLock = 1100137,
    RogueTalentTreeNodeMaxLevel = 1100138,
    RogueTalentTreeConsumeNoEnough = 1100139,
    RogueRoadRandomRoleBuffError = 1100140,
    ActivityFuncNotOpen = 1100141,
    RogueGuideInstNotSupport = 1100142,
    ErrPayReceiptCannotRefundClose = 1100143,
    ErrPayReceiptRefundCloseFail = 1100144,
    PayRefundOverdueBan = 1100145,
    UnknowChannelId = 1100146,
    LoginServerNotFind = 1100147,
    OldGameNodeLogoutFail = 1100148,
    LoginHandleSwitchError = 1100149,
    NoAvailableLoginService = 1100150,
    ServerIsClosing = 1100151,
    AddPlayerRecordFail = 1100152,
    FindGatewayFail = 1100153,
    CommonFightRolesInfoError = 1100154,
    CurRoleEntityNotFind = 1100155,
    ScenePlayerInfoNotFind = 1100156,
    IncrAdviceVoteError = 1100157,
    InsertAdviceError = 1100158,
    UpdateAdviceError = 1100159,
    DeleteAdviceError = 1100160,
    EntityNoInWater = 1100161,
    AttributeComponentNotFind = 1100162,
    TryAddItemDataFail = 1100163,
    ItemConfigTypeNotRight = 1100164,
    ItemLogicNotFind = 1100165,
    RemoveItemLogicNotFind = 1100166,
    AddItemLogicNotFind = 1100167,
    AddItemFail = 1100168,
    UpdatePlayerAremarkFail = 1100169,
    DeleteFriendApplyFail = 1100170,
    DeleteFriendshipFail = 1100171,
    WorldTeamIsNull = 1100172,
    TeamCountNotRight = 1100173,
    AddCalabashExpFail = 1100174,
    SendRequestToSdkFail = 1100175,
    DirtyWordErrorCode = 1100176,
    HarvestActivityNotOpen = 1100177,
    HarvestActivityPointReceived = 1100178,
    HarvestActivityPointNotConfig = 1100179,
    HarvestActivityPointNotEnough = 1100180,
    HarvestActivityLevelNoData = 1100181,
    HarvestActivityLevelReceived = 1100182,
    HarvestActivityLevelNotConfig = 1100183,
    HarvestActivityLevelNotEnough = 1100184,
    HarvestActivityLevelDiffNotConfig = 1100185,
    RoguelikeEventConfigNotFind = 1100186,
    RoguelikeEventIndexError = 1100187,
    RoguelikeInstConfigNotFind = 1100188,
    RoguelikeMainRoleError = 1100189,
    RoguelikeEventIsEmpty = 1100190,
    RoguelikeEventRandomError = 1100191,
    RoguelikeEventRandomEmpty = 1100192,
    PhantomCollectActivityNotOpen = 1100193,
    PhantomCollectActivitynNotConfig = 1100194,
    PhantomCollectActivitynNoData = 1100195,
    PhantomCollectActivityReceived = 1100196,
    HarvestInstIdInValid = 1100197,
    HarvestVarNotExist = 1100198,
    HarvestResultCacheNotExist = 1100199,
    HarvestInstNotOpen = 1100200,
    HarvestActivityLimitDataNotFind = 1100201,
    HarvestDiffConfigNotFind = 1100202,
    HarvestActivityDiffConfigNotFind = 1100203,
    ErrIllustratedConfigNotFind = 1100204,
    CharacterAlreadyCreated = 1100205,
    SdkHelperInternalError = 1100206,
    GameServiceControllerInternalError = 1100207,
    DoGetCacheInfoInternalError = 1100208,
    DoGetCacheInfosInternalError = 1100209,
    DeleteFriendLoadedInternalError = 1100210,
    UpdateFriendRemarkInternalError = 1100211,
    CheckApplyRequestInternalError = 1100212,
    OnReLoginInternalError = 1100213,
    CreateCharacterRequestInternalError = 1100214,
    LoginRequestInternalError = 1100215,
    LoginRequestInternalError2 = 1100216,
    EnterGameRequestInternalError = 1100217,
    ReconnectRequestInternalError = 1100218,
    ReconnectRequestInternalError2 = 1100219,
    SwitchNodeInternalError = 1100220,
    InnerLoginInternalError = 1100221,
    AccessTokenInternalError = 1100222,
    CreateCharacterInternalError = 1100223,
    RogueSeasonNotValid = 1100224,
    RogueCurRoleNotFind = 1100225,
    RogueSeasonNotMatch = 1100226,
    RogueGainLogicNotFind = 1100227,
    RogueBuffConfigNotFind = 1100228,
    RoguePhantomNotConfig = 1100229,
    RogueRoleNotConfig = 1100230,
    RoguePopularSlotConfigNotFind = 1100231,
    RoguePopularCountIsMax = 1100232,
    RoguePopularConfigNotFind = 1100233,
    RogueRoleNotOpen = 1100234,
    RogueGuideInstError = 1100235,
    RogueMainRoleChange = 1100236,
    RogueShopConfigNull = 1100237,
    RogueGainIsSelect = 1100238,
    RogueGainNoRefresh = 1100239,
    RogueRefreshCostNotFind = 1100240,
    RogueNotMaxLayer = 1100241,
    RogueRoomSubLevelNotFind2 = 1100242,
    RoguePopularSlotArgConfigNotFind = 1100243,
    RogueInstSeasonNotMatch = 1100244,
    RogueSeasonTalentTreeNotFind = 1100245,
    RogueGainOptionsNotFind = 1100246,
    RogueGainIsSell = 1100247,
    RogueMiracleCreationConfNotFind = 1100248,
    RogueGainPackageError = 1100249,
    RogueTrialRoleIdsCountNotRight = 1100250,
    RogueVarNotExist = 1100251,
    RougeWhiteCatConfigNotFind = 1100252,
    RougeWhiteCatNotOpen = 1100253,
    RougeWhiteCatLimitedTime = 1100254,
    RougeWhiteCatRewardLock = 1100255,
    RougeWhiteCatRewardIsReceive = 1100256,
    RougeWhiteCatRewardIndexErr = 1100257,
    RougeWhiteCatInstIndexErr = 1100258,
    RougeWhiteCatInstLock = 1100259,
    RougeWhiteCatBossRewardIndexErr = 1100260,
    RougeWhiteCatBossRewardLock = 1100261,
    RougeWhiteCatBossRewardIsReceive = 1100262,
    RougeWhiteCatLevelPlayIndexErr = 1100263,
    RougeWhiteCatLevelPlayLock = 1100264,
    RougeWhiteCatLevelPlayIsReceive = 1100265,
    ResourceVersionTooLow = 1100266,
    RogueLimitTimeRewardConfigNotFind = 1100267,
    RogueWhiteCatLimitedTimeOut = 1100268,
    RougeWhiteCatLimitedRewardLock = 1100269,
    RougeWhiteCatLimitedRewardIsReceive = 1100270,
    RougeWhiteCatBlackFlowerNoCount = 1100271,
    RogueInstCountNotRight = 1100272,
    RogueInstFightFormationNotConfig = 1100273,
    RogueTrialRoleNotValid = 1100274,
    RogueRoleNotValid = 1100275,
    ErrorBlackFlowerEntityNotRight = 1100276,
    ErrorBlackFlowerStatus = 1100277,
    ErrorBlackFlowerCanNotReward = 1100278,
    ErrorBlackFlowerRewardFail = 1100279,
    ErrorPhantomUnlockError = 1100280,
    ErrorPhantomSwitchError = 1100281,
    ActivityConfigNotFind = 1100282,
    ActivityNotOpen = 1100283,
    DirectTrainActivityConfigNotFind = 1100284,
    ActivityTypeNotFind = 1100285,
    SetGlobalVarFail = 1100286,
    ErrMultigame = 1100287,
    RogueWeeklyCycleNoFind = 1100288,
    RogueWeeklyCycleIdNotMatch = 1100289,
    RogueWeeklyCycleInstIdNotMatch = 1100290,
    RogueWeeklyCycleSexNotMatch = 1100291,
    RogueWeeklyCycleActivityIdNotMatch = 1100292,
    RogueWeeklyCycleAwardNotFind = 1100293,
    RogueWeeklyCycleAwardStateNotMatch = 1100294,
    RogueWeeklyInstResultFail = 1100295,
    RogueWeeklyGoldNoEnough = 1100296,
    HasRogueProgressCanNotChangeSex = 1100297,
    RogueWeeklyWorldLevelNotMatch = 1100298,
    ResourceVersionIsTooLowWithTips = 1100299,
    RogueResInstIdNotMatch = 1100300,
    RogueResInstGridConfigNotFind = 1100301,
    RogueResInBossInst = 1100302,
    RogueResCurGridIsNull = 1100303,
    RogueResPathCountFail = 1100304,
    RogueResPathRepeat = 1100305,
    RogueResPathGridNoNear = 1100306,
    RogueResPathGridNoValid = 1100307,
    RogueResPathGridNoVision = 1100308,
    RogueResPathGridBlock = 1100309,
    RogueResPlayerBanMove = 1100310,
    RogueResHasOp = 1100311,
    RogueResTrialRoleNotFind = 1100312,
    RogueResTrialRoleNoValid = 1100313,
    RogueResThemeConfNotFind = 1100314,
    RogueResCrossInstDataIsNull = 1100315,
    RogueResOpEmpty = 1100316,
    RogueResOpNotMatch = 1100317,
    RogueResCollectionConfNotFind = 1100318,
    RogueResCollectionStateErr = 1100319,
    RogueResEndingAwardConfNotFind = 1100320,
    RogueResEndingAwardIsReceived = 1100321,
    RogueResEndingAwardNotFinish = 1100322,
    RogueResTalentConfNotFind = 1100323,
    RogueResTaskConfNotFind = 1100324,
    RogueResTaskDatanotFind = 1100325,
    RogueResTaskAwardIsReceived = 1100326,
    RogueResTaskAwardNotFinish = 1100327,
    RogueResEffectConfNoFind = 1100328,
    RogueResOptionNoRestCount = 1100329,
    RogueResOptionCantMulti = 1100330,
    RogueResOptionRepeat = 1100331,
    RogueResOptionCantGiveUp = 1100332,
    RogueResOptionCantRefresh = 1100333,
    RogueResOptionIndexNoValid = 1100334,
    RogueResOptionTypeError = 1100335,
    RogueResOptionIsSelect = 1100336,
    RogueResCollectionIndexMax = 1100337,
    RogueResCollectionDropConfigNoFind = 1100338,
    RogueResNoBaseGainLogic = 1100339,
    RogueResGridAwardNoConf = 1100340,
    RogueResBornPositionNoConf = 1100341,
    RogueResEffectExecFail = 1100342,
    RogueResOptionFinish = 1100343,
    RogueResOpTypeErr = 1100344,
    RogueResGridEventNoConf = 1100345,
    RogueResGridEventStepNoConf = 1100346,
    RogueResStepOptionNoValid = 1100347,
    RogueResGridEventNoData = 1100348,
    RogueResSelectIndexNoValid = 1100349,
    RogueResGotoNextRoomNoData = 1100350,
    RogueResSelectOpNoData = 1100351,
    RogueResBranchTaskNoConf = 1100352,
    RogueResBranchTaskNoData = 1100353,
    RogueResBranchTaskIsReceive = 1100354,
    RogueResBranchTaskNoFinish = 1100355,
    RogueResTokenShopPriceErr = 1100356,
    RogueResTaskTypeErr = 1100357,
    RogueRandomNoPoolName = 1100358,
    RogueRandomNoHit = 1100359,
    RogueRandomNoValidResult = 1100360,
    RogueResFormationIdNoValid = 1100361,
    RogueResFormationRoleCountErr = 1100362,
    RogueResRandomNoShopGoods = 1100363,
    RogueResRandomShopGoodsNoEnough = 1100364,
    RogueResFormationRoleNoValid = 1100365,
    RogueResFormationNoData = 1100366,
    RogueResGainRoleNoFind = 1100367,
    RogueResGainRoleLvNoConf = 1100368,
    RogueResGainRoleLvNoLv = 1100369,
    RogueResGainRoleNoBuffPool = 1100370,
    RogueResGainRoleNoMaxlv = 1100371,
    RogueResGainRoleRollEffectNoValid = 1100372,
    RogueResGainRoleRollRetNoMatch = 1100373,
    RogueResGainRoleShopNoConf = 1100374,
    RogueResGainRoleRollEmpty = 1100375,
    RogueResGainRoleBondNoValid = 1100376,
    RogueResRoleBuffGuaranteeFail = 1100377,
    RogueResRoleBuffNoMore = 1100378,
    RogueResRoleBuffRandomFail = 1100379,
    RogueRoleShopRollFail = 1100380,
    RogueResGridRangeTrigger = 1100381,
    RogueResCurGridTrigger = 1100382,
    RogueResEventTypeErr = 1100383,
    RogueResEventOpNoData = 1100384,
    RogueResGridNoConf = 1100385,
    RogueResEffectExceFail = 1100386,
    RogueResNoInBattle = 1100387,
    RogueResStepCondLock = 1100388,
    RogueResGridEventOpNotFind = 1100389,
    RogueNoLastInstData = 1100390,
    RogueNoRollBuffBondLinkOp = 1100391,
    RogueTokenCantRepeat = 1100392,
    RogueFailCountMax = 1100393,
    RogueResRoleMaxLv = 1100394,
    RogueResPerInstNoPass = 1100395,
    RogueResCreateInstCacheErr = 1100396,
    LordGymRepeatChallenge = 1100397,
    NewbieCarnivalNoLicense = 1100398,
    NewbieCarnivalParamNoConf = 1100399,
    NewbieCarnivalNoRoleId = 1100400,
    NewbieCarnivalNoData = 1100401,
    NewbieCarnivalTaskNoConf = 1100402,
    NewbieCarnivalTaskTaken = 1100403,
    NewbieCarnivalTaskRunning = 1100404,
    ErrShopFixeNotExist = 1100405,
    ErrShopBankNoExist = 1100406,
    ErrShopBankNoMatch = 1100407,
    ErrPayShopBuyCountOverFlow = 1100408,
    ErrShopBuyCountOverFlow = 1100409,
    EnergyOverFlow = 1100410,
    EnergyNotEnough = 1100411,
    StoreEnergyNotEnough = 1100412,
    ErrPayShopBuyLimitCondition = 1100413,
    ErrPayShopEchoItemOver2 = 1100414,
    ErrPayShopEchoItemOver3 = 1100415,
    ErrPayShopEchoItemOver4 = 1100416,
    CreateReceiptCoreParamError = 1100417,
    ReceiptsDealCloseCoreParamError = 1100418,
    ReceiptsRefundCloseCoreParamError = 1100419,
    CreateReceiptCoreException = 1100420,
    ReceiptsDealCloseCoreException = 1100421,
    ReceiptsRefundCloseCoreFail = 1100422,
    ReceiptsRefundCloseCoreException = 1100423,
    ErrPayConfigClientCantBuy = 1100424,
    RecyclePersonalGiftNoData = 1100425,
    ErrPersonalGiftBuyLimit = 1100426,
    CreateReceiptNoConf = 1100427,
    CreateReceiptFail = 1100428,
    ReceiptDealCloseNoData = 1100429,
    ReceiptDealClosePlayerIdUnMatch = 1100430,
    ReceiptCannotDealClose = 1100431,
    ReceiptDealCloseFail = 1100432,
    ReceiptRefundCloseNoData = 1100433,
    ReceiptRefundClosePlayerIdUnMatch = 1100434,
    ReceiptCannotRefundClose = 1100435,
    ReceiptRefundCloseFail = 1100436,
    ReceiptRefundNoData = 1100437,
    CreateReceiptParamError = 1100438,
    CreateReceiptException = 1100439,
    ReceiptsDealCloseParamError = 1100440,
    ReceiptsDealCloseException = 1100441,
    ReceiptRefundCloseParamError = 1100442,
    ReceiptRefundCloseException = 1100443,
    ReceiptRefundParamError = 1100444,
    UpdateRogueWeeklyArtifactsFail = 1100445,
    RogueWeeklyArtifactsEmpty = 1100446,
    RogueWeeklyArtifactsIndexErr = 1100447,
    RogueResSkipBattleLvNoEnough = 1100448,
    RogueResCanNotSkipBattle = 1100449,
    RogueResRandomGridNoConf = 1100450,
    StrangerHostCount = 1100451,
    SurvivorsNotInitOp = 1100452,
    SurvivorsNotSelectOp = 1100453,
    SurvivorsNoRoleAndWeapon = 1100454,
    SurvivorsRandomEmpty = 1100455,
    SurvivorsOptionEmpty = 1100456,
    SurvivorsSelectEmpty = 1100457,
    SurvivorsRandomGuaranteedEmpty = 1100458,
    SurvivorsLevelNoConf = 1100459,
    SurvivorsWeaponPoolEmpty = 1100460,
    SurvivorsWeaponEmpty = 1100461,
    SurvivorsGoldNoEnough = 1100462,
    SurvivorsWeaponMax = 1100463,
    SurvivorsNoSupportGiveUp = 1100464,
    SurvivorsNoRestSelectCount = 1100465,
    SurvivorsNoSupportMultiSelect = 1100466,
    SurvivorsRepeatSelect = 1100467,
    SurvivorsComponentNoFind = 1100468,
    SurvivorsOpNotFind = 1100469,
    SurvivorsSelectErr = 1100470,
    SurvivorsNoVar = 1100471,
    SurvivorsInstIdErr = 1100472,
    SurvivorsInstLock = 1100473,
    SurvivorsInstNotTime = 1100474,
    SurvivorsInstNoEndlessMode = 1100475,
    SurvivorsNeedContinue = 1100476,
    SurvivorsLevelCantUseRole = 1100477,
    SurvivorsLevelCantUseWeapon = 1100478,
    SurvivorsRoleNoMatch = 1100479,
    SurvivorsStepErr1 = 1100480,
    SurvivorsStepErr2 = 1100481,
    SurvivorsStepErr3 = 1100482,
    SurvivorsCantBuy = 1100483,
    SurvivorsRoleLvNoConf = 1100484,
    SurvivorsWeaponLvNoConf = 1100485,
    SurvivorsRoleNoFind = 1100486,
    SurvivorsWaponNoFind = 1100487,
    SurvivorsItemNoFind = 1100488,
    SurvivorsRoleLvMax = 1100489,
    SurvivorsWeaponLvMax = 1100490,
    SurvivorsRefreshOpFail = 1100491,
    SurvivorsWeightEmpty = 1100492,
    SurvivorsActivityNoData = 1100493,
    SurvivorsRefreshCostFail = 1100494,
    SurvivorsNoLastInstData = 1100495,
    SurvivorsActivityNoConf = 1100496,
    SurvivorsTalentNoConf = 1100497,
    SurvivorsTaskCountMax = 1100498,
    SurvivorsTaskNoConf = 1100499,
    SurvivorsTaskCantReward = 1100500,
    SurvivorsTaskRepeat = 1100501,
    SurvivorsTaskEmpty = 1100502,
    SurvivorsAllLock = 1100503,
    RogueResNoGlobalConf = 1100504,
    SurvivorsNeedPassNormalMode = 1100506,
    SurvivorsOptionIsSelect = 1100507,
    SurvivorsTalentTreeConditionNotMet = 1100508,
    SurvivorsTalentTreePerNodeLock = 1100509,
    SurvivorsTalentTreeNodeMaxLevel = 1100510,
    SurvivorsTalentTreeConsumeNoEnough = 1100511,
    ShortMessageNoConf = 1100512,
    ShortMessageOptionNoConf = 1100513,
    ShortMessageNoData = 1100514,
    ShortMessageNoData2 = 1100515,
    ShortMessageIsRead = 1100516,
    ShortMessageNoMatch = 1100517,
    ShortMessageNoReply = 1100518,
    ShortMessageIsReply = 1100519,
    ShortMessageNoReward = 1100520,
    ShortMessageIsReward = 1100521,
    ShortMessageNoFinish = 1100522,
    ErrorPhantomCollectMax = 1100523,
    ErrorPhantomCollectRepeated = 1100524,
    ErrorPhantomCollectLock = 1100525,
    ErrorNoSceneComp = 1100526,
    ErrorNewTowerIsInChallenge = 1100527,
    ErrorNewTowerIsNoChallenge = 1100528,
    ErrorNewTowerLevelIdNotMatch = 1100529,
    ErrorNewTowerLevelNoConf = 1100530,
    ErrorNewTowerCycleNoConf = 1100531,
    ErrorNewTowerLevelNoData = 1100532,
    ErrorNewTowerNoTeamCache = 1100533,
    ErrorNewTowerNoIndexTeamCache = 1100534,
    ErrorNewTowerScoreRewardMax = 1100535,
    ErrorNewTowerScoreNoConf = 1100536,
    ErrorNewTowerScoreRepeat = 1100537,
    ErrorNewTowerScoreEmpty = 1100538,
    ErrorNewTowerCycleNoInTime = 1100539,
    ErrorNewTowerCurCycleNoConf = 1100540,
    ErrorNewTowerCycleNoLevel = 1100541,
    ErrorNewTowerTeamIndexErr = 1100542,
    ErrorNewTowerTeamMax = 1100543,
    ErrorNewTowerBuffMax = 1100544,
    ErrorNewTowerBuffRepeat = 1100545,
    ErrorNewTowerRoleRepeat = 1100546,
    ErrorNewTowerRoleEmpty = 1100547,
    ErrorNewTowerRoleNoMatch = 1100548,
    ErrorNewTowerRoleMax = 1100549,
    ErrorNewTowerBuffNoMatch = 1100550,
    ErrorNewTowerRoleNoEnergy = 1100551,
    ErrorNewTowerWaveNoInfo = 1100552,
    ErrorNewTowerWaveFinish = 1100553,
    ErrorShortMessageNoOpen = 1100554,
    ErrorShortMessageOptionMax = 1100555,
    ErrorShortMessageBubbleNoData = 1100556,
    ErrorShortMessageBubbleNoConf = 1100557,
    ErrorShortMessageChatBgNoData = 1100558,
    ErrorShortMessageChatBgNoConf = 1100559,
    ErrorNewTowerInstStateErr = 1100560,
    ErrorNewTowerRoleWeaponPhantom = 1100561,
    ErrorNewTowerNoTeamData = 1100562,
    ErrorNewTowerNoCacheSet = 1100563,
    ErrorNewTowerRoleWeaponNoMatch = 1100564,
    ErrorNewTowerRolePhantomNoMatch = 1100565,
    ShortMessageOptionNoConf2 = 1100566,
    ShortMessageOptionNoConf3 = 1100567,
    ErrorNewTowerEntityNoFind = 1100568,
    ErrorNewTowerSeasonAwardNoConf = 1100569,
    ErrorNewTowerSeasonNoConf = 1100570,
    ErrorNewTowerSeasonNotOpen = 1100571,
    ErrorNewTowerSeasonAwardRepeat = 1100572,
    ErrorNewTowerSeasonAwardEmpty = 1100573,
    NewTowerNoLastCycleIdReview = 1100574,
    NewTowerNoActivityData = 1100575,
    NewTowerNoHistoryData = 1100576,
    ErrorNewTowerSeasonAwardTimeout = 1100577,
    RogueInstConfNoFind = 1100578,
    RoguePhantomLock = 1100579,
    RogueRoleLock = 1100580,
    RogueHotEntryLock = 1100581,
    KurotatoNoActivityData = 1100582,
    KurotatoNoActivityConf = 1100583,
    KurotatoTaskLenghMax = 1100584,
    KurotatoTaskNoConf = 1100585,
    KurotatoTaskNoFinish = 1100586,
    KurotatoTaskIsAward = 1100587,
    KurotatoTaskRepeat = 1100588,
    KurotatoTaskZero = 1100589,
    KurotatoCanNotSaveRole = 1100590,
    KurotatoIsSaveRole = 1100591,
    KurotatoIsInInst = 1100592,
    KurotatoNoLevelData = 1100593,
    KurotatoNoRoleData = 1100594,
    KurotatoNoRecord = 1100595,
    KurotatoHasRecord = 1100596,
    KurotatoRoleNoRecord = 1100597,
    KurotatoRecordLevelNotMatch = 1100598,
    KurotatoRoleNotMatch = 1100599,
    HasRoguelikeProgressCanNotChangeSex = 1100600,
    WeeklyFrameNoData = 1100601,
    WeeklyFrameScoreAwardMax = 1100602,
    WeeklyFrameNotInTime = 1100603,
    WeeklyFrameNoConf = 1100604,
    WeeklyFrameScoreAwardNoConf = 1100605,
    WeeklyFrameScoreAwardGroupNotMatch = 1100606,
    WeeklyFrameScoreAwardNotFinish = 1100607,
    WeeklyFrameScoreAwardTaken = 1100608,
    WeeklyFrameScoreAwardRepeat = 1100609,
    WeeklyFrameScoreAwardZero = 1100610,
    KurotatoItemLock = 1100611,
    KurotatoWeaponLock = 1100612,
    KurotatoLevelNotOpen = 1100613,
    HasTempRogueArchive = 1100614,
    NoTempRogueArchive = 1100615,
    RogueArchiveSlotErr = 1100616,
    NoRogueTowerTrial = 1100617,
    RogueTowerTrialInstNoConf = 1100618,
    RogueTowerTrialNoConf = 1100619,
    RogueTowerTrialInstNoData = 1100620,
    RogueTowerTrialInstInChallenge = 1100621,
    RogueTowerTrialInstBossErr = 1100622,
    NoRogueArchive = 1100623,
    RogueArchiveInvalid = 1100624,
    KurotatoStepErr1 = 1100625,
    KurotatoStepErr2 = 1100626,
    ErrorNewTowerAwardCantGet = 1100627,
    KurotatoShopProductIsBought = 1100628,
    KurotatoShopProductAllLock = 1100629,
    ErrorShortMessageChatThemeNoData = 1100630,
    ErrorShortMessageChatThemeNoConf = 1100631,
    ErrorShortMessageChatCharmNoData = 1100632,
    ErrorShortMessageChatCharmNoConf = 1100633,
    RogueResLimitedRoles = 1100505,
    RoleTowerNoActivityData = 1100634,
    RoleTowerNoActivityConf = 1100635,
    RoleTowerTaskLengthMax = 1100636,
    RoleTowerTaskNoConf = 1100637,
    RoleTowerTaskNoFinish = 1100638,
    RoleTowerTaskIsAward = 1100639,
    RoleTowerTaskRepeat = 1100640,
    RoleTowerTaskZero = 1100641,
    RoleTowerNotInLimitTime = 1100642,
    RoleTowerContextEmpty = 1100643,
    RoleTowerMultiModeNotSupported = 1100644,
    RoleTowerLevelConfigNotExist = 1100645,
    RoleTowerLevelNotOpen = 1100646,
    RoleTowerNoLevelData = 1100647,
    RoleTowerNoRecord = 1100648,
    RoleTowerRecordLevelNotMatch = 1100649,
    RoleTowerRoleCountInvalid = 1100650,
    RoleTowerRoleNotMatch = 1100651,
    RoleTowerRoleRepeat = 1100652,
    RoleTowerRoleEnergyNotEnough = 1100653,
    RoleTowerStageConfigNotExist = 1100654,
    RoleTowerTokenConfigNotExist = 1100655,
    RoleTowerTokenActivityNotMatch = 1100656,
    RoleTowerConsumeItemNotInProgress = 1100666,
    RoleTowerConsumeItemNotUsable = 1100667,
    RoleTowerConsumeItemNotEnough = 1100668,
    RoleTowerRankRequestInCd = 1100669,
    ErrChatNotFriendNorOnline = 1200000,
    ErrChatContentFilterFailed = 1200001,
    ErrChatLockState = 1200002,
    ErrChatEmojiNotValid = 1200003,
    ErrChatSendTooFast = 1200004,
    ErrChatMuteNotValidId = 1200005,
    ErrBanChatDefault = 1200006,
    ErrRoleQuestFuncNotOpen = 1200007,
    ErrRoleQuestMaxCount = 1200008,
    ErrRoleQuestUnlockPointNotEnough = 1200009,
    ErrDailyQuestNotFoundArea = 1200010,
    ErrDailyQuestNotFoundInfluence = 1200011,
    ErrDailyQuestRewardAlreadyGet = 1200012,
    ErrDailyQuestDataError = 1200013,
    ErrDailyQuestCantGetReward = 1200014,
    ErrEntityBuffProducerStateError = 1200015,
    ErrEntityBuffProducerNotFound = 1200016,
    ErrVoiceRemainChangeRoleNotInFormation = 1200017,
    ErrVoiceRemainChangeRoleNotAlive = 1200018,
    ErrApplyEffectFail = 1300000,
    ErrOutofBattleTargetNotMonster = 1300001,
    ErrMonsterBoomEntityNotExists = 1300002,
    ErrMonsterBoomNotMonster = 1300003,
    ErrMonsterBoomIsDead = 1300004,
    ErrAnimationStateSpecialFuncException = 1300005,
    ErrPayConfigNotFound = 1400000,
    ErrPayCreateReceiptFail = 1400001,
    ErrPayReceiptNotFound = 1400002,
    ErrPayReceiptPlayerIdUnMatch = 1400003,
    ErrPayReceiptCannotDealClose = 1400004,
    ErrPayReceiptDealCloseFail = 1400005,
    ErrPayNotEnable = 1400006,
    ErrPayDataChanged = 1400007,
    ErrPayUpdateReceiptFail = 1400008,
    ErrGachaConfigNotFound = 1400009,
    ErrGachaRuleGroupConfigNotFound = 1400010,
    ErrGachaRulesNotFound = 1400011,
    ErrGachaTypeKnowns = 1400012,
    ErrGachaDailyTimesLimit = 1400013,
    ErrGachaTotalTimesLimit = 1400014,
    ErrGachaDailyTotalTimesLimit = 1400015,
    ErrGachaIsNotOpen = 1400016,
    ErrGachaIsNotInOpenTime = 1400017,
    ErrGachaFuncIsNotOpen = 1400018,
    ErrItemExchageConfigNotFound = 1400019,
    ErrItemExchageDailyTimesLimit = 1400020,
    ErrItemExchangeTotalTimesLimit = 1400021,
    ErrGachaLimitNotFound = 1400022,
    ErrGachaLimitsEmpty = 1400023,
    ErrTextServerTimeout = 1400024,
    ErrTextServerResFail = 1400025,
    ErrTextServerResEmpty = 1400026,
    ErrTextServerResException = 1400027,
    ErrItemExchageParamError = 1400028,
    ErrBattlePassFuncIsNotOpen = 1400029,
    ErrPayShopFuncIsNotOpen = 1400030,
    ErrGachaPoolConfigNotFound = 1400031,
    ErrGachaPoolIsNotOpen = 1400032,
    ErrGachaPoolIsNotInOpenTime = 1400033,
    ErrGachaPoolLimitNotFound = 1400034,
    ErrGachaPoolNotBelongToGacha = 1400035,
    ErrGachaUsePoolIdNotSet = 1400036,
    ErrGachaTimesNonsupport = 1400037,
    ErrGachaFrontRuleGroupNotFinish = 1400038,
    ErrGachaRuleGroupFinish = 1400039,
    ErrPayGiftBuyLimit = 1400040,
    ErrPayGiftTypeUnknown = 1400041,
    ErrPayGiftNotInSellTime = 1400042,
    ErrBattlePassBuyLevelLimit = 1400043,
    ErrBattlePassBuyLevelError = 1400044,
    ErrJsFileNotFound = 1400045,
    ErrPayReceiptIsRefunded = 1400046,
    ErrPayReceiptIsNotPay = 1400047,
    ErrPayReceiptRefundFail = 1400048,
    ErrPayGiftLocked = 1400049,
    ErrPayGiftBuyConditionLimit = 1400050,
    ErrMapMarkConfigIdNotExist = 1400051,
    ErrTreasureSlotMarkNotExist = 1400052,
    ErrTreasureBoxMarkNotExist = 1400053,
    ErrTreasureSlotMarkExist = 1400054,
    ErrMapMarkTypeNotCustom = 1400055,
    ErrPayShopIsNotOpen = 1400056,
    ErrConsumptiveActivityNotOpen = 1400057,
    ErrConsumptiveTaskNotFound = 1400058,
    ErrConsumptiveTaskNotFinish = 1400059,
    ErrConsumptiveTaskRewarded = 1400060,
    ErrConsumptiveActivityNotFound = 1400061,
    ErrPayGiftPersonalNoPermission = 1400062,
    ErrPayGiftPersonalPermissionExpired = 1400063,
    ErrPayGiftNotPersonal = 1400064,
    ErrPayGiftPersonalPermissionExist = 1400065,
    ErrCouponId = 1400066,
    ErrCouponConfigNotFound = 1400067,
    ErrCouponNotEnough = 1400068,
    ErrCouponTargetItemNotFound = 1400069,
    ErrCouponTargetItemCount = 1400070,
    ErrGachaTypeNotPersonal = 1400071,
    ErrGachaPersonalPermissionExist = 1400072,
    ErrGachaPersonalNoPermission = 1400073,
    ErrGachaPersonalPermissionExpired = 1400074,
    ErrPayGiftVersionConfigNotFound = 1400075,
    ErrPayGiftSelfDefineVersionError = 1400076,
    ErrInfoDisplayId = 1500000,
    ErrItemAlreadyInCd = 1500001,
    ErrCantFinAdventureConfig = 1500002,
    ErrAdventureRewardReceived = 1500003,
    ErrAdventureTaskCache = 1500004,
    ErrAdventureState = 1500005,
    ErrAdventureRewardOrder = 1500006,
    ErrAdventureChapterState = 1500007,
    ErrCantDetectRepeat = 1500008,
    ErrNotInCurrentFollowList = 1500009,
    ErrCantDetectOtherDetectionType = 1500010,
    ErrNotSelectCurrentDetectionId = 1500011,
    ErrDetectionConfigNotFound = 1500012,
    ErrDetectionListCantBeEmpty = 1500013,
    ErrCantFindAnyDetectionTarget = 1500014,
    ErrCantFindTurntableComponentEntity = 1500015,
    ErrHaveNoTurntableControlComponent = 1500016,
    ErrCantFindLevitationMagnetComponentEntity = 1500017,
    ErrHaveNoLevitaionMagnetComponent = 1500018,
    ErrCantFindBoardEntity = 1500019,
    ErrCantFindPlacementEntity = 1500020,
    ErrHaveNoPlacementComponent = 1500021,
    ErrCantFindBoardEntityComponent = 1500022,
    ErrBoardHaveNoAnyPlacement = 1500023,
    ErrBoardNotActiveAllGrid = 1500024,
    ErrNeedBeControlledBefore = 1500025,
    ErrPlaceFailOfAlreadyOnBoard = 1500026,
    ErrInvalidBoardPosition = 1500027,
    ErrNeedRemoveControlRelation = 1500028,
    ErrNotOccupyOnBoard = 1500029,
    ErrHaveNoFillRule = 1500030,
    ErrGridPosAlreadyOccupied = 1500031,
    ErrHaveNoJigsawFoundationConfig = 1500032,
    ErrInvalidGridPos = 1500033,
    ErrGridPosAlreadyActive = 1500034,
    ErrCantPlaceItemOnBoard = 1500035,
    ErrNeedJigsawFoundationComponentWhenBeControlled = 1500036,
    ErrHaveNoBoardComponentConfig = 1500037,
    ErrNeedJigsawItemComponentWhenBeControlled = 1500038,
    ErrCantFindOriginBoardEntity = 1500039,
    ErrDistanceNotInRangeBetweenEntity = 1500040,
    ErrJigsawFoundationIsAlreadySilent = 1500041,
    ErrCrystalEntityNotFound = 1500042,
    ErrGachaBoardEntityNotFound = 1500043,
    ErrNotCrystalEntity = 1500044,
    ErrNotGachaFoundationEntity = 1500045,
    ErrGachaHoleIsFull = 1500046,
    ErrJigsawItemSilent = 1500047,
    ErrThrowDamageConfigNotExists = 1500048,
    ErrThrowDamageComponetNotExists = 1500049,
    ErrEggNotMatchEggFoundation = 1500050,
    ProgressBarEntityNotFound = 1500051,
    NotProgressBarEntity = 1500052,
    ProgressBarIsSilent = 1500053,
    ScenePlayerInfoNotFound = 1500054,
    PlayerNotInAnyScene = 1500055,
    TeleportNotInValidDistance = 1500056,
    AddMapMarkInfoLackOfTeleportParam = 1500057,
    TemporaryTeleportNotExists = 1500058,
    ErrNotHostPlayer = 1500059,
    ErrMarkIdNotExists = 1500060,
    ErrCantUpdateTemporaryTeleportMarkInfo = 1500061,
    NotHostCantAddTemporaty = 1500062,
    TemporaryTeleportPosIsNotWalkable = 1500063,
    BadTemporaryTeleportConfig = 1500064,
    HaveNoTemporaryTeleportComponent = 1500065,
    ErrCantDetectAtInvalidPoint = 1500066,
    ErrNotDetectionTreasureBoxBefore = 1500067,
    ErrJigsawFoundationIsCompleteCantModifyGridState = 1500068,
    GridIsActiveCantSwitchState = 1500069,
    GridIsOccupiedCantSwitchState = 1500070,
    ErrHaveNoBaseInfoComponent = 1500071,
    ErrHaveNoParentEntity = 1500072,
    NotRelationEntity = 1500073,
    ErrLevelPlayNotRunning = 1500074,
    ErrStateCantChangeWhenLifeCycleDestroy = 1500075,
    ErrRangeEntityIdNotFoundWhenForbidTempTeleport = 1500076,
    TemporaryTeleportIsForbidden = 1500077,
    ErrGravityGearIsComplete = 1500078,
    ErrInvalidRoleWhenUpdatePassiveSkill = 1500079,
    ErrInvalidRolePassiveSkillId = 1500080,
    ErrPassiveSkillNotAddBuff = 1500081,
    ErrPassiveSkillCantSpecifyBuff = 1500082,
    ErrPassiveSkillAddBuffFail = 1500083,
    ErrPassiveSkillAddBulletFail = 1500084,
    ErrBuffCreatePassiveSkillFail = 1500085,
    ErrInvalidPreContext = 1500086,
    ErrBadPassiveSkillId = 1500087,
    ErrPassiveSkillComponentNotFound = 1500088,
    ErrRepeatePassiveSkill = 1500089,
    ErrBadPassiveSkillTriggerType = 1500090,
    ErrAddPassiveSkillFailOfEntityNotFound = 1500091,
    ErrEntityNotClientControlWhenAddPassiveSkill = 1500092,
    ErrEntityNotClientControlWhenRemovePassiveSkill = 1500093,
    ErrPassiveSkillNotFoundWhenRemovePassiveSkill = 1500094,
    ErrRepeatedBattleContext = 1500095,
    ErrPassiveSkillAddSkillFail = 1500096,
    ErrCombatSendPackAbnormal = 1500097,
    ErrContextCheckFail = 1500098,
    ErrFsmComponentNotFound = 1500099,
    ErrFsmCreateContextFail = 1500100,
    ErrFsmStateBehaviorPreMessageCantBeZero = 1500101,
    ErrFsmBehaviorCheckBattleContextFail = 1500102,
    ErrFsmPlayMontageLackPreMessage = 1500103,
    ErrFsmPlayMontageCheckContextFail = 1500104,
    ErrFsmPlayMontageConfigCheckFail = 1500105,
    ErrSkillFlowNotExist = 1500106,
    ErrGetReportDataOverLimit = 1500107,
    ErrGetReportDataTooFast = 1500108,
    ErrNotInAnyScene = 1500109,
    ErrAceLogDataNotFound = 1500110,
    ErrAceLogDataRepeatReport = 1500111,
    ErrAceInvalidLogId = 1500112,
    ErrAceSceneGlobalObjNotFound = 1500113,
    ErrAceBadParam = 1500114,
    ErrS2CconfirmIdNotExists = 1500115,
    ErrPassiveSkillConfigNotFound = 1500116,
    ErrNotBehaviorController = 1500117,
    ErrPlayMontageFail = 1500118,
    ErrFightDataInConsistent = 1500119,
    ErrNotInAoiSight = 1500120,
    ErrPassiveSkillNotOwner = 1500121,
    ErrReportStartFirstly = 1500122,
    ErrBattleEntityNotFound = 1500123,
    ErrBattleCampNotDefined = 1500124,
    ErrOtherInternalError = 1500125,
    ErrHaventBattleComponent = 1500126,
    ErrLackCombinePartInfoParam = 1500127,
    ErrCombinerEntityNotExists = 1500128,
    ErrTargetEntityNotExists = 1500129,
    ErrTargetPartNotExists = 1500130,
    ErrCombineComponentNotExists = 1500131,
    ErrAlreadyCombineToOtherEntity = 1500132,
    ErrLackCombinerOffsetPos = 1500133,
    ErrLackCombinerOffsetRotate = 1500134,
    ErrCombineEntityNotFound = 1500135,
    ErrDissolveCheckBattleContextFail = 1500136,
    ErrRepeatedRole = 1500137,
    ErrNotStateMachineBehavior = 1500138,
    ErrDiscardMsgWhenChangeSceneMultiMode = 1500140,
    ErrMayOccurDbAbnormal = 1500139,
    ErrEntityLivingStatusNotifyCheckFsmPlayMontageOfFsmGroupConfigNotExists = 1500141,
    ErrEntityLivingStatusNotifyCheckFsmPlayMontageOfConfigNotExists = 1500142,
    ErrEntityLivingStatusNotifyCheckFsmPlayMontageFail = 1500143,
    ErrFsmActionCheckFsmPlayMontageFail = 1500144,
    ErrFsmActionCheckSkillFail = 1500145,
    ErrFsmActionCheckBufflFail = 1500146,
    ErrFsmPlayMontageCheckAnParamError = 1500147,
    ErrFsmPlayMontageCheckAnMontageConfigNotFound = 1500148,
    ErrFsmPlayMontageCheckAnMontageFail = 1500149,
    ErrMontageConfigNotFound = 1500150,
    ErrAnconfigNotFound = 1500151,
    ErrBulletConfigNotFound = 1500152,
    ErrFsmVersion = 1500153,
    ErrUpdateFightRoleRepeated = 1500154,
    TrialRoleEntityOverLimit = 1500155,
    ErrHaveNoRoleInfos = 1500156,
    ErrTeamHaveNoAnyRoles = 1500157,
    ErrSkillMontageNotNormalMontage = 1500158,
    ErrMontageContextConfigNotFound = 1500159,
    ErrMontageContextCheckAnFail = 1500160,
    ErrLivingStatusContextCheckPlayMontageContextFail = 1500161,
    ErrContextHaveNoFsmGroupConfig = 1500162,
    ErrLivingStatusContextFsmGroupConfigNotFound = 1500163,
    ErrFsmActionCheckPlayEntityMontageFail = 1500164,
    ErrMontageContext1CheckPlayEntityMontageFail = 1500165,
    ErrSkillGahaveNoModifyCommonEnemyPro = 1500166,
    ErrNotSummonEntity = 1500167,
    ErrSkillGahaveNoTagId = 1500168,
    ErrCombatSkillGahandleGetEntityFailed = 1600000,
    ErrCombatMaterialHandleGetEntityFailed = 1600001,
    ErrCombatParticleHandleGetEntityFailed = 1600002,
    ErrCombatPartLifeChangeEntityNotExisted = 1600003,
    ErrCombatCreateBulletTargetNotExisted = 1600004,
    ErrCombatDeleteBulletTargetNotExisted = 1600005,
    ErrCombatDeleteBulletGetEntityFailed = 1600006,
    ErrCombatBulletTargetNoExist = 1600007,
    ErrPartEntityNotExisted = 1600008,
    ErrNoAiControlRights = 1600009,
    ErrAiHateComponent = 1600010,
    ErrSummonerPlayerControl = 1600011,
    ErrConfDamageNotFound = 1600012,
    ErrProcessDamageFailed = 1600013,
    ErrInjuryFreeLandingTag = 1600014,
    ErrNotFindActiveGameplayEffect = 1600015,
    NotClientControlBuff = 1600016,
    ErrBuffNoEffectConf = 1600017,
    ErrBuffCannotCreateBullet = 1600018,
    ErrBuffCannotCreateBuff = 1600019,
    ErrNoBuffConf = 1600020,
    ErrStoppedAi = 1600021,
    ErrEntityIsNotAlive = 1600022,
    ErrSummonCannotSwitchAiControl = 1600023,
    ErrAiControlNotChange = 1600024,
    ErrPlayerCannotControlEntity = 1600025,
    ErrNotFoundBuffEffect = 1600026,
    ErrBuffEffectAuthority = 1600027,
    ErrConcomitantDestroy = 1600028,
    ErrPlayerFollowersComponent = 1600029,
    ErrBuffComponentNotExist = 1600030,
    ErrOrderApplyBuffFailed = 1600031,
    ErrPlayerBuff = 1600032,
    ErrFindPathNoEndPos = 1600033,
    ErrNotGetCurRole = 1600034,
    ErrFindPathFailed = 1600035,
    ErrEntityFsmMachineNotExist = 1700000,
    ErrEntityFsmStateIncorrect = 1700001,
    ErrIsNotAiControler = 1700002,
    ErrIentityFsmCantTransit = 1700003,
    ErrIentityFsmTransitCondition = 1700004,
    ErrIentityFsmTransitToState = 1700005,
    ErrIentityFsmConfirmNotExist = 1700006,
    ErrIentityFsmConfirmNotWait = 1700007,
    ErrItest = 1700008,
    ErrItest1 = 1700009,
    ErrItest2 = 1700010,
    ErrIentityFsmCondCantPass = 1700011,
    ErrIentityFsmActionParamType = 1700012,
    ErrIentityFsmActionParam = 1700013,
    ErrIentityFsmActionExecuted = 1700014,
    ErrIentityFsmActionNotMatchState = 1700015,
    ErrSkillNotExecuting = 1700016,
    ErrExecuteSkillNotMatch = 1700017,
    ErrBlueprintPinNotSupport = 1700018,
    ErrBlueprintPinNotMontage = 1700019,
    ErrBlueprintPinMontageIndex = 1700020,
    ErrConfSkillNotExist = 1700021,
    ErrSkillGanotExist = 1700022,
    ErrSkillGahaveNoBuff = 1700023,
    ErrSkillGahaveNoBuffId = 1700024,
    ErrSkillGahaveNoBullet = 1700025,
    ErrSkillGahaveNoBulletId = 1700026,
    ErrMontageNotMatchSkill = 1700027,
    ErrMontageIndexError = 1700028,
    ErrVisiionSkillNotEquip = 1700029,
    ErrSkillCd = 1700030,
    ErrHaveNoBattleContext = 1700031,
    ErrContextFsmActionOnce = 1700032,
    ErrPlayMontageButNoSkill = 1700033,
    ErrMontageNotExist = 1700034,
    ErrMontageNotContainBuff = 1700035,
    ErrMontageNotContainBullet = 1700036,
    ErrMontageCantBring = 1700037,
    ErrSkillInfoParamError = 1700038,
    ErrNoWorldTeam = 1800000,
    ErrWorldTeamNoMember = 1800001,
    ErrNoHostIs = 1800002,
    ErrNoInstId = 1800003,
    ErrNoTeamInfo = 1800004,
    ErrHostNoTeamInfo = 1800005,
    ErrHasInMatchTeam = 1800006,
    ErrNotInMatchTeam = 1800007,
    ErrHostIsParam = 1800008,
    ErrMatchModeIparam = 1800009,
    ErrMatchTeamFull = 1800010,
    ErrLocalTeamCanNotOpt = 1800011,
    ErrHostInLocalTeam = 1800012,
    ErrNoMatchNodeId = 1800013,
    ErrPlayerSceneIsNull = 1800014,
    ErrPlayerSceneRolesNull = 1800015,
    ErrInvalidMatchState = 1800016,
    ErrRepeatedMatchState = 1800017,
    ErrApplyrPlayerInMatchNotEnterMatchTeam = 1800018,
    ErrOtherVersionLowNoOperate = 1800019,
    ErrPlayerVersionLowNeedUpdate = 1800020,
    ErrMultiGameModeNoWorldLevelDown = 1800021,
    ErrMultiGameModeNoWorldLevelRegain = 1800022,
    ErrOtherHasOnline = 1800023,
    SwitchRoleNotInCurrentFormation = 1800024,
    ErrNoChangeRoles = 1800025,
    ErrExploreSkillPullGiantMultiGame = 1800026,
    ErrExploreSkillPullGiantNotExist = 1800027,
    ErrHttpRpcParam = 1800028,
    ErrPlayerNotInGameNode = 1800029,
    ErrApplyJoinPlayerCurRoleIsDead = 1800030,
    ErrPlayerCurRoleIsDeadNoJoin = 1800031,
    ErrPlayerCurRoleIsDead = 1800032,
    ErrSwitchMultiverse = 1800033,
    ErrSwitchNode = 1800034,
    ErrMatchConfirmPlayerDead = 1800035,
    ErrCheckPublicResourceVersionLower = 1800036,
    ErrCheckPublicResourceVersionHigher = 1800037,
    ErrCheckPublicResourceClientVersionErr = 1800038,
    ErrCheckPublicResourceServerVersionErr = 1800039,
    ErrCheckPublicResourceClientVersionParamErr = 1800040,
    ErrCheckClientVersionNeedUpdate = 1800041,
    ErrBranchNameNotMatch = 1800042,
    ErrMatchRpcAlready = 1800043,
    ErrOtherPlayerEnterHost = 1800044,
    ErrPlayerEnterHost = 1800045,
    ErrLevelPlayChangeSprotModeInMutile = 1800046,
    ErrMatchingNotInvite = 1800047,
    ErrEnableFunctionFb = 1800048,
    ErrMatchSelectTrialRole = 1800049,
    ErrNoFindLastBigScene = 1800050,
    ErrExploreSkillCustomMultiGame = 1800051,
    ErrExploreSkillCustomNotExist = 1800052,
    ErrMatchInviteMemberDead = 1800053,
    ErrMatchAcceptInviteMemberDead = 1800054,
    ErrNoFishBoat = 1800055,
    ErrTemplateNotExists = 1800056,
    ErrDisableSubLevels = 1800058,
    ErrSubLevelsClientNoPermission = 1800059,
    ErrExploreSkillCustomNoActions = 1800057,
    ErrMatchClientVersion = 1800060,
    ErrCurNodeIsTainted = 1800061,
    ErrTeamMateNodeIsTainted = 1800062,
    ErrMapFuctionNotOpen = 1800063,
    ErrNotSceneOwner = 1800064,
    ErrLevelPlayUiListIsNull = 1800065,
    ErrOpenLevelPlayNotClientUi = 1800066,
    ErrOpenLevelPlayFailed = 1800067,
    ErrBlackSwordChallengeId = 1800068,
    ErrCalabashLevelRequest = 1900000,
    ErrCalabashLevelRewardDone = 1900001,
    ErrCalabashLevelConfig = 1900002,
    ErrSkillTreeActiveConsume = 1900003,
    ErrLoadFriendData = 1900004,
    ErrNoLoadPrivateChatData = 1900005,
    ErrNotInWolrd = 1900006,
    ErrNotInGround = 1900007,
    ErrInFighting = 1900008,
    ErrNotHaveCountryAccess = 1900009,
    ErrSkillIsEffect = 1900010,
    ErrNoSoundBox = 1900011,
    ErrConsumeNotEnough = 1900012,
    ErrExploreSkillCountLimit = 1900013,
    ErrLegalAreaNoTreasureBox = 1900014,
    ErrTreasureBoxAllActive = 1900015,
    ExploreProgressNoCountry = 1900016,
    ExploreProgressNoScoreCfg = 1900017,
    ExploreProgressLackProgress = 1900018,
    ExploreProgressRewardDone = 1900019,
    ExploreProgressNoArea = 1900020,
    ExploreToolNotConfirm = 1900021,
    ExploreToolNotOpen = 1900022,
    ErrTreasureBoxPlaceFail = 1900023,
    ErrTreasureBoxData = 1900024,
    ErrPayShopBuyCondition = 1900025,
    ErrGatherActivityData = 1900026,
    ErrGatherTaskNoFinish = 1900027,
    ErrHadGatherReward = 1900028,
    ErrHadGetSharedReward = 1900029,
    ErrSharedPlat = 1900030,
    ErrTowerTargetComplete = 1900031,
    ErrTowerGuideRewardHad = 1900032,
    ErrTowerGuideNoOpen = 1900033,
    ErrTowerGuideConfig = 1900034,
    ErrNewBieCourseConfig = 1900035,
    ErrNewBieCourseRewardHad = 1900036,
    ErrNewBieCourseLevel = 1900037,
    ErrDetectionTargetSilence = 1900038,
    ErrRoleTrialNotInit = 1900039,
    ErrRoleTrialNoFinish = 1900040,
    ErrRoleTrialReward = 1900041,
    ErrRoleTrialRewardDone = 1900042,
    ErrAdventureTaskReward = 1900043,
    ErrChapterReward = 1900044,
    ErrSilentFirstPassStatus = 1900045,
    ErrSilentFirstPassReward = 1900046,
    ErrPayShopEchoRole = 1900047,
    ErrPayShopEchoItemOver = 1900048,
    ErrDailyAdventureActivityInit = 1900049,
    ErrDailyAdventureActivityPtEnough = 1900050,
    ErrDailyAdventureActivityRewardDone = 1900051,
    ErrDailyAdventureActivityRewardTake = 1900052,
    ErrDailyAdventureActivityTaskDone = 1900053,
    ErrRoleTrialTimeOut = 1900054,
    ErrFriendRemarkNull = 1900055,
    ErrTrackMoonRoleUnLock = 1900056,
    ErrTrackMoonTrigger = 1900057,
    ErrTrackMoonBuildingUnLock = 1900058,
    ErrTrackMoonBuildingCurve = 1900059,
    ErrTrackMoonBuildingLock = 1900060,
    ErrMoonEntrustCfg = 1900061,
    ErrMoonRoleCfg = 1900062,
    ErrMoonRoleTrailCurve = 1900063,
    ErrMoonBuildingCfg = 1900064,
    ErrMoonItemConsume = 1900065,
    ErrDirtyWordDeserialize = 1900066,
    ErrMoonTargetNoFinish = 1900067,
    ErrMoonActivityReward = 1900068,
    ErrMoonActivityOpen = 1900069,
    ErrCircumDoReward = 1900070,
    ErrRetrunRewardCfg = 1900071,
    ErrRetrunRewardLevel = 1900072,
    ErrRetrunHaddone = 1900073,
    ErrSignRewardCfg = 1900074,
    ErrCircumSignHadRwd = 1900075,
    ErrCircumNoSign = 1900076,
    ErrScoreRewardCfg = 1900077,
    ErrCircumScoreHadRwd = 1900078,
    ErrCircumScoreLack = 1900079,
    ErrCircumTaskNoFinish = 1900080,
    LoginServiceInvalidToken = 1900081,
    LoginFusing = 1900082,
    LoginRateLimiterRejected = 1900083,
    LoginTimeoutRejected = 1900084,
    AccountInputErr = 1900085,
    DevInvalidLoginType = 1900086,
    GarinvalidLoginType = 1900087,
    GardevInvalidLoginType = 1900088,
    SdkserverTimeOut = 1900089,
    ReconnectInvalidOperation = 1900090,
    PbMessageAppVersionNotMatch = 1900091,
    ErrPluginReconnectIpWhiteList = 1900092,
    NotInUserIdWhiteListWithChannel = 1900093,
    ErrPluginReconnectChannelWhiteList = 1900094,
    PluginPlayerLoggingIn = 1900095,
    LoginFusing2 = 1900096,
    SoundBoxExploreFull = 1900097,
    ErrMoonEntrustNoData = 1900098,
    ErrMoonMoneyNotEnough = 1900099,
    ErrCircumFluenceTimeIn = 1900100,
    DragonPoolRewardWayErr = 1900101,
    DragonPoolNoHandIn = 1900102,
    WeaponSkinNoEquiped = 1900103,
    WeaponSkinDataErr = 1900104,
    WeaponSkinUnLockErr = 1900105,
    WeaponSkinEquipDone = 1900106,
    WeaponSkinTypeErr = 1900107,
    FriendOfflineMsgErr = 1900108,
    FindSpringSignConfigErr = 1900109,
    SpringSignDataErr = 1900110,
    SpringSignRewardDone = 1900111,
    SpringSignRewardGetErr = 1900112,
    SpringSignNoOpen = 1900113,
    SpringSignNoTask = 1900114,
    SpringSignInviteNum = 1900115,
    SpringSignRolePool = 1900116,
    SpringSignDrawPoolNull = 1900117,
    FarmGoldActivityNotOpen = 1900118,
    FarmGoldActivityPointReceived = 1900119,
    FarmGoldActivityPointNotConfig = 1900120,
    FarmGoldActivityPointNotEnough = 1900121,
    FarmGoldActivityLevelNoData = 1900122,
    FarmGoldActivityLevelReceived = 1900123,
    FarmGoldActivityLevelNotConfig = 1900124,
    FarmGoldActivityLevelDiffNotConfig = 1900125,
    FarmGoldInstIdInValid = 1900126,
    FarmGoldVarNotExist = 1900127,
    FarmGoldResultCacheNotExist = 1900128,
    FarmGoldInstNotOpen = 1900129,
    FarmGoldActivityLimitDataNotFind = 1900130,
    MapTravelDataErr = 1900131,
    MapTravelConfigErr = 1900132,
    MapTravelCannotReward = 1900133,
    MapTravelRewardGet = 1900134,
    MapTravelMaxLevel = 1900135,
    MapTravelLackExp = 1900136,
    FarmGoldLevelNotOpen = 1900137,
    MapTravelAreaLock = 1900138,
    MapTravelAreaConfigErr = 1900139,
    SlashAndTowerCacheErr = 1900142,
    SlashAndTowerConfigErr = 1900143,
    SlashAndTowerDataErr = 1900144,
    SlashAndTowerReceivedLevelAward = 1900145,
    SlashAndTowerNotReward = 1900146,
    SlashAndTowerRoleNum = 1900147,
    SlashAndTowerBuffNum = 1900148,
    SlashAndTowerBuffConfig = 1900149,
    SlashAndTowerBuffAccess = 1900150,
    SlashAndTowerRoleSame = 1900151,
    SlashAndTowerSeasonErr = 1900152,
    SlashAndTowerBuffLack = 1900153,
    MapTravelLevelUpCfgErr = 1900140,
    MapTravelLevelCfgErr = 1900141,
    TeamParkOurTaskCfgErr = 1900154,
    TeamParkOurDataErr = 1900155,
    TeamParkOurCfgNoMatch = 1900156,
    TeamParkOurTaskDoing = 1900157,
    TeamParkOurTaskTaken = 1900158,
    TeamParkOurLevelLock = 1900159,
    TeamParkOurFindNoLevel = 1900160,
    SlashAndTowerRewardErr = 1900161,
    SlashAndTowerBuffSeasonErr = 1900162,
    SlashAndTowerNotOpen = 1900163,
    SlashAndTowerSeasonToCfgErr = 1900164,
    SlashAndTowerSeasonNoUpdate = 1900165,
    SlashAndTowerFirstNoPass = 1900166,
    SlashAndTowerLevelErr = 1900167,
    TeamParkMemberErr = 1900168,
    AvignonNotOpen = 1900172,
    AvignonNotConfig = 1900173,
    AvignonTaskNotFinish = 1900174,
    AvignonTaskNotData = 1900175,
    AvignonHadReward = 1900176,
    SlashAndTowerTeamErr = 1900169,
    SlashAndTowerBuffErr = 1900170,
    SeasonTowerNoMatch = 1900171,
    TeamParkOurMemberErr = 1900177,
    BattlePassRewardDone = 1900178,
    SlashAndTowerAwardCfgErr = 1900179,
    NoFlySkinItem = 1900180,
    FlySkinHadWear = 1900181,
    NoOldFlySkinItem = 1900182,
    OldFlySkinNoWear = 1900183,
    RoleWearNoFlySkin = 1900184,
    FlySkinParaGliderNoOpen = 1900185,
    FlySkinSoaringWingNoOpen = 1900186,
    NoRoleWearFlySkinSucc = 1900187,
    FlySkinItemNoConfig = 1900188,
    SlashAndTowerLevelSettle = 1900189,
    FlySkinTypeErr = 1900190,
    FlySkinTrialRole = 1900191,
    RegreeNotOpen = 1900192,
    RegressNoConfig = 1900193,
    RegressWrongId = 1900194,
    NoRegressRoundData = 1900195,
    NotInRegress = 1900196,
    AskRewardNoFinish = 1900197,
    BirthdayNotArrived = 1900198,
    BirthdayRoleInvalid = 1900199,
    BirthdayNoCfg = 1900200,
    HadBirthDayReward = 1900201,
    BirthdayNoReset = 1900202,
    BirthDayRewardTimeInvalid = 1900203,
    BirthdayMustNowYear = 1900204,
    BirthDayNotOpen = 1900205,
    BirthDayRewardNotFinish = 1900206,
    BirthdayRoleDone = 1900207,
    MoraleNoRewardGet = 1900208,
    FloroNoRanchData = 1900209,
    FloroNoRanchTech = 1900210,
    FloroTechUnLock = 1900211,
    FloroPreNodeLock = 1900212,
    FloroTechPoint = 1900213,
    FloroNoFindActivityTask = 1900214,
    FloroNoFindTaskCfg = 1900215,
    FloroTaskIsRunning = 1900216,
    FloroTaskInValidTime = 1900217,
    FloroRewardDone = 1900218,
    FloroMilestoneNoFind = 1900219,
    FloroDropNoFind = 1900220,
    FloroNoReward = 1900221,
    FloroActivityParamErr = 1900222,
    ConditionTimeOut = 1900223,
    ConditionModuleErr = 1900224,
    ConditionGroupIdErr = 1900225,
    ConditionRegisterFail = 1900226,
    FloroRanchSetAgain = 1900227,
    MoonPhaseNoOpen = 1900228,
    MoonPhaseNoCount = 1900229,
    ErrRandomDone = 1900230,
    PhaseMoonRandomErr = 1900231,
    PhaseMoonCfgErr = 1900232,
    MoonLabelErr = 1900233,
    MoonPhaseRewardErr = 1900234,
    MoonPhaseRewardDone = 1900235,
    PhaseMoonItemErr = 1900236,
    MoonPhaseBuffFail = 1900237,
    CoopRoleMaxLevelErr = 1900238,
    CoopRoleNoUnLock = 1900239,
    CoopRoleSpRewradErr = 1900240,
    CoopRoleRewardErr = 1900241,
    NoRewardCanGain = 1900242,
    CoopRoleDataErr = 1900243,
    CoopIdErr = 1900244,
    MotorcycleMotorIpErr = 1900245,
    MotorcycleRewardEmpty = 1900246,
    RoadBookNotData = 1900247,
    RoadBookNotInOpen = 1900248,
    RoadBookTaskCannotReward = 1900249,
    RoadBookRewardHad = 1900250,
    RoadBookOverMaxLevel = 1900251,
    RoadBookCfgErr = 1900252,
    RoadBookTaskLock = 1900253,
    RoadBookChallengeScoreErr = 1900254,
    RoadBookDuplicateReward = 1900255,
    RoadBookNoRewardGet = 1900256,
    GuessJokerLevelErr = 1900257,
    GuessJokerNotInScene = 1900258,
    GuessJokerNotGameObj = 1900259,
    GuessJokerPlayCardErr = 1900260,
    GuessJokerActionPhaseErr = 1900261,
    GuessJokerDeckErr = 1900262,
    GuessJokerNoHandCard = 1900263,
    GuessJokerBanCard = 1900264,
    GuessJokerOnlyPair = 1900265,
    GuessJokerPairAll = 1900266,
    GuessJokerCardRefuse = 1900267,
    GuessJokerDrawCardErr = 1900268,
    JokerSkillTriggerFail = 1900269,
    JokerSkillNoConfig = 1900270,
    JokerSkillOwnerErr = 1900271,
    GuessJokerPlayCardOver = 1900272,
    GuessJokerPlayCardAgain = 1900273,
    JokerGuessLevelLock = 1900274,
    JokerGuessPreLevelLock = 1900275,
    GuessJokerInitErr = 1900276,
    NoDrinkRequireListConfig = 1900277,
    DrinkBaseCountError = 1900278,
    DrinkBaseNoConfig = 1900279,
    DrinkBatchingCountErr = 1900280,
    DrinkBatchingConfigErr = 1900281,
    DrinkOrnaNoConfig = 1900282,
    DrinkRoleLikeNoConfig = 1900283,
    DrinkLikePointErr = 1900284,
    GuessJokerScriptPlayErr = 1900285,
    GuessJokerScriptDrawErr = 1900286,
    GuessJokerNoPlayerWait = 1900287,
    GuessJokerSkillUseOver = 1900288,
    DrinkBatchingRepeate = 1900289,
    SpringFestivalNoReward = 1900290,
    SpringFestivalDropNoFind = 1900291,
    SpringFestivalTaskRunning = 1900292,
    SpringFestivalHadReward = 1900293,
    SpringFestivalActivityIdErr = 1900294,
    SpringFestivalTaskFindErr = 1900295,
    SpringFestivalNoActivityCfg = 1900296,
    SpringFestivalParamCountErr = 1900297,
    SpringFestivalParamAgainErr = 1900298,
    PayShopGoodsIdRepeatErr = 1900299,
    PayShopBindActivityErr = 1900300,
    PayShopActivityFromErr = 1900301,
    PayShopFromErr = 1900302,
    ErrPayShopGoodsTypeCountOverFlow = 1900303,
    ErrPayShopTotalCountOverFlow = 1900304,
    ErrRegressDevelopTask = 1900305,
    ErrRegressVersion = 1900306,
    DrinksLevelLock = 1900307,
    DrinksRoleInviteErr = 1900308,
    DrinksNoRoleData = 1900309,
    GuessJokerTraceOver = 1900310,
    GuessJokerDataErr = 1900311,
    GuessJokerTaskRunning = 1900312,
    GuessJokerTaskDone = 1900313,
    DrinksRewardTaskRunning = 1900314,
    DrinksRewardTaskDone = 1900315,
    CoopReqLenthOver = 1900316,
    CoopRewardIdErr = 1900317,
    CoopRewardConfigNotFind = 1900318,
    CoopRewardDiff = 1900319,
    CoopActivityDataErr = 1900320,
    CoopRewardDone = 1900321,
    CoopRewardNoFinish = 1900322,
    CoopItemDropErr = 1900323,
    CoopItemDropErr2 = 1900324,
    PinballParamLimitOver = 1900325,
    PinballTaskConfigLoss = 1900326,
    PinballActivityErr = 1900327,
    PinballActivityHadReward = 1900328,
    PinballActivityTaskRunning = 1900329,
    PinballActivityTaskDropNoFind = 1900330,
    PinballActivityTaskParamAgainErr = 1900331,
    PinballActivityTaskNoReward = 1900332,
    PinballRoleAddLevelErr = 1900333,
    PinballRoleConfigLoss = 1900334,
    PinballRoleDataLoss = 1900335,
    PinballRoleDataMaxLvLoss = 1900336,
    PinballRoleMaxLvErr = 1900337,
    PinballActivityConfigLoss = 1900338,
    PinballRoleExpItemLack = 1900339,
    PinballWeaponIsTrail = 1900340,
    PinballWeaponNoExist = 1900341,
    PinballWeaponConfigNoExist = 1900342,
    PinballRoleNoExist = 1900343,
    PinballRoleConfigNoExist = 1900344,
    PinballWeaponRoleMismatch = 1900345,
    PinballPersonWeaponChangeErr = 1900346,
    PinballWearRepeat = 1900347,
    PinballWeaponOverLimit = 1900348,
    PinballWeaponRepeat = 1900349,
    PinballWeaponLock = 1900350,
    PinballWeaponIsEquip = 1900351,
    PinballWeaponNoDel = 1900352,
    ActivityRewardParamOver = 1900353,
    ActivityNotSame = 1900354,
    ActivityRewardDropErr = 1900355,
    ActivityNoRewardCanGet = 1900356,
    ActivityTaskTimeErr = 1900357,
    ActivityTaskRewardRepeat = 1900358,
    ActivityDataNotFind = 1900359,
    PinballWeaponInitErr = 1900360,
    PinballNoFriend = 1900361,
    PinballTowerRankCd = 1900362,
    PinballRankDataEmpty = 1900363,
    PinballLevelLock = 1900364,
    PinballDailyInstErr = 1900365,
    PinballRoleNotOpen = 1900366,
    PinballDailyNotOpen = 1900367,
    PinballWeaponPersonMismatch = 1900368,
    NoRoleVoiceDataSet = 1900369,
    RoleVoiceSettingParamOver = 1900370,
    RoleVoiceParamRepeat = 1900371,
    PlayerUsingSkill = 1900372,
    PlayerClimbing = 1900373,
    PlayerInWater = 1900374,
    PlayerInAir = 1900375,
    OnlineMotorLevelLock = 1900376,
    OnlineMotorMemberErr = 1900377,
    OnlineMotorMemberActivityClose = 1900378,
    RealmBetweenOverMaxLevel = 1900379,
    ActivityTaskLock = 1900380,
    ErrRecallItemBagItemTypeNotSupport = 1900381,
    ErrRecallItemBagFull = 1900382,
    ActivityPlayingCanNotDissolve = 1900383,
    ErrRecallActionNotSupport = 1900384,
    ErrAdventureSilentCtxNull = 1900385,
    ErrAdventureSilentId = 1900386,
    RoverRogueTalentAlreadyUnlock = 1900387,
    RoverRoguePreTalentNotUnlock = 1900388,
    RoverRogueTalentConditionNotFinish = 1900389,
    ErrMailFavoriteCannotDelete = 1900390,
    ErrMailFavoriteNeedRead = 1900391,
    ErrMailFavoriteNeedTake = 1900392,
    ErrMailFavoriteFull = 1900393,
    ErrPhantomItemUse = 1900394,
    ErrRoleTrailItemUse = 1900395,
    ErrInFlowItemUse = 1900396,
    ErrInVehicleItemUse = 1900397,
    ErrZitherNoMainRoleInFormation = 1900398,
    ErrTrialFollowShooterItemUse = 1900399,
    ErrMainRoleVoiceRoleId = 1900400,
    ErrTrackMoonNotInRecall = 1900401,
    MatchThreeRoleLock = 1900402,
    ErrMatchThreeScoreInvalid = 1900403,
    ErrMatchThreeHourGlassInvalid = 1900404,
    ErrMatchThreeBottleInvalid = 1900405,
    ErrMatchThreeRecordTypeMismatch = 1900406,
    CameraModeNotOpen = 1900407,
    ErrItemUseBanned = 1900408,
    ErrItemUseMainRoleDead = 1900409,
    ErrWaitingOtherJoinBanUse = 1900410,
    ErrPhantomSelfPortrait = 1900411,
    ErrRoleTrailSelfPortrait = 1900412,
    ErrPlayerStateSelfPortrait = 1900413,
    ErrMainRoleDeadSelfPortrait = 1900414,
    ErrInFlowSelfPortrait = 1900415,
    ErrInVehicleSelfPortrait = 1900416,
    ErrMainRoleActiveSelfPortrait = 1900417,
    ErrZitherItemUse = 1900418,
    InTeleportItemUse = 1900419,
    IsMatchingItemUse = 1900420,
    InTeleportSelfPortrait = 1900421,
    IsMatchingSelfPortrait = 1900422,
    ErrRecallItemUseFail = 1900460,
    ErrRecallItemUseTypeNotSupport = 1900461,
    ErrRecallItemTypeNotSupport = 1900462,
    ErrRecallItemCountInvalid = 1900463,
    ErrRecallItemCanNotUse = 1900464,
    ErrWaitingOtherJoinSelfPortrait = 1900467,
    ErrDoCommonRewardConfigError = 2000000,
    InstPlayNotSettle = 2000001,
    InstPlayNotFinishExecute = 2000002,
    ErrResetItemEntityNotContain = 2000003,
    InstPlayExchangeRewardNotExist = 2000004,
    MapConfigNull = 2000005,
    MapConfigError = 2000006,
    InstPlayComponentNotExist = 2000007,
    InstTeleportResetPlayerDead = 2000008,
    DrownEndTeleportInBigWorld = 2000009,
    ErrFightTrialRoleRoldIdsError = 2000010,
    ErrFightTrialRoleFromationError = 2000011,
    ErrInstSaveFail = 2000012,
    ErrActiveFoundationControlPlayerError = 2000013,
    ErrActiveFoundationOccupation = 2000014,
    ErrSingleInstanceCanNotOnline = 2000015,
    ErrInstanceRechallengeLimit = 2000016,
    ErrTargetSame = 2000017,
    ErrAttachTargetType = 2000018,
    ErrAttachInfoNull = 2000019,
    ErrLevelPlayChallengeFail = 2000020,
    ErrGmtip = 2000021,
    ErrPosSenderEntityNoExist = 2000022,
    ErrPosSenderComponentNoExist = 2000023,
    ErrPosSenderParamError = 2000024,
    ErrPosSenderRemoveSenderNotSame = 2000025,
    ErrConnectorEntityNoExist = 2000026,
    ErrConnectorCompNoExist = 2000027,
    ErrConnectorPreIdError = 2000028,
    ErrConnectorCompleteState = 2000029,
    ErrConnectorActiveStateError = 2000030,
    ErrConnectorMatchErro = 2000031,
    ErrActiveControlOccupation = 2000032,
    ErrPortalCreatorActive = 2000033,
    ErrComponentNull = 2000034,
    ErrPortalCreatorConfigError = 2000035,
    ErrPortalCreatorCreateFail = 2000036,
    ErrTrialRoleEnterInst = 2000037,
    ErrNpcInVehicle = 2000038,
    ErrSceneItemBbnotChange = 2000039,
    ErrInitMatchNotSuccess = 2000040,
    ErrFightFormationSameRoleError = 2000041,
    ErrFightMainRoleConflict = 2000042,
    ErrVehicleItemConfigError = 2000043,
    ErrVehicleCreateError = 2000044,
    ErrVehicleEntityTypeError = 2000045,
    ErrInstanceActivityExpire = 2000046,
    ErrSlashAndTowerLevelUnlock = 2000047,
    ErrInstLevelUnlock = 2000048,
    ErrEnterInstConfigError = 2000049,
    ErrMoveWithSplineConfigError = 2000050,
    ErrMoveWithSplineIdError = 2000051,
    ErrMoveWithSplineStop = 2000052,
    ErrMoveWithSplineEntityConfigError = 2000053,
    ErrMoveWithSplineControlPlayerError = 2000054,
    ErrLeaveSceneStarted = 2000055,
    ErrInstTeleportInstLimit = 2000056,
    ErrGetOnVehicleNotInAoiSight = 2000057,
    ErrHoldHandCharacterNotCurrent = 2000058,
    ErrEnterInstPreInstNotComplete = 2000059,
    ErrSetSystemVarError = 2000060,
    ErrSetVarPermission = 2000061,
    ErrSetVarCtxError = 2000062,
    ErrSetVarGameCtxSceneError = 2000063,
    ErrSetVarNotPublic = 2000064,
    ErrSetVarComponent = 2000065,
    ErrSetVarTargetNull = 2000066,
    ErrVehicleNotMotor = 2000067,
    ErrVehicleFormationError = 2000068,
    ErrShareRideConflict = 2000069,
    ErrShareRideNotExist = 2000070,
    ErrNotInShareRide = 2000071,
    ErrNotInVehicle = 2000072,
    ErrSceneRoadGraphNotExist = 2000073,
    ErrVehicleFlowRewardMax = 2000074,
    ErrGetOnVehicleFailInFlow = 2000075,
    ErrGetOnPlayerVechielGravityDifferent = 2000076,
    ErrChangeFightState = 2100000,
    ErrAddFragileFail = 2100001,
    ErrStoreEnergyClose = 2100002,
    ErrAttrOverMax = 2100003,
    ErrBattleVersion = 2100004,
    ErrGmkillEntityNotValid = 2200000,
    ErrSplineConfigNotExist = 2200001,
    BossRushActivityNotOpen = 2200002,
    BossRushActivityScoreRewardNotExist = 2200003,
    BossRushActivityLevelRewardNotExist = 2200004,
    BossRushActivityScoreNotEnough = 2200005,
    BossRushActivityLevelNotPass = 2200006,
    BossRushActivityRewardClaimed = 2200007,
    BossRushActivityBuffSelectionNotValid = 2200008,
    BossRushActivityConfigNotExist = 2200009,
    BossRushActivityCharacterSelectionNotValid = 2200010,
    BossRushActivityComponentNotExist = 2200011,
    BossRushActivityCharacterSelectionEmpty = 2200012,
    BossRushActivityBuffSelectionEmpty = 2200013,
    BossRushActivityLevelNotOpen = 2200014,
    InRangeEntityDuplicate = 2200015,
    InRangeEntityNotExist = 2200016,
    NpcPerformComponentNotExist = 2200017,
    NpcPerformStateNotInit = 2200018,
    NpcPerformActionTargetEntityNotExist = 2200019,
    ActionQueueTypeNotExist = 2200020,
    ActionQueueCtxTypeNotExist = 2200021,
    ExecuteQueueOwnerHasAction = 2200022,
    ActionQueueExceedMaxCount = 2200023,
    ActionQueueStartActionGroupFail = 2200024,
    ActionQueueComponentNotExist = 2200025,
    ActionQueueNotInit = 2200026,
    ChangeBatchEntitiesStateError = 2200027,
    EnableNearbyTrackingTargetEntityNotExist = 2200028,
    EnableNearbyTrackingSelfNotEntity = 2200029,
    EnableNearbyTrackingSelfComponentNotExist = 2200030,
    SetTeleControlEntityNotExist = 2200031,
    SetTeleControlTypeNotExist = 2200032,
    SetTeleControlComponentNotExist = 2200033,
    SetTeleControlCoordEntityNotExist = 2200034,
    SceneItemAttributeIdNotInType = 2200035,
    SceneItemAttributeIdNotExist = 2200036,
    ModifySceneItemAttributeEntityNotExist = 2200037,
    AddSceneItemAttributeTagDuplicate = 2200038,
    RemoveSceneItemAttributeTagNotExist = 2200039,
    AttributeEntityLock = 2200040,
    AttributeEntitySilent = 2200041,
    ModifySceneItemAttributeTagNotExist = 2200042,
    ErrEnterInstCtx = 2200043,
    ErrEnterInstBlackboardValueNotExist = 2200044,
    TriggerLocked = 2200045,
    TriggerIgnore = 2200046,
    TriggerEntityNull = 2200047,
    TriggerEntityNotMatch = 2200048,
    TriggerMatchCountNotMet = 2200049,
    TriggerActionEmpty = 2200050,
    ExceedMaxTriggerCount = 2200051,
    TriggerAlreadyLeaveWhenEnterCondFail = 2200052,
    TriggerLeaveConfigEmpty = 2200053,
    TriggerConditionNotMet = 2200054,
    TriggerRangeRationalityFail = 2200055,
    TrampleEntityNotMatch = 2200056,
    TrampleConditionNotMet = 2200057,
    TrampleMatchCountNotMet = 2200058,
    HasDestroySelfActionInQueue = 2200059,
    EntityWillDestroy = 2200060,
    AddInRangePlayerDuplicate = 2200061,
    AddInRangeEntityDuplicate = 2200062,
    RemoveInRangePlayerNotExist = 2200063,
    RemoveInRangeEntityNotExist = 2200064,
    GravityDirectionNoChange = 2200072,
    HookExitWayNotExist = 2200065,
    HookLockPointLocked = 2200066,
    HookLockAddPlayerDuplicate = 2200067,
    HookLockRemovePlayerNotExist = 2200068,
    KiteHookLockPointOnlyOnePlayer = 2200069,
    EffectAreaAddBuffFail = 2200073,
    ErrGravityInteractNoPermission = 2200074,
    PlayerNotInAnyGravityRegion = 2200075,
    ErrSceneEntityAlreadyExist = 2200070,
    PlayerLeaveGravityRegionInAbnormalGravity = 2200076,
    HookLockPointConditionNotMet = 2200071,
    PlayerInAbnormalGravity = 2200077,
    GravityFlipIndexNoChange = 2200078,
    GravityFlipIndexNotExist = 2200079,
    GravityFlipTypeNoChange = 2200080,
    GravityFlipTypeNotExist = 2200081,
    GravityFlipNotUpdateToTargetDirection = 2200082,
    ReliablePosNotInGravityRegion = 2200083,
    TargetPosNotInGravityRegion = 2200084,
    PlayerCurGravityDirectionNotInOptions = 2200085,
    WaterfallClimbingParticipatorCountErr = 2200086,
    WaterfallClimbingParticipatorNoVehicle = 2200087,
    WaterfallClimbingVehicleNoPassenger = 2200088,
    WaterfallClimbingPlayerNotInPassenger = 2200089,
    ErrVehicleEntityNotExist = 2200090,
    GravityFlipLocked = 2200091,
    ClientActionSkipped = 2200092,
    AbnormalGravityCannotAddTemporary = 2200093,
    ErrReliablePosEntityNotExist = 2200094,
    UpdateReliablePosGravityDirNotMatch = 2200095,
    UpdateReliablePosNotWalkable = 2200096,
    UpdateReliablePosSelfNotEntity = 2200097,
    GravityDirectionHasNaN = 2200098,
    ErrHostInAbnormalGravity = 2200099,
    MultiModeCannotTeleport = 2200100,
    NotTrapDefenseInst = 2200101,
    GridCellCreateFuncNotFound = 2200102,
    GridSystemNotExist = 2200103,
    ErrGridCellType = 2200104,
    GridCellDirectionNotSpecified = 2200105,
    NotGridObjectEntity = 2200106,
    GridNotExist = 2200107,
    GridCellNotExist = 2200108,
    GridCellAlreadyOccupied = 2200109,
    GridCellDisabled = 2200110,
    GridCellPositionNotValid = 2200111,
    SimpleCombatEntityNotMonster = 2200112,
    GridObjectEntityNotFound = 2200113,
    NotLowMemoryPlatform = 2200114,
    ErrLowMemorySwitchingScene = 2200115,
    SimpleCombatEntityBuffNotExist = 2200116,
    SimpleCombatEntityBuffDuplicate = 2200117,
    TrapDefenseRandomBdFail = 2200118,
    TrapDefenseBdConfigNotExist = 2200119,
    TrapDefenseBdGroupConfigOfBdNotExist = 2200120,
    TrapDefenseRandomQualityFail = 2200121,
    TrapDefenseRandomBdGroupFail = 2200122,
    TrapDefenseBdGroupPoolEmpty = 2200123,
    TrapDefenseNoAvailableBd = 2200124,
    TrapDefenseNoAvailableQuality = 2200125,
    TrapDefenseDrawActionConfigNotExist = 2200126,
    TrapDefenseInstConfigNotExist = 2200127,
    TrapDefenseWaveConfigNotExist = 2200128,
    TrapDefenseSpawnMonsterWaveConfigNotExist = 2200129,
    TrapDefenseMonsterConfigNotExist = 2200130,
    TrapDefenseMonsterDataDuplicate = 2200131,
    TrapDefenseTrapConfigNotExist = 2200132,
    TrapDefenseTrapEntityNotFound = 2200133,
    TrapDefenseNotPreviewStep = 2200134,
    TrapDefenseBdGroupConfigNotExist = 2200135,
    TrapDefenseBdPoolEmpty = 2200136,
    TrapDefenseQualityPoolEmpty = 2200137,
    ErrTrapDefenseRequestParam = 2200138,
    ErrTrapDefenseRequestParamDuplicate = 2200139,
    ErrTrapDefenseRewardConfig = 2200140,
    ErrTrapDefenseRewardTime = 2200141,
    ErrTrapDefenseRewardReceived = 2200142,
    ErrTrapDefenseRewardNotComplete = 2200143,
    ErrTrapDefenseSceneErr = 2200144,
    ErrTrapDefenseContextEmpty = 2200145,
    ErrTrapDefenseChallengeConfig = 2200146,
    ErrTrapDefenseChallengeNotOpen = 2200147,
    ErrTrapDefenseBuildingLock = 2200148,
    ErrTrapDefenseAuxiliaryLock = 2200149,
    ErrTrapDefenseSlotCount = 2200150,
    ErrTrapDefenseSlotBuildingDuplicate = 2200151,
    ErrTrapDefenseSlotDuplicate = 2200152,
    ErrTrapDefenseBuildingConfig = 2200153,
    ErrTrapDefenseAuxiliaryConfig = 2200154,
    ErrTrapDefenseSlotAuxiliaryDuplicate = 2200155,
    ErrTrapDefenseForceBuilding = 2200156,
    ErrTrapDefenseForceAuxiliary = 2200157,
    ErrTrapDefenseBuildingLevelConfig = 2200158,
    ErrTrapDefenseBuildingMaxLevel = 2200159,
    ErrTrapDefenseActivityConfig = 2200160,
    ErrTrapDefenseAuxiliaryLevelConfig = 2200161,
    ErrTrapDefenseAuxiliaryMaxLevel = 2200162,
    ErrTrapDefenseAuxiliaryBranch = 2200163,
    ErrTrapDefenseBuildingBranch = 2200164,
    ErrTrapDefenseAuxiliaryReset = 2200165,
    ErrTrapDefenseBuildingReset = 2200166,
    ErrTrapDefenseTechConfig = 2200167,
    ErrTrapDefenseTechUnlock = 2200168,
    ErrTrapDefenseTechPreNodeConfig = 2200169,
    ErrTrapDefenseTechPreNodeLock = 2200170,
    ErrTrapDefenseTechCostNotEnough = 2200171,
    TrapDefenseItemConfigNotExist = 2200172,
    TrapDefenseItemExceedCarryLimit = 2200173,
    TrapDefenseItemNotEnough = 2200174,
    TrapDefenseBuyShopBdGroupNotExist = 2200175,
    TrapDefenseBuyShopItemNotExist = 2200176,
    TrapDefenseGoldNotEnough = 2200177,
    TrapDefensePayGoldFail = 2200178,
    TrapDefenseBuyShopItemNotEnough = 2200179,
    TrapDefenseShopDataNotExist = 2200180,
    TrapDefenseShopConfigNotExist = 2200181,
    TrapDefenseShopRefreshCountNotEnough = 2200182,
    TrapDefenseGoldenCoinConfigNotExist = 2200183,
    TrapDefenseNotGoldenCoinEntity = 2200184,
    TrapDefenseNotMonsterEntity = 2200185,
    TrapDefenseNotBuildingEntity = 2200186,
    TrapDefenseVarCannotUnderZero = 2200187,
    TrapDefenseVarTypeNotExist = 2200188,
    TrapDefenseBuyShopBdGroupSold = 2200189,
    TrapDefenseTrapAmountExceedLimit = 2200190,
    TrapDefenseNotRewardStep = 2200191,
    TrapDefenseBdDrawDataNotExist = 2200192,
    TrapDefenseBdDrawRefreshCountNotEnough = 2200193,
    TrapDefenseSpecialCellConfigNotExist = 2200194,
    TrapDefenseSpecialCellCertainLevelConfigNotExist = 2200195,
    ErrActionTargetEntityNotExist = 2200196,
    TrapDefenseCleanerNotEnough = 2200197,
    TrapDefenseItemConfigError = 2200198,
    TrapDefenseSpecialCellTypeNotSame = 2200199,
    TrapDefenseCellNotSpecialCell = 2200200,
    TrapDefenseExceedMaxTrapCount = 2200201,
    GridSystemCannotGetWorldGridPos = 2200202,
    TrapDefenseTrapCannotSelfDestruct = 2200203,
    TrapDefenseBdDrawResultEmpty = 2200204,
    TrapDefenseBdGroupMaxLevel = 2200205,
    TrapDefenseCannotDeductHealth = 2200206,
    SimpleCombatBuffConfigNotExist = 2200207,
    SimpleCombatSubTypeNotExist = 2200208,
    TrapDefenseBdGroupDisabled = 2200209,
    TrapDefenseMonsterDisabled = 2200210,
    TrapDefenseGainExceedLimit = 2200211,
    OnlyOneRollBlockCanBeActivated = 2200212,
    RollBlockGroupConfigNotExist = 2200213,
    RollBlockDifficultyConfigNotExist = 2200214,
    RollBlockNextDifficultyConfigNotExist = 2200215,
    RollBlockGamePlayIdNotExist = 2200216,
    RollBlockGamePlayNotExist = 2200217,
    RollBlockMovementEntityNotExist = 2200218,
    RollBlockMovementEntityNotBlock = 2200219,
    RollBlockMainControlPlayerNotExist = 2200220,
    RollBlockEntityNotMoving = 2200221,
    RollBlockNoGridAfterRoll = 2200222,
    RollBlockTemplateConfigNotExist = 2200223,
    RollBlockNotMainControlPlayer = 2200224,
    RollBlockEntityNotExist = 2200225,
    RollBlockBlockWillChangeState = 2200226,
    RollBlockBlockStateNoChange = 2200227,
    RollBlockMainControlBlockNotExist = 2200228,
    RollBlockDifficultyDataNotExist = 2200229,
    RollBlockPositionNoFloor = 2200230,
    RollBlockPositionItemCannotPass = 2200231,
    RollBlockPositionBlockCannotPass = 2200232,
    RollBlockInputCannotMove = 2200233,
    RollBlockPositionFloorCannotPass = 2200234,
    RollBlockHintAlreadyActive = 2200235,
    RollBlockHintBlockNotExist = 2200236,
    RollBlockHintEmpty = 2200237,
    RollBlockAlreadyRewarded = 2200238,
    RollBlockDifficultyNotPassed = 2200239,
    RollBlockHintStepNotExecuting = 2200240,
    RollBlockCurDifficultyConfigNotExist = 2200241,
    RollBlockHintStepExecuting = 2200242,
    RollBlockChildQuestGamePlayActived = 2200243,
    RollBlockOnlyOneControllableBlock = 2200244,
    RollBlockGamePlayStateNoChange = 2200245,
    RollBlockGamePlayCannotReady = 2200246,
    RollBlockGamePlayNotReady = 2200247,
    RollBlockSendResetTooFrequently = 2200248,
    EasterEggIdNotExist = 2200249,
    TargetSceneNotBigWorldInst = 2200250,
    EnterBigWorldInstCtxNotImplemented = 2200251,
    EasterEggOwnerTypeNotImplemented = 2200252,
    RollBlockCannotReset = 2200253,
    CurSceneNotBigWorld = 2200254,
    KurotatoCharacterConfigNotExist = 2200255,
    KurotatoLevelInfoNotExist = 2200256,
    KurotatoUpgradeRewardStepNotActive = 2200257,
    KurotatoUpgradeRewardItemNotFound = 2200258,
    KurotatoUpgradeRewardDataNotExist = 2200259,
    KurotatoActivityConfigNotExist = 2200260,
    KurotatoWeaponConfigNotExist = 2200261,
    KurotatoWeaponNotExist = 2200262,
    KurotatoItemConfigNotExist = 2200263,
    KurotatoItemNotExist = 2200264,
    KurotatoChestRewardStepNotActive = 2200265,
    KurotatoChestRewardItemNotExist = 2200266,
    KurotatoEffectConfigNotExist = 2200267,
    KurotatoEffectNotExist = 2200268,
    KurotatoDrawConfigNotExist = 2200269,
    KurotatoPropertyConfigNotExist = 2200270,
    KurotatoPropertyNotInSimpleCombatDefine = 2200271,
    KurotatoShopStepNotActive = 2200272,
    KurotatoShopProductNotFound = 2200273,
    KurotatoShopGoldNotEnough = 2200274,
    KurotatoVarNameNotExist = 2200275,
    KurotatoVarNotExist = 2200276,
    KurotatoRoleLevelConfigNotExist = 2200277,
    KurotatoCharacterEntityNotExist = 2200278,
    KurotatoGoldNotEnough = 2200279,
    KurotatoMonsterConfigNotExist = 2200280,
    KurotatoMultiModeNotSupported = 2200281,
    KurotatoLevelConfigNotExist = 2200282,
    KurotatoDropInfoNotExist = 2200283,
    KurotatoDropConfigNotExist = 2200284,
    KurotatoWaveConfigNotExist = 2200285,
    KurotatoWeaponCountReachLimit = 2200286,
    KurotatoWeaponRefineNotEnoughSameQuality = 2200287,
    KurotatoItemStackFull = 2200288,
    KurotatoCharacterUpgradeConfigNotExist = 2200289,
    KurotatoUpgradeRewardDrawIdNotExist = 2200290,
    KurotatoDrawResultEmpty = 2200291,
    KurotatoLuckPropertyNotExist = 2200292,
    KurotatoGoldGainPropertyNotExist = 2200293,
    KurotatoCharacterAlreadyExist = 2200294,
    KurotatoCombatStepNotActive = 2200295,
    KurotatoNotInSpecialWave = 2200296,
    KurotatoWeaponCountReachMin = 2200297,
    KurotatoNotSupportedProductType = 2200298,
    KurotatoStructureConfigNotExist = 2200299,
    KurotatoStructureCannotCreate = 2200300,
    NotQaAccountWithHiddenServer = 2300000,
    DisabledFuncInHiddenServer = 2300001,
    ErrActionExecutorFinishConditionNotSport = 2400000,
    ErrAlreadyInSwitchNode = 2400001,
    ErrCornActivityId = 2500000,
    ErrCornActivityNoOpen = 2500001,
    ErrCornNoActivityData = 2500002,
    NoPlayIdCorniceReward = 2500003,
    ActivityNoOpenCorniceReward = 2500004,
    ScoreLimitCorniceReward = 2500005,
    RewardedCorniceReward = 2500006,
    NoUnlockCorniceReward = 2500007,
    NoScoreCorniceReward = 2500008,
    TrackMoonPhaseNoConfig = 2500009,
    TrackMoonPhaseActivityNoOpen = 2500010,
    TrackMoonPhaseNoPolulary = 2500011,
    TrackMoonPhaseNoData = 2500012,
    TrackMoonPhaseRewarded = 2500013,
    TrackMoonPhaseDataNoConfig = 2500014,
    TrackMoonPhaseDataNoOpen = 2500015,
    TrackMoonPhaseDataNoData = 2500016,
    BctrewardNoTconfig = 2500017,
    BctrewardNoOpenActivity = 2500018,
    BctrewardNoData = 2500019,
    BctrewardNoUnlock = 2500020,
    BctrewardNoComplete = 2500021,
    BcarewardNoRconfig = 2500022,
    BcarewardNoOpenActivity = 2500023,
    BcarewardNoData = 2500024,
    BcarewardNoActive = 2500025,
    Bcarewarded = 2500026,
    BcarewardNoActiveReward = 2500027,
    CornTranNoPlayConfig = 2500028,
    CornTranNoOpenPlay = 2500029,
    CornTranNoOpenActivity = 2500030,
    CornTranNoEntityConfig = 2500031,
    BcarewardRepeatRewardId = 2500032,
    BcarewardDifferActivityId = 2500033,
    BctrewardNoActivity = 2500034,
    BctrewardIllegalRewardNum = 2500035,
    BctrewardNoUnlockStage = 2500036,
    PreheatSignNodeNoConfig = 2500037,
    PreheatSignNodeNoData = 2500038,
    PreheatSignActivityOnOpen = 2500039,
    PreheatSignNodeNoUnlock = 2500040,
    PreheatSignNodeNoRewardStatus = 2500041,
    PreheatSignNodeNoAnswer = 2500042,
    ScratchCardNoRoundConfig = 2500043,
    ScratchCardNoActivityConfig = 2500044,
    ScratchCardNoDbData = 2500045,
    ScratchCardIllegalIndex = 2500046,
    ScratchCardIndexRewarded = 2500047,
    ScratchCardNoTime = 2500048,
    ScratchCardNoRandomReward = 2500049,
    ScratchCardRoundNoUnlock = 2500050,
    BossRushPlayerNoSceneData = 2500051,
    BossRushPlayerCanNoChooseBuff = 2500052,
    BossRushPlayerIllegalIndex = 2500053,
    BossRushHadSameBuffId = 2500054,
    BossRushBuffCountLimit = 2500055,
    BossRushBuffIllegal = 2500056,
    BossRushBuffNoConfig = 2500057,
    ScratchCardActivityNoOpen = 2500058,
    MowToweNoLevelConfig = 2500059,
    MowTowerActivityNoOpen = 2500060,
    MowToweNoCacheData = 2500061,
    MowTowerNoPassFirstInst = 2500062,
    MowTowerLevelsIdError = 2500063,
    MowTowerRoleIdError = 2500064,
    MowTowerBuffIdError = 2500065,
    MowTowerNoComponent = 2500066,
    MowTowerHadSameRole = 2500067,
    MowTowerBuffCountError = 2500068,
    MowTowerNoFirstInst = 2500069,
    MowTowerNoActivityData = 2500070,
    MowTowerScoreLimit = 2500071,
    MowTowerScoreRewarded = 2500072,
    MowTowerScoreRewardConfig = 2500073,
    MowTowerNoInScene = 2500074,
    MowTowerNoRewardConfig = 2500075,
    MaterialReplaceNoTargetConfig = 2500076,
    MaterialReplaceNoConsumeConfig = 2500077,
    MaterialReplaceNoSameGroup = 2500078,
    MaterialReplaceErrConsumeNum = 2500079,
    ErrRoleSkinTrialNotInit = 2500080,
    ErrRoleSkinTrialNoFinish = 2500081,
    ErrRoleSkinTrialReward = 2500082,
    ErrRoleSkinTrialRewardDone = 2500083,
    ErrRoleSkinTrialTimeOut = 2500084,
    PhantomEquipGroupNoEquipPhantom = 2500085,
    PhantomEquipGroupCountLimit = 2500086,
    PhantomEquipGroupHadInTop = 2500087,
    PhantomEquipGroupNameEmpty = 2500088,
    PhantomEquipGroupNameCountLimiy = 2500089,
    PhantomRecommendFuncNoOpen = 2500090,
    BossRushTaskNoFinish = 2500091,
    PhantomGroupUseSame = 2500092,
    PhantomGroupFunNoOpen = 2500093,
    NoFishingActivityConfig = 2500094,
    NoInFishingActivityTime = 2500095,
    FishingActivityCanNoReward = 2500096,
    NoFishingActivityMileConfig = 2500097,
    FishingActivitySameMileId = 2500098,
    FishingActivityRewarded = 2500099,
    PlayerTitleFuncNoOpen = 2500100,
    PlayerTitleHadUndress = 2500101,
    PlayerTitleHadDress = 2500102,
    PlayerTitleNoUnlock = 2500103,
    BabelTowerActivityNoOpen = 2500104,
    BabelTowerLevelNoOpen = 2500105,
    BabelTowerDeEffectNoFind = 2500106,
    BabelTowerDeEffectCanNoChoose = 2500107,
    BabelTowerDeEffectMutex = 2500108,
    BabelTowerNoDailyTask = 2500109,
    BabelTowerTaskNoComplete = 2500110,
    BabelTowerTaskRewarded = 2500111,
    BabelTowerRoleLimit = 2500112,
    BabelTowerNoBuffConfig = 2500113,
    BabelTowerBuffCanNoChoose = 2500114,
    BabelTowerBuffNumIllegal = 2500115,
    BabelTowerBuffChooseCountLimit = 2500116,
    NoBabelTowerInsComponent = 2500117,
    BabelTowerNoSelectBuff = 2500118,
    PhantomPolishFuncNoOpen = 2500119,
    PhantomPolishHadLevelUp = 2500120,
    PhantomPolishQualityLimit = 2500121,
    PhantomPolishSamePro = 2500122,
    ExploreActivityNoOpen = 2500123,
    ExploreActivityTaskNoFinish = 2500124,
    ExploreActivityTaskRewarded = 2500125,
    BabelTowerDeTermNoUnlock = 2500126,
    BabelTowerBuffNoUnlock = 2500127,
    BabelTowerIsNoDifficult = 2500128,
    PlayerTitleNoConfig = 2500129,
    PlayerFixIndexIllegal = 2500130,
    PlayerFixHadFlag = 2500131,
    PlayerFixFrontNoFlag = 2500132,
    BabelTowerNoSelectRoles = 2500133,
    PhBaErrSelectNum = 2500134,
    PhBaErrSelectTarget = 2500135,
    PhBaNoFighterLogic = 2500136,
    PhBaNoChallengeConf = 2500137,
    PhBaSlotIllegal = 2500138,
    PhBaNoEvolveNum = 2500139,
    PhBaNoFighter = 2500140,
    PhBaNoHandCard = 2500141,
    PhBaNoCardConf = 2500142,
    PhBaNoCardEvolveCountInValid = 2500143,
    PhBaErrCostNoMatch = 2500144,
    PhBaNoCardRoleConf = 2500145,
    PhBaNoContainSkill = 2500146,
    PhBaNoSkillConf = 2500147,
    PhBaNoCardCanNoUse = 2500148,
    PhBaIsWaitClient = 2500149,
    PhBaNoDeployRound = 2500150,
    PhBaNoClientParam = 2500151,
    PhBaNoClientParamMatch = 2500152,
    PhBaErrSelectCardNum = 2500153,
    PhBaErrSelectReplace = 2500154,
    PhBaErrPreStep = 2500155,
    PhBaCanNoSelect = 2500156,
    PhBaConditionLimit = 2500157,
    PhBaNoOwnerFighter = 2500158,
    PhBaNoTriggerFighter = 2500159,
    PhBaSelectParamNull = 2500160,
    PhBaNoBuffData = 2500161,
    PhBaNoBuffConf = 2500162,
    PhBaErrCardGroupIndex = 2500163,
    PhBaCardRoleUnlock = 2500164,
    PhBaNoCardGroupConf = 2500165,
    PhBaSloFull = 2500166,
    PhBaHandCardEmpty = 2500167,
    PbSelectCardNotFound = 2500168,
    PbSelectCardCount = 2500169,
    PbNoNpcFighter = 2500170,
    PbNoPlayerFighter = 2500171,
    PbBteventDataIllegal = 2500172,
    PbNoCardFighter = 2500173,
    PbNoHandCardVarData = 2500174,
    PbNoPosVarData = 2500175,
    PbNoHandCard = 2500176,
    PbNoCardFighterVarData = 2500177,
    PbNoNpcConfig = 2500178,
    PbNoCostPoint = 2500179,
    PbNoActiveSkill = 2500180,
    PbErrSelectTargets = 2500181,
    PbErrHadFighter = 2500182,
    PbErrCardUnmovable = 2500183,
    PbNoCanSelectTarget = 2500184,
    PbCanNotOpOtherUnit = 2500185,
    PbCanNoBeEvolve = 2500186,
    PbKeyCostCanNotOp = 2500187,
    PbNoReChallenge = 2500188,
    PbFuncNoOpen = 2500189,
    PbNoRequestReChallenge = 2500190,
    PbNoSameSlotIndex = 2500191,
    PhBaHaCardEmpty = 2500192,
    PhBaPassedChallenge = 2500193,
    FunPlayNoConfig = 2500194,
    FunPlayNoFinish = 2500195,
    FunPlayHadRewarded = 2500196,
    PbCardSkillCountLimit = 2500197,
    BabelRoleIsSelected = 2500248,
    BabelNoTalentConfig = 2500249,
    BabelInnerCanNoOpe = 2500250,
    BabelTalentIsLearned = 2500251,
    BabelTalentNoFinish = 2500252,
    BabelNoActivityConfig = 2500253,
    BabelRankListInCd = 2500254,
    FunPlayDayNoOpen = 2500255,
    CheckShopConditionFail = 2500262,
    HonamiStoryBagUpdateRepeat = 2500263,
    HonamiStoryBagNoConfig = 2500264,
    HonamiStoryNormalAddItemErr = 2500265,
    HonamiStoryNoBagDb = 2500266,
    HonamiStoryUpdateItemSizeIllegal = 2500267,
    HonamiStoryItemUpdateRepeat = 2500268,
    HonamiStoryBagHadItem = 2500269,
    HonamiStoryBagPosIllegal = 2500270,
    HonamiStoryItemUpdateTypeErr = 2500271,
    HonamiStoryBagNoHadItem = 2500272,
    HonamiStoryDropItemNoEntityId = 2500273,
    HonamiStoryDropItemNoEntity = 2500274,
    HonamiStoryNoDropEntity = 2500275,
    HonamiStoryDropNoItemDb = 2500276,
    HonamiStoryItemNoBalance = 2500277,
    HonamiStoryNoUpdateItem = 2500278,
    HonamiStoryCheckMarkNoItemConfig = 2500279,
    HonamiStoryCheckMarkOverSize = 2500280,
    HonamiStoryCheckWideOverSize = 2500281,
    HonamiStoryCheckMarkRepeat = 2500282,
    HonamiStoryActivityNoSame = 2500283,
    HonamiStoryNoEquipRackData = 2500284,
    HonamiStoryNoEquipType = 2500285,
    HonamiStoryNoEquipConfig = 2500286,
    HonamiStoryNoEquipTypeNoMatch = 2500287,
    HonamiStoryActivityIdErr = 2500288,
    HonamiStoryInnerCanNoModifyRole = 2500289,
    HonamiStoryRoleNoValid = 2500290,
    HonamiStoryNoChallengeConfig = 2500291,
    NoHonamiStoryInstType = 2500292,
    HonamiStoryNoItemConfig = 2500293,
    HonamiStoryPickDistanceErr = 2500294,
    HonamiStoryNoDressWeapon = 2500295,
    HonamiStoryNoWeaponConfig = 2500296,
    HonamiStoryWeaponPluginPosErr = 2500297,
    HonamiStoryWeaponPluginTypeNoMatch = 2500298,
    HonamiStoryWareHouseOverSize = 2500299,
    HonamiStoryNoOpenSafeBag = 2500300,
    HonamiStoryNoOpenSafeBagItem = 2500301,
    HonamiStoryBagTypeNoMatch = 2500302,
    HonamiStoryWeaponPluginNoUnDress = 2500303,
    HonamiStoryChangeItemPosSame = 2500304,
    HonamiStoryEquipPosError = 2500305,
    HonamiStoryEquipPosRepeat = 2500306,
    HonamiStoryOriPosErr = 2500307,
    HonamiStoryInstCanNoOpe = 2500308,
    HonamiStoryIsRewarded = 2500309,
    HonamiStoryNoFinish = 2500310,
    HonamiStoryNoConfig = 2500311,
    HonamiStoryHadActivateTalent = 2500312,
    HonamiStoryIsUnlocked = 2500313,
    HonamiStoryItemCanNoSell = 2500314,
    HonamiStoryRewardRepeat = 2500315,
    HonamiStoryInstHadFinish = 2500316,
    HonamiStoryLifeSupportFullLevel = 2500317,
    HonamiStoryDangerLevelIllegal = 2500318,
    HonamiStoryWeaponHadDress = 2500319,
    HonamiStoryItemIsLock = 2500320,
    HonamiStoryFunNoOpen = 2500321,
    HonamiStoryHadSafeLeave = 2500322,
    HonamiStoryInTop = 2500323,
    HonamiStoryAreaNoUnlock = 2500324,
    HonamiStoryPreSlotUnlock = 2500325,
    TotalTopUpNoActivityConfig = 2500326,
    TotalTopUpNoFinish = 2500327,
    TotalTopUpRewarded = 2500328,
    TotalTopUpRoleFullChain = 2500329,
    WeekCardUseRepeat = 2500330,
    WeekCardNoConfig = 2500331,
    WeekCardNoActive = 2500332,
    WeekCardCanNoReward = 2500333,
    WeekCardRewarded = 2500334,
    WeekCardNoEffect = 2500335,
    FlagChallengeTaskNoFinish = 2500336,
    FlagChallengeNoConfig = 2500337,
    FlagChallengeTaskRewarded = 2500338,
    FlagChallengeRewardRepeat = 2500339,
    FlagChallengeNoUnlock = 2500340,
    FeiXuePreheatNoFinish = 2500341,
    FeiXuePreheatNoConfig = 2500342,
    FeiXuePreheatRewarded = 2500343,
    FeiXuePreheatRewardRepeat = 2500344,
    ErrOrnamentConfig = 2500345,
    ErrOrnamentLocked = 2500346,
    ErrOrnamentDressed = 2500347,
    ErrOrnamentCanNoDress = 2500348,
    ErrOrnamentNoDress = 2500349,
    BossPilingTaskNoFinish = 2500350,
    BossPilingNoConfig = 2500351,
    BossPilingTaskRewarded = 2500352,
    BossPilingRewardRepeat = 2500353,
    BossPilingNoUnlock = 2500354,
    FlagChallengePassed = 2500355,
    BossPilingFirstNoFinish = 2500356,
    BossPilingRoleErr = 2500357,
    GachaAccumulateNoConfig = 2500358,
    GachaAccumulateNoInTime = 2500359,
    GachaAccumulateRewarded = 2500360,
    GachaAccumulateNoFinish = 2500361,
    GachaAccumulateErrSelectParam = 2500362,
    RequestParamRepeat = 2500363,
    BossPilingActivityNoOpen = 2500364,
    BossPilingErrMultiGame = 2500365,
    SheriffNoConfig = 2500366,
    SheriffActivityNoOpen = 2500367,
    SheriffFuncNotOpen = 2500368,
    SheriffClueNotUnlock = 2500369,
    SheriffEndingAlreadyFinish = 2500370,
    ErrGiftResonantChainOptionalLimit = 2500371,
    DangoBrawlNoConfig = 2500372,
    DangoBrawlErrMultiGame = 2500373,
    DangoBrawlRoleErr = 2500374,
    DangoBrawlNoComponent = 2500375,
    DangoBrawlSceneStateErr = 2500376,
    DangoBrawlCoinNotEnough = 2500377,
    DangoBrawlShopSlotErr = 2500378,
    DangoBrawlBagOpTypeErr = 2500379,
    DangoBrawlBagItemNotFound = 2500380,
    DangoBrawlBagSlotErr = 2500381,
    DangoBrawlBagSlotOccupied = 2500382,
    DangoBrawlBagOpDuplicateUid = 2500383,
    DangoBrawlComposeNoRecipe = 2500384,
    DangoBrawlSpeItemLimit = 2500385,
    DangoBrawlCostOverLimit = 2500386,
    DangoBrawlBenchFull = 2500387,
    CreateCharacterNoContext = 2500388,
    CreateCharacterStateErr = 2500389,
    CreateCharacterFlagErr = 2500390,
    ErrAlertAreaId = 2600000,
    ErrAlertAreaEnable = 2600001,
    ErrAlertAreaDisable = 2600002,
    ErrAlertUiEnable = 2600003,
    ErrAlertUiDisable = 2600004,
    ErrAlertUiVisible = 2600005,
    ErrAlertUiInvisible = 2600006,
    ErrAlertValueError = 2600007,
    ErrAlertSetAlertValueType = 2600008,
    LevelPlayReportConfigNotExist = 2600009,
    LevelPlayReportTypeError = 2600010,
    LevelPlayReportVarsEmpty = 2600011,
    LevelPlayReportPlayVarsError = 2600012,
    CameraAlertHasAlert = 2600013,
    CameraAlertHasNotAlert = 2600014,
    CameraAlertTagIdNotExist = 2600015,
    LevelPlayConfigNotExist = 2600016,
    LevelPlayRepeateInstId = 2600017,
    LevelPlayNotBelongInst = 2600018,
    LevelPlayIdsNotExist = 2600019,
    LevelPlayInstCountError = 2600020,
    LevelPlayCountError = 2600021,
    TimerHasPause = 2600022,
    TimerHasNotPause = 2600023,
    TimerHasFinish = 2600024,
    CanNotContinueInst = 2600025,
    ChapterNotExist = 2600026,
    ChapterResultHasFinish = 2600027,
    ChapterResultNotFinish = 2600028,
    ChapterResultRewardCanNotTake = 2600029,
    ChoiceHasUnlock = 2600030,
    ChoiceNotUnlock = 2600031,
    ChapterResultNotExist = 2600032,
    ChapterResultRewardHasTake = 2600033,
    ChoiceNotExist = 2600034,
    InspirationNotEnough = 2600035,
    ActivityRewardCanNotTake = 2600036,
    ActivityResultNotExist = 2600037,
    ScheduleRewardNotExist = 2600038,
    ScheduleRewardHasTake = 2600039,
    ScheduleRewardCanNotTake = 2600040,
    ActivityRewardHasTake = 2600041,
    QuestWaitConfirmResource = 2600042,
    HasGuest = 2600043,
    QuestConfigNotExist = 2600044,
    QuestDataNotExist = 2600045,
    ErrorQuestState = 2600046,
    QuestCanNotSetFocus = 2600047,
    ErrorQuestNotFocus = 2600048,
    QuestIncDicNotExist = 2600049,
    QuestFocusWaitAccept = 2600050,
    ErrQuestResourceState = 2600051,
    NoNeedDownloadQuestResource = 2600052,
    HasFinishQuestResource = 2600053,
    ErrInteracTreeSuspend = 2600054,
    InQuestFocusMode = 2600055,
    MultiModeCannotSetQuestFocus = 2600056,
    MultiModeCannotCancelQuestFocus = 2600057,
    InstanceCannotSetQuestFocus = 2600058,
    InstanceCannotCancelQuestFocus = 2600059,
    QuestFocusModeCannotAcceptQuest = 2600060,
    ErrorNotFocusWaitQuest = 2600061,
    ErrorBanInteractEntity = 2600062,
    DisabledFocusMode = 2600063,
    ErrorNotVaildGlobalSetting = 2600064,
    ErrorSameGlobalSetting = 2600065,
    ErrorNotVaildBtObjSetting = 2600066,
    ErrorSameBtObjSetting = 2600067,
    CopyUserLoginInvalidToken = 2600068,
    LoginInvalidToken = 2600069,
    AccessInvalidToken = 2600070,
    ErrNotAtomicProcessChildQuest = 2600071,
    ErrNotAtomicProcessStoveCoreFall = 2600072,
    ErrSplineEntityIdNotExist = 2600073,
    ErrSplineEntityNotExist = 2600074,
    MotorFightInstNotLevelConfig = 2600075,
    MotorFightSubLevelNotFind = 2600076,
    MotorFightNextSubLevelNotExist = 2600077,
    MotorFightErrorSubLevel = 2600078,
    MotorFightHasEnterSubLevel = 2600079,
    MotorFightCurSubLevelNotKillFinish = 2600080,
    MotorFightHasNotEnterSubLevel = 2600081,
    MotorFightCusSubLevelHasNotKillFinish = 2600082,
    MotorFightCurWaveNotFound = 2600083,
    MotorFightErrorKillParams = 2600084,
    MotorFighttExceedRefreshMonster = 2600085,
    MotorFighttExceedRefreshBoss = 2600086,
    MotorFightNotKillBoss = 2600087,
    MotorFightBossDropCollectionHasSelect = 2600088,
    MotorFightSelectPosCollectionNotExist = 2600089,
    MotorFightCollectionConfigNotExist = 2600090,
    MotorFightWaveGroupNotExist = 2600091,
    MotorFightBossMustKilled = 2600092,
    MotorFightBossNotExist = 2600093,
    MotorFightSelectBuffGateNotExist = 2600094,
    MotorFightCurSubLevelCollectionNoSelectFinish = 2600095,
    MotorFightSubLevelConfigNotFind = 2600096,
    MotorFightSubLevelIndexError = 2600097,
    MotorFightBossHasKilled = 2600098,
    MotorFightBossHasNotRefreshTimes = 2600099,
    MotorFightBossDropCollectionError = 2600100,
    MotorFightHasNotKillBossInfo = 2600101,
    MotorFightIsLastBoss = 2600102,
    MotorFightBuffGateHasSelect = 2600103,
    MotorFightSubLevelStateError = 2600104,
    MotorFightSubLevelNotFound = 2600105,
    MotorFightHasGameOver = 2600106,
    MotorFightCurWaveNotExist = 2600107,
    MotorFightCurWaveNotKillFinish = 2600108,
    SpecificInstDbDataLoading = 2600109,
    SpecificInstDbDataLoadError = 2600110,
    RecallConfigNotExist = 2600111,
    QuestNotFinish = 2600112,
    HasInRecallScene = 2600113,
    EntityConfigNotFound = 2600114,
    InstConfigNotFound = 2600115,
    FastReturnPosTypeNotSupport = 2600116,
    FastReturnPosConfigError = 2600117,
    TrackCustomBoardIsNull = 2600118,
    BtNodeConfigNotExist = 2600119,
    BtNodeObjNotExist = 2600120,
    LevelPlayNotExist = 2600121,
    LevelPlayDbDataNotExist = 2600122,
    NotInRecallInst = 2600123,
    SpecificRecallInfoNotExist = 2600124,
    ElementalTypeNotMatch = 2600125,
    ElementalMatchTypeIsEmpty = 2600126,
    ErrComponentConfigNotExist = 2600127,
    ErrHasHandCasketCore = 2600128,
    ErrNotHandCasketCore = 2600129,
    ErrFoundationHasCore = 2600130,
    ErrFoundationHasNotCore = 2600131,
    ErrFoundationInteractCondNotMatch = 2600132,
    ErrElementalTypeNotMatch = 2600133,
    ErrElementalHolderNotEnable = 2600134,
    ElementalBoxNotInFoundation = 2600135,
    ErrElementalBoxConfigNotExist = 2600136,
    ErrMultigameQuestRecall = 2600137,
    ErrActionGetItemHasNotFreeSize = 2600138,
    IdCreateRuleChange2 = 2700001,
    RacingBetsActivityIdErr = 2700002,
    RacingBetsActivityDataErr = 2700003,
    RacingBetsRewardConfErr = 2700004,
    RacingBetsTaskIdNotExist = 2700005,
    RacingBetsTaskRewarded = 2700006,
    RacingBetsTaskUndone = 2700007,
    RacingBetsTaskRewardFail = 2700008,
    RacingBetsSeasonConfErr = 2700009,
    RacingBetsLegMatchIdNotExist = 2700010,
    RacingBetsOddsTimesErr = 2700011,
    RacingBetsOddsVersionErr = 2700012,
    RacingBetsOddsVersionCodeErr = 2700013,
    RacingBetsOddsConfErr = 2700014,
    RacingBetsOddsDangoErr = 2700015,
    RacingBetsOddsDangoConfErr = 2700016,
    RacingBetsOddsTimeErr = 2700017,
    RacingBetsOddsRetry = 2700018,
    RacingBetsCostFundsErr = 2700019,
    RacingBetsGearTimesLimit = 2700020,
    RacingBetsGearInfoErr = 2700021,
    RacingBetsGearRefundParamErr = 2700022,
    RacingBetsGearRefundFail = 2700023,
    RacingBetsGearNotRefund = 2700024,
    RacingBetsActivityConfErr = 2700025,
    RacingBetsOddsFundsLack = 2700026,
    RacingBetsOddsLack = 2700027,
    RacingBetsGroupNotExist = 2700028,
    RacingBetsGroupConfNotExist = 2700029,
    RacingBetsGroupConfError = 2700030,
    RacingBetsNotInInst = 2700031,
    RacingBetsFundsNotEnough = 2700032,
    RacingBetsFundsCalcErr = 2700033,
    RacingBetsLegMatchNotOpen = 2700034,
    RacingBetsDangoActionTypeErr = 2700035,
    RacingBetsDangoMatchParamErr = 2700036,
    RacingBetsInstSubTypeErr = 2700037,
    RacingBetsEntityErr = 2700038,
    RacingBetsLegMatchIdErr = 2700039,
    RacingBetsBulletScreenTableNotExist = 2700040,
    RacingBetsBulletScreenIdErr = 2700041,
    RacingBetsBulletActionIndexNotExist = 2700042,
    RacingBetsBulletScreenLegMatchNotExist = 2700043,
    RacingBetsBulletNotFundOpenRankCurTime = 2700044,
    RacingBetsBulletGetRankErr = 2700045,
    RacingBetsBulletCd = 2700046,
    ActivityLinkageActivityIdConfErr = 2700047,
    ActivityLinkageConfErr = 2700048,
    ActivityLinkageRewardErr = 2700049,
    ActivityLinkageDataErr = 2700050,
    ActivityLinkageConfIndexErr = 2700051,
    ActivityLinkageRewardStatusErr = 2700052,
    ActivityLinkagePageTimeErr = 2700053,
    RacingBetsdynamicOddsConfErr = 2700054,
    RacingBetsdynamicOddsProportionErr = 2700055,
    RacingBetsDangoDataNotExist = 2700056,
    RacingBetsMatchDataErr = 2700057,
    RacingBetsMatchRoundTimeErr = 2700058,
    RacingBetsNotGear = 2700059,
    RacingBetsLegSettle = 2700060,
    RacingBetsRankNotOpen = 2700061,
    RacingBetsActionIndexErr = 2700062,
    RacingBetsMatchRoundNotFund = 2700063,
    RacingBetsDangoConfNotExist = 2700064,
    RacingBetsDangoTemplateConfNotExist = 2700065,
    RacingBetsDangoEntityCreateFail = 2700066,
    RacingBetsLegMatchConfNotExist = 2700067,
    PhantomBattleCardUnlocked = 2700068,
    PhantomBattleActDataNotExist = 2700069,
    RacingBetsLegNextOddsErr = 2700070,
    RacingBetsRankNotExist = 2700071,
    RacingBetsPlayerNotExistInRank = 2700072,
    PhantomBattleCardTableNotExist = 2700073,
    PhantomBattleCardNumLimit = 2700074,
    PhantomBattleActTableNotExist = 2700075,
    PhantomBattleCardGroupLimit = 2700076,
    PhantomBattleGroupLimit = 2700077,
    PhantomBattleMainCostLimit = 2700078,
    PhantomBattleCardLocked = 2700079,
    PhantomBattleGroupNameLimit = 2700080,
    PhantomBattleGroupNotExist = 2700081,
    PhantomBattleRomoveReason = 2700082,
    RacingBetsBulletIdErr = 2700083,
    RacingBetsPlayerDataNotExist = 2700084,
    RacingBetsTaskDataNotExist = 2700085,
    PhantomBattleGroupElementLimit = 2700086,
    PhantomBattleOutLookUped = 2700087,
    PhantomBattleTaskConfNotExist = 2700088,
    PhantomBattleConditionDataErr = 2700089,
    PhantomBattleConditionConfNotExist = 2700090,
    PhantomBattleConditionGroupConfNotExist = 2700091,
    PhantomBattleTaskStutas = 2700092,
    PhantomBattleTaskReward = 2700093,
    PhantomBattleTaskNotFinish = 2700094,
    PhantomBattleTaskProgressErr = 2700095,
    PhantomBattleBadgeUnlocked = 2700096,
    PhantomBattleLevelConfNotExist = 2700097,
    PhantomBattBadgeRewardConfNotExist = 2700098,
    PhantomBattCardRewardConfNotExist = 2700099,
    PhantomBattBadgeUnlockNotEnough = 2700100,
    PhantomBattBadgeConfErr = 2700101,
    PhantomBattBadgeRewardErr = 2700102,
    PhantomBattleLevelRewardConfNotExist = 2700103,
    PhantomBattCardUnlockNotEnough = 2700104,
    PhantomBattCardRewardConfErr = 2700105,
    PhantomBattLvNotEnough = 2700106,
    PhantomBattLvRewardConfErr = 2700107,
    PhantomBattCardRewardErr = 2700108,
    PhantomBattLvRewardErr = 2700109,
    PhantomBattLvRewardConfNotExist = 2700110,
    PhantomBattRoleLock = 2700111,
    PhantomBattRoleReward = 2700112,
    PhantomBattRoleRewardErr = 2700113,
    PhantomBattPassRewardErr = 2700114,
    PhantomBattFirstPassRewardErr = 2700115,
    PhantomBattleChallengeConfNotFind = 2700116,
    PhantomBattleChallengeOpenLimit = 2700117,
    PhantomBattleParamLimit = 2700118,
    PhantomBattleParamRepeat = 2700119,
    PhantomBattleFuncOpenChallengeRepeat = 2700120,
    PhantomBattleFuncOpenCardGroup = 2700121,
    PhantomBattleFuncOpenCard = 2700122,
    PhantomBattleFuncOpenCardOutlookUp = 2700123,
    PhantomBattleCardNotAllowBuy = 2700124,
    FloroRanchHasUnSettleIns = 2700125,
    FloroRanchSubInsConfNotExist = 2700126,
    FloroRanchRacesLimit = 2700127,
    FloroRanchBaseTerrainLimit = 2700128,
    FloroRanchBaseCardLimit = 2700129,
    FloroRanchRaceRepeat = 2700130,
    FloroRanchGamePlayNotExist = 2700131,
    FloroRanchGamePlayGachaNotExist = 2700132,
    FloroRanchCardNotExistInGacha = 2700133,
    FloroRanchPhantomCreateFail = 2700134,
    FloroRanchTributeCoinLimit = 2700135,
    FloroRanchRaceNotEnough = 2700136,
    FloroRanchStageNotSettle = 2700137,
    FloroRanchActivitysNotFind = 2700138,
    FloroRanchGamePlayShopNotExist = 2700139,
    FloroRanchGamePlayShopItemNotExist = 2700140,
    FloroRanchDiamondNotEnough = 2700141,
    FloroRanchCardTableNotExist = 2700142,
    FloroRanchToyTableNotExist = 2700143,
    FloroRanchTerrainTableNotExist = 2700144,
    FloroRanchToyNumLimit = 2700145,
    FloroRanchNotEnoughToyIndex = 2700146,
    FloroRanchToyCreateFail = 2700147,
    FloroRanchCardGroupTableNotExist = 2700148,
    FloroRanchCardGroupNotExist = 2700149,
    FloroRanchBuffTableNotExist = 2700150,
    FloroRanchRemoveUnitNotExist = 2700151,
    PhantomBattleCardMaxLimit = 2700152,
    FloroRanchRefreshTableNotExist = 2700153,
    FloroRanchPlayTaskNotExist = 2700154,
    FloroRanchSkillTableNotExist = 2700155,
    FloroRanchSkillUseTimesLimit = 2700156,
    FloroRanchSkillLogicNotExist = 2700157,
    FloroRanchSkillLogicErr = 2700158,
    FloroRanchSkillLogicFail = 2700159,
    PhantomBattleNameLimitConfErr = 2700160,
    FloroRanchNotEnableUnlimitedMode = 2700161,
    PhantomBattleLimitTime = 2700162,
    FloroRanchToyWeightConfErr = 2700163,
    FloroRanchAnimalNumLimit = 2700164,
    FloroRanchRaceTableNotFind = 2700165,
    FloroRanchRaceLock = 2700166,
    FloroRanchSkillLock = 2700167,
    FloroRanchSubInsLock = 2700168,
    RoleSelfBgmUnableOperate = 2700169,
    RoleSelfBgmFlyTableNotExist = 2700170,
    RoleSelfBgmUnableOp = 2700171,
    FloroRanchInsLock = 2700172,
    FloroRanchInsConfNotFind = 2700173,
    FloroRanchInsIdNotFind = 2700174,
    FloroRanchInsRaceNotInReq = 2700175,
    FloroRanchUnEnableTribute = 2700176,
    FloroRanchIsMulti = 2700177,
    ButtonLockParamLimit = 2700178,
    ButtonLockParamRepeat = 2700179,
    PhantomBattlePlayerCardGroupBuildErr = 2700180,
    PhantomBattleNpcCardGroupBuildErr = 2700181,
    PhantomBattleNpcRecordLoadFail = 2700182,
    PhantomBattleCardTypeErr = 2700183,
    PhantomBattleActTypeErr = 2700184,
    PhantomNoDurableSkill = 2700185,
    PhantomBattleAreaCardNumLimit = 2700186,
    PhantomBattleItemCardNumLimit = 2700187,
    PhantomBattleCallTaskNumErr = 2700188,
    PhantomBattleNotCanSoltIndex = 2700189,
    PhantomBattleSoltMaxLimit = 2700190,
    PhantomBattleTaskIdErr = 2700191,
    PhantomBattleActIdErr = 2700192,
    PhantomBattleConditionParamErr = 2700193,
    PhantomBattleAreaCardConfigErr = 2700194,
    PhantomBattleAreaLock = 2700195,
    PhantomBattleGuideActTableNotExist = 2700196,
    NewPlayerSupportTaskConfNotExist = 2700197,
    NewPlayerSupportTaskRewardIndex = 2700198,
    NewPlayerSupportTaskParamLimit = 2700199,
    NewPlayerSupportTaskParamRepeat = 2700200,
    NewPlayerSupportTaskReward = 2700201,
    NewPlayerSupportTaskNotFinish = 2700202,
    NewPlayerSupportDataNotEixst = 2700203,
    NewPlayerSupportNotFindTrialRole = 2700204,
    NewPlayerSupportRoleLock = 2700205,
    NewPlayerSupportWorldLvErr = 2700206,
    NewPlayerSupportRolePageErr = 2700207,
    NewTrialRoleActivityRoleCheckErr = 2700208,
    NewTrialRoleActivityInsNotFind = 2700209,
    NewTrialRoleActivityInsUnableUse = 2700210,
    NewTrialRoleActivityLogicNotFind = 2700211,
    NewTrialRoleActivityDelegateNotFind = 2700212,
    RegressNotFindTrialRole = 2700213,
    RegressNotFindActivityTrialRole = 2700214,
    RegressDisposableReward = 2700215,
    RegressDisposableRewardConfigNotFind = 2700216,
    RegressPayBonusLimit = 2700217,
    GachaRoleDevelopInsNotOpen = 2700218,
    PhantomBattleNoComponent = 2700219,
    PhantomBattleUnableSkip = 2700220,
    RegressMaxBonusItemNum = 2700221,
    RegressBonusRewardNoConfig = 2700222,
    RegressBonusBuyLvLimit = 2700223,
    PhantomBattleCallCardLimt = 2700224,
    PhantomBattleCallItemLimt = 2700225,
    GachaRoleDevelopInsLimit = 2700226,
    GachaRolesilentAreaIdNotFind = 2700227,
    GachaRolesilentAreaUnlock = 2700228,
    NewTrialRoleActivityNotFindTrialRoleConf = 2700229,
    PhantomBattleSkipErr = 2700230,
    NewTrialRoleGachaRoleDevelopNotFind = 2700231,
    RegressTrialRoleLock = 2700232,
    NewPlayerSupportTrialRoleLock = 2700233,
    PhantomBattleCopyCardLimit = 2700234,
    RegressTrialNoCanRewardTask = 2700235,
    NewPlayerSupportNoCanRewardTask = 2700236,
    FarmGoldPointReqLimit = 2700237,
    FarmGoldPointReqErr = 2700238,
    FarmGoldPointReqRe = 2700239,
    FarmGoldLevelReqLimit = 2700240,
    FarmGoldLevelReqErr = 2700241,
    FarmGoldLevelReqRe = 2700242,
    FarmGoldPointRewardDropErr = 2700243,
    FarmGoldLevelRewardDropErr = 2700244,
    NewTrialRoleInCurTeam = 2700245,
    RegressEndTimeErr = 2700246,
    NewTrialRoleMatchingUnableTrial = 2700247,
    PhBaFindCd = 2700248,
    PhBaTargetPlanNotExist = 2700249,
    PhBaPlanUploadErr = 2700250,
    PhBaPlanErr = 2700251,
    PhBaSaveCd = 2700252,
    PhBaAttrNotInCost = 2700253,
    PhBaCostMainPropNotExist = 2700254,
    PhBaFetterGroupLimit = 2700255,
    PhBaAttrTypeLimit = 2700256,
    PhBaCostTypeNumLimit = 2700257,
    PhBaCodeLimit = 2700258,
    PhBaCodeIllegal = 2700259,
    PhBaCurUsePlanNotExist = 2700260,
    PhBaTargetSuitNotExist = 2700261,
    PhBaPlanFuncNotOpen = 2700262,
    InsUnableMultiEnter = 2700263,
    PhBaPlanSuitCountErr = 2700264,
    PhBaPlanTargetSuitNotExist = 2700265,
    NewTrialRoleNumLimit = 2700266,
    PhBaPlanCostTypeRepeat = 2700267,
    PhBaPlanSuitIdRepeat = 2700268,
    RacingBetsNotFindOrderListDango = 2700269,
    PhBaPlanBatchOperEnumErr = 2700270,
    PhBaPlanRemotePlanNotExist = 2700271,
    H5ViewActivityNotFindDb = 2700272,
    ForoRanchWeekInsNotFind = 2700273,
    PhBaPlanCodeCreateErr = 2700274,
    RacingBetsOddsDangoIllegal = 2700275,
    ForoRanchNotCurEvent = 2700276,
    ForoRanchEventIncIdErr = 2700277,
    ForoRanchEventChoiceErr = 2700278,
    ForoRanchEventSubChoicesConfNotExist = 2700279,
    ForoRanchEventChoicesConfNotExist = 2700280,
    ForoRanchSubInsIdNoActivityId = 2700281,
    ForoRanchGetSubWeekInsErr = 2700282,
    ForoRanchSubInsNotOpen = 2700283,
    ForoRanchVoidToyNotEnough = 2700284,
    ForoRanchToyNotEnough = 2700285,
    RoverRogueInsNotExist = 2700286,
    RoverRogueCompCreateFail = 2700287,
    RoverRogueNotFindRoomConf = 2700288,
    RoverRogueNotFindBornEntityId = 2700289,
    RoverRogueLootLock = 2700290,
    RoverRogueComponentNotFind = 2700291,
    RoverRogueGetNextRoomFail = 2700292,
    RoverRogueExitNumLess = 2700293,
    RoverRogueExitEntityConfNotFind = 2700294,
    RoverRogueCreateExitEntityFail = 2700295,
    RoverRogueNotFindRoomTypeConf = 2700296,
    RoverRoguePortalEntityNotFind = 2700297,
    RoverRogueNotFindEntityLogic = 2700298,
    RoverRogueInsCtxNotExist = 2700299,
    RoverRogueRoleConfNotExist = 2700300,
    RoverRogueGainDataTypeErr = 2700301,
    RoverRogueGainActiveErr = 2700302,
    RoverRogueRewardDbNotExist = 2700303,
    RoverRogueGainSelectNotExist = 2700304,
    RoverRogueGainTagChangeErr = 2700305,
    RoverRogueGainLogicNotFind = 2700306,
    RoverRogueGainLogicErr = 2700307,
    ForoRanchWeeklyCanNotMilestone = 2700308,
    RoverRogueRewardDbToken = 2700309,
    RoverRogueRoleTypeConfNotExist = 2700310,
    RoverRogueRoomRewardNotInit = 2700311,
    RoverRogueRoomTaskNotExist = 2700312,
    RoverRogueRoomBlessGroupTaskNotExist = 2700313,
    RoverRogueRoomTaskTypeErr = 2700314,
    RoverRogueRoomTaskParamErr = 2700315,
    RoverRogueRoomTaskIsToken = 2700316,
    RoverRogueNotCompletedInsErr = 2700317,
    RoverRogueNotFindRoomRewardConf = 2700318,
    RoverRogueRoomNotFinish = 2700319,
    RoverRogueRoomRewardNotActivate = 2700320,
    RoverRogueRoomRewardCache = 2700321,
    RoverRogueRoleListErr = 2700322,
    RoverRogueRoleTypeErr = 2700323,
    RoverRogueNotFindActConf = 2700324,
    RoverRogueLootNumLimit = 2700325,
    RoverRogueNotFindBlessRoleConf = 2700326,
    RoverRogueNotFindBollToPortal = 2700327,
    RoverRogueNotFindBollNotExist = 2700328,
    RoverRogueNotFindRoomRoadConf = 2700329,
    RoverRogueNotFindChaosConf = 2700330,
    RoverRogueChaosExitNotEnough = 2700331,
    RoverRoguePosNotFind = 2700332,
    RoverRogueChangeSameLoot = 2700333,
    GateUdpPortNotExist = 2800000,
    GateKcpGetConvFail = 2800001,
    ErrScreenActionExecutorNotFind = 2900000,
    ErrScreenActionInfoNotFind = 2900001,
    ErrScreenActionTypeNotMatch = 2900002,
    ErrScreenActionNotFadeInScreen = 2900003,
    ErrScreenActionNotFadeOutScreen = 2900004,
    ErrSceneItemSequenceFrameRegister = 2900005,
    ErrSceneItemSequenceFrameComponentConfig = 2900006,
    ErrSceneItemSequenceFrameComponent = 2900007,
    ErrSceneItemSequenceFrame = 2900008,
    ErrSceneItemSequenceFrameMatchEventCbType = 2900009,
    ErrSceneItemSequenceFrameNeedServerAction = 2900010,
    ErrSceneItemSequenceFrameUnAnsEvent = 2900011,
    ErrSceneBlockSplitNotBlock = 2900027,
    ErrSunSpiritCollectOverNum = 2900012,
    ErrSunSpiritInteractAction = 2900013,
    ErrSunSpiritGetComponentConfig = 2900014,
    ErrSunSpiritEntityRemove = 2900015,
    ErrSunSpiritRepeateAdd = 2900016,
    ErrSunSpiritNumUnEnough = 2900017,
    ErrSunSpiritEnableEntityHistoryUse = 2900018,
    ErrSunSpiritGetPlayerTempOccupySunSpiritInfo = 2900019,
    ErrSunSpiritActionExecutorNotFind = 2900020,
    ErrSunSpiritActionInfoNotFind = 2900021,
    ErrSunSpiritActionIndexNotMatch = 2900022,
    ErrSunSpiritActionTypeNotMatch = 2900023,
    ErrSunSpiritActionParamOperationType = 2900024,
    ErrSunSpiritAreaConfig = 2900025,
    ErrSunSpiritNotHostPlayer = 2900026,
    ErrResourcePackageSdkCheck = 2900028,
    ErrResourcePackageFuncClose = 2900029,
    ErrResourcePackageRedisWrite = 2900030,
    ErrResourcePackageInvalidToken = 2900031,
    ErrResourcePackageRateLimiterCircuit = 2900032,
    ErrResourcePackageRateLimiterRejected = 2900033,
    ErrResourcePackageException = 2900034,
    ErrResourcePackageTimeoutRejected = 2900035,
    ErrAvoidBlockGetInstComponent = 2900036,
    ErrLoadCalabashDefault = 3000000,
    CalabashSkinUnLockErr = 3000001,
    RepeqtedRequest = 3000002,
    ErrSynthesisBatchItemNotSupport = 3000003,
    ErrSynthesisBatchItemDeadLoop = 3000004,
    ErrSynthesisBatchItemCntNotMatch = 3000005,
    ErrSynthesisBatchItemReapeat = 3000006,
    ErrBatchSynthesisCoinOverflow = 3000007,
    ErrPhantomSkinNotExist = 3000008,
    ErrPhantomSkinNotUnlock = 3000009,
    ErrPhantomInteractionNotUnlock = 3000010,
    ErrIllustratedNotUnlock = 3000011,
    ErrPhantomInteractionEquipCountNotMatch = 3000012,
    ErrPhantomInteractionConfigNotFind = 3000013,
    ErrPhantomSkinRepeatOperation = 3000044,
    ErrPhantomInteractionNotShowable = 3000045,
    ErrArtemisNodeNoConfig = 3000014,
    ErrArtemisActivityNotOpen = 3000015,
    ErrArtemisNodeNoData = 3000016,
    ErrArtemisNodeNoUnlock = 3000017,
    ErrArtemisNodeNotRewardStatus = 3000018,
    ErrArtemisNodeAlreadyFixed = 3000019,
    ErrArtemisNodeAlreadyFinished = 3000020,
    ErrArtemisState = 3000021,
    ErrArtemisInvalidUpdate = 3000022,
    ErrIndexOutOfRange = 3000023,
    ErrArtemisActivityNotExist = 3000024,
    MotorParkourLevelNoOpen = 3000025,
    MotorParkourNoConfig = 3000026,
    MotorParkourInstComponentNotExist = 3000027,
    MotorParkourGetTimeRecordFailed = 3000028,
    ErrMotorParkourLevelRewardIndexLengthError = 3000029,
    ErrMotorParkourActivityDataNotExist = 3000030,
    ErrMotorParkourLevelRewardIndexInvalid = 3000031,
    ErrMotorParkourRecordCfgNotExist = 3000032,
    ErrMotorParkourGetRewardStatusFail = 3000033,
    ErrMotorParkourRewardNotAvailable = 3000034,
    ErrMotorParkourLevelRewardIndexDuplicate = 3000035,
    ErrMotorParkourRewardCfgNotExist = 3000043,
    ErrMotorParkourLevelInfoFailed = 3000046,
    ErrMotorParkourPreLevelNotComplete = 3000073,
    ErrAreaTerminalPinFull = 3000036,
    ErrAreaTerminalPinExist = 3000037,
    ErrAreaTerminalPinNotExist = 3000038,
    ErrAreaTerminalParamError = 3000039,
    ErrActivityLock = 3000040,
    ErrFuncLock = 3000041,
    ErrNoAreaTerminalCfg = 3000042,
    ErrSpringFestivalActivityConfigNotFound = 3000068,
    ErrSpringFestivalActivityDataNotFound = 3000069,
    ErrSpringFestivalComponentNotFound = 3000070,
    ErrSpringFestivalRuntimeInfoNotFound = 3000071,
    ErrMultiModeCanNotRequest = 3000083,
    ErrSpringFestivalActivityNotOpen = 3000084,
    ErrSpringFestivalFuncNotExist = 3000100,
    ErrSpringFestivalFuncLock = 3000101,
    ErrSpringFestivalActivityIdNotMatch = 3000103,
    ErrFurnitureDiyActivityConfigNotFound = 3000047,
    ErrFurnitureAreaIdNotExistInActivity = 3000048,
    ErrFurnitureAreaConfigNotFound = 3000049,
    ErrFurnitureDiyActivityDataNotFound = 3000050,
    ErrFurnitureSlotConfigNotExist = 3000051,
    ErrFurnitureConfigNotExist = 3000052,
    ErrFurnitureNotAvailable = 3000053,
    ErrFurnitureAreaDataNotFound = 3000054,
    ErrSpecificFurnitureCount = 3000055,
    ErrFurnitureSlotNotSupportChildTag = 3000056,
    ErrFurnitureSlotNotSupportFurniture = 3000057,
    ErrFurnitureDiyComponentNotFound = 3000058,
    ErrNotFurnitureSlotConfig = 3000059,
    ErrFurnitureSlotConfigSlotCountError = 3000060,
    ErrFurnitureSlotSubFurnitureCount = 3000061,
    ErrFurnitureEntityConfigNotFound = 3000062,
    ErrEntityCreateFailed = 3000063,
    ErrFurnitureUnloadSubFurnitureNotEmpty = 3000064,
    ErrEntitiesNull = 3000065,
    ErrConvertShortIdToFurnitureIdFailed = 3000066,
    ErrFurnitureSlotDuplicateInRequest = 3000067,
    ErrFurnitureSlotNotBelongToArea = 3000106,
    ErrSpringFestivalFurnitureHadUnlock = 3000117,
    ErrExploreDegreeTypeNotFound = 3000072,
    ErrActivityBrochureCfgNotFound = 3000074,
    ErrBrochureRewardRequestEmptyBookItems = 3000075,
    ErrBrochureCfgNotFound = 3000076,
    ErrBrochureNotBelongToActivity = 3000077,
    ErrBrochureNotUnlock = 3000078,
    ErrBrochureRewardRequestDuplicateBookItem = 3000079,
    ErrBookItemCfgNotFound = 3000080,
    ErrBookItemNotFinish = 3000081,
    ErrBookItemAlreadyRewarded = 3000082,
    ErrBrochureRewardTimeInvalid = 3000091,
    ErrBookItemNotBelongToBrochure = 3000104,
    ErrExhibitionComponentCfgNotExist = 3000085,
    ErrItemTypeNotMatchExhibitType = 3000086,
    ErrExhibitConfigTypeInvalid = 3000087,
    ErrExhibitionComponentNotExist = 3000088,
    ErrExhibitionPhantomNotExist = 3000089,
    ErrRepeatedEntity = 3000092,
    ErrPhonographMusicNotExist = 3000093,
    ErrMusicNotUnlocked = 3000102,
    ErrInstanceConfigNotFound = 3000090,
    ErrInvalidEnterContextCase = 3000094,
    ErrTetrisLevelInfoDuplicate = 3000095,
    ErrTetrisActivityDataNotFound = 3000096,
    ErrTetrisLevelConfigNotExist = 3000097,
    ErrTetrisLevelRewardDuplicate = 3000098,
    ErrTetrisLevelStateInvalid = 3000099,
    ErrActivityNotMatchTetrisLevel = 3000118,
    ErrTetrisTargetScoreIdNotExist = 3000119,
    ErrTetrisLevelHasRewarded = 3000120,
    ErrTetrisLevelNotFinished = 3000121,
    ErrTetrisLevelDifficultyIndexInvalid = 3000126,
    ErrTetrisTargetScoreCountNotMatch = 3000127,
    ErrTetrisScoreInvalid = 3000128,
    ErrGetVarDefine = 3000129,
    ErrDataPersistenceTetrisBoardGame = 3000130,
    ErrTetrisTargetScoreNotEnough = 3000131,
    ErrTetrisConfigInvalid = 3000132,
    ErrTetrisPreLevelNotFinished = 3000137,
    WlfTest = 3000105,
    DropCatchNoConfig = 3000107,
    DropCatchLevelNotOpen = 3000108,
    ErrDropCatchLevelRewardIndexLengthError = 3000109,
    ErrDropCatchActivityDataNotExist = 3000110,
    ErrDropCatchLevelRewardIndexDuplicate = 3000111,
    ErrDropCatchLevelRewardIndexInvalid = 3000112,
    ErrDropCatchRewardCfgNotExist = 3000113,
    ErrDropCatchRewardNotAvailable = 3000114,
    ErrDropCatchLevelInfoFailed = 3000115,
    ErrDropCatchLevelScoreInvalid = 3000116,
    ErrDropCatchEntityComponentNotExist = 3000122,
    ErrDropCatchComponentNotExist = 3000123,
    ErrDropCatchRewardCfgNotActivity = 3000124,
    ErrDropCatchDataNotExist = 3000125,
    ErrGetSelfXboxOnlineId = 3000133,
    ErrXboxPlayerInfoRequestLimit = 3000134,
    ErrGetXboxUserPlayerErr = 3000135,
    ErrXboxAccountBlocked = 3000136,
    InfrV2LevelCfgNotFound = 3000138,
    InfrV2ShopPhaseCfgNotFound = 3000139,
    InfrV2ShopGoodsCfgNotFound = 3000140,
    InfrV2FireAddExpNegative = 3000141,
    InfrV2FireAlreadyMaxLevel = 3000142,
    InfrV2FireConditionNotMet = 3000143,
    InfrV2FireExpNotEnough = 3000144,
    InfrV2TreeNotInProgress = 3000145,
    InfrV2FireLevelNotInProgress = 3000146,
    InfrV2TreeConditionNotMet = 3000147,
    InfrV2NotOpen = 3000148,
    InfrV2TreeBuildCfgNotFound = 3000149,
    InfrV2TreeCannotTrace = 3000150,
    InfrV2TreeSwitchTraceInMulti = 3000151,
    InfrV2TaskFindErr = 3000152,
    InfrV2HadReward = 3000153,
    InfrV2TaskRunning = 3000154,
    InfrV2DropNoFind = 3000155,
    InfrV2ParamAgainErr = 3000156,
    InfrV2NoReward = 3000157,
    InfrV2ParamCountErr = 3000158,
    SlashAndTowerParamErr = 3000159,
    RogueWeeklyParamErr = 3000160,
    ErrMatchRoleChangeSkillBranchCountNotEqualRoleCount = 3000161,
    ErrActivityPreQuestNotFinished = 3000162,
    ErrInstNotEdgeRunnerLordGym = 3000163,
    ErrEdgeRunnerLordGymConfigNotExist = 3000164,
    ErrEdgeRunnerDbDataNotExist = 3000165,
    ErrEdgeRunnerPreLordGymNotPassed = 3000166,
    EdgeRunnerParamCountErr = 3000167,
    EdgeRunnerDropNoFind = 3000168,
    EdgeRunnerNoReward = 3000169,
    EdgeRunnerTaskFindErr = 3000170,
    EdgeRunnerActivityIdErr = 3000171,
    EdgeRunnerHadReward = 3000172,
    EdgeRunnerTaskRunning = 3000173,
    EdgeRunnerParamAgainErr = 3000174,
    ErrEdgeRunnerActivityConfigNotFound = 3000175,
    ErrEdgeRunnerActivityFuncUnlockConfigNotFound = 3000176,
    ErrEdgeRunnerActivityFuncAlreadyUnlock = 3000177,
    ErrEdgeRunnerActivityFuncUnlockConditionNotReached = 3000178,
    ErrEdgeRunnerActivityFuncUnlockFailed = 3000179,
    ErrEdgeRunnerLordGymLevelVarNotExist = 3000187,
    EdgeRunnerLordGymRepeatChallenge = 3000188,
    ErrEdgeRunnerLordGymBtTreeNotFound = 3000189,
    ErrEdgeRunnerLordGymNotInPlay = 3000190,
    ErrGolemCrackLevelConfigNotExist = 3000180,
    ErrActivityNotMatchGolemCrackLevel = 3000181,
    ErrGolemCrackActivityDataNotFound = 3000182,
    ErrGolemCrackPreLevelNotFinished = 3000183,
    ErrGolemCrackLevelRewardDuplicate = 3000184,
    ErrGolemCrackPreLevelLocked = 3000186,
    ErrPhantomRecommendConfigNotFound = 3000185,
    ErrXboxGiftNotEnable = 3000191,
    ErrXboxGiftDbError = 3000192,
    ErrXboxGiftAlreadyClaimed = 3000193,
    ErrXboxGiftNotXboxUser = 3000194,
    ErrXboxGiftRequestErr = 3000195,
    ErrXboxGiftRequestDataErr = 3000196,
    ErrXboxGiftNotFound = 3000197,
    ErrXboxGiftException = 3000198,
    ErrXboxGiftHttpError = 3000199,
    ErrXboxGiftXstsTokenInvalid = 3000201,
    ErrXboxGiftXstsTokenMismatch = 3000202,
    DangoBrawlActivityNotExist = 3000200,
    DangoBrawlParamCountErr = 3000218,
    DangoBrawlTaskFindErr = 3000219,
    DangoBrawlActivityIdErr = 3000220,
    DangoBrawlHadReward = 3000221,
    DangoBrawlTaskRunning = 3000222,
    DangoBrawlDropNoFind = 3000223,
    DangoBrawlParamAgainErr = 3000224,
    DangoBrawlNoReward = 3000225,
    DangoBrawlPreChallengeNotPass = 3000226,
    DangoBrawlWinError = 3000227,
    QingXiaoParamCountErr = 3000203,
    QingXiaoTaskFindErr = 3000204,
    QingXiaoActivityIdErr = 3000205,
    QingXiaoHadReward = 3000206,
    QingXiaoTaskRunning = 3000207,
    QingXiaoDropNoFind = 3000208,
    QingXiaoParamAgainErr = 3000209,
    QingXiaoNoReward = 3000210,
    ErrVersionSignInNotOpen = 3000211,
    ErrVersionSignInNoData = 3000212,
    ErrVersionSignInRewardNotExist = 3000213,
    ErrVersionSignInRewardAlreadyTaken = 3000214,
    ErrVersionSignInDaysNotEnough = 3000215,
    ErrVersionSignInParamError = 3000216,
    ErrTimePointRewardActivityConditionNotFinish = 3000217,
    MaterialReplaceTargetMismatch = 3000228,
    MaterialReplaceNoExchangeParamConfig = 3000229,
    ErrMotorParkourNotFound = 3000230,
    ErrTetrisLevelDropIdNotExist = 3000231,
    PhotoFightLevelNotFound = 3100000,
    PhotoFightActivityNotOpen = 3100001,
    PhotoFightSceneComponentInfoLost = 3100002,
    PhotoFightGroupNotFound = 3100003,
    PhotoFightLevelNotOpen = 3100004,
    PhotoFightPreInstNotCleared = 3100005,
    PhotoFightSceneComponentLost = 3100006,
    PhotoFightTargetStatusHigher = 3100007,
    PhotoFightNoTargetRole = 3100008,
    PhotoFightRoleInvalid = 3100009,
    PhotoFightRewardCfgNotFound = 3100010,
    PhotoFightRewardInvalidInput = 3100011,
    PhotoFightAlreadyRewarded = 3100012,
    PhotoFightCannotRewarded = 3100081,
    PhotoFightInvalidInstType = 3100306,
    WuWuKujiQuestNotFound = 3100013,
    WuWuKujiQuestNotOpenDay = 3100014,
    WuWuKujiQuestAllFinished = 3100015,
    WuWuKujiActivityNotOpen = 3100016,
    WuWuKujiPreGuideNotFinished = 3100017,
    WuWuKujiPreQuestNotFinished = 3100018,
    WuWuKujiActivityConfigNotFound = 3100019,
    WuWuKujiTrickTypeInvalid = 3100020,
    WuWuKujiNoAvailableTrick = 3100021,
    WuWuKujiAwardGroupCfgNotFound = 3100022,
    WuWuKujiNoAvailableGroup = 3100023,
    WuWuKujiGroupRewardTimesExceededMax = 3100024,
    WuWuKujiGroupRemainTimesError = 3100025,
    WeatherCtlInvalidInMulti = 3100026,
    WeatherCtlMapDataNotFound = 3100027,
    WeatherCtlSwitchCfgNotFound = 3100028,
    WeatherCtlSwitchLocked = 3100029,
    WeatherCtlAreaWeatherLocked = 3100030,
    WeatherCtlAreaDateLocked = 3100031,
    WeatherCtlAddHourNoNegative = 3100032,
    WeatherCtlInMulti = 3100052,
    WeatherCtlInBattle = 3100053,
    WeatherCtlInQuest = 3100054,
    WeatherCtlNotInBigWorld = 3100082,
    WeatherCtlNotOpen = 3100162,
    InfrArchiveTaskCfgNotFound = 3100033,
    InfrLevelCfgNotFound = 3100034,
    InfrShopPhaseCfgNotFound = 3100035,
    InfrShopGoodsCfgNotFound = 3100036,
    InfrFireAddExpNegative = 3100037,
    InfrFireAlreadyMaxLevel = 3100038,
    InfrFireConditionNotMet = 3100039,
    InfrRoadBuildCfgNotFound = 3100040,
    InfrRoadCannotTrace = 3100041,
    InfrFireShopGoodsNotFound = 3100042,
    InfrFireShopPhaseNotUnlocked = 3100043,
    InfrFireShopBuyLimitExceeded = 3100044,
    InfrFireShopBuyTimesInvalid = 3100045,
    InfrArchiveTaskInvalidInput = 3100046,
    InfrArchiveTaskCannotRewarded = 3100047,
    InfrFireExpNotEnough = 3100048,
    InfrRoadNotInProgress = 3100049,
    InfrFireLevelNotInProgress = 3100050,
    InfrRoadConditionNotMet = 3100051,
    InfrNotOpen = 3100073,
    InfrPhoneTaskInvalidInput = 3100083,
    InfrPhoneTaskCannotRewarded = 3100084,
    InfrPhoneTaskCfgNotFound = 3100085,
    InfrRoadSwitchTraceInMulti = 3100173,
    InfrThemeActivityTaskNotFound = 3100074,
    InfrThemeActivityTaskRewarded = 3100075,
    InfrThemeActivityTaskNotComplete = 3100076,
    InfrThemeActivityInputInvalid = 3100077,
    InfrThemeActivityIdMisMatch = 3100094,
    InfrThemeActivityTaskCannotRewarded = 3100078,
    MotorTechPreNodeLock = 3100055,
    MotorTechPreNotUpgraded = 3100056,
    MotorTechLevelLower = 3100057,
    MotorTechNotUnlocked = 3100058,
    MotorTechLevelMax = 3100059,
    MotorTechCfgNotFound = 3100060,
    MotorLvlConfigNotFound = 3100061,
    MotorFuncNotOpen = 3100062,
    MotorTaskCannotRewarded = 3100063,
    MotorTaskInvalidParam = 3100064,
    MotorTaskCfgNotFound = 3100065,
    MotorTechTreeCfgNotFound = 3100066,
    MotorTechTreeLocked = 3100067,
    MotorPlayerUsingSkill = 3100068,
    MotorPlayerClimbing = 3100069,
    MotorPlayerInWater = 3100070,
    MotorPlayerInAir = 3100071,
    MotorPlayerOnMotor = 3100072,
    MotorTaskNotInOneTree = 3100079,
    MotorExpInvalid = 3100080,
    MotorOutlookLocked = 3100086,
    MotorFrameCfgNotFound = 3100087,
    MotorStickerCfgNotFound = 3100088,
    MotorOutlookBanInRegion = 3100089,
    MotorOutlookConflict = 3100090,
    MotorStickerPartCountError = 3100091,
    MotorStickerPartIdError = 3100092,
    MotorOutlookNotOwned = 3100093,
    MotorOutlookNotShow = 3100320,
    MotorLevelRewardAllClaimed = 3100159,
    PlayerNotInMotorArea = 3100160,
    MotorDecorationsCfgNotFound = 3100164,
    MotorDecorationsPartCountError = 3100165,
    MotorDecorationsPartIdError = 3100166,
    MotorTechTreeInCd = 3100175,
    ArrowSwordLevelCfgNotFound = 3100095,
    ArrowSwordTalentCfgNotFound = 3100096,
    ArrowSwordTaskCfgNotFound = 3100097,
    ArrowSwordItemCfgNotFound = 3100098,
    ArrowSwordRoleCfgNotFound = 3100099,
    ArrowSwordPreNotCleared = 3100100,
    ArrowSwordActivityTypeNotMatch = 3100101,
    ArrowSwordTaskStatusHigher = 3100102,
    ArrowSwordInvalidInput = 3100103,
    ArrowSwordDataOwnerErr = 3100104,
    ArrowSwordActivityIdMisMatch = 3100105,
    ArrowSwordTalentLocked = 3100106,
    ArrowSwordTalentAlreadyInUse = 3100107,
    ArrowSwordTalentPreNodeNotInUse = 3100108,
    ArrowSwordActivityCfgNotFound = 3100109,
    ArrowSwordTalentNotInUse = 3100110,
    ArrowSwordActivityNotOpen = 3100111,
    ArrowSwordActivityEnd = 3100112,
    ArrowSwordComponentNoFind = 3100113,
    ArrowSwordSaveDataLost = 3100114,
    ArrowSwordRoleNotUnlock = 3100115,
    ArrowSwordRankListCd = 3100116,
    ArrowSwordComponentInstLost = 3100117,
    ArrowSwordInstNotInProgress = 3100118,
    ArrowSwordInstNotInitialized = 3100119,
    ArrowSwordHaveUnfinishedInst = 3100120,
    MotorFightLevelCfgNotFound = 3100121,
    MotorFightTalentCfgNotFound = 3100122,
    MotorFightTaskCfgNotFound = 3100123,
    MotorFightItemCfgNotFound = 3100124,
    MotorFightRoleCfgNotFound = 3100125,
    MotorFightPreNotCleared = 3100126,
    MotorFightActivityTypeNotMatch = 3100127,
    MotorFightTaskStatusHigher = 3100128,
    MotorFightInvalidInput = 3100129,
    MotorFightDataOwnerErr = 3100130,
    MotorFightActivityIdMisMatch = 3100131,
    MotorFightTalentLocked = 3100132,
    MotorFightTalentAlreadyInUse = 3100133,
    MotorFightTalentPreNodeNotInUse = 3100134,
    MotorFightActivityCfgNotFound = 3100135,
    MotorFightTalentNotInUse = 3100136,
    MotorFightActivityNotOpen = 3100137,
    MotorFightActivityEnd = 3100138,
    MotorFightComponentNoFind = 3100139,
    MotorFightSaveDataLost = 3100140,
    MotorFightRoleNotUnlock = 3100141,
    MotorFightRankListCd = 3100142,
    MotorFightComponentInstLost = 3100143,
    MotorFightInstNotInProgress = 3100144,
    MotorFightInstNotInitialized = 3100145,
    MotorFightHaveUnfinishedInst = 3100146,
    MotorFightAlreadySettle = 3100147,
    MotorFightNoInfiniteLevel = 3100161,
    MotorFightNoRoundData = 3100163,
    MotorFightLevelNotOpen = 3100170,
    MotorFightFirstClearRewardFail = 3100171,
    MotorFightSettleResultNull = 3100172,
    EncircleChallengeCfgNotFound = 3100148,
    EncircleChallengeNotOpen = 3100149,
    EncirclePreChallengeNotPass = 3100150,
    EncircleChallengeStepErr = 3100151,
    EncircleActivityTypeNotMatch = 3100174,
    EncircleChallengeNotStart = 3100177,
    EncircleMultiGame = 3100179,
    EncircleActivityNotOpen = 3100206,
    FetterGroupNotExist = 3100152,
    DirectRefiningQualityErr = 3100153,
    FetterGroupConfigErr = 3100154,
    FetterGroupDirectRefineLocked = 3100155,
    PhantomDirectRefiningFetterNotFound = 3100156,
    PhantomDirectRefiningWeekTimesLimit = 3100157,
    PhantomMonsterConfNotFound = 3100158,
    GivebackScoreCannotReward = 3100167,
    GivebackScoreCfgNotFound = 3100168,
    GivebackNotOpen = 3100169,
    GivebackInvalidInput = 3100176,
    GivebackParseDataError = 3100178,
    RhythmNotInLimitTime = 3100180,
    RhythmTaskCfgNotFound = 3100181,
    RhythmActivityCfgNotFound = 3100182,
    RhythmPlanetCfgNotFound = 3100183,
    RhythmLevelCfgNotFound = 3100184,
    RhythmSubLevelCfgNotFound = 3100185,
    RhythmRoleCfgNotFound = 3100190,
    RhythmInputInvalid = 3100186,
    RhythmActivityDataNotExist = 3100187,
    RhythmTaskCannotReward = 3100188,
    RhythmInMulti = 3100189,
    RhythmSubLevelNotOpen = 3100191,
    RhythmPreSubLevelNotCompleted = 3100192,
    RhythmLevelNotOpen = 3100193,
    RhythmPlanetNotOpen = 3100194,
    RhythmRoleLock = 3100195,
    RhythmActivityIdMisMatch = 3100196,
    RhythmInvalidPayload = 3100197,
    RhythmActivityTypeNotMatch = 3100198,
    RhythmNoPartnerRoleId = 3100199,
    RhythmGetRankInCd = 3100200,
    RhythmInstNotMatch = 3100201,
    RhythmActivityEnd = 3100202,
    RhythmPartnerNotSet = 3100203,
    RhythmStartInfoLost = 3100204,
    RhythmSubLevelNotMatch = 3100205,
    PinballLevelCfgNotFound = 3100207,
    PinballActivityNotOpen = 3100208,
    PinballTrialLevelNotUseRole = 3100209,
    PinballRoleCountInvalid = 3100210,
    PinballRoleNotOwn = 3100211,
    PinballSceneNotExists = 3100212,
    PinballComponentNotExists = 3100213,
    PinballEnterCtxInvalid = 3100214,
    PinballCompStateInvalid = 3100215,
    PinballGameAlreadySettled = 3100216,
    PinballBonusScoreNotFull = 3100217,
    PinballLevelTypeInvalid = 3100218,
    PinballAlreadySettled = 3100219,
    PinballNoReviveTimesLeft = 3100220,
    PinballCompStateChangeReasonInvalid = 3100221,
    PinballSettlePayloadInvalid = 3100222,
    PinballRoleCfgNotFound = 3100223,
    PinballTemplateEntityConfigNotFound = 3100224,
    PinballLevelNotBonus = 3100225,
    PinballLevelTargetCondTypeNotSupport = 3100226,
    PinballEffectCfgNotFound = 3100227,
    PinballEffectHandlerNotFound = 3100228,
    PinballTrialLevelEnterWithRoles = 3100229,
    PinballBtObjNotExists = 3100230,
    PinballLevelEnterWithNoRole = 3100231,
    PinballRoleLevelCfgNotFound = 3100238,
    PinballDailyConfigInvalid = 3100239,
    MotorOutlookPresetNameInvalid = 3100232,
    MotorOutlookPresetNotOwn = 3100233,
    MotorOutlookPresetMax = 3100234,
    MotorOutlookPresetNotExist = 3100235,
    MotorOutlookPresetInputInvalid = 3100236,
    MotorOutlookPresetIdInvalid = 3100237,
    MotorSkinCfgNotFound = 3100243,
    MotorSkinNotEditable = 3100244,
    MotorSceneCfgNotFound = 3100289,
    CdkGiftCdkBatchAlreadyGot = 3100240,
    CdkGiftCdkPayGiftAlreadyBought = 3100241,
    CdkGiftCdkPayGiftTypeErr = 3100242,
    CdkGiftCdkPayGiftOldVersion = 3100287,
    NewBieMainlineActivityNotOpen = 3100245,
    NewBieMainlineActivityEnd = 3100246,
    NewBieMainlineInvalidInput = 3100247,
    NewBieMainlineRewardActivityMismatch = 3100248,
    NewBieMainlineTabTaskCfgNotFound = 3100249,
    NewBieMainlineTabRewardCfgNotFound = 3100250,
    NewBieMainlineRewardAlreadyTaken = 3100251,
    NewBieMainlineScoreNotEnough = 3100252,
    NewBieMainlineRewardDropFail = 3100253,
    NewBieAdventureActivityNotOpen = 3100254,
    NewBieAdventureActivityCfgNotFound = 3100255,
    NewBieAdventureActivityEnd = 3100256,
    NewBieAdventureInvalidInput = 3100257,
    NewBieAdventureChapterV2CfgNotFound = 3100258,
    NewBieAdventureTaskV2CfgNotFound = 3100259,
    NewBieAdventureV2RewardAlreadyTaken = 3100260,
    NewBieAdventureV2RewardDropFail = 3100261,
    NewBieAdventureV2ChapterLocked = 3100262,
    NewBieAdventureV2ChapterRewardNotReady = 3100263,
    NewBieAdventureV2NoTaskConfig = 3100264,
    NewBieAdventureV2RoleNotSet = 3100265,
    NewBieAdventureV2RoleInvalid = 3100266,
    NewBieAdventureV2TasksCrossChapter = 3100267,
    NewBieAdventureV2TaskNotComplete = 3100268,
    GeneralDailyResetCronNotFound = 3100269,
    ActivitySignConfigNotFound = 3100270,
    SignActivityFreeDropNoConfig = 3100271,
    SignActivityFreeDropAlreadyTaken = 3100272,
    NewBieV2NoLicense = 3100273,
    NewBieV1NoLicense = 3100274,
    WeekCardPersonalNoPermission = 3100275,
    WeekCardPersonalPermissionExpired = 3100276,
    WeekCardNotPersonal = 3100277,
    WeekCardInputErr = 3100286,
    WeekCardRecycleNoPermission = 3100278,
    NewPlayerSupportSameParent = 3100323,
    NewBieCourseV2ActivityNotOpen = 3100279,
    NewBieCourseV2ActivityCfgNotFound = 3100280,
    NewBieCourseV2ActivityEnd = 3100281,
    NewBieCourseV2InvalidInput = 3100282,
    NewBieCourseV2CfgNotFound = 3100283,
    NewBieCourseV2LevelNotEnough = 3100284,
    NewBieCourseV2RewardAlreadyTaken = 3100285,
    SolarSchoolInputInvalid = 3100324,
    SolarSchoolTaskCfgNotFound = 3100325,
    SolarSchoolTaskCannotReward = 3100326,
    SolarSchoolActivityDataNotExist = 3100327,
    SolarSchoolNotInLimitTime = 3100328,
    SolarSchoolActivityTypeNotMatch = 3100329,
    SolarSchoolActivityEnd = 3100330,
    SolarSchoolActivityCfgNotFound = 3100331,
    SolarSchoolGalleryCfgNotFound = 3100332,
    SolarSchoolGalleryLock = 3100333,
    SolarSchoolGalleryCannotReward = 3100334,
    SolarSchoolSaveInvalid = 3100335,
    SolarSchoolSaveReasonInvalid = 3100336,
    SolarSchoolSaveSlotInvalid = 3100337,
    SolarSchoolSaveSlotNotFound = 3100338,
    SolarSchoolSaveDataCorrupted = 3100339,
    SolarSchoolInProgressNotInitialized = 3100340,
    SolarSchoolBatchSyncCountInvalid = 3100341,
    SolarSchoolSyncTypeInvalid = 3100342,
    SolarSchoolSyncPayloadMismatch = 3100343,
    SolarSchoolGameCfgNotFound = 3100344,
    SolarSchoolDuplicateInput = 3100345,
    SolarSchoolNotUnlocked = 3100346,
    SolarSchoolAlreadyUnlocked = 3100347,
    SolarSchoolAlreadyCompleted = 3100348,
    SolarSchoolNpcFavorLevelNotIncreased = 3100349,
    SolarSchoolValueOutOfRange = 3100350,
    SolarSchoolItemInsufficient = 3100351,
    SolarSchoolItemStackLimitExceeded = 3100352,
    SolarSchoolShopGoodsInvalid = 3100353,
    SolarSchoolShopBuyLimitExceeded = 3100354,
    SolarSchoolEndingCfgNotFound = 3100355,
    SolarSchoolActivityIdNotMatch = 3100356,
    SolarSchoolLimitAccNotEnough = 3100358,
    SolarSchoolLimitAccAlreadyRewarded = 3100359,
    SolarSchoolQuestNotSavable = 3100371,
    SolarSchoolRewardedBitInvalid = 3100372,
    SolarSchoolSaveVersionInvalid = 3100378,
    SolarSchoolShopRecordInvalid = 3100379,
    SolarSchoolSummaryInvalid = 3100380,
    SolarSchoolInfluenceLevelInvalid = 3100386,
    SolarSchoolGiftCountInvalid = 3100387,
    ErrNotGetFightInfoDtType = 3200000,
    ErrFightInfoDtType = 3200001,
    ErrSetFightInfoDtType = 3200002,
    ErrChangeDtTypeParamError = 3200003,
    ErrCurSceneNotMatch = 3300000,
    ErrTargetNotForDeadEye = 3300001,
    ErrDeadEyeCount = 3300002,
    ErrNoMoveEvent = 3300003,
    ErrEntityTypeNotSupport = 3300004,
    ErrExploreSkillNoActions = 3300005,
    ErrExploreSkillActionMultiGame = 3300006,
    ErrExploreSkillActionNotExist = 3300007,
    ErrMonsterNotGameplayTagComp = 3300008,
    ErrDecalEffectNotFound = 3300009,
    ErrEntityNotInCurMap = 3300010,
    ErrEntityNotInCurScene = 3300011,
    ErrEntityNotForNpcTimeSchedule = 3300012,
    ErrEntityNotInAnySchedule = 3300013,
    ErrSplineConfigNotFound = 3300014,
    ErrSplineIdxOutofBound = 3300015,
    ErrPositionConfig = 3300016,
    ErrSplineNotForTimeSchedule = 3300017,
    ErrHostPlayerNotSame = 3300018,
    ErrInteractType = 3300019,
    ErrEntityAlreadyDie = 3300020,
    ErrEntityAlreadyDestroyed = 3300021,
    ErrRequestLimitExceeded = 3300022,
    ErrOptionOutofBound = 3300023,
    ErrHoldHandInvalidActionType = 3300024,
    ErrMultiPreconditionChainError = 3300025,
    ErrBlackScreenIllegalSource = 3400000,
    ErrBeamReceiveEntityNotConfig = 3400001,
    ErrBeamReceiveCondition = 3400002,
    ErrBeamReceiveEntityNotConfig2 = 3400003,
    ErrFlowerPollutionNotConfig = 3400004,
    ErrorFlowerPollutionState = 3400005,
    ErrDollGarbTimeOut = 3400006,
    ErrGrabMachineEntityNotConfig = 3400007,
    ErrGrabItemNotInConfig = 3400008,
    ErrItemHasBeenClipped = 3400009,
    ErrDollGrabNoCsvConfig = 3400010,
    ErrPlayerNotStartedPlaying = 3400011,
    ErrGrabShowcaseEntityNotConfig = 3400012,
    ErrPlayerBagNotDoll = 3400013,
    ErrDollIsDelivered = 3400014,
    ErrPlayerAlreadyPlaying = 3400015,
    ErrGrabItemTypeMismatch = 3400016,
    ErrDollGrabAlreadyPaused = 3400017,
    ErrDollGrabNotPaused = 3400018,
    ErrDollMachineDropReward = 3400019,
    RoleTowerPrepareStepTypeMismatch = 1100670,
    RoleTowerPrepareStepAlreadyFinished = 1100671,
    RoleTowerSettleStepTypeMismatch = 1100672,
    RoleTowerTeamUpgradeStepTypeMismatch = 1100673,
    RoleTowerLoadSubLevelStepTypeMismatch = 1100674,
    RoleTowerUnsupportedStepRequest = 1100675,
    RoleTowerRequestRepeated = 1100676,
    RoleTowerRankEntranceClosed = 1100687,
    ErrInForceClosedArea = 2600139,
    DetailRecallInfoNotExist = 2600140,
    RecallDungeonIdNotExist = 2600141,
    SolarSchoolInMultiGame = 3100416,
    SolarSchoolAttrCfgNotFound = 3100419,
    SolarSchoolRoleCfgNotFound = 3100420,
    SolarSchoolInfluenceCfgNotFound = 3100421,
    SolarSchoolBehaviorCfgNotFound = 3100422,
    SolarSchoolExamCfgNotFound = 3100423,
    SolarSchoolItemCfgNotFound = 3100424,
    SolarSchoolShopCfgNotFound = 3100425,
    SolarSchoolShopGoodsCfgNotFound = 3100426,
    SolarSchoolQuestCfgNotFound = 3100427,
    SolarSchoolQuestRewardCfgNotFound = 3100428,
    SolarSchoolQuestNodeCfgNotFound = 3100429,
    SolarSchoolTraceIdNotMatch = 3100463,
    SolarSchoolGameDayBackward = 3100464,
    SolarSchoolGameDayExceedMax = 3100465,
    SolarSchoolGameDaySkipInvalid = 3100466,
    SolarSchoolSyncCountExceeded = 3100467,
    SolarSchoolAttrGainExceeded = 3100468,
    SolarSchoolItemDailyMaxExceeded = 3100469,
    SolarSchoolSpecialItemCannotCost = 3100470,
    SolarSchoolSaveCycleCountNotMatch = 3100471,
    SolarSchoolSaveGameDayNotMatch = 3100472,
    SolarSchoolSaveAttrNotMatch = 3100473,
    SolarSchoolSaveItemNotMatch = 3100474,
    SolarSchoolSaveGotItemBitNotMatch = 3100475,
    SolarSchoolNewCycleAttrInvalid = 3100476,
    SolarSchoolNewCycleItemInvalid = 3100477,
    SolarSchoolPreviewAttrNotMatch = 3100478,
    SolarSchoolAttrAddCostInvalid = 3100479,
    SolarSchoolAttrDuplicate = 3100480,
    SolarSchoolAttrFinalCfgNotFound = 3100481,
    SolarSchoolAttrFinalOutOfRange = 3100482,
    SolarSchoolMoneyAddExceeded = 3100483,
    SolarSchoolClientResVersionTooLow = 3100484,
    SolarSchoolParamCfgNotFound = 3100485,
}

impl Default for ErrorCode {
    fn default() -> Self {
        Self::Success
    }
}

impl crate::wire::PbEnum for ErrorCode {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Success,
            1 => Self::RequestParamError,
            2 => Self::InternalError,
            3 => Self::UnKnownError,
            4 => Self::ServerNotReady,
            5 => Self::ServerFullLoad,
            6 => Self::InvalidUserId,
            7 => Self::InvalidToken,
            8 => Self::InvalidRequest,
            9 => Self::GmFail,
            10 => Self::GmParamError,
            11 => Self::GmException,
            12 => Self::NotEnableGm,
            13 => Self::NotElegantName,
            14 => Self::ContainsDirtyWord,
            15 => Self::DirtyWordServerError,
            16 => Self::DirtyWordServerReturnEmpty,
            17 => Self::DirtyWordCheckException,
            18 => Self::ComponentNotExist,
            19 => Self::ServerFullLoadGame,
            20 => Self::ServerFullLoadGate,
            21 => Self::PlayerLoggingInFlag,
            22 => Self::GateLoginException,
            23 => Self::GateLoginSeverSelectErr,
            24 => Self::GateTokenAccessErr,
            25 => Self::GateTokenAccessException,
            26 => Self::GateLoginUserIdErr,
            27 => Self::GateLoginNodeIdErr,
            28 => Self::GateLoginCreateCharacterErr,
            29 => Self::GateCreateCharacterException,
            30 => Self::GateEnterGameAddressNotFound,
            31 => Self::GateEnterGameException,
            32 => Self::GateEnterGameCreatePlayerErr,
            33 => Self::GateEnterGameAddPlayerErr,
            34 => Self::GameGateNodeNotFound,
            35 => Self::GamePlayerAdminExist,
            36 => Self::GameReloginGateNodeNotFound,
            37 => Self::GameReloginPlayerNotFound,
            38 => Self::ServerNotOpen,
            39 => Self::ServerMaintenance,
            40 => Self::InvalidLoginType,
            41 => Self::InvalidGateway,
            42 => Self::SdkserverError,
            43 => Self::TokenNotAuthrized,
            44 => Self::HadBan,
            45 => Self::NotInUserIdWhiteList,
            46 => Self::NoHealthyGamesvr,
            47 => Self::NoHealthyGateway,
            48 => Self::GarFailed,
            49 => Self::GarSdkCheckFail,
            50 => Self::GarNoneUserInfo,
            51 => Self::GarQueryUserInfoError,
            52 => Self::GarNoRegion,
            53 => Self::InternalExceptionCode,
            54 => Self::DecodeExceptionCode,
            55 => Self::EncodeExceptionCode,
            56 => Self::InvalidRequestExceptionCode,
            57 => Self::MessageOutOfLimitExceptionCode,
            58 => Self::MessageNoHandler,
            59 => Self::EncryptionNoCreate,
            60 => Self::DecryptFail,
            61 => Self::PlayerNotInTheScene,
            62 => Self::NonReentrantExceptionCode,
            63 => Self::PlayerLoggedOut,
            64 => Self::MsgFunctionClose,
            65 => Self::SeqNoError,
            66 => Self::InvalidMessageType,
            67 => Self::InvalidMessageHeader,
            68 => Self::InvalidSeqNo,
            69 => Self::InvalidMessageId,
            70 => Self::ProtobufDecodeFailed,
            71 => Self::ErrProtoSeedCheck,
            72 => Self::MessageCouldNotBeRouted,
            73 => Self::ErrPlayerLogined,
            100000 => Self::ClosedRegister,
            100001 => Self::RegisterOutOfLimit,
            100002 => Self::HaveNoCharacter,
            100003 => Self::InvalidCharacterName,
            100004 => Self::CreateCharacterFailed,
            100005 => Self::CreateCharacterDuplicateKey,
            100006 => Self::PlayerAlreadyLogin,
            100007 => Self::PlayerLoggingIn,
            100008 => Self::ErrLoginGwreconnecting,
            100009 => Self::LoginRetry,
            100010 => Self::QueryPlayerDataFailed,
            100011 => Self::CheckPlayerDataFailed,
            100012 => Self::CheckPlayerDataFailedDebug,
            100013 => Self::LogoutUnknownError,
            100014 => Self::AccountLoggedInElsewhere,
            100015 => Self::AccountIsBlocked,
            100016 => Self::DataOverflow,
            100017 => Self::AccountBeKick,
            100018 => Self::AppVersionNotMatch,
            100019 => Self::LauncherVersionIsTooLow,
            100020 => Self::ResourceVersionIsTooLow,
            100021 => Self::CloseConnection,
            100022 => Self::ErrAcquirePlayerLockFailed,
            100023 => Self::ErrPlayerLoggingOut,
            100024 => Self::MessageChecksumFailed,
            100025 => Self::LoginTimeout,
            200000 => Self::ErrWeaponDefault,
            200001 => Self::ErrWeaponLevelLimit,
            200002 => Self::ErrWeaponBreachLimit,
            200003 => Self::ErrWeaponConsumeInvalid,
            200004 => Self::ErrWeaponPkgFull,
            200005 => Self::ErrRoleNoConfig,
            200006 => Self::ErrRoleIsActive,
            200007 => Self::ErrRoleNotActive,
            200008 => Self::ErrRoleOverNotEnough,
            200009 => Self::ErrRoleLevelNotEnough,
            200010 => Self::ErrRoleException,
            200011 => Self::ErrRoleNotExchange,
            200012 => Self::ErrRoleResonNotActive,
            200013 => Self::ErrRoleResonIsActive,
            200014 => Self::ErrRoleConfigNotRight,
            200015 => Self::ErrRoleLevelMax,
            200016 => Self::ErrRolePerResonNotActive,
            200017 => Self::ErrRoleConditionNotFind,
            200018 => Self::ErrRoleConditionNoEnough,
            200019 => Self::ErrRoleInvalidNameLength,
            200020 => Self::ErrRoleExpInvalid,
            200021 => Self::ErrRoleActiveNeedNoEnough,
            200022 => Self::ErrRoleResonMaxLevel,
            200023 => Self::ErrRoleProtoError,
            200024 => Self::ErrRoleItemListEmpty,
            200025 => Self::ErrRoleItemListCountOutRange,
            200026 => Self::ErrRoleItemExpError,
            200027 => Self::ErrRolePhantPosError,
            200028 => Self::ErrRolePhantSameError,
            200029 => Self::ErrRolePhantEmptyError,
            200030 => Self::ErrRoleItemListNoEnough,
            200031 => Self::ErrRoleGetSkillByIdFailed,
            200032 => Self::ErrRoleFavorLevelNotEnough,
            200033 => Self::ErrRolSkillNodeType,
            200034 => Self::ErrRolSkillNodeTypeActive,
            200035 => Self::ErrRolSkillNodeTypeUlock,
            200036 => Self::ErrRolSkillPointsNotEnough,
            200037 => Self::ErrTrialRoleExist,
            200038 => Self::ErrTrialRoleNotExist,
            200039 => Self::ErrTrialRoleRegionDataExist,
            200040 => Self::ErrTrialRoleBtObjDataExist,
            200041 => Self::ErrTrialRoleRegionExist,
            200042 => Self::ErrTrialRoleRegionNotExist,
            200043 => Self::ErrLoadEquipDefault,
            200044 => Self::ErrLoadEquipInvalidPos,
            200045 => Self::ErrLoadEquipInvalidRole,
            200046 => Self::ErrLoadEquipRoleConfig,
            200047 => Self::ErrPhantomIdNotExist,
            200048 => Self::ErrPhantomNotExist,
            200049 => Self::ErrPhantomLvupMax,
            200050 => Self::ErrPhantomLvupMismatchItemId,
            200051 => Self::ErrPhantomLvupNoItem,
            200052 => Self::ErrPhantomLvupLimit,
            200053 => Self::ErrPhantomItemType,
            200054 => Self::ErrPhantomInvalidPos,
            200055 => Self::ErrPhantomConfigNotFound,
            200056 => Self::ErrPhantomItemNotExist,
            200057 => Self::ErrPhantomPropNotExist,
            200058 => Self::ErrPhantomQaulityNotExist,
            200059 => Self::ErrPhantomBreachNotExist,
            200060 => Self::ErrPhantomLevelNotEnough,
            200061 => Self::ErrPhantomExpItemNotExist,
            200062 => Self::ErrPhantomSubPropRandomErr,
            200063 => Self::ErrPhantomSubPropNotEnough,
            200064 => Self::ErrPhantomSubPropGenDupicate,
            200065 => Self::ErrPhantomSubStrengthenPropNotExist,
            200066 => Self::ErrPhantomLevelConfigNotExist,
            200067 => Self::ErrPhantomLevelUpConsumeItemNotEnough,
            200068 => Self::ErrPhantomLevelUpMaterialLock,
            200069 => Self::ErrPhantomLevelUpConsumeItemErr,
            200070 => Self::ErrPhantomLevelUpRepeatItem,
            200071 => Self::ErrPhantomMainPropNotExist,
            200072 => Self::ErrPhantomGrowthNotExist,
            200073 => Self::ErrPhantomBreachItemCount,
            200074 => Self::ErrPhantomBreachRepeatItem,
            200075 => Self::ErrPhantomDecomposeEquiped,
            200076 => Self::ErrPhantomDecomposeFail,
            200077 => Self::ErrPhantomBreachBindItem,
            200078 => Self::ErrPhantomBreachErrItem,
            200079 => Self::ErrPhantomRecommendNoData,
            200080 => Self::ErrPhantomCannotTakeOff,
            200081 => Self::ErrPhantomCannotReplace,
            200082 => Self::ErrVisionSkillFavoriteTypeLimit,
            200083 => Self::ErrVisionSkillFavoriteCountLimit,
            200084 => Self::ErrVisionSkillCfgNotFound,
            200085 => Self::ErrVisionSkillNotFound,
            200086 => Self::ErrVisionSkillLevelUpMax,
            200087 => Self::ErrVisionSkillLevelUpLimit,
            200088 => Self::ErrVisionSkillSlotNotFound,
            200089 => Self::ErrVisionSkillEquipTypeLimit,
            200090 => Self::ErrVisionSkillUnEquipLimit,
            200091 => Self::ErrVisionSkillGemCfgNotFound,
            200092 => Self::ErrVisionSkillEquipLimit,
            200093 => Self::ErrVisionSkillGemLimit,
            200094 => Self::ErrVisionSkillOperFail,
            200095 => Self::ErrVisionSkillSlotEquipLimit,
            200096 => Self::ErrExploreSkillRouletteRepeat,
            200097 => Self::ErrItemCfgNotFound,
            200098 => Self::ErrItemNotFound,
            200099 => Self::ErrItemNotEnough,
            200100 => Self::ErrItemDecomposeLimit,
            200101 => Self::ErrItemUseLevelLimit,
            200102 => Self::ErrItemLockLimit,
            200103 => Self::ErrItemInvalidParams,
            200104 => Self::ErrItemDecomposeFail,
            200105 => Self::ErrItemUseFail,
            200106 => Self::ErrExchangeRewardCostItemNotEnough,
            200107 => Self::ExchangeRewardSuccess,
            200108 => Self::ErrPkgCapacityNotEnough,
            200109 => Self::ErrGiftOptionalCount,
            200110 => Self::ErrGiftOptionalNotExists,
            200111 => Self::ErrGiftNotExists,
            200112 => Self::ErrItemCount,
            200113 => Self::ErrItemIdNotContain,
            200114 => Self::ErrItemTypeNotContain,
            200115 => Self::ErrCalabashMaxLevel,
            200116 => Self::ErrCalabashConfig,
            200117 => Self::ErrCalabashLevelUp,
            200118 => Self::ErrCalabashExp,
            200119 => Self::ErrCalabashDevelopNoReward,
            200120 => Self::ErrCalabashMonsterNotFound,
            200121 => Self::PropRewardTips,
            200122 => Self::ErrEnergyMaxCharge,
            200123 => Self::ErrStateCanotTeleport,
            200124 => Self::ErrStateCannotEnterInst,
            200125 => Self::ErrStateCannotOnline,
            200126 => Self::ErrStateCannotChangeFormation,
            200127 => Self::ErrReportPlayerCountLimit,
            200128 => Self::ErrReportPlayerReasonNotFound,
            200129 => Self::ErrReportMessageLengthLimit,
            200130 => Self::ErrCookingToolFixed,
            200131 => Self::ErrCookingFormulaNotFound,
            200132 => Self::ErrCookingCount,
            200133 => Self::ErrCookingProcessNotFound,
            200134 => Self::ErrCookingLevelNotFound,
            200135 => Self::ErrCookingLevelLimt,
            200136 => Self::ErrCookingInteractiveNotFound,
            200137 => Self::ErrCookingFuncNotOpen,
            200138 => Self::ErrChallengeNotFound,
            200139 => Self::ErrChallengeNoTeam,
            200140 => Self::ErrChallengeTeamLimit,
            200141 => Self::ErrChallengeTeamMemLimit,
            200142 => Self::ErrChallengeChangeFormation,
            200143 => Self::ErrChallengeFunNotOpen,
            200144 => Self::ErrChallengeSeasonUpdate,
            200145 => Self::ErrChallengeLockRoleLimit,
            200146 => Self::ErrChallengeRoleLocked,
            200147 => Self::ErrChallengeNoRoleAlive,
            200148 => Self::ErrChallengeFormationEmpty,
            200149 => Self::ErrCycleChallengeNoRoleAlive,
            200150 => Self::ErrCycleChallengeFormationEmpty,
            200151 => Self::ErrInfluenceLocked,
            200152 => Self::ErrInfluenceRewardNotFound,
            200153 => Self::ErrInfluenceConfigNotFound,
            200154 => Self::ErrReputationLimit,
            200155 => Self::ErrInfluenceRewardFailed,
            200156 => Self::ErrInfluenceFunNotOpen,
            200157 => Self::ErrForgeFuncNotOpen,
            200158 => Self::ErrForgeCountLimit,
            200159 => Self::ErrForgeLocked,
            200160 => Self::ErrForgeConfigNotFound,
            200161 => Self::ErrForgeUnlocked,
            200162 => Self::ErrSynthesisFuncNotOpen,
            200163 => Self::ErrSynthesisConfigNotFound,
            200164 => Self::ErrSynthesisCountLimit,
            200165 => Self::ErrSynthesisLocked,
            200166 => Self::ErrSynthesisLevelNotFound,
            200167 => Self::ErrSynthesisLevelLimit,
            200168 => Self::ErrSynthesisCannotUnlock,
            200169 => Self::ErrSynthesisUnlocked,
            200170 => Self::ErrTrialRoleCannotMatch,
            200171 => Self::ErrPhantomFormationTeleport,
            200172 => Self::ErrPhantomFormationEnterInst,
            200173 => Self::ErrPhantomFormationMultiPlay,
            200174 => Self::ErrPhantomFormationAdvice,
            200175 => Self::ErrPhantomFormationChangeFormation,
            200176 => Self::ErrPhantomFormationRepeat,
            200177 => Self::ErrPhantomFormationChangeFailed,
            200178 => Self::ErrRoleChangeRoleCreateFailed,
            200179 => Self::ErrRoleChangeRoleUpdateCreateFailed,
            200180 => Self::ErrRoleChangeRoleNotUnlock,
            200181 => Self::ErrRoleChangeMultiPlay,
            200182 => Self::ErrRoleChangeInst,
            200183 => Self::ErrRoleChangeElementFunc,
            200184 => Self::ErrPhantomFormationCannotJoin,
            200185 => Self::ErrPhantomFormationHost,
            200186 => Self::ErrRoleChangeShowAllRole,
            200187 => Self::ErrInteractBoardEntityNotFound,
            200188 => Self::ErrInteractBoardRange,
            200189 => Self::ErrInteractBoardSystemNotFound,
            200190 => Self::ErrInteractBoardEntityConfig,
            200191 => Self::ErrInteractEntranceNotFound,
            200192 => Self::ErrInteractEntranceNotMatch,
            200193 => Self::ErrItemMaxUseCount,
            200194 => Self::ErrFuncNotExist,
            200195 => Self::ErrPhantomChangeInBattle,
            200196 => Self::ErrItemCanNotDestroy,
            200197 => Self::ErrPhantomEquipSourceCost,
            200198 => Self::ErrPhantomEquipTargetCost,
            200199 => Self::ErrPhantomEquipDuplicate,
            200200 => Self::ErrPhantomAutoEquipFromOther,
            200201 => Self::ErrPhantomConsumeItemCount,
            200202 => Self::ErrPhantomConsumeItemDuplicate,
            200203 => Self::ErrPhantomConsumeItemIncrDuplicate,
            200204 => Self::ErrPhantomConsumeItem,
            200205 => Self::ErrPhantomConsumeNoExp,
            200206 => Self::ErrPhantomBreachPos,
            200207 => Self::ErrPhantomBreachSuspend,
            200208 => Self::ErrPhantomBreachQuality,
            200209 => Self::ErrPhantomBreachExp,
            200210 => Self::ErrPhantomBreachConsumeItem,
            200211 => Self::ErrPhantomBreachNoSuspend,
            200212 => Self::ErrPhantomSpecialSkillRole,
            200213 => Self::ErrPhantomNotEquip,
            200214 => Self::ErrPhantomSpecilSkillPos,
            200215 => Self::ErrPhantomSubPropPlanConfig,
            200216 => Self::ErrPhantomMainPropGenFail,
            200217 => Self::ErrLivenessFuncNotOpen,
            200218 => Self::ErrLivenessTaskNotFound,
            200219 => Self::ErrLivenessTaskDataNotFound,
            200220 => Self::ErrLivenessTaskNotFinish,
            200221 => Self::ErrLivenessTaskRewarded,
            200222 => Self::ErrLivenessRewardNotFound,
            200223 => Self::ErrLivenessGoalNotReach,
            200224 => Self::ErrLivenessRewardParam,
            200225 => Self::ErrLivenessTaskRewardParam,
            200226 => Self::ErrWeaponLevelUpComsumeCount,
            200227 => Self::ErrWeaponConsumeSelf,
            200228 => Self::ErrWeaponConsumeItemNotFound,
            200229 => Self::ErrWeaponConsumeItemIdNotFound,
            200230 => Self::ErrWeaponLocked,
            200231 => Self::ErrWeaponConsumeDuplicate,
            200232 => Self::ErrWeaponEquiped,
            200233 => Self::ErrWeaponLevelUpItemDuplicate,
            200234 => Self::ErrWeaponLevelUpNoExp,
            200235 => Self::ErrWeaponLevelUpLevel,
            200236 => Self::ErrPhantomMainPropNotMatch,
            200237 => Self::ErrPhantomSubPropNotMatch,
            200238 => Self::ErrPhantomEquiped,
            200239 => Self::ErrAdviceNotInit,
            200240 => Self::ErrTowerChallengeNotOpen,
            200241 => Self::ErrTowerNotInChallenge,
            200242 => Self::ErrTowerConfigNotFound,
            200243 => Self::ErrTowerChallengeNotInOpenTime,
            200244 => Self::ErrTowerInChallenge,
            200245 => Self::ErrTowerFormationCount,
            200246 => Self::ErrTowerFormationRoleDuplicate,
            200247 => Self::ErrTowerRoleCost,
            200248 => Self::ErrTowerDifficultyNotClear,
            200249 => Self::ErrTowerFloorNotClear,
            200250 => Self::ErrTowerAreaNotClear,
            200251 => Self::ErrTowerRecommendNotSettle,
            200252 => Self::ErrTowerRewardNotFound,
            200253 => Self::ErrTowerNoReward,
            200254 => Self::ErrTowerRewarded,
            200255 => Self::ErrTowerRewardTarget,
            200256 => Self::ErrTowerSeasonUpdate,
            200257 => Self::ErrLordGymConfigNotFound,
            200258 => Self::ErrLordGymNotInPlay,
            200259 => Self::ErrLordGymLock,
            200260 => Self::ErrLordGymBtTreeNotFound,
            200261 => Self::ErrRoleSexFuncNotOpen,
            200262 => Self::ErrPhantomSubPropLocked,
            200263 => Self::ErrPhantomIdentifyNoCost,
            200264 => Self::ErrGiftPackType,
            200265 => Self::ErrGiftPackUseLimit,
            200266 => Self::ErrCdKeyNotEnable,
            200267 => Self::ErrCdKeyRequestCount,
            200268 => Self::ErrCdKeyRequestErr,
            200269 => Self::ErrCdKeyRequestDataErr,
            200270 => Self::ErrCdKeyException,
            200271 => Self::ErrCdKeyProcessCount,
            200272 => Self::ErrCdKeyNotFound,
            200273 => Self::ErrCdKeyBatchNotFound,
            200274 => Self::ErrCdKeyNotInValidTime,
            200275 => Self::ErrCdKeyBatchMaxCount,
            200276 => Self::ErrCdKeyEachPlayerMaxCount,
            200277 => Self::ErrCdKeyGroupCount,
            200278 => Self::ErrCdKeyCondition,
            200279 => Self::ErrCdKeyAddCountFail,
            200280 => Self::ErrCdKeyLength,
            200281 => Self::ErrCdKeyCharacter,
            200282 => Self::ErrGiftPackRandomErr,
            200283 => Self::ErrReconnectUserWhiteList,
            200284 => Self::ErrReconnectChannelWhiteList,
            200285 => Self::ErrReconnectIpInvalid,
            200286 => Self::ErrReconnectIpWhiteList,
            200287 => Self::ErrCdKeyExpire,
            200288 => Self::ErrWeaponResonLevelLimit,
            200289 => Self::ErrWeaponConfigNotFound,
            200290 => Self::ErrWeaponResonConfigNotFound,
            200291 => Self::ErrWeaponResonConsumeItem,
            200292 => Self::ErrWeaponResonConsumeGold,
            200293 => Self::ErrDestroyItemDuplicate,
            200294 => Self::ErrDestroyWeapon,
            200295 => Self::ErrCannotDestroyItem,
            200296 => Self::ErrCannotDestroyPhantom,
            200297 => Self::ErrCannotDestroyWeaponForm,
            200298 => Self::ErrCannotDestroyItemUnknown,
            200299 => Self::ErrWeaponConsumeQuality,
            200300 => Self::ErrPhantomSkinChangeCd,
            200301 => Self::ErrPhantomSkinUnlock,
            200302 => Self::ErrPhantomSkinMatch,
            200303 => Self::ErrLoginGameTainted,
            200304 => Self::ErrCookLimitCount,
            200305 => Self::ErrCookLimitTime,
            200306 => Self::ErrForgeLimitCount,
            200307 => Self::ErrForgeLimitTime,
            200308 => Self::ErrSynthesisLimitCount,
            200309 => Self::ErrSynthesisLimitTime,
            200310 => Self::ErrLoginIpBan,
            200311 => Self::ErrLoginDeviceBan,
            200312 => Self::ErrRoleNameEmpty,
            200313 => Self::ErrAdviceLength,
            200314 => Self::ErrPhantomRefiningCount,
            200315 => Self::ErrPhantomRefiningScore,
            200316 => Self::ErrPhantomRefiningTotalScore,
            200317 => Self::ErrPhantomRefiningDeveloped,
            200318 => Self::ErrPhotoMemoryCollectConfig,
            200319 => Self::ErrPhotoMemoryFuncNotOpen,
            200320 => Self::ErrPhotoMemoryCollectLock,
            200321 => Self::ErrPhotoMemoryCollectRewarded,
            200322 => Self::ErrRoleCount,
            200323 => Self::ErrCookFormulaUnlocked,
            200324 => Self::ErrForgeFormulaUnlocked,
            200325 => Self::ErrSynthesisFormulaUnlocked,
            200326 => Self::ErrCookFormulaBuyCount,
            200327 => Self::ErrForgeFormulaBuyCount,
            200328 => Self::ErrSynthesisFormulaBuyCount,
            200329 => Self::ErrCdKeyDailyVerifyCount,
            200330 => Self::ErrLongShanTaskNotFound,
            200331 => Self::ErrLongShanActivityClosed,
            200332 => Self::ErrLongShanTaskNotAccept,
            200333 => Self::ErrLongShanTaskNotComplete,
            200334 => Self::ErrLongShanTaskRewarded,
            200335 => Self::ErrTowerDefenceRewardParamErr,
            200336 => Self::ErrTowerDefenceInstanceNotFound,
            200337 => Self::ErrTowerDefenceActivityNotOpen,
            200338 => Self::ErrTowerDefenceActivityDataNotFound,
            200339 => Self::ErrTowerDefenceInstDataNotFound,
            200340 => Self::ErrTowerDefenceInstRewarded,
            200341 => Self::ErrTowerDefenceInstScoreNotEnough,
            200342 => Self::ErrTowerDefenceScoreRewarded,
            200343 => Self::ErrTowerDefenceScoreRewardNotEnough,
            200344 => Self::ErrTowerDefenceScoreRewardNotFound,
            200345 => Self::ErrTowerDefenceInstBuffNotEnable,
            200346 => Self::ErrTowerDefencePhantomDuplicate,
            200347 => Self::ErrTowerDefencePhantomNotSelect,
            200348 => Self::ErrorTowerDefenceInstNotOpen,
            200349 => Self::ErrorTowerDefenceInstCondition,
            200350 => Self::ErrNameModifyCd,
            200351 => Self::ErrNameVerifying,
            200352 => Self::ErrTimePointRewardActivityConfigNotFound,
            200353 => Self::ErrTimePointRewardActivityNotOpen,
            200354 => Self::ErrTimePointRewardActivityRewarded,
            200355 => Self::ErrTimePointRewardActivityTime,
            200356 => Self::ErrCdkeyVerifying,
            200357 => Self::ErrorTowerDefenceInstLocked,
            200358 => Self::ErrTrackMoonEntrustLocked,
            200359 => Self::ErrTrackMoonRoleLocked,
            200360 => Self::ErrTrackMoonBuildLocked,
            200361 => Self::ErrTrackMoonBuildEntity,
            200362 => Self::ErrCopyUserRequestErr,
            200363 => Self::ErrCopyUserInserting,
            200364 => Self::ErrCopyUserErr,
            200365 => Self::ErrCopyUserDataErr,
            200366 => Self::ErrCopyUserInsertErr,
            200367 => Self::ErrCopyUserInsertFailed,
            200368 => Self::ErrRiskHarvestBuffGroupNotFound,
            200369 => Self::ErrRiskHarvestActivityClosePlay,
            200370 => Self::ErrRiskHarvestModeChangeClosePlay,
            200371 => Self::ErrRiskHarvestLeaveClosePlay,
            200372 => Self::ErrRiskHarvestInstNotFound,
            200373 => Self::ErrRiskHarvestActivityNotOpen,
            200374 => Self::ErrRiskHarvestInstDataNotFound,
            200375 => Self::ErrRiskHarvestInstRewarded,
            200376 => Self::ErrRiskHarvestInstNotPass,
            200377 => Self::ErrRiskHarvestScoreRewardNotFound,
            200378 => Self::ErrRiskHarvestScoreRewarded,
            200379 => Self::ErrRiskHarvestScoreNotEnough,
            200380 => Self::ErrRiskHarvestRoleTrial,
            200381 => Self::ErrRiskHarvestMatching,
            200382 => Self::ErrRiskHarvestNotDefaultWorld,
            200383 => Self::ErrRiskHarvestMultiMode,
            200384 => Self::ErrRiskHarvestInstLocked,
            200385 => Self::ErrRiskHarvestInstTeleportEntityNotFound,
            200386 => Self::ErrRiskHarvestInstOpen,
            200387 => Self::ErrRiskHarvestPlayOpenFailed,
            200388 => Self::ErrRiskHarvestPlayDataNotFound,
            200389 => Self::ErrRiskHarvestBuffNoReward,
            200390 => Self::ErrRiskHarvestBuffRewarded,
            200391 => Self::ErrRiskHarvestBuffLocked,
            200392 => Self::ErrRiskHarvestBuffCountRewardNotFound,
            200393 => Self::ErrRiskHarvestBuffCountRewarded,
            200394 => Self::ErrRiskHarvestBuffCountNotEnough,
            200395 => Self::ErrRiskHarvestInstScoreNotEnough,
            200396 => Self::ErrItemDisuseLimit,
            200397 => Self::ErrItemDisuseFunc,
            200398 => Self::ErrPhantomRefiningMaxCount,
            200399 => Self::ErrPhantomRefiningDulplicate,
            200400 => Self::ErrPhantomNotNormal,
            200401 => Self::ErrGetSelfPsnOnlineId,
            200402 => Self::ErrGetPsnUserPlayerErr,
            200403 => Self::ErrTowerDefenceHostLeave,
            200404 => Self::ErrRiskHarvestNotInInst,
            200405 => Self::ErrEnterInstTypeErr,
            200406 => Self::ErrInputSettingCount,
            200407 => Self::ErrInputSettingDeviceType,
            200408 => Self::ErrInputSettingActionCount,
            200409 => Self::ErrInputSettingAxisCount,
            200410 => Self::ErrInputCombinationActionCount,
            200411 => Self::ErrInputCombinationAxisCount,
            200412 => Self::ErrInputSettingActionName,
            200413 => Self::ErrInputSettingAxisName,
            200414 => Self::ErrInputCombinationActionName,
            200415 => Self::ErrInputCombinationAxisName,
            200416 => Self::ErrInputActionKeyNameLength,
            200417 => Self::ErrInputActionKeyLength,
            200418 => Self::ErrInputAxisKeyNameLength,
            200419 => Self::ErrInputAxisKeyLength,
            200420 => Self::ErrInputCombinationActionKeyNameLength,
            200421 => Self::ErrInputCombinationActionKeyLength,
            200422 => Self::ErrInputCombinationActionKeyListLength,
            200423 => Self::ErrInputCombinationAxisKeyNameLength,
            200424 => Self::ErrInputCombinationAxisKeyLength,
            200425 => Self::ErrInputCombinationAxisKeyListLength,
            200426 => Self::ErrInputDeviceSubTypeLength,
            200427 => Self::ErrPsnaccountBlocked,
            200428 => Self::ErrRoleNameInvalid,
            200429 => Self::ErrMailBindRewardNotBind,
            200430 => Self::ErrMailBindRewarded,
            200431 => Self::ErrInputSettingNull,
            200432 => Self::ErrMultiInstExchangeCountErr,
            200433 => Self::ErrMultiInstExchangeFirstPass,
            200434 => Self::ErrMultiInstExchangeActivity,
            200435 => Self::ErrMultiInstExchangeFuncNotOpen,
            200436 => Self::ErrMultiInstExchangeTypeErr,
            200437 => Self::ErrMultiInstExchangeLevelTypeErr,
            200438 => Self::ErrRoleSkinLocked,
            200439 => Self::ErrRoleSkinConfig,
            200440 => Self::ErrRoleSkinNotMatch,
            200441 => Self::ErrRoleSkinWeaponNotSuit,
            200442 => Self::ErrSdkLoginResetByPeer,
            200443 => Self::ErrSdkLoginHttpRequestException,
            200444 => Self::ErrSdkLoginTaskTimeout,
            200445 => Self::ErrSdkLoginTaskCanceled,
            200446 => Self::ErrSexChangeCd,
            200447 => Self::ErrSexChangeLogout,
            200448 => Self::ErrSexChangeTrialActive,
            200449 => Self::ErrLobbyListRequestLimit,
            200450 => Self::ErrLobbyQueryPlayerRequestLimit,
            200451 => Self::ErrPlayerBasicInfoRequestLimit,
            200452 => Self::ErrBlockPlayerRequestLimit,
            200453 => Self::ErrPsnPlayerInfoRequestLimit,
            200454 => Self::ErrFishingPosInvalidX,
            200455 => Self::ErrFishingPosInvalidY,
            200456 => Self::ErrFishingPosOverlap,
            200457 => Self::ErrFishingParamLengthCabinLeft,
            200458 => Self::ErrFishingParamLengthCabinRight,
            200459 => Self::ErrFishingCabinNotOpen,
            200460 => Self::ErrFishingCabinNotFound,
            200461 => Self::ErrFishingCountNotMatch,
            200462 => Self::ErrFishingDuplicate,
            200463 => Self::ErrFishingRemoveItemErr,
            200464 => Self::ErrFishingItemNotFound,
            200465 => Self::ErrFishingCountNotMatchRequest,
            200466 => Self::ErrFishingCountNotMatchCabin,
            200467 => Self::ErrFishingHandInNotMatch,
            200468 => Self::ErrFishingQuickSellNoItem,
            200469 => Self::ErrFishingNoQuickSell,
            200470 => Self::ErrFishingQuickSellConfig,
            200471 => Self::ErrFishingQuickSellNotFilled,
            200472 => Self::ErrFishingQuickSellItemErr,
            200473 => Self::ErrFishingNoSellItem,
            200474 => Self::ErrFishingCanNotSell,
            200475 => Self::ErrFishingItemConfigNotFound,
            200476 => Self::ErrFishingSellItemDuplicate,
            200477 => Self::ErrFishingPriceCalFailed,
            200478 => Self::ErrFishingSellCount,
            200479 => Self::ErrFishingPointNotOpen,
            200480 => Self::ErrFishingPointConfig,
            200481 => Self::ErrFishingPointCount,
            200482 => Self::ErrFishingTempCabinMax,
            200483 => Self::ErrFishingPointGenFail,
            200484 => Self::ErrFishingTechLevel,
            200485 => Self::ErrFishingPreNodeLock,
            200486 => Self::ErrFishingTechLock,
            200487 => Self::ErrFishingTechConfig,
            200488 => Self::ErrFishingLevelMax,
            200489 => Self::ErrFishingSkinConfig,
            200490 => Self::ErrFishingPortConfig,
            200491 => Self::ErrFishingEntrustConfig,
            200492 => Self::ErrFishingEntrustNotAccepted,
            200493 => Self::ErrFishingEntrustNotFound,
            200494 => Self::ErrFishingEntrustDestination,
            200495 => Self::ErrFishingEntrustItemNotEnough,
            200496 => Self::ErrFishingEntrustItemTotalNotMatch,
            200497 => Self::ErrFishingEntrustDumplicate,
            200498 => Self::ErrFishingEntrustEmptyItem,
            200499 => Self::ErrFishingEntrustItemNotMatch,
            200500 => Self::ErrFishingEntrustRefreshInitial,
            200501 => Self::ErrFishingEntrustRefreshPrice,
            200502 => Self::ErrFishingEntrustNotAcceptable,
            200503 => Self::ErrFishingNotInBigWorld,
            200504 => Self::ErrFishingMultiMode,
            200505 => Self::ErrFishingNotOwner,
            200506 => Self::ErrFishingSkinLock,
            200507 => Self::ErrFishingPortLock,
            200508 => Self::ErrFishingCageLock,
            200509 => Self::ErrFishingCageConfig,
            200510 => Self::ErrFishingHandInConfig,
            200511 => Self::ErrFishingHandInCountNotMatch,
            200512 => Self::ErrFishingTempPointData,
            200513 => Self::ErrFishingTempPointConfig,
            200514 => Self::ErrFishingBombItem,
            200515 => Self::ErrFishingBombNoPoint,
            200516 => Self::ErrFishingBaitArea,
            200517 => Self::ErrFishingNotSailing,
            200518 => Self::ErrFishingInPort,
            200519 => Self::ErrFishingInShip,
            200520 => Self::ErrFishingIllustratedRewardNotFound,
            200521 => Self::ErrFishingIllustratedRewarded,
            200522 => Self::ErrFishingIllustratedCondition,
            200523 => Self::ErrFishingTechOutputNotFound,
            200524 => Self::ErrFishingShipNotFound,
            200525 => Self::ErrFishingPortTech,
            200526 => Self::ErrFishingRemoveFuncNotOpen,
            200527 => Self::ErrFishingIllustratedFuncNotOpen,
            200528 => Self::ErrFishingSellFuncNotOpen,
            200529 => Self::ErrFishingTechFuncNotOpen,
            200530 => Self::ErrFishingEntrustHandInQuickSell,
            200531 => Self::ErrActivityPreOpenLock,
            200532 => Self::ErrActivityShowLock,
            200533 => Self::ErrShopGoodsVisibleCondition,
            200534 => Self::ErrShopGoodsDisableCondition,
            200535 => Self::ErrFishingEntrustRefreshFail,
            200536 => Self::ErrPlayerDataVersion,
            200537 => Self::ErrBrokenCircuitRejected,
            200538 => Self::ErrRateLimiterRejected,
            200539 => Self::ErrTimeoutRejected,
            200540 => Self::ErrLoginEnvironment,
            200541 => Self::ErrLoginUserEmpty,
            200542 => Self::ErrOldGameNodeLogoutFail,
            200543 => Self::ErrOldGameNodeLogoutOffline,
            200544 => Self::ErrReloginBranchNameNotMatch,
            200545 => Self::ErrReLoginFightDataInConsistent,
            200546 => Self::ErrCreatePlayerData,
            200547 => Self::ErrReLoginPlayerLoggingOut,
            200548 => Self::ErrAccountLoggedInElsewhere,
            200551 => Self::ErrTowerChallengeTeleportLocked,
            200552 => Self::ErrTowerDefenceGroupConfig,
            200553 => Self::ErrTowerDefencePreInstNotPass,
            200554 => Self::ErrTowerDefenceRankCd,
            200549 => Self::ErrFishingFixItemNotEnough,
            200550 => Self::ErrFishingEntrustUpdateItemNotEnough,
            200555 => Self::ErrTowerDefenceGroupActivityC,
            200556 => Self::ErrTowerDefenceGroupActivity,
            200557 => Self::ErrAbyssComNotFound,
            200558 => Self::ErrAbyssInstConfig,
            200559 => Self::ErrAbyssRoomConfig,
            200560 => Self::ErrAbyssEnterCtx,
            200561 => Self::ErrAbyssActivityNotOpen,
            200562 => Self::ErrAbyssRewardConfig,
            200563 => Self::ErrAbyssRewardNotOpen,
            200564 => Self::ErrAbyssRewardClosed,
            200565 => Self::ErrAbyssRewarded,
            200566 => Self::ErrAbyssRewardCondition,
            200567 => Self::ErrAbyssLastActionNotFinish,
            200568 => Self::ErrAbyssNextRoomNotFound,
            200569 => Self::ErrAbyssRoleNotEquip,
            200570 => Self::ErrAbyssEquipRoleNotExist,
            200571 => Self::ErrAbyssEquipRoleDuplicate,
            200572 => Self::ErrAbyssRoomNoBox,
            200573 => Self::ErrAbyssRewardPlayerCount,
            200574 => Self::ErrAbyssRewardBoxEntityIncId,
            200575 => Self::ErrAbyssBoxRewarded,
            200576 => Self::ErrAbyssBoxNotBelong,
            200577 => Self::ErrAbyssBoxConfigNotFound,
            200578 => Self::ErrAbyssBoxDropFailed,
            200579 => Self::ErrAbyssChallengeLocked,
            200580 => Self::ErrAbyssChallengeNoConsume,
            200581 => Self::ErrAbyssChallengeUnlockItem,
            200582 => Self::ErrAbyssLikePlayerNotInScene,
            200583 => Self::ErrAbyssLikeDuplicate,
            200584 => Self::ErrAbyssRoleItemNotFound,
            200585 => Self::ErrAbyssRoleConfig,
            200586 => Self::ErrAbyssRoleLevelConfig,
            200587 => Self::ErrAbyssRoleMaxPluginCount,
            200588 => Self::ErrAbyssRoleLevelMaxPlugin,
            200589 => Self::ErrAbyssPutOnDuplicate,
            200590 => Self::ErrAbyssEquipOldRoleFailed,
            200591 => Self::ErrAbyssSysthesisMaxCount,
            200592 => Self::ErrAbyssSysthesisItemDuplicate,
            200593 => Self::ErrAbyssPluginItemNotFound,
            200594 => Self::ErrAbyssPluginItemLocked,
            200595 => Self::ErrAbyssPluginConfigNotFound,
            200596 => Self::ErrAbyssPluginEquipped,
            200597 => Self::ErrAbyssQuialityConfig,
            200598 => Self::ErrAbyssSysthesisConsumeItemDuplicate,
            200599 => Self::ErrAbyssRankListCd,
            200600 => Self::ErrAbyssRoleUpLevel,
            200601 => Self::ErrAbyssRoleUpItem,
            200602 => Self::ErrAbyssRoleUpNoComsume,
            200603 => Self::ErrAbyssPreChallengeNotPass,
            200604 => Self::ErrAbyssSlotConfig,
            200605 => Self::ErrAbyssSlotNotMatch,
            200606 => Self::ErrAbyssInstUnlockTime,
            200607 => Self::ErrAbyssSettled,
            200608 => Self::ErrFormationTrailGender,
            200609 => Self::ErrLongShanStageLocked,
            200610 => Self::ErrAbyssRewardIdRepeated,
            200611 => Self::ErrAbyssRewardCount,
            200612 => Self::ErrOtherPlayerCondition,
            200613 => Self::ErrAbyssItemSkillBelongTo,
            200614 => Self::ErrLongShanScoreRewardCount,
            200615 => Self::ErrLongShanScoreRewardDuplicate,
            200616 => Self::ErrLongShanScoreRewardConfig,
            200617 => Self::ErrLongShanScoreRewardActivity,
            200618 => Self::ErrLongShanScoreRewarded,
            200619 => Self::ErrLongShanScoreNotEnough,
            200620 => Self::ErrAbyssDuplicatePassive,
            200621 => Self::ErrActivityLifePointChallengeNotFound,
            200622 => Self::ErrActivityLifePointNotOpen,
            200623 => Self::ErrActivityLifePointPreLocked,
            200624 => Self::ErrPhantomSettingRuleDuplicate,
            200625 => Self::ErrPhantomSettingRuleNotFound,
            200626 => Self::ErrPhantomSettingEmpty,
            200627 => Self::ErrPhantomSettingType,
            200628 => Self::ErrPhantomSettingNameLength,
            200629 => Self::ErrPhantomSettingNull,
            200630 => Self::ErrPhantomSettingIndexRange,
            200631 => Self::ErrPhantomRuleIndexRange,
            200632 => Self::ErrPhantomRuleCount,
            200633 => Self::ErrPhantomBathchOperCount,
            200634 => Self::ErrPhantomBathchType,
            200635 => Self::ErrMobileSettingRequestParam,
            200636 => Self::ErrMobileSettingParamDuplicate,
            200637 => Self::ErrCommonUiSettingRequestParam,
            200638 => Self::ErrCommonUiSettingParamDuplicate,
            200639 => Self::ErrCommonUiSettingConfig,
            200640 => Self::ErrActivityNotOpenTip,
            200641 => Self::ErrLineCrossChallengeNotFound,
            200642 => Self::ErrActivitylineCrossNotOpen,
            200643 => Self::ErrActivitylineCrossPreLocked,
            200644 => Self::ErrPhantomSettingFuncNotOpen,
            200645 => Self::ErrTowerSeasonConfig,
            200646 => Self::ErrAccountDeactivation,
            200647 => Self::ErrPlayerAccountDeactivation,
            200648 => Self::ErrPlayerSkinChange,
            200649 => Self::ErrTrapDefenseNotMulti,
            200650 => Self::ErrNotInTrapDefenseInst,
            200651 => Self::ErrBossRushTaskNoConfig,
            200652 => Self::ErrBossRushRequestActivityNotMatch,
            200653 => Self::ErrPhantomBatchPolishCount,
            200654 => Self::ErrPhantomBatchPolishDuplicate,
            200655 => Self::ErrPhantomBatchPolishPropNotMatch,
            200656 => Self::ErrPhantomBatchPolishNoValidItem,
            200657 => Self::ErrLoginEmptyToken,
            200658 => Self::ErrPhantomVicePolishLimit,
            200659 => Self::ErrPhantomVicePolishParam,
            200660 => Self::ErrPhantomVicePolishItemPropLimit,
            200661 => Self::ErrPhantomVicePolishItemLevelLimit,
            200662 => Self::ErrPhantomVicePolishParamDuplicate,
            200663 => Self::ErrPhantomVicePolishPropGen,
            200664 => Self::ErrPhantomVicePolishNoneProp,
            200665 => Self::ErrSkinRewardNotFound,
            200666 => Self::ErrSkinRewardState,
            200667 => Self::ErrTowerForbidRechallenge,
            200668 => Self::ErrInputSettingActoinType,
            200669 => Self::ErrCreateCharacterGender,
            200670 => Self::ErrSkillBranchRoleValid,
            200671 => Self::ErrSkillBranchNotValid,
            200672 => Self::ErrTrailRoleSkillBranchNotValid,
            200673 => Self::ErrRoleDevConfigVersion,
            200674 => Self::ErrRoleDevConfigVersionTime,
            200675 => Self::ErrRoleDevConfigNoUpdate,
            200676 => Self::ErrMotorDevelopRequestParam,
            200677 => Self::ErrMotorDevelopParamDuplicate,
            200678 => Self::ErrMotorDevelopTaskNotFound,
            200679 => Self::ErrMotorDevelopActivityNotOpen,
            200680 => Self::ErrMotorDevelopTaskNotComplete,
            200681 => Self::ErrPhantomVicePolishNoAck,
            200682 => Self::ErrClientStorageSystemCount,
            200683 => Self::ErrClientStorageSystemDuplicate,
            200684 => Self::ErrClientStorageSystem,
            200685 => Self::ErrClientStorageStringLength,
            200686 => Self::ErrClientStorageType,
            200687 => Self::ErrClientStorageCapacity,
            200688 => Self::ErrClientStorageSystemTypeErr,
            200689 => Self::ErrXboxRegionNotFound,
            200690 => Self::ErrNotXboxChannelId,
            200691 => Self::ErrXboxRegionSet,
            200692 => Self::ErrXboxRegionSetException,
            200693 => Self::ErrXboxRegionGetException,
            200694 => Self::ErrXboxRegionGetFailed,
            200695 => Self::ErrQuestBranchPageNotFound,
            200696 => Self::ErrQuestBranchQuestState,
            200697 => Self::ErrQuestBranchComplete,
            200698 => Self::ErrQuestBranchNotComplete,
            200699 => Self::ErrFightFormationNameChangeCd,
            200700 => Self::ErrFightFormationNameLength,
            200701 => Self::ErrRoleSkillLevel,
            200702 => Self::ErrQuestBranchConfig,
            200703 => Self::ErrConditionStateRequestParam,
            200704 => Self::ErrConditionStateRequestModule,
            200705 => Self::ErrPolishPhantomStackKey,
            200706 => Self::ErrSplitPhantomStackKey,
            200707 => Self::ErrConsumePhantomStackKey,
            200708 => Self::ErrBatchPolishPhantomStackKey,
            200709 => Self::ErrPhantomMaxStackCount,
            200710 => Self::ErrReturnPhantomStackKey,
            200711 => Self::ErrRefiningPhantomStackKey,
            200712 => Self::ErrPhantomPkgFull,
            300000 => Self::ErrMapDefault,
            300001 => Self::ErrMapMarkNumLimit,
            300002 => Self::ErrMapNoFogConfig,
            300003 => Self::ErrMapFogAlreadyUnlock,
            300004 => Self::ErrFormationEmpty,
            300005 => Self::ErrFormationUnknown,
            300006 => Self::ErrFormationDead,
            300007 => Self::ErrFormationRoleRepeat,
            300008 => Self::ErrFormationRoleNotActive,
            300009 => Self::ErrFormationRoleIndexOut,
            300010 => Self::ErrFormationRoleCountOut,
            300011 => Self::ErrFightFormationRoleNotExist,
            300012 => Self::ErrFightFormationRoleIdNotMatch,
            300013 => Self::ErrFightFormationRoleCountNotMatch,
            300014 => Self::ErrFightFormationRoleCareerNotMatch,
            300015 => Self::ErrFightFormationRoleElementNotMatch,
            300016 => Self::ErrFightFormationCannotTrial,
            300017 => Self::ErrFightFormationTrialRoleNotMatch,
            300018 => Self::ErrFormationOverSize,
            300019 => Self::ErrSwitchRoleIsDead,
            300020 => Self::ErrUpdateFormationCurRoleIsDead,
            300021 => Self::ErrUpdateFormationRoleIdsIsNull,
            300022 => Self::ErrFormationIdOutOfRange,
            300023 => Self::ErrCanNotCancelCurFormation,
            300024 => Self::ErrCurRoleNotInFormationRoleIds,
            300025 => Self::ErrUpateFormationNotInSingleWorld,
            300026 => Self::ErrSwitchRoleTypeSignleWorld,
            300027 => Self::ErrSwitchRoleTypeMultiWorld,
            300028 => Self::ErrSwitchRoleTypeFbInstance,
            300029 => Self::ErrSwitchRoleTypeUndefine,
            300030 => Self::ErrSingWorldCanNotUpdateFightRoles,
            300031 => Self::ErrUpdateFightRolesIsNull,
            300032 => Self::ErrUpdateFightRolesCurIdNotExist,
            300033 => Self::ErrInStroyCharacterCanNotSwitchRole,
            300034 => Self::ErrSwitchRoleNotInFightRoles,
            300035 => Self::ErrCanNotSwitchRepeat,
            300036 => Self::ErrSwitchRoleEntityIdNotExist,
            300037 => Self::ErrSwitchRoleEntityNotExist,
            300038 => Self::ErrSitchRoleEntityIsDead,
            300039 => Self::ErrorTeamOperaFail,
            300040 => Self::ErrorPlayerAlreadyHaveTeam,
            300041 => Self::ErrorTeamInviteContentInvalid,
            300042 => Self::ErrorPlayerInBanTime,
            300043 => Self::ErrorPlayerInInviteCd,
            300044 => Self::ErrorPlayerAlreadyInTeam,
            300045 => Self::ErrorKickOutPermissionNotEnough,
            300046 => Self::ErrorTeamIsFull,
            300047 => Self::ErrorTeamServiceNotReady,
            300048 => Self::ErrorTeamPlayerJoinRepeat,
            300049 => Self::ErrorPlayerNotInTeam,
            300050 => Self::ErrorInvitePlayerNotExist,
            300051 => Self::ErrorKickPlayerNotInTeam,
            300052 => Self::ErrorDismissPermissionNotEnough,
            300053 => Self::ErrorTeamRoleIdNotActive,
            300054 => Self::ErrorTeamRoleIdRepeat,
            300055 => Self::ErrorJoinOtherWorldOtherNotExist,
            300056 => Self::ErrorJoinOtherWorldOtherNotInScene,
            300057 => Self::ErrorJoinOtherWorldSceneNotExist,
            300058 => Self::ErrorTeamNotExist,
            300059 => Self::ErrRewardCfgNotFound,
            300060 => Self::ErrTeleportIdNotExist,
            300061 => Self::ErrTeleportIdNotActivate,
            300062 => Self::ErrTeleportCreatureIdNotExist,
            300063 => Self::ErrTeleportIdAlreadyActivate,
            300064 => Self::ErrTeleportGmGetPlayerFailed,
            300065 => Self::ErrTeleportGmGetCreatureGenCfgFailed,
            300066 => Self::ErrTgmNotExitst,
            300067 => Self::ErrTgmNotPlayer,
            300068 => Self::ErrTgmNotGenCfg,
            300069 => Self::ErrTgmInsId,
            300070 => Self::ErrTeleportEntityNotExist,
            300071 => Self::ErrTeleportComponentNotExist,
            300072 => Self::ErrTeleportComponentNotMatch,
            300073 => Self::ErrAreaEnterRepeated,
            300074 => Self::ErrAreaIdNotExist,
            300075 => Self::ErrAreaIdNoNeedRecord,
            300076 => Self::ErrPlayerIsNotDead,
            300077 => Self::ErrPlayerCanNotRevive,
            300078 => Self::ErrPlayerReviveCountReachMax,
            300079 => Self::ErrPlayerReviveDelayNotReach,
            300080 => Self::ErrAutoReviveNotRequest,
            300081 => Self::ErrReviveRegionExisted,
            300082 => Self::ErrReviveRegionNotExisted,
            300083 => Self::ErrReviveRegionConfigNotExist,
            300084 => Self::ErrCanNotUseItemRevive,
            300085 => Self::ErrIsMatching,
            300086 => Self::ErrNotInMatcing,
            300087 => Self::ErrMatchPoolNotExist,
            300088 => Self::ErrNotFindMatchResult,
            300089 => Self::ErrConfirmResultRepeat,
            300090 => Self::ErrAlreadyHaveFbTeam,
            300091 => Self::ErrFbTeamNotExist,
            300092 => Self::ErrPlayerNotInFbTeam,
            300093 => Self::ErrHostCanNotReady,
            300094 => Self::ErrChangeReadyRepeat,
            300095 => Self::ErrFbTeamHaveSameRole,
            300096 => Self::ErrReadyStateCanNotChangeRole,
            300097 => Self::ErrChangeSameRole,
            300098 => Self::ErrNotHaveKickPermission,
            300099 => Self::ErrBeKickNotInFbTeam,
            300100 => Self::ErrNotHaveFightPermission,
            300101 => Self::ErrFbTeamNotAllReady,
            300102 => Self::ErrFbInstIdNotExist,
            300103 => Self::ErrFbMatchRoleNotMatch,
            300104 => Self::ErrSingleInstCanNotMatch,
            300105 => Self::ErrWaitOtherEnterSceneForbidMatch,
            300106 => Self::ErrIsEnteringOtherSceneForbidMatch,
            300107 => Self::InstPlayBtObjNotFound,
            300108 => Self::InstPlayNotSuccess,
            300109 => Self::InstPlayAlreadyGetReward,
            300110 => Self::InstPlayExchangeRewardFail,
            300111 => Self::InstPlaySetterRepeat,
            300112 => Self::InstEntranceNotUnlock,
            300113 => Self::InstEntranceNotOpen,
            300114 => Self::EnterInstLevelNotEnough,
            300115 => Self::EnterInstWorldLevelNotEnough,
            300116 => Self::EnterInstQuestNotEnough,
            300117 => Self::ErrForbidEnterInstInMatch,
            300118 => Self::ErrForbidEnterInstInEnteringOtherWorld,
            300119 => Self::ErrForbidEnterInstInWaitingOtherEnterWorld,
            300120 => Self::ErrEnterInstTypeNotMatch,
            300121 => Self::ErrNotHaveGetRewardCount,
            300122 => Self::ErrInMatching,
            300123 => Self::ErrNotInMatching,
            300124 => Self::ErrNotFindValidMatchServer,
            300125 => Self::ErrNotFindMatchServerPrx,
            300126 => Self::ErrNotHaveMatchTeamInfo,
            300127 => Self::ErrAlreadyConfirmMatchResult,
            300128 => Self::ErrMatchTeamNotInReadyState,
            300129 => Self::ErrMatchRoleNotActive,
            300130 => Self::ErrMatchReadyRepeat,
            300131 => Self::ErrMatchPlayerNotReady,
            300132 => Self::ErrMatchNotHostCanNotKick,
            300133 => Self::ErrMatchNotHostCanNotSetMatching,
            300134 => Self::ErrSetMatchFlagRepeat,
            300135 => Self::ErrPlayerNotInMatchTeam,
            300136 => Self::ErrGetMatchPoolFail,
            300137 => Self::ErrPlayerInMatchPool,
            300138 => Self::ErrPlayerNotInMatchPool,
            300139 => Self::ErrPlayerInMatchTeamCanNotCancel,
            300140 => Self::ErrPlayerIsConfirmResult,
            300141 => Self::ErrNotFindMatchTeam,
            300142 => Self::ErrPlayerIsReadyCanNotChangeRole,
            300143 => Self::ErrNotHostCanNotSetMultRoles,
            300144 => Self::ErrCanNotSetRepeatRole,
            300145 => Self::ErrPlayerNotReadyCanNotCancel,
            300146 => Self::ErrRoleRepeatCanNotReady,
            300147 => Self::ErrBeKickNotInMatchTeam,
            300148 => Self::ErrNotHostCanNotKick,
            300149 => Self::ErrNotHostCanNotSetTeamState,
            300150 => Self::ErrTeamMatchingCanNotStartInst,
            300151 => Self::ErrMatchTeamHavePlayerNotReady,
            300152 => Self::ErrNotHostCanNotEnterInst,
            300153 => Self::ErrMatchTeamIsNotEnterInstState,
            300154 => Self::ErrMatchInstIdNotExist,
            300155 => Self::ErrSingleInstanceCanNotMatch,
            300156 => Self::ErrOnlineStateCanNotMatch,
            300157 => Self::ErrTeamHaveSameRoleCanNotBegin,
            300158 => Self::ErrNotJoinChatChannel,
            300159 => Self::ErrChatChannelNotFound,
            300160 => Self::ErrChatChannelTypeNotMatch,
            300161 => Self::ErrChatContentTooLong,
            300162 => Self::ErrFightRoleIsAllDied,
            300163 => Self::ErrLoadingSceneIdNotMatch,
            300164 => Self::ErrLoadingPlayerNotInScene,
            300165 => Self::ErrPlayerIsSceneLoadingCanNotBeKick,
            300166 => Self::ErrTeamPlayerIsSceneLoadingCanNotDissolve,
            300167 => Self::ErrIsSceneLoadingCanNotDissolve,
            300168 => Self::ErrSceneLoadingCanNotEnterInst,
            300169 => Self::ErrActivateResetPointNotEntity,
            300170 => Self::ErrHostIsLoadingScene,
            300171 => Self::ErrHostIsLoadingSceneCanNotApply,
            300172 => Self::ErrIsLoadingSceneCanNotAcceptApply,
            300173 => Self::ErrNotFindHostWorldScene,
            300174 => Self::ErrCanNotRepeatCreateNeedSaveScene,
            300175 => Self::DeadStateCanNotAgreeOherEnter,
            300176 => Self::HostIsDeadStateCanNotEnter,
            300177 => Self::ErrSceneIsLoadingCanNotLeave,
            300178 => Self::ErrInstCanNotReChallenge,
            300179 => Self::ErrInstMemberNotEnoughCanNotReChallenge,
            300180 => Self::ErrInstHavePlayerLeaveCanNotReChallenge,
            300181 => Self::ErrInstHavePlayerNotDeadCanNotReChallenge,
            300182 => Self::ErrInstNotSettleCanNotReChallenge,
            300183 => Self::ErrInstCanNotRepetApplyRechallenge,
            300184 => Self::ErrInstCanNotRepetReceiveRechallenge,
            300185 => Self::ErrInstOwnerCanIniviteRechallenge,
            300186 => Self::ErrInstOwnerCanNotReceiveRechallenge,
            300187 => Self::ErrPlayerIsLogoutCanNotCreateScene,
            300188 => Self::ErrPlayerIsCreatingScene,
            300189 => Self::ErrPlayerCreateSceneFail,
            300190 => Self::ErrBigWorldCanNotReset,
            300191 => Self::ErrMultiGameModeCanNotReset,
            300192 => Self::ErrIsEnterSceneApplyingCanNotDoRepeate,
            300193 => Self::ErrIsQueryLobbyFriendDetailCanNotDoRepeate,
            300194 => Self::ErrIsQueryLobbyPlayerDetailCanNotDoRepeate,
            300195 => Self::ErrPlayerIsLoadingCanNotDoTeleport,
            300196 => Self::ErrPlayerIsTeleportCanNotDoTeleport,
            300197 => Self::ErrTeleportPositionIllegal,
            300198 => Self::ErrPlayerIsLoadingCanNotRevive,
            300199 => Self::ErrPlayerIsTeleportCanNotRevive,
            300200 => Self::ErrPlayerIsInTeleportCanNotBeKick,
            300201 => Self::ErrTeamPlayerIsInTeleportCanNotDissolve,
            300202 => Self::ErrHostIsInTeleportCanNotApply,
            300203 => Self::ErrIsInTeleportCanNotAcceptApply,
            400000 => Self::ErrStrNotIllegal,
            400001 => Self::ErrBasicInfoPhotoUnlocked,
            400002 => Self::ErrBasicInfoFrameUnlocked,
            400003 => Self::ErrCanNotGetSelfBasicInfo,
            400004 => Self::ErrMailNotExist,
            400005 => Self::ErrMailAlreadyRead,
            400006 => Self::ErrNoMailCanGet,
            400007 => Self::ErrMailNoAttachment,
            400008 => Self::ErrMailAttachmentIsGet,
            400009 => Self::ErrMailAttachmentNotGet,
            400010 => Self::ErrMailNotRead,
            400011 => Self::ErrNoMailCanDelete,
            400012 => Self::ErrMailItemBagFull,
            400013 => Self::ErrMailFuncNotOpen,
            400014 => Self::ErrMailOverSize,
            400015 => Self::ErrMailTakeLimit,
            400016 => Self::ErrMailAttachmentItemInvalidCount,
            400017 => Self::ErrMailAttachmentItemNoConf,
            400018 => Self::ErrMailNoConf,
            400019 => Self::ErrShopIdNotExit,
            400020 => Self::ErrShopInfoExist,
            400021 => Self::ErrShopTimeLimit,
            400022 => Self::ErrShopMoneyId,
            400023 => Self::ErrShopNumLimit,
            400024 => Self::ErrShopCondLimit,
            400025 => Self::ErrShopBankNoExit,
            400026 => Self::ErrShopNoShow,
            400027 => Self::ErrShopVersion,
            400028 => Self::ErrShopIlligalParam,
            400029 => Self::ErrDragonPoolConf,
            400030 => Self::ErrFullLevel,
            400031 => Self::ErrItemConf,
            400032 => Self::ErrNotEnoughItem,
            400033 => Self::NotMingSuTi,
            400034 => Self::HadFinishMingSuTi,
            400035 => Self::MingSuCallEntityFail,
            400036 => Self::ErrDragonPoolFuncNotOpen,
            400037 => Self::ErrWorldLevelHadDown,
            400038 => Self::ErrWorldLevelNotDown,
            400039 => Self::ErrWorldLevelMin,
            400040 => Self::ErrWorldLevelCd,
            400041 => Self::ErrIsBlockedPlayer,
            400042 => Self::ErrIsNotBlockedPlayer,
            400043 => Self::ErrBlockListCountMax,
            400044 => Self::ErrYouAreBlocked,
            400045 => Self::ErrAlreadyOnFriendList,
            400046 => Self::ErrNotOnFriendList,
            400047 => Self::ErrAlreadyOnFriendApplyList,
            400048 => Self::ErrFriendApplyNotExists,
            400049 => Self::ErrFriendListCountMax,
            400050 => Self::ErrInitiatorFriendListCountMax,
            400051 => Self::ErrReceiverApplyListCountMax,
            400052 => Self::ErrCanNotFriendApplySendToSelf,
            400053 => Self::ErrFriendApplySended,
            400054 => Self::ErrFriendRemarkLengthLimit,
            400055 => Self::ErrFriendApplyRequestLimit,
            400056 => Self::ErrFriendRequestEmpty,
            400057 => Self::ErrFriendRequestOverSize,
            400058 => Self::ErrPayShopNotExists,
            400059 => Self::ErrPayShopDisabled,
            400060 => Self::ErrPayShopGoodsNotExists,
            400061 => Self::ErrPayShopGoodsDisabled,
            400062 => Self::ErrPayShopGoodsLocked,
            400063 => Self::ErrPayShopGoodsOutSellTime,
            400064 => Self::ErrPayShopGoodsBuyLimit,
            400065 => Self::ErrPayShopDataChanged,
            400066 => Self::ErrPayShopIllegalBuyCount,
            400067 => Self::ErrPayShopIsDirect,
            400068 => Self::ErrPayShopIsNotDirect,
            400069 => Self::ErrPayShopTabDisabled,
            400070 => Self::ErrMonthCardWithoutValidity,
            400071 => Self::ErrMonthCardUpdateConfNotExist,
            400072 => Self::ErrMonthCardDaysMax,
            400073 => Self::ErrMonthCardRewardGot,
            400074 => Self::ErrMonthCardConfNotExist,
            400075 => Self::ErrIsNotSpecialItem,
            400076 => Self::ErrNoEquipSpecialItem,
            400077 => Self::ErrNoValidBattlePass,
            400078 => Self::ErrBattlePassRewardNotFound,
            400079 => Self::ErrBattlePassNotPaid,
            400080 => Self::ErrBattlePassIsPaid,
            400081 => Self::ErrBattlePassRewardLocked,
            400082 => Self::ErrBattlePassRewardTaken,
            400083 => Self::ErrBattlePassCanNotRepeatActive,
            400084 => Self::BattlePassNoRecurringReward,
            400085 => Self::ErrBattlePassIsAdvanced,
            400086 => Self::ErrBattlePassTaskNotFound,
            400087 => Self::ErrBattlePassTaskNotFinished,
            400088 => Self::ErrBattlePassTaskTaken,
            400089 => Self::ErrBattlePassExpIsFull,
            400090 => Self::ErrAdviceNotFound,
            400091 => Self::ErrConjunctionCanNotWord,
            400092 => Self::ErrAdviceTextNotExists,
            400093 => Self::ErrAdviceWordNotExists,
            400094 => Self::ErrAdviceTemplateNotExists,
            400095 => Self::ErrAdviceCellCalcException,
            400096 => Self::ErrIsNotAdviceEntity,
            400097 => Self::ErrAdviceCreateLimit,
            400098 => Self::ErrAdviceContentCanNotEmpty,
            400099 => Self::ErrAdviceEntityNotFount,
            400100 => Self::ErrAdviceVoteLimit,
            400101 => Self::ErrAdviceIsVoteUp,
            400102 => Self::ErrAdviceIsVoteDown,
            400103 => Self::ErrNoAdviceItem,
            400104 => Self::ErrAdviceCreateNotOpen,
            400105 => Self::ErrAdviceCanNotCreateByVisitor,
            400106 => Self::ErrAdviceSetingIsShow,
            400107 => Self::ErrAdviceSetingIsNoShow,
            400108 => Self::ErrAdviceUpMaxValue,
            400109 => Self::ErrAdviceDownMaxValue,
            400110 => Self::ProtoVersionCheckFail,
            400111 => Self::ProtoMd5CheckFail,
            400112 => Self::ConfigVersionCheckFail,
            400113 => Self::ConfigMd5CheckFail,
            400114 => Self::ErrInvalidMonthCardDays,
            400115 => Self::ErrMonthCardExtendedDaysMax,
            400116 => Self::ErrMobileButtonNoCfg,
            400117 => Self::ErrMoneyWrongPayCount,
            400118 => Self::ErrMailTextSenderNotFound,
            400119 => Self::ErrMailTextTitleNotFound,
            400120 => Self::ErrMailTextContentNotFound,
            400121 => Self::ErrAdviceIsNotVoteUp,
            400122 => Self::ErrParkourChallengeNoConf,
            400123 => Self::ErrParkourLocationNoConf,
            400124 => Self::ErrParkourChallengeNotOpen,
            400125 => Self::ErrParkourChallengeNoData,
            400126 => Self::ErrParkourChallengeTaken,
            400127 => Self::ErrParkourChallengeUnderscore,
            400128 => Self::ErrParkourChallengeScoreNoConf,
            400129 => Self::ErrParkourTakeFail,
            400130 => Self::ErrShopIllegalBuyCount,
            500000 => Self::ErrQuestErrTaskId,
            500001 => Self::ErrQuestErrStepId,
            500002 => Self::ErrQuestErrTaskBag,
            500003 => Self::ErrQuestStepStatusNotCanAccept,
            500004 => Self::ErrQuestStepStatusNotCanCommit,
            500005 => Self::ErrQuestStepConf,
            500006 => Self::ErrQuestStepData,
            500007 => Self::ErrQuestCanNotAccept,
            500008 => Self::ErrAreaQuestDelegationBoardRequest,
            500009 => Self::ErrAreaQuestAreaIdErr,
            500010 => Self::ErrAreaQuestExpired,
            500011 => Self::ErrDevoteLevel,
            500012 => Self::ErrDevoteRewardReceived,
            500013 => Self::ErrQuestNotFinish,
            500014 => Self::ErrDevoteId,
            500015 => Self::ErrAreaQuestLimit,
            500016 => Self::ErrQuestNodeNotActive,
            500017 => Self::ErrQuestNotActiveId,
            500018 => Self::ErrQuestNodeNotFound,
            500019 => Self::ErrQuestComNotFound,
            500020 => Self::ErrQuestTraceType,
            500021 => Self::ErrQuestNotProgress,
            500022 => Self::ErrQuestNoCombatState,
            500023 => Self::ErrQuestNodeData,
            500024 => Self::ErrQuestNotChildQuestNode,
            500025 => Self::ErrQuestNotClientSubmit,
            500026 => Self::ErrQuestAccepted,
            500027 => Self::ErrResourceOccupation,
            500028 => Self::ErrRequestOccupationType,
            500029 => Self::ErrNotFoundOccupation,
            500030 => Self::ErrNotOnlineQuestAccept,
            500031 => Self::ErrQuestDestroy,
            500032 => Self::ErrTreeNodeNotFind,
            500033 => Self::ErrTreeNodeNotActive,
            500034 => Self::ErrIsNotChildQuestNode,
            500035 => Self::ErrChildQuestConditionCanNotSubmit,
            500036 => Self::ErrNodeNotFindAction,
            500037 => Self::ErrNodeActionIsFinish,
            500038 => Self::ErrNodeActionGetItemIsNotQuestItem,
            500039 => Self::ErrNodeActionGetItemHasNotFreeSize,
            500040 => Self::ErrInvalidBtType,
            500041 => Self::ErrTimerNotFind,
            500042 => Self::ErrPreCondition,
            500043 => Self::ErrHandIdItemData,
            500044 => Self::ErrTreeNotFailedNode,
            500045 => Self::ErrTreeNotFailConf,
            500046 => Self::ErrTreeNotGiveUpConf,
            500047 => Self::ErrTreeNotRollback,
            500048 => Self::ErrNodeNotFindNpcId,
            500049 => Self::ErrNotRollbackPermission,
            500050 => Self::ErrNotRollbackRepeat,
            500051 => Self::ErrTreeSuspend,
            500052 => Self::ErrPlayerNotInQuestMap,
            500053 => Self::ErrSaveNewNotRollback,
            500054 => Self::ErrUiPlayType,
            500055 => Self::ErrOccupationTime,
            500056 => Self::ErrReleaseTime,
            500057 => Self::ErrActionSetTime,
            500058 => Self::ErrForcedOccupationResource,
            500059 => Self::ErrAddPlayBubble,
            500060 => Self::ErrDisableSwitchOccupation,
            500061 => Self::ErrOpenSystemBoardResultFail,
            500062 => Self::ErrEntityNoInhaledComponent,
            500063 => Self::ErrEntityInhaledStrength,
            500064 => Self::ErrDisableSwitchGender,
            600000 => Self::ErrTapeDefault,
            600001 => Self::ErrTapeInvalidPos,
            600002 => Self::ErrTapeIsNotActiveRole,
            600003 => Self::ErrTapeItemTypeFail,
            600004 => Self::ErrTapeNotExistTapeItem,
            600005 => Self::ErrTapeNotExistTapeConfig,
            600006 => Self::ErrTapeNotExistTapeProps,
            600007 => Self::ErrTapeHasTakeOnTape,
            600008 => Self::ErrTapeHasNotTakeOnTape,
            600009 => Self::ErrTapeNotExistTapeQualityConfig,
            600010 => Self::ErrTapeNotExistLevelUpExpConfig,
            600011 => Self::ErrTapeInvalidLevelUpExpValue,
            600012 => Self::ErrTapeNotExistExpDecayRatioConfig,
            600013 => Self::ErrTapeLevelUpEqualItem,
            600014 => Self::ErrTapeLevelUpRepeatItem,
            600015 => Self::ErrTapeLevelUpInvalidExpItemNum,
            600016 => Self::ErrTapeLevelUpInvalidExpRate,
            600017 => Self::ErrTapeLevelUpInvalidAddExp,
            600018 => Self::ErrTapeLevelUpMaxLevel,
            600019 => Self::ErrTapeLevelUpConsumeItemNotEnough,
            600020 => Self::ErrTapeLevelUpMaterialLock,
            600021 => Self::ErrTapeTransferEqualItem,
            600022 => Self::ErrTapeTransferQualityNotEqual,
            600023 => Self::ErrTapeTransferSuitNotEqual,
            600024 => Self::ErrTapeTransferMaterialLock,
            600025 => Self::ErrTapeTransferMaterialEquipped,
            600026 => Self::ErrTapeNotExistTransferPropNumConfig,
            600027 => Self::ErrTapeTransferPropNumIsMax,
            600028 => Self::ErrTapeTransferRandomSubPropFail,
            600029 => Self::ErrTapeResetTransferHasNotProp,
            600030 => Self::ErrTapeResetTransferMaterialNotEnough,
            600031 => Self::ErrTapeNotExistTapeExpItem,
            600032 => Self::ErrTapeNotExistTapeExpItemConfig,
            600033 => Self::ErrCollectEntityNotExist,
            600034 => Self::ErrCollectInvalidEntityMainType,
            600035 => Self::ErrRunningLevelPlayNotFind,
            600036 => Self::ErrLevelPlayInteractionEntity,
            600037 => Self::ErrLevelPlayNotExistByConfId,
            600038 => Self::ErrLevelPlayNotCreate,
            600039 => Self::ErrLevelPlayRewarded,
            600040 => Self::ErrLevelPlayInteractionType,
            600041 => Self::ErrLevelPlayNotPlayer,
            600042 => Self::ErrLevelPlayNotComplete,
            600043 => Self::ErrLevelPlayRewardFail,
            600044 => Self::ErrLevelPlayNotWaitState,
            600045 => Self::ErrLevelPlayAction,
            600046 => Self::ErrLevelPlayGetRewardLimit,
            600047 => Self::ErrFlowNotExist,
            600048 => Self::ErrFlowHaveNotActionWait,
            600049 => Self::ErrFlowHaveNotTalkWait,
            600050 => Self::ErrFlowHaveNotOptionWait,
            600051 => Self::ErrFlowInvalidOptionId,
            600052 => Self::ErrInteractFlowCanNotPlay,
            600053 => Self::ErrInteractInvalidFlowState,
            600054 => Self::ErrInteractOptionOwnerIsNotFlowOwner,
            600055 => Self::ErrInteractOptionOwnerIsNotActionOwner,
            600056 => Self::ErrActionOwnerIsNotEntity,
            600057 => Self::ErrActionOwnerNotFound,
            600058 => Self::ErrActionSceneNotFound,
            600059 => Self::ErrActionGroupNotFound,
            600060 => Self::ErrFinishClientActionFail,
            600061 => Self::ErrActionHostPlayerNotFound,
            600062 => Self::ErrActionFail,
            600063 => Self::ErrActionPlayerNotFound,
            600064 => Self::ErrInteractMultiGameMode,
            600065 => Self::ErrInteractAddFlowFail,
            600066 => Self::ErrBehaviorTreeOwnerNotFound,
            600067 => Self::ErrBehaviorTreeNotFound,
            600068 => Self::ErrBehaviorTreePending,
            600069 => Self::ErrBehaviorTreeTimerTypeNotFound,
            600070 => Self::ErrBehaviorTreeStopTimerFail,
            600071 => Self::ErrBehaviorTreeTimerCompNotFound,
            600072 => Self::ErrInteractCd,
            600073 => Self::ErrInteractRange,
            600074 => Self::ErrDropPickRange,
            600075 => Self::ErrBtTmpItemContextNotExist,
            600076 => Self::ErrBtTmpItemBtObjNotExist,
            600077 => Self::ErrPlayerBigWorldNotExist,
            600078 => Self::ErrRoleEntityNotExist,
            600079 => Self::ErrAddFlowFail,
            600080 => Self::ErrInteracting,
            600081 => Self::ErrInteractCollectBagFull,
            600082 => Self::ErrBtObjIsNotInstPlay,
            600083 => Self::ErrReviveConfigNotExist,
            600084 => Self::ErrFinishFlowFail,
            600085 => Self::ErrFlowActionFail,
            600086 => Self::ErrGmSubmitChildQuestNodeMaxDepth,
            600087 => Self::ErrGmSubmitChildQuestNodeIsNotProgress,
            600088 => Self::ErrEntityPatrolComponentNotExist,
            600089 => Self::ErrInteractIsNotParticipant,
            600090 => Self::ErrVisionEntityInteractFail,
            600091 => Self::ErrMaxDropTimes,
            600092 => Self::ErrStateEntityMultiHang,
            600093 => Self::ErrPlayerLoading,
            600094 => Self::ErrPlayerTeleporting,
            600095 => Self::ErrInteractBtPending,
            600096 => Self::ErrInteractDead,
            600097 => Self::ErrMultiHangEntity,
            600098 => Self::ErrRenjuCanNotResetWhenComplete,
            600099 => Self::ErrRenjuCanNotMove,
            600100 => Self::ErrEntityNotFound,
            600101 => Self::ErrSceneHostPlayerNotMatch,
            600102 => Self::ErrVehicleComponentNotFound,
            600103 => Self::ErrVehicleSeatNotFound,
            600104 => Self::ErrVehicleGettingOn,
            600105 => Self::ErrPortalEntityNotFound,
            600106 => Self::ErrPortalCompNotFound,
            600107 => Self::ErrPortalTeleportPosNotEqual,
            600108 => Self::ErrPlayerNotInVehicle,
            600109 => Self::ErrVehiclePassengerRoleExist,
            600110 => Self::ErrCreateVehiclePassengerEntityFail,
            600111 => Self::ErrVehiclePassengerNotFound,
            600112 => Self::ErrCanNotMovePlacement,
            600113 => Self::ErrGmSetLimitTeleportDungeon,
            600114 => Self::ErrMaxSetTagIdDepth,
            600115 => Self::ErrBeforeSetStateTagId,
            600116 => Self::ErrSetStateTagIdLock,
            600117 => Self::ErrDangoMonopolyActivityDataNotFound,
            600118 => Self::ErrDangoMonopolyReqTaskRewardMax,
            600119 => Self::ErrDangoMonopolyTaskConfigNotFound,
            600120 => Self::ErrDangoMonopolyTaskNotCompleted,
            600121 => Self::ErrDangoMonopolyTaskRewardHasGet,
            600122 => Self::ErrDangoMonopolyBoardConfigNotFound,
            600123 => Self::ErrDangoMonopolyGridRewardNotGet,
            600124 => Self::ErrDangoMonopolyHasNotDiceItem,
            600125 => Self::ErrDangoMonopolyHasNotGridReward,
            600126 => Self::ErrDangoMonopolyGridConfigNotFound,
            600127 => Self::ErrDangoMonopolyReqBoardRewardMax,
            600128 => Self::ErrDangoMonopolyBoardNotCompleted,
            600129 => Self::ErrDangoMonopolyBoardRewardHasGet,
            600130 => Self::ErrDangoMonopolyActivityConfigNotFound,
            600131 => Self::ErrDangoMonopolyBoardLock,
            600132 => Self::ErrChangeEntityStateActionEntityNotFound,
            600133 => Self::ErrDangoMonopolyDiceNumInvalid,
            600134 => Self::ErrDangoMonopolyBoardCompleted,
            600135 => Self::ErrLevelSequenceFrameEventCompNotFound,
            600136 => Self::ErrLevelSequenceFrameEventDataNotFound,
            600137 => Self::ErrLevelSequenceFrameEventSectionsNotFound,
            600138 => Self::ErrActionCtxInvalid,
            600139 => Self::ErrActionOwnerInvalid,
            600140 => Self::ErrGlobalFixCfgNotFound,
            600141 => Self::ErrActionGetItemIsNotQuestItem,
            600142 => Self::ErrGlobalFixExecuteCountMax,
            600143 => Self::ErrActionGroupCreateTooFrequently,
            600144 => Self::ErrActionGroupParallelTooMuch,
            600145 => Self::ErrCanNotRemoveLastTrialRole,
            600146 => Self::ErrEntityIsNotActivateState,
            600147 => Self::ErrRestoreTrialRoleNotInRegion,
            600148 => Self::ErrRemoveTrialRoleNotExist,
            600149 => Self::ErrRemoveLastTrialRole,
            600150 => Self::ErrSaveNodeDeActiveTrialRoleEmpty,
            600151 => Self::ErrNewHandInItemTypeInvalid,
            600152 => Self::ErrIgameObjectHostIsNotIplayer,
            700000 => Self::ErrSceneWorldNotExist,
            700001 => Self::ErrPlayerNotInScene,
            700002 => Self::ErrDropEntityNotExist,
            700003 => Self::ErrDropComponentNotExist,
            700004 => Self::ErrDropOwnerError,
            700005 => Self::ErrPlayerAlreadyInScene,
            700006 => Self::ErrSceneIdParseError,
            700007 => Self::ErrJoinSceneIdNotExist,
            700008 => Self::ErrSceneInviteFail,
            700009 => Self::ErrSceneInvitePlayerNotExist,
            700010 => Self::ErrSceneInviteTokenInvalid,
            700011 => Self::ErrSceneInviterNotExist,
            700012 => Self::ErrSceneInviteeIdNotMatch,
            700013 => Self::ErrSceneTeamIsFull,
            700014 => Self::ErrScenePlayerIsInTeam,
            700015 => Self::ErrSceneInviteerIsInPlayeInst,
            700016 => Self::ErrBeKickerNotInScene,
            700017 => Self::ErrorCanNotSceneKickSelf,
            700018 => Self::ErrCanNotKickOtherInPlayInst,
            700019 => Self::ErrCanNotKickOtherWhoIsInPlayInst,
            700020 => Self::ErrNoSceneKickPermission,
            700021 => Self::ErrInviterIsInOtherScene,
            700022 => Self::ErrInOtherSceneCanNotInvite,
            700023 => Self::ErrSceneBackSceneFlagError,
            700024 => Self::ErrLeaveSceneIdNotMatch,
            700025 => Self::ErrPrewarTeamAlreadyExist,
            700026 => Self::ErrPrewarTeamNotExist,
            700027 => Self::ErrCreatePrewarTeamPermissionNotEnough,
            700028 => Self::ErrPrewarTeamInvitePermissionNotEnough,
            700029 => Self::ErrPrewarBeInviterNotInScene,
            700030 => Self::ErrPrewarBeInviterInInstance,
            700031 => Self::ErrPlayerAlreadyInPrewarTeam,
            700032 => Self::ErrPlayerNotInPrewarTeam,
            700033 => Self::ErrPlayerSetPrewarReadyStateRepeat,
            700034 => Self::ErrPrewarTeamBeKickerNotInScene,
            700035 => Self::ErrPrewarTeamKickPermissionNotEnough,
            700036 => Self::ErrPrewarIniviteTooFrequently,
            700037 => Self::ErrPrewarJoinInstanceIdNotMatch,
            700038 => Self::ErrPrewarCaptainCanNotLeave,
            700039 => Self::ErrPrewarReadyStateCanNotChangeRole,
            700040 => Self::ErrScenePrewarTeamHavePlayerNotReady,
            700041 => Self::ErrAlreadyInMultiScene,
            700042 => Self::ErrOnlyCaptainCanDissolvePrewarTeam,
            700043 => Self::ErrMultiChangeRoleIndexInvalid,
            700044 => Self::ErrMultiCanNotChangeOtherRole,
            700045 => Self::ErrMultiChangeRoleEntityNorExist,
            700046 => Self::ErrSceneCanNotUseThisFunc,
            700047 => Self::ErrSceneCanNotUseThisItem,
            700048 => Self::ErrSceneFightRoleIdRepeat,
            700049 => Self::ErrShieldAddEntityNotExist,
            700050 => Self::ErrShieldAddShieldIdExisted,
            700051 => Self::ErrShieldChangeEntityNotExist,
            700052 => Self::ErrShieldChangeShieldIdNotExist,
            700053 => Self::ErrShieldRemoveEntityNotExist,
            700054 => Self::ErrShieldRemoveShieldIdNotExist,
            700055 => Self::ErrHardnessModeChangedEntityNotExist,
            700056 => Self::ErrSceneEntityNotExist,
            700057 => Self::ErrSceneEntityNotHavePartData,
            700058 => Self::ErrsceneEntityNotHavePartId,
            700059 => Self::ErrChangeControlRoleRepeat,
            700060 => Self::ErrVisionSkillCallEntityFail,
            700061 => Self::ErrSceneDataLoadError,
            700062 => Self::ErrCreatureDataError,
            700063 => Self::ErrCreatureGenIsExist,
            700064 => Self::ErrCreatureGenIsNotExist,
            700065 => Self::ErrCreatureGenIsControlByOther,
            700066 => Self::ErrCreatureGenNotHaveControlPerm,
            700067 => Self::ErrCreatureReachMaxCount,
            700068 => Self::ErrCreatureConditionNotMatch,
            700069 => Self::ErrCreatureTimeIntervalError,
            700070 => Self::ErrCreatureCfgNotExist,
            700071 => Self::ErrCreatureEntityIsNotValidity,
            700072 => Self::ErrUniqueEntityCanNotCreateTwice,
            700073 => Self::ErrRoleNotHaveVisionSkill,
            700074 => Self::ErrHitGearEntityNotExist,
            700075 => Self::ErrHitGearHaveNotEntityConfig,
            700076 => Self::ErrHitGearHaveNotGearConfig,
            700077 => Self::ErrHitGearHaveNotGameplayConfig,
            700078 => Self::ErrHitGearHaveNotStepConfig,
            700079 => Self::ErrHitGearHaveEntityCommonTag,
            700080 => Self::ErrHitGearEntityFunctionTypeFail,
            700081 => Self::ErrHitGearAcceptStepFail,
            700082 => Self::ErrCreateInstanceNotContainEntrance,
            700083 => Self::ErrCreateInstanceHaveNotEntranceConfig,
            700084 => Self::ErrCreateInstanceEntranceLock,
            700085 => Self::ErrCreateInstanceHaveNotConfig,
            700086 => Self::ErrCreateInstanceEnterCountNotEnough,
            700087 => Self::ErrCreateInstanceConditionNotMatch,
            700088 => Self::ErrEnterCountRequestHaveNotConfig,
            700089 => Self::ErrUnlockInstanceEntranceHaveNotConfig,
            700090 => Self::ErrUnlockInstanceEntranceNotNeedUnlock,
            700091 => Self::ErrUnlockInstanceEntranceUnlocked,
            700092 => Self::ErrUnlockInstanceEntranceCondiitonNotMatch,
            700093 => Self::ErrEnterSceneGameplayRequestHaveNotConfig,
            700094 => Self::ErrEnterSceneGameplayRequestAccepted,
            700095 => Self::ErrEnterSceneGameplayRequestAcceptFail,
            700096 => Self::ErrStoryCharacterCreatFail,
            700097 => Self::ErrStoryCharacterCreatRepeat,
            700098 => Self::ErrStoryCharacterNotExist,
            700099 => Self::ErrCheckGearEntityNotExist,
            700100 => Self::ErrCheckGearType,
            700101 => Self::ErrCheckGearNotEntityConfig,
            700102 => Self::ErrCheckGearActive,
            700103 => Self::ErrCheckGearInactive,
            700104 => Self::ErrTargetGearGroupEntityNotExist,
            700105 => Self::ErrTargetGearGroupConfigNotExist,
            700106 => Self::ErrTargetGearEntityNotExist,
            700107 => Self::ErrTargetGearConfigNotExist,
            700108 => Self::ErrTargetGearStartTypeIsNotHit,
            700109 => Self::ErrTargetGearStartTypeIsNotAction,
            700110 => Self::ErrTargetGearStarted,
            700111 => Self::ErrTargetGearFinished,
            700112 => Self::ErrTargetGearIsNotInCreatedConsole,
            700113 => Self::ErrTargetGearGroupEntityIsNotAllInit,
            700114 => Self::ErrLanternCatNotExit,
            700115 => Self::ErrLanternCatConfNotExit,
            700116 => Self::ErrLanternCatType,
            700117 => Self::ErrLanternActived,
            700118 => Self::ErrLanternTargetNotExit,
            700119 => Self::ErrCaptureFail,
            700120 => Self::ErrDyingFail,
            700121 => Self::ErrThrowDamageEntityNotExit,
            700122 => Self::ErrThrowDamageCalculateId,
            700123 => Self::ErrThrowDamageIdNotExit,
            700124 => Self::ErrThrowDamageRoleIdConf,
            700125 => Self::ErrThrowDamageTypeNotExit,
            700126 => Self::InstIdNotExist,
            700127 => Self::ErrControlObjectEntityNotExist,
            700128 => Self::ErrControlObjectConfigNotExist,
            700129 => Self::ErrControlGroupConfigNotExist,
            700130 => Self::ErrControlObjectLocked,
            700131 => Self::ErrControlGroupLocked,
            700132 => Self::ErrControlCanNotPutTarget,
            700133 => Self::ErrControlTargetOccupied,
            700134 => Self::ErrControlObjectCatching,
            700135 => Self::ErrControlObjectNotCatching,
            700136 => Self::ErrControlObjectOtherCatching,
            700137 => Self::ErrEntityPositionIllegal,
            700138 => Self::ErrTreasureBoxNot,
            700139 => Self::ErrTreasureBoxNotInit,
            700140 => Self::ErrTreasureBoxNotConfig,
            700141 => Self::ErrTreasureBoxHadReward,
            700142 => Self::ErrTreasureBoxNotInteraction,
            700143 => Self::ErrTreasureBoxNotDropId,
            700144 => Self::ErrTreasureBoxDropErr,
            700145 => Self::ErrTreasureBoxNotExist,
            700146 => Self::ErrTreasureBoxInvalidTag,
            700147 => Self::ErrTreasureBoxHadTag,
            700148 => Self::ErrTreasureBoxNotTag,
            700149 => Self::ErrSneakGameNotOpen,
            700150 => Self::ErrSneakFinishRepeat,
            700151 => Self::ErrClientControlDamage,
            700152 => Self::ErrSceneDataSaveFail,
            700153 => Self::NotInFbInstance,
            700154 => Self::GmerrCanNotCreateWorldInst,
            700155 => Self::GmerrPlayerAlreadyInFbInst,
            700156 => Self::GmerrTagetInstanceIsNotMulti,
            700157 => Self::GmerrPlayerNotFound,
            700158 => Self::ErrEntityFlowTooMuch,
            700159 => Self::GmErrIsWalkable,
            700160 => Self::GmErrIsNotWalkable,
            700161 => Self::GmErrNoNavmesh,
            700162 => Self::ErrBigWorldInstIdNotExist,
            700163 => Self::ErrInstIdNotBigWorld,
            700164 => Self::ErrInInstanceNotSwitchBigWorld,
            700165 => Self::ErrAlreadyInThisBigWorld,
            700166 => Self::ErrNoPermissionGetTreasureBox,
            700167 => Self::ErrCreateBigWorldRepeat,
            700168 => Self::DebugErrInstIdNotExist,
            700169 => Self::ErrSceneAiStopped,
            700170 => Self::ErrGlobalEntityConfigNotExist,
            700171 => Self::ErrSceneFixedConfigNotExist,
            700172 => Self::ErrSceneFixedEntityNotFound,
            700173 => Self::ErrSceneGlobalEntityNotFount,
            700174 => Self::ErrEntityNotHaveVarComponent,
            700175 => Self::ErrEntityVarNameNotExist,
            700176 => Self::ErrEntityVarTypeError,
            700177 => Self::ErrEntityConfigNotOffer,
            700178 => Self::ErrConfigTypeNotGloabl,
            700179 => Self::ErrConfigTypeNotSceneFixed,
            700180 => Self::ErrConfigTypeNotCharacter,
            700181 => Self::ErrEntityPosNotOffer,
            700182 => Self::ErrSceneCellPosNotFount,
            700183 => Self::ErrEntityCongigNotInSleep,
            700184 => Self::ErrSummonCfgNotFound,
            700185 => Self::ErrSummonAddEntityFail,
            700186 => Self::ErrSummonMaxCount,
            700187 => Self::ErrSummonMaxGenerations,
            700188 => Self::ErrSummonEntityIdAlreadyExist,
            700189 => Self::ErrSummonerEntityType,
            700190 => Self::ErrEntityStatusIsNotDead,
            700191 => Self::ErrEntityNotHaveAttributeComp,
            700192 => Self::ErrEntityDbData,
            700193 => Self::ErrSceneFixedEntityCreated,
            700194 => Self::ErrInvalidAwakeEntityContext,
            700195 => Self::ErrTriggerComponentNotExist,
            700196 => Self::ErrTriggerComponentMaxCount,
            700197 => Self::ErrNotSelfRole,
            700198 => Self::ErrNoControlRights,
            700199 => Self::ErrEntityHaveNotEntityOwner,
            700200 => Self::ErrEntityOwnerNotMatch,
            700201 => Self::ErrCreateSceneFixedEntitiesEmpty,
            700202 => Self::ErrInteractComponentNotExist,
            700203 => Self::ErrInteractOptionIndexInvalid,
            700204 => Self::ErrOnlineInteractNoPermission,
            700205 => Self::ErrOnlineInteractNotOpen,
            700206 => Self::ErrAwakeEntityNoPermission,
            700207 => Self::ErrCannotUseSkillStatus,
            700208 => Self::ErrInteractOptionGuidInvalid,
            700209 => Self::ErrAddInteractOptionFail,
            700210 => Self::ErrRemoveInteractOptionFail,
            700211 => Self::ErrInteractOptionOwnerNotFound,
            700212 => Self::ErrSummonPlayerId,
            700213 => Self::ErrSummonTemplateCfgNotFound,
            700214 => Self::ErrAttributeComponent,
            700215 => Self::ErrAnimFsmComponent,
            700216 => Self::ErrStateComponent,
            700217 => Self::ErrBattleComponent,
            700218 => Self::ErrPartComponent,
            700219 => Self::ErrAiControlComponent,
            700220 => Self::ErrSummonsComponent,
            700221 => Self::ErrAiBlackboardComponent,
            700222 => Self::ErrSetVarInvalidContext,
            700223 => Self::ErrSetVarInvalidVarRefPb,
            700224 => Self::ErrSetVarGetRightVarDefineFail,
            700225 => Self::ErrSetVarSetLeftVarDefineFail,
            700226 => Self::ErrCalcVarInvalidContext,
            700227 => Self::ErrCalcVarInvalidVarRef,
            700228 => Self::ErrCalcVarGetVarDefineFail,
            700229 => Self::ErrCalcVarInvalidVarType,
            700230 => Self::ErrCalcVarInvalidOp,
            700231 => Self::ErrCalcVarSetResultFail,
            700232 => Self::ErrActionEntityNoExist,
            700233 => Self::ErrActionNoInteractConfig,
            700234 => Self::ErrActionIdNoExist,
            700235 => Self::ErrActionBtObjNoExist,
            700236 => Self::ErrActionNodeNoExist,
            700237 => Self::ErrActionNoChildQuest,
            700238 => Self::ErrActionParams,
            700239 => Self::ErrActionNotEntityContext,
            700240 => Self::ErrActionExecutorNotFind,
            700241 => Self::ErrActionSessionNotFind,
            700242 => Self::ErrActionCreateSessionIdFail,
            700243 => Self::ErrActionPathConvertFail,
            700244 => Self::ErrActionConfigNotFind,
            700245 => Self::ErrActionHaveNotHandler,
            700246 => Self::ErrActionInternalError,
            700247 => Self::ErrActionInvalidIndex,
            700248 => Self::ErrActionIsNotServer,
            700249 => Self::ErrActionRemainActionNotFinish,
            700250 => Self::ErrActionExecutorIsNotBlackbard,
            700251 => Self::ErrContinuityActionNotFinish,
            700252 => Self::ErrActionIsNotContinuity,
            700253 => Self::ErrResetLocationEntityNotExist,
            700254 => Self::ErrEntityPosAbnormalNotExists,
            700255 => Self::ErrGmRemoveEntityNotExists,
            700256 => Self::ErrDrownEntityNotExists,
            700257 => Self::ErrTargetGearNotExists,
            700258 => Self::ErrOutofBattleEntityNotExists,
            700259 => Self::ErrOrderAddBuffEntityNotExists,
            700260 => Self::ErrOrderRemoveBuffEntityNotExists,
            700261 => Self::ErrActivateBuffEntityNotExists,
            700262 => Self::ErrToughCalcExtraRatioChangeEntityNotExists,
            700263 => Self::ErrAdsorbEntityNotExist,
            700264 => Self::ErrAdsorbCondNotMeet,
            700265 => Self::ErrTimelineTrackMultiGameForbid,
            700266 => Self::ErrTimelineTraceEntityNotExists,
            700267 => Self::ErrTimelineTraceComponentNotExists,
            700268 => Self::ErrTimelineTraceGroupIndex,
            700269 => Self::ErrTimelineTraceFinish,
            700270 => Self::ErrTimelineTraceCondition,
            700271 => Self::ErrTimelineTraceTargetEmpty,
            700272 => Self::ErrTimelineTraceControl,
            700273 => Self::ErrTimelineTraceFinishCondition,
            700274 => Self::ErrTimelineTraceNotInControl,
            700275 => Self::ErrForbidEnterInstance,
            700276 => Self::ErrForbitEnterBigWorld,
            700277 => Self::ErrPrefabIncIdExist,
            700278 => Self::ErrPrefabIdExist,
            700279 => Self::ErrPrefabNumberIsZero,
            700280 => Self::ErrPrefabEntityIsExist,
            700281 => Self::ErrPrefabTreasureBox,
            700282 => Self::ErrPrefabActionCreate,
            700283 => Self::ErrPrefabVarNoExist,
            700284 => Self::ErrClientOnlyEntityCantCreate,
            700285 => Self::ErrTimelineTraceActionRun,
            700286 => Self::ErrGmActivateTeleportSceneNotExist,
            700287 => Self::ErrGmCreateInstSceneHasExist,
            700288 => Self::ErrVfxNpcNotExist,
            700289 => Self::ErrVfxNpcIsNotVfxNpc,
            700290 => Self::ErrBlackboardLimit,
            700291 => Self::ErrBlackboardArrayLimit,
            700292 => Self::ErrBlackboardStringLimit,
            800000 => Self::ErrReconnectGwgetGatePlayerFailed,
            800001 => Self::ErrGwreconnectGwinvalidPlayerState,
            800002 => Self::ErrGwreconnectGwverifyTokenFailed,
            800003 => Self::ErrGwreconnectGwbackOnlineAsyncFailed,
            800004 => Self::ErrGwreconnectGwbackOnlineAsyncException,
            800005 => Self::ErrReconnectGwclientLatestSeqNoNotHit,
            800006 => Self::ErrGwreconnectConfirmGetPlayerFailed,
            800007 => Self::ErrAttrChangeHandleInvalidClientAction,
            800008 => Self::ErrThrowDamageReqGetStateComponentFailed,
            800009 => Self::ErrThrowDamageReqEntityIsAlreadyDead,
            800010 => Self::ErrAnimalDieRequestForceSetDieError,
            800011 => Self::ErrCollectEntityForceSetEntityDieError,
            800012 => Self::ErrMonsterBoomForceSetDieError,
            800013 => Self::ErrAttrChangeReqReplaceAttrListFailed,
            800014 => Self::ErrReconnectInvalidOperation,
            800015 => Self::ErrReconnectGwNodeTainted,
            900000 => Self::ErrFavorRoleNotFound,
            900001 => Self::ErrFavorConfNotFound,
            900002 => Self::ErrFavorQuestNotFound,
            900003 => Self::ErrFavorLevelRewardLimit,
            900004 => Self::ErrFavorQuestAcceptLimit,
            900005 => Self::ErrFavorItemLocked,
            900006 => Self::ErrFavorItemHasUnLocked,
            900007 => Self::ErrElevatorEntityNotExit,
            900008 => Self::ErrElevatorConfigNotExit,
            900009 => Self::ErrElevatorLocked,
            900010 => Self::ErrElevatorIsNotReverse,
            900011 => Self::ErrElevatorIsNotForward,
            900012 => Self::ErrElevatorIsNotStart,
            900013 => Self::ErrElevatorIsNotEnd,
            900014 => Self::ErrElevatorFloorError,
            900015 => Self::ErrHostRefuse,
            900016 => Self::ErrHostOffline,
            900017 => Self::ErrHostHasOnline,
            900018 => Self::ErrHostPlayerMax,
            900019 => Self::ErrHostRefuseStrangers,
            900020 => Self::ErrHostForbidJoin,
            900021 => Self::ErrHostTemporarilyForbidJoin,
            900022 => Self::ErrSlaveInBlockList,
            900023 => Self::ErrExceedJoinLevelDiff,
            900024 => Self::ErrHostNotOpenOnlineFunc,
            900025 => Self::ErrHostInOtherPlayer,
            900026 => Self::ErrHostInForbidOnlineQuest,
            900027 => Self::ErrSlaveHasOnline,
            900028 => Self::ErrSlaveInForbidOnlineQuest,
            900029 => Self::ErrSlaveNotOpenOnlineFunc,
            900030 => Self::ErrSlaveApplyRepeated,
            900031 => Self::ErrSlaveTryApplySelf,
            900032 => Self::ErrLobbyTryQuerySelf,
            900033 => Self::ErrSlaveRequestExpired,
            900034 => Self::ErrEnterringOtherScene,
            900035 => Self::ErrWaitingOtherJoin,
            900036 => Self::ErrWaitListFull,
            900037 => Self::ErrAlreayInWaitEnterList,
            900038 => Self::ErrHostNotInBigWorld,
            900039 => Self::ErrPlayerNotInBigWorld,
            900040 => Self::ErrPlayerNotInWaitList,
            900041 => Self::ErrForbidOperaInMatching,
            900042 => Self::ErrLobbyNotFoundPlayer,
            900043 => Self::ErrRoleTrailCannotOnline,
            900044 => Self::ErrHostRoleTrail,
            900045 => Self::ErrInMatchingCanNotJoinOther,
            900046 => Self::ErrInMatchCanNotBeApply,
            900047 => Self::ErrInMatchCanNotAcceptApply,
            900048 => Self::ErrSlaveInFlow,
            900049 => Self::ErrHostInFlow,
            900050 => Self::ErrAchievementNotClinet,
            900051 => Self::ErrTriggerConditionNotMet,
            900052 => Self::ErrNpcTraceNotConf,
            900053 => Self::ErrBuffProducerConfNotFound,
            900054 => Self::ErrBuffProducerHasDone,
            900055 => Self::ErrBuffConsumerConfNotFound,
            900056 => Self::ErrBuffConsumerBuffNotFound,
            900057 => Self::ErrBuffConsumerEntityNotFound,
            900058 => Self::ErrItemPosInvaild,
            900059 => Self::ErrItemIdInvaild,
            900060 => Self::ErrRouletteFuncIdInvaild,
            900061 => Self::ErrStateIsRunning,
            900062 => Self::ErrSceneEntityNotFind,
            900063 => Self::ErrActionPlayersIsEmpty,
            900064 => Self::ErrFireBulletNoLauncher,
            900065 => Self::ErrFireBulletNoTarget,
            900066 => Self::ErrTurntableConfigNotFound,
            900067 => Self::ErrTurntableActivityNotOpen,
            900068 => Self::ErrTurntableActivityIsFinish,
            900069 => Self::ErrTurntableActivityQuestNotFinish,
            900070 => Self::ErrTurntableActivityRoundConfigNotFound,
            900071 => Self::ErrEnrichmentAreaIsEmpty,
            900072 => Self::ErrEnrichmentAreaNotFind,
            900073 => Self::ErrEnrichmentAreaInCd,
            900074 => Self::ErrEnrichmentAreaInFog,
            900075 => Self::ErrEntityWalkingPoint,
            900076 => Self::ErrServerConfigReload,
            900077 => Self::ErrAreaCheckFailed,
            900078 => Self::ErrHostSceneBlockSplitFail,
            900079 => Self::ErrSlaveSceneBlockSplitFail,
            900080 => Self::ErrHookLockBatchCollectMaxCount,
            900081 => Self::ErrHookLockBatchCollectFail,
            900082 => Self::ErrEntityPackIdErr,
            900083 => Self::ErrEntityInQuickHackSkill,
            900084 => Self::ErrEntityQuickHackSkillNoAction,
            900085 => Self::ErrVehicleInPhantomFormation,
            900086 => Self::ErrGiantSystemNotOpen,
            900087 => Self::ErrGiantEcoIsLock,
            900088 => Self::ErrGiantItemInvaild,
            900089 => Self::ErrGiantItemNotEnough,
            900090 => Self::ErrGiantItemNotMatch,
            900091 => Self::ErrGiantMaxLevel,
            900092 => Self::ErrGiantMaxLevelDropLimit,
            900093 => Self::ErrGiantNotMaxLevel,
            900094 => Self::ErrGiantActivityReward,
            900095 => Self::ErrGiantActivityNodeNotActive,
            900096 => Self::ErrLoopTeleportVolumeConfig,
            900097 => Self::ErrLoopTeleportVolumeCondition,
            1000000 => Self::ErrBuffItemConfig,
            1000001 => Self::ErrBuffItemNotShare,
            1000002 => Self::ErrBuffItemShareRoleId,
            1000003 => Self::ErrBuffItemRoleIdNotExist,
            1000004 => Self::ErrBuffItemNotEnough,
            1000005 => Self::ErrBuffItemMultiUse,
            1000006 => Self::ErrBuffItemCdLimit,
            1000007 => Self::ErrBuffItemNumZero,
            1000008 => Self::ErrBuffItemNotPlayer,
            1000009 => Self::ErrSceneItemNotExit,
            1000010 => Self::ErrSceneItemType,
            1000011 => Self::ErrSceneItemOperate,
            1000012 => Self::ErrSceneItemState,
            1000013 => Self::ErrStateEntityNoExit,
            1000014 => Self::ErrStateEntityNotTagComp,
            1000015 => Self::ErrStateEntityTypeNotExit,
            1000016 => Self::ErrStateEntityStateNotExit,
            1000017 => Self::ErrStateEntityNotConfig,
            1000018 => Self::ErrStateEntityStateType,
            1000019 => Self::ErrStateEntityStateNoChange,
            1000020 => Self::ErrStateEntitySilent,
            1000021 => Self::ErrStateEntityComplete,
            1000022 => Self::ErrStateEntityLock,
            1000023 => Self::ErrStateEntityNotBorn,
            1000024 => Self::ErrStateNameNoExit,
            1000025 => Self::ErrStateInBorn,
            1000026 => Self::ErrStateCondition,
            1000027 => Self::ErrStateNotOwner,
            1000028 => Self::ErrChangeSelfStateObjNotEntity,
            1000029 => Self::ErrFoundationNotExists,
            1000030 => Self::ErrTeleControlNotExists,
            1000031 => Self::ErrFoundationNotComponent,
            1000032 => Self::ErrFoundationActived,
            1000033 => Self::ErrFoundationUnActived,
            1000034 => Self::ErrFoundationNotMatch,
            1000035 => Self::ErrFoundationNotInRange,
            1000036 => Self::ErrFoundationNotStateId,
            1000037 => Self::ErrGravityGearNotExists,
            1000038 => Self::ErrGravityGearNotConfig,
            1000039 => Self::ErrGravityGearForbidReset,
            1000040 => Self::ErrFollowTrackEntityNoExist,
            1000041 => Self::ErrFollowTrackNotComp,
            1000042 => Self::ErrFollowTrackNotFoundationId,
            1000043 => Self::ErrFollowTrackNotFoundation,
            1000044 => Self::ErrFollowTrackActiveed,
            1000045 => Self::ErrThrowPlayerNotExit,
            1000046 => Self::ErrAnimalEntityNotExist,
            1000047 => Self::ErrNotAnimalEntity,
            1000048 => Self::ErrSneakBtObjNotExist,
            1000049 => Self::ErrSneakNodeIdNotExist,
            1000050 => Self::ErrSneakNotFailedNode,
            1000051 => Self::ErrSneakNotTime,
            1000052 => Self::ErrSneakTime,
            1000053 => Self::ErrInSneak,
            1000054 => Self::ErrNotInSneak,
            1000055 => Self::ErrBeControlledEntityNotExist,
            1000056 => Self::ErrNotBeControlledEntity,
            1000057 => Self::ErrNotBeControlledPlayer,
            1000058 => Self::ErrNotBeControlledNotPlayer,
            1000059 => Self::ErrBeControlledShowEntityNotExist,
            1000060 => Self::ErrNotBeControlledShowEntity,
            1000061 => Self::ErrNotBeControlledShowPlayer,
            1000062 => Self::ErrBeControlledShowNoChange,
            1000063 => Self::ErrGravityGearCondition,
            1000064 => Self::ErrChairEntityNoExist,
            1000065 => Self::ErrChairSitDownErr,
            1000066 => Self::ErrChairEntity,
            1000067 => Self::ErrPlayerAlreadySit,
            1000068 => Self::ErrChairNotStateConfig,
            1000069 => Self::ErrSneakBtObjIncId,
            1000070 => Self::ErrTimelineMove,
            1000071 => Self::ErrBeControlledConfig,
            1000072 => Self::ErrBeControlledThrow,
            1000073 => Self::ErrBeControlledTimeNull,
            1000074 => Self::ErrTriggerEnterActionEffective,
            1000075 => Self::ErrTriggerLeaveActionEffective,
            1000076 => Self::ErrTriggerLastActionStateError,
            1100000 => Self::GuideGroupInfoIsNull,
            1100001 => Self::GuideStateError,
            1100002 => Self::GuideConfigNotFind,
            1100003 => Self::GuideNoEnough,
            1100004 => Self::GuideIsFinish,
            1100005 => Self::GuidePerIsNotFinish,
            1100006 => Self::GuideNoCondition,
            1100007 => Self::GuideNoCurGroup,
            1100008 => Self::GuideIsServerMonitor,
            1100009 => Self::GuideNoPending,
            1100010 => Self::GuideStepRepeat,
            1100011 => Self::GuideGroupNoClient,
            1100012 => Self::GuideGroupDoing,
            1100013 => Self::GuideGroupIsNotRepeat,
            1100014 => Self::GuideTutorialConfigNotFind,
            1100015 => Self::GuideTutorialIsUnlock,
            1100016 => Self::GuideTutorialNotUnlock,
            1100017 => Self::GuideTutorialIsReceive,
            1100018 => Self::GuideTutorialAwardConfigNotFind,
            1100019 => Self::GuideTutorialAwardError,
            1100020 => Self::GuideGroupIdNoMatch,
            1100021 => Self::ErrRequestTypeNotExist,
            1100022 => Self::ErrIllustratedEntryLock,
            1100023 => Self::ErrIllustratedEntryBanUnlock,
            1100024 => Self::ErrRequestTypeMax,
            1100025 => Self::AchievementEntryNotExist,
            1100026 => Self::AchievementEntryNotFinish,
            1100027 => Self::AchievementEntryIsReceive,
            1100028 => Self::AchievementEntryNoConfig,
            1100029 => Self::AchievementEntryNotOpen,
            1100030 => Self::AchievementGroupEntryNotExist,
            1100031 => Self::AchievementGroupEntryNotFinish,
            1100032 => Self::AchievementGroupEntryIsReceive,
            1100033 => Self::AchievementGroupEntryNoConfig,
            1100034 => Self::AchievementGroupEntryNotOpen,
            1100035 => Self::SilentAreaNotConfig,
            1100036 => Self::SilentAreaNotUnlock,
            1100037 => Self::SilentAreaNotFinish,
            1100038 => Self::SilentAreaReceive,
            1100039 => Self::AchievementEntryIsFinish,
            1100040 => Self::AchievementEntryNeedCondition,
            1100041 => Self::AchievementSceneNotFind,
            1100042 => Self::BirthdayIsSetting,
            1100043 => Self::BirthdayInValid,
            1100044 => Self::RoleShowListMaxCount,
            1100045 => Self::RoleShowListHasRepeatId,
            1100046 => Self::RoleShowListHasInValidId,
            1100047 => Self::CardShowListMaxCount,
            1100048 => Self::CardShowListHasRepeatId,
            1100049 => Self::CardShowListHasInValidId,
            1100050 => Self::CardRepeatSet,
            1100051 => Self::CardIsInValidId,
            1100052 => Self::CardIsRead,
            1100053 => Self::RoleShowListEmpty,
            1100054 => Self::SettingNotFind,
            1100055 => Self::RogueRoadConfigNotFind,
            1100056 => Self::RollRogueRoomError,
            1100057 => Self::RollRogueBuffError,
            1100058 => Self::GetRogueRoomIdsError,
            1100059 => Self::GetRoguePortalEntityNotFind,
            1100060 => Self::GetRoguePortalLocationNotFind,
            1100061 => Self::HttpTimeout,
            1100062 => Self::HttpResultUndefine,
            1100063 => Self::ConvGateTimeout,
            1100064 => Self::ProtoKeyTimeout,
            1100065 => Self::LoginReqTimeout,
            1100066 => Self::EnterGameTimeout,
            1100067 => Self::ReReconvReqTimeout,
            1100068 => Self::RecvSeqNoNotHit,
            1100069 => Self::AchievementFuncNotOpen,
            1100070 => Self::RoguelikeInstComponentNotFind,
            1100071 => Self::RogueCurRoomDataIsNull,
            1100072 => Self::LevelPlayComponentNotFind,
            1100073 => Self::OpenLevelPlayFail,
            1100074 => Self::CloseLevelPlayFail,
            1100075 => Self::RogueRoomConfigNotFind,
            1100076 => Self::RogueRoomTypeNotRight,
            1100077 => Self::RogueRoomTypeNotConfig,
            1100078 => Self::RogueRoomSubLevelNotFind,
            1100079 => Self::SelectNextRoomIsValid,
            1100080 => Self::RogueGainPackageFail,
            1100081 => Self::RogueGainListIsNull,
            1100082 => Self::RogueGainIdValid,
            1100083 => Self::QulityListCountNotRight,
            1100084 => Self::RandomResultCountNotRight,
            1100085 => Self::GuaranteeRogueBuffInValid,
            1100086 => Self::RoleBuffPoolNotFind,
            1100087 => Self::NotValidBuff,
            1100088 => Self::NotValidPhantom,
            1100089 => Self::RandomPhantomFail,
            1100090 => Self::NotValidRole,
            1100091 => Self::RandomRoleFail,
            1100092 => Self::RogueRoadNotFind,
            1100093 => Self::ResultCountNotMatch,
            1100094 => Self::InValidRoomCountNotMatch,
            1100095 => Self::GuaranteeRogueRoomInValid,
            1100096 => Self::InstIdNotMatchLevelPlayId,
            1100097 => Self::GetRoomBornPositionFail,
            1100098 => Self::RoguePortalDataNotClean,
            1100099 => Self::RoguePortalRoomDataNotFind,
            1100100 => Self::RogueSelectRoomFail,
            1100101 => Self::RogueProgressDataIsEmpty,
            1100102 => Self::RogueGainTypeIsValid,
            1100103 => Self::RougeNotOpen,
            1100104 => Self::RougeInstIdIsValid,
            1100105 => Self::RogueRoleListCountNotRight,
            1100106 => Self::RogueMainRoleConfigNotFind,
            1100107 => Self::RogueGainDataDictError,
            1100108 => Self::RogueDiscountedBuffConfigNotFind,
            1100109 => Self::RogueDiscountedRoomTypeConfigNotFind,
            1100110 => Self::RogueDiscountedShopConfigNotFind,
            1100111 => Self::RogueDiscountedCalculateFail,
            1100112 => Self::RogueMoneyNotEnough,
            1100113 => Self::RougeShopRefreshTimeEmpyt,
            1100114 => Self::RougeCurRoomNotFinish,
            1100115 => Self::PlayerDataRepairErrorDebug,
            1100116 => Self::PlayerDataRepairError,
            1100117 => Self::CreateCharacterReqTimeout,
            1100118 => Self::SignActivityNotOpen,
            1100119 => Self::SignActivityNoConfig,
            1100120 => Self::SignActivityIndexValid,
            1100121 => Self::SignActivityNoData,
            1100122 => Self::SignActivityStateNotRight,
            1100123 => Self::RogueSeasonDataNull,
            1100124 => Self::RogueSeasonConfigNotFind,
            1100125 => Self::RogueTokenConfigNotFind,
            1100126 => Self::RogueTokenStatusVaild,
            1100127 => Self::RogueSeasonRewardConfigNotFind,
            1100128 => Self::RogueSeasonRewardIsReceive,
            1100129 => Self::RougeSeasonPointNotEnough,
            1100130 => Self::RougeRoomDataError,
            1100131 => Self::RogueGainDataError,
            1100132 => Self::RogueRoleIdsError,
            1100133 => Self::RogueRogueRoomRouteError,
            1100134 => Self::RogueGetCurRoomLevelPlayError,
            1100135 => Self::RogueTalentTreeConfigNotFind,
            1100136 => Self::RogueTalentTreeConditionNotMet,
            1100137 => Self::RogueTalentTreePerNodeLock,
            1100138 => Self::RogueTalentTreeNodeMaxLevel,
            1100139 => Self::RogueTalentTreeConsumeNoEnough,
            1100140 => Self::RogueRoadRandomRoleBuffError,
            1100141 => Self::ActivityFuncNotOpen,
            1100142 => Self::RogueGuideInstNotSupport,
            1100143 => Self::ErrPayReceiptCannotRefundClose,
            1100144 => Self::ErrPayReceiptRefundCloseFail,
            1100145 => Self::PayRefundOverdueBan,
            1100146 => Self::UnknowChannelId,
            1100147 => Self::LoginServerNotFind,
            1100148 => Self::OldGameNodeLogoutFail,
            1100149 => Self::LoginHandleSwitchError,
            1100150 => Self::NoAvailableLoginService,
            1100151 => Self::ServerIsClosing,
            1100152 => Self::AddPlayerRecordFail,
            1100153 => Self::FindGatewayFail,
            1100154 => Self::CommonFightRolesInfoError,
            1100155 => Self::CurRoleEntityNotFind,
            1100156 => Self::ScenePlayerInfoNotFind,
            1100157 => Self::IncrAdviceVoteError,
            1100158 => Self::InsertAdviceError,
            1100159 => Self::UpdateAdviceError,
            1100160 => Self::DeleteAdviceError,
            1100161 => Self::EntityNoInWater,
            1100162 => Self::AttributeComponentNotFind,
            1100163 => Self::TryAddItemDataFail,
            1100164 => Self::ItemConfigTypeNotRight,
            1100165 => Self::ItemLogicNotFind,
            1100166 => Self::RemoveItemLogicNotFind,
            1100167 => Self::AddItemLogicNotFind,
            1100168 => Self::AddItemFail,
            1100169 => Self::UpdatePlayerAremarkFail,
            1100170 => Self::DeleteFriendApplyFail,
            1100171 => Self::DeleteFriendshipFail,
            1100172 => Self::WorldTeamIsNull,
            1100173 => Self::TeamCountNotRight,
            1100174 => Self::AddCalabashExpFail,
            1100175 => Self::SendRequestToSdkFail,
            1100176 => Self::DirtyWordErrorCode,
            1100177 => Self::HarvestActivityNotOpen,
            1100178 => Self::HarvestActivityPointReceived,
            1100179 => Self::HarvestActivityPointNotConfig,
            1100180 => Self::HarvestActivityPointNotEnough,
            1100181 => Self::HarvestActivityLevelNoData,
            1100182 => Self::HarvestActivityLevelReceived,
            1100183 => Self::HarvestActivityLevelNotConfig,
            1100184 => Self::HarvestActivityLevelNotEnough,
            1100185 => Self::HarvestActivityLevelDiffNotConfig,
            1100186 => Self::RoguelikeEventConfigNotFind,
            1100187 => Self::RoguelikeEventIndexError,
            1100188 => Self::RoguelikeInstConfigNotFind,
            1100189 => Self::RoguelikeMainRoleError,
            1100190 => Self::RoguelikeEventIsEmpty,
            1100191 => Self::RoguelikeEventRandomError,
            1100192 => Self::RoguelikeEventRandomEmpty,
            1100193 => Self::PhantomCollectActivityNotOpen,
            1100194 => Self::PhantomCollectActivitynNotConfig,
            1100195 => Self::PhantomCollectActivitynNoData,
            1100196 => Self::PhantomCollectActivityReceived,
            1100197 => Self::HarvestInstIdInValid,
            1100198 => Self::HarvestVarNotExist,
            1100199 => Self::HarvestResultCacheNotExist,
            1100200 => Self::HarvestInstNotOpen,
            1100201 => Self::HarvestActivityLimitDataNotFind,
            1100202 => Self::HarvestDiffConfigNotFind,
            1100203 => Self::HarvestActivityDiffConfigNotFind,
            1100204 => Self::ErrIllustratedConfigNotFind,
            1100205 => Self::CharacterAlreadyCreated,
            1100206 => Self::SdkHelperInternalError,
            1100207 => Self::GameServiceControllerInternalError,
            1100208 => Self::DoGetCacheInfoInternalError,
            1100209 => Self::DoGetCacheInfosInternalError,
            1100210 => Self::DeleteFriendLoadedInternalError,
            1100211 => Self::UpdateFriendRemarkInternalError,
            1100212 => Self::CheckApplyRequestInternalError,
            1100213 => Self::OnReLoginInternalError,
            1100214 => Self::CreateCharacterRequestInternalError,
            1100215 => Self::LoginRequestInternalError,
            1100216 => Self::LoginRequestInternalError2,
            1100217 => Self::EnterGameRequestInternalError,
            1100218 => Self::ReconnectRequestInternalError,
            1100219 => Self::ReconnectRequestInternalError2,
            1100220 => Self::SwitchNodeInternalError,
            1100221 => Self::InnerLoginInternalError,
            1100222 => Self::AccessTokenInternalError,
            1100223 => Self::CreateCharacterInternalError,
            1100224 => Self::RogueSeasonNotValid,
            1100225 => Self::RogueCurRoleNotFind,
            1100226 => Self::RogueSeasonNotMatch,
            1100227 => Self::RogueGainLogicNotFind,
            1100228 => Self::RogueBuffConfigNotFind,
            1100229 => Self::RoguePhantomNotConfig,
            1100230 => Self::RogueRoleNotConfig,
            1100231 => Self::RoguePopularSlotConfigNotFind,
            1100232 => Self::RoguePopularCountIsMax,
            1100233 => Self::RoguePopularConfigNotFind,
            1100234 => Self::RogueRoleNotOpen,
            1100235 => Self::RogueGuideInstError,
            1100236 => Self::RogueMainRoleChange,
            1100237 => Self::RogueShopConfigNull,
            1100238 => Self::RogueGainIsSelect,
            1100239 => Self::RogueGainNoRefresh,
            1100240 => Self::RogueRefreshCostNotFind,
            1100241 => Self::RogueNotMaxLayer,
            1100242 => Self::RogueRoomSubLevelNotFind2,
            1100243 => Self::RoguePopularSlotArgConfigNotFind,
            1100244 => Self::RogueInstSeasonNotMatch,
            1100245 => Self::RogueSeasonTalentTreeNotFind,
            1100246 => Self::RogueGainOptionsNotFind,
            1100247 => Self::RogueGainIsSell,
            1100248 => Self::RogueMiracleCreationConfNotFind,
            1100249 => Self::RogueGainPackageError,
            1100250 => Self::RogueTrialRoleIdsCountNotRight,
            1100251 => Self::RogueVarNotExist,
            1100252 => Self::RougeWhiteCatConfigNotFind,
            1100253 => Self::RougeWhiteCatNotOpen,
            1100254 => Self::RougeWhiteCatLimitedTime,
            1100255 => Self::RougeWhiteCatRewardLock,
            1100256 => Self::RougeWhiteCatRewardIsReceive,
            1100257 => Self::RougeWhiteCatRewardIndexErr,
            1100258 => Self::RougeWhiteCatInstIndexErr,
            1100259 => Self::RougeWhiteCatInstLock,
            1100260 => Self::RougeWhiteCatBossRewardIndexErr,
            1100261 => Self::RougeWhiteCatBossRewardLock,
            1100262 => Self::RougeWhiteCatBossRewardIsReceive,
            1100263 => Self::RougeWhiteCatLevelPlayIndexErr,
            1100264 => Self::RougeWhiteCatLevelPlayLock,
            1100265 => Self::RougeWhiteCatLevelPlayIsReceive,
            1100266 => Self::ResourceVersionTooLow,
            1100267 => Self::RogueLimitTimeRewardConfigNotFind,
            1100268 => Self::RogueWhiteCatLimitedTimeOut,
            1100269 => Self::RougeWhiteCatLimitedRewardLock,
            1100270 => Self::RougeWhiteCatLimitedRewardIsReceive,
            1100271 => Self::RougeWhiteCatBlackFlowerNoCount,
            1100272 => Self::RogueInstCountNotRight,
            1100273 => Self::RogueInstFightFormationNotConfig,
            1100274 => Self::RogueTrialRoleNotValid,
            1100275 => Self::RogueRoleNotValid,
            1100276 => Self::ErrorBlackFlowerEntityNotRight,
            1100277 => Self::ErrorBlackFlowerStatus,
            1100278 => Self::ErrorBlackFlowerCanNotReward,
            1100279 => Self::ErrorBlackFlowerRewardFail,
            1100280 => Self::ErrorPhantomUnlockError,
            1100281 => Self::ErrorPhantomSwitchError,
            1100282 => Self::ActivityConfigNotFind,
            1100283 => Self::ActivityNotOpen,
            1100284 => Self::DirectTrainActivityConfigNotFind,
            1100285 => Self::ActivityTypeNotFind,
            1100286 => Self::SetGlobalVarFail,
            1100287 => Self::ErrMultigame,
            1100288 => Self::RogueWeeklyCycleNoFind,
            1100289 => Self::RogueWeeklyCycleIdNotMatch,
            1100290 => Self::RogueWeeklyCycleInstIdNotMatch,
            1100291 => Self::RogueWeeklyCycleSexNotMatch,
            1100292 => Self::RogueWeeklyCycleActivityIdNotMatch,
            1100293 => Self::RogueWeeklyCycleAwardNotFind,
            1100294 => Self::RogueWeeklyCycleAwardStateNotMatch,
            1100295 => Self::RogueWeeklyInstResultFail,
            1100296 => Self::RogueWeeklyGoldNoEnough,
            1100297 => Self::HasRogueProgressCanNotChangeSex,
            1100298 => Self::RogueWeeklyWorldLevelNotMatch,
            1100299 => Self::ResourceVersionIsTooLowWithTips,
            1100300 => Self::RogueResInstIdNotMatch,
            1100301 => Self::RogueResInstGridConfigNotFind,
            1100302 => Self::RogueResInBossInst,
            1100303 => Self::RogueResCurGridIsNull,
            1100304 => Self::RogueResPathCountFail,
            1100305 => Self::RogueResPathRepeat,
            1100306 => Self::RogueResPathGridNoNear,
            1100307 => Self::RogueResPathGridNoValid,
            1100308 => Self::RogueResPathGridNoVision,
            1100309 => Self::RogueResPathGridBlock,
            1100310 => Self::RogueResPlayerBanMove,
            1100311 => Self::RogueResHasOp,
            1100312 => Self::RogueResTrialRoleNotFind,
            1100313 => Self::RogueResTrialRoleNoValid,
            1100314 => Self::RogueResThemeConfNotFind,
            1100315 => Self::RogueResCrossInstDataIsNull,
            1100316 => Self::RogueResOpEmpty,
            1100317 => Self::RogueResOpNotMatch,
            1100318 => Self::RogueResCollectionConfNotFind,
            1100319 => Self::RogueResCollectionStateErr,
            1100320 => Self::RogueResEndingAwardConfNotFind,
            1100321 => Self::RogueResEndingAwardIsReceived,
            1100322 => Self::RogueResEndingAwardNotFinish,
            1100323 => Self::RogueResTalentConfNotFind,
            1100324 => Self::RogueResTaskConfNotFind,
            1100325 => Self::RogueResTaskDatanotFind,
            1100326 => Self::RogueResTaskAwardIsReceived,
            1100327 => Self::RogueResTaskAwardNotFinish,
            1100328 => Self::RogueResEffectConfNoFind,
            1100329 => Self::RogueResOptionNoRestCount,
            1100330 => Self::RogueResOptionCantMulti,
            1100331 => Self::RogueResOptionRepeat,
            1100332 => Self::RogueResOptionCantGiveUp,
            1100333 => Self::RogueResOptionCantRefresh,
            1100334 => Self::RogueResOptionIndexNoValid,
            1100335 => Self::RogueResOptionTypeError,
            1100336 => Self::RogueResOptionIsSelect,
            1100337 => Self::RogueResCollectionIndexMax,
            1100338 => Self::RogueResCollectionDropConfigNoFind,
            1100339 => Self::RogueResNoBaseGainLogic,
            1100340 => Self::RogueResGridAwardNoConf,
            1100341 => Self::RogueResBornPositionNoConf,
            1100342 => Self::RogueResEffectExecFail,
            1100343 => Self::RogueResOptionFinish,
            1100344 => Self::RogueResOpTypeErr,
            1100345 => Self::RogueResGridEventNoConf,
            1100346 => Self::RogueResGridEventStepNoConf,
            1100347 => Self::RogueResStepOptionNoValid,
            1100348 => Self::RogueResGridEventNoData,
            1100349 => Self::RogueResSelectIndexNoValid,
            1100350 => Self::RogueResGotoNextRoomNoData,
            1100351 => Self::RogueResSelectOpNoData,
            1100352 => Self::RogueResBranchTaskNoConf,
            1100353 => Self::RogueResBranchTaskNoData,
            1100354 => Self::RogueResBranchTaskIsReceive,
            1100355 => Self::RogueResBranchTaskNoFinish,
            1100356 => Self::RogueResTokenShopPriceErr,
            1100357 => Self::RogueResTaskTypeErr,
            1100358 => Self::RogueRandomNoPoolName,
            1100359 => Self::RogueRandomNoHit,
            1100360 => Self::RogueRandomNoValidResult,
            1100361 => Self::RogueResFormationIdNoValid,
            1100362 => Self::RogueResFormationRoleCountErr,
            1100363 => Self::RogueResRandomNoShopGoods,
            1100364 => Self::RogueResRandomShopGoodsNoEnough,
            1100365 => Self::RogueResFormationRoleNoValid,
            1100366 => Self::RogueResFormationNoData,
            1100367 => Self::RogueResGainRoleNoFind,
            1100368 => Self::RogueResGainRoleLvNoConf,
            1100369 => Self::RogueResGainRoleLvNoLv,
            1100370 => Self::RogueResGainRoleNoBuffPool,
            1100371 => Self::RogueResGainRoleNoMaxlv,
            1100372 => Self::RogueResGainRoleRollEffectNoValid,
            1100373 => Self::RogueResGainRoleRollRetNoMatch,
            1100374 => Self::RogueResGainRoleShopNoConf,
            1100375 => Self::RogueResGainRoleRollEmpty,
            1100376 => Self::RogueResGainRoleBondNoValid,
            1100377 => Self::RogueResRoleBuffGuaranteeFail,
            1100378 => Self::RogueResRoleBuffNoMore,
            1100379 => Self::RogueResRoleBuffRandomFail,
            1100380 => Self::RogueRoleShopRollFail,
            1100381 => Self::RogueResGridRangeTrigger,
            1100382 => Self::RogueResCurGridTrigger,
            1100383 => Self::RogueResEventTypeErr,
            1100384 => Self::RogueResEventOpNoData,
            1100385 => Self::RogueResGridNoConf,
            1100386 => Self::RogueResEffectExceFail,
            1100387 => Self::RogueResNoInBattle,
            1100388 => Self::RogueResStepCondLock,
            1100389 => Self::RogueResGridEventOpNotFind,
            1100390 => Self::RogueNoLastInstData,
            1100391 => Self::RogueNoRollBuffBondLinkOp,
            1100392 => Self::RogueTokenCantRepeat,
            1100393 => Self::RogueFailCountMax,
            1100394 => Self::RogueResRoleMaxLv,
            1100395 => Self::RogueResPerInstNoPass,
            1100396 => Self::RogueResCreateInstCacheErr,
            1100397 => Self::LordGymRepeatChallenge,
            1100398 => Self::NewbieCarnivalNoLicense,
            1100399 => Self::NewbieCarnivalParamNoConf,
            1100400 => Self::NewbieCarnivalNoRoleId,
            1100401 => Self::NewbieCarnivalNoData,
            1100402 => Self::NewbieCarnivalTaskNoConf,
            1100403 => Self::NewbieCarnivalTaskTaken,
            1100404 => Self::NewbieCarnivalTaskRunning,
            1100405 => Self::ErrShopFixeNotExist,
            1100406 => Self::ErrShopBankNoExist,
            1100407 => Self::ErrShopBankNoMatch,
            1100408 => Self::ErrPayShopBuyCountOverFlow,
            1100409 => Self::ErrShopBuyCountOverFlow,
            1100410 => Self::EnergyOverFlow,
            1100411 => Self::EnergyNotEnough,
            1100412 => Self::StoreEnergyNotEnough,
            1100413 => Self::ErrPayShopBuyLimitCondition,
            1100414 => Self::ErrPayShopEchoItemOver2,
            1100415 => Self::ErrPayShopEchoItemOver3,
            1100416 => Self::ErrPayShopEchoItemOver4,
            1100417 => Self::CreateReceiptCoreParamError,
            1100418 => Self::ReceiptsDealCloseCoreParamError,
            1100419 => Self::ReceiptsRefundCloseCoreParamError,
            1100420 => Self::CreateReceiptCoreException,
            1100421 => Self::ReceiptsDealCloseCoreException,
            1100422 => Self::ReceiptsRefundCloseCoreFail,
            1100423 => Self::ReceiptsRefundCloseCoreException,
            1100424 => Self::ErrPayConfigClientCantBuy,
            1100425 => Self::RecyclePersonalGiftNoData,
            1100426 => Self::ErrPersonalGiftBuyLimit,
            1100427 => Self::CreateReceiptNoConf,
            1100428 => Self::CreateReceiptFail,
            1100429 => Self::ReceiptDealCloseNoData,
            1100430 => Self::ReceiptDealClosePlayerIdUnMatch,
            1100431 => Self::ReceiptCannotDealClose,
            1100432 => Self::ReceiptDealCloseFail,
            1100433 => Self::ReceiptRefundCloseNoData,
            1100434 => Self::ReceiptRefundClosePlayerIdUnMatch,
            1100435 => Self::ReceiptCannotRefundClose,
            1100436 => Self::ReceiptRefundCloseFail,
            1100437 => Self::ReceiptRefundNoData,
            1100438 => Self::CreateReceiptParamError,
            1100439 => Self::CreateReceiptException,
            1100440 => Self::ReceiptsDealCloseParamError,
            1100441 => Self::ReceiptsDealCloseException,
            1100442 => Self::ReceiptRefundCloseParamError,
            1100443 => Self::ReceiptRefundCloseException,
            1100444 => Self::ReceiptRefundParamError,
            1100445 => Self::UpdateRogueWeeklyArtifactsFail,
            1100446 => Self::RogueWeeklyArtifactsEmpty,
            1100447 => Self::RogueWeeklyArtifactsIndexErr,
            1100448 => Self::RogueResSkipBattleLvNoEnough,
            1100449 => Self::RogueResCanNotSkipBattle,
            1100450 => Self::RogueResRandomGridNoConf,
            1100451 => Self::StrangerHostCount,
            1100452 => Self::SurvivorsNotInitOp,
            1100453 => Self::SurvivorsNotSelectOp,
            1100454 => Self::SurvivorsNoRoleAndWeapon,
            1100455 => Self::SurvivorsRandomEmpty,
            1100456 => Self::SurvivorsOptionEmpty,
            1100457 => Self::SurvivorsSelectEmpty,
            1100458 => Self::SurvivorsRandomGuaranteedEmpty,
            1100459 => Self::SurvivorsLevelNoConf,
            1100460 => Self::SurvivorsWeaponPoolEmpty,
            1100461 => Self::SurvivorsWeaponEmpty,
            1100462 => Self::SurvivorsGoldNoEnough,
            1100463 => Self::SurvivorsWeaponMax,
            1100464 => Self::SurvivorsNoSupportGiveUp,
            1100465 => Self::SurvivorsNoRestSelectCount,
            1100466 => Self::SurvivorsNoSupportMultiSelect,
            1100467 => Self::SurvivorsRepeatSelect,
            1100468 => Self::SurvivorsComponentNoFind,
            1100469 => Self::SurvivorsOpNotFind,
            1100470 => Self::SurvivorsSelectErr,
            1100471 => Self::SurvivorsNoVar,
            1100472 => Self::SurvivorsInstIdErr,
            1100473 => Self::SurvivorsInstLock,
            1100474 => Self::SurvivorsInstNotTime,
            1100475 => Self::SurvivorsInstNoEndlessMode,
            1100476 => Self::SurvivorsNeedContinue,
            1100477 => Self::SurvivorsLevelCantUseRole,
            1100478 => Self::SurvivorsLevelCantUseWeapon,
            1100479 => Self::SurvivorsRoleNoMatch,
            1100480 => Self::SurvivorsStepErr1,
            1100481 => Self::SurvivorsStepErr2,
            1100482 => Self::SurvivorsStepErr3,
            1100483 => Self::SurvivorsCantBuy,
            1100484 => Self::SurvivorsRoleLvNoConf,
            1100485 => Self::SurvivorsWeaponLvNoConf,
            1100486 => Self::SurvivorsRoleNoFind,
            1100487 => Self::SurvivorsWaponNoFind,
            1100488 => Self::SurvivorsItemNoFind,
            1100489 => Self::SurvivorsRoleLvMax,
            1100490 => Self::SurvivorsWeaponLvMax,
            1100491 => Self::SurvivorsRefreshOpFail,
            1100492 => Self::SurvivorsWeightEmpty,
            1100493 => Self::SurvivorsActivityNoData,
            1100494 => Self::SurvivorsRefreshCostFail,
            1100495 => Self::SurvivorsNoLastInstData,
            1100496 => Self::SurvivorsActivityNoConf,
            1100497 => Self::SurvivorsTalentNoConf,
            1100498 => Self::SurvivorsTaskCountMax,
            1100499 => Self::SurvivorsTaskNoConf,
            1100500 => Self::SurvivorsTaskCantReward,
            1100501 => Self::SurvivorsTaskRepeat,
            1100502 => Self::SurvivorsTaskEmpty,
            1100503 => Self::SurvivorsAllLock,
            1100504 => Self::RogueResNoGlobalConf,
            1100506 => Self::SurvivorsNeedPassNormalMode,
            1100507 => Self::SurvivorsOptionIsSelect,
            1100508 => Self::SurvivorsTalentTreeConditionNotMet,
            1100509 => Self::SurvivorsTalentTreePerNodeLock,
            1100510 => Self::SurvivorsTalentTreeNodeMaxLevel,
            1100511 => Self::SurvivorsTalentTreeConsumeNoEnough,
            1100512 => Self::ShortMessageNoConf,
            1100513 => Self::ShortMessageOptionNoConf,
            1100514 => Self::ShortMessageNoData,
            1100515 => Self::ShortMessageNoData2,
            1100516 => Self::ShortMessageIsRead,
            1100517 => Self::ShortMessageNoMatch,
            1100518 => Self::ShortMessageNoReply,
            1100519 => Self::ShortMessageIsReply,
            1100520 => Self::ShortMessageNoReward,
            1100521 => Self::ShortMessageIsReward,
            1100522 => Self::ShortMessageNoFinish,
            1100523 => Self::ErrorPhantomCollectMax,
            1100524 => Self::ErrorPhantomCollectRepeated,
            1100525 => Self::ErrorPhantomCollectLock,
            1100526 => Self::ErrorNoSceneComp,
            1100527 => Self::ErrorNewTowerIsInChallenge,
            1100528 => Self::ErrorNewTowerIsNoChallenge,
            1100529 => Self::ErrorNewTowerLevelIdNotMatch,
            1100530 => Self::ErrorNewTowerLevelNoConf,
            1100531 => Self::ErrorNewTowerCycleNoConf,
            1100532 => Self::ErrorNewTowerLevelNoData,
            1100533 => Self::ErrorNewTowerNoTeamCache,
            1100534 => Self::ErrorNewTowerNoIndexTeamCache,
            1100535 => Self::ErrorNewTowerScoreRewardMax,
            1100536 => Self::ErrorNewTowerScoreNoConf,
            1100537 => Self::ErrorNewTowerScoreRepeat,
            1100538 => Self::ErrorNewTowerScoreEmpty,
            1100539 => Self::ErrorNewTowerCycleNoInTime,
            1100540 => Self::ErrorNewTowerCurCycleNoConf,
            1100541 => Self::ErrorNewTowerCycleNoLevel,
            1100542 => Self::ErrorNewTowerTeamIndexErr,
            1100543 => Self::ErrorNewTowerTeamMax,
            1100544 => Self::ErrorNewTowerBuffMax,
            1100545 => Self::ErrorNewTowerBuffRepeat,
            1100546 => Self::ErrorNewTowerRoleRepeat,
            1100547 => Self::ErrorNewTowerRoleEmpty,
            1100548 => Self::ErrorNewTowerRoleNoMatch,
            1100549 => Self::ErrorNewTowerRoleMax,
            1100550 => Self::ErrorNewTowerBuffNoMatch,
            1100551 => Self::ErrorNewTowerRoleNoEnergy,
            1100552 => Self::ErrorNewTowerWaveNoInfo,
            1100553 => Self::ErrorNewTowerWaveFinish,
            1100554 => Self::ErrorShortMessageNoOpen,
            1100555 => Self::ErrorShortMessageOptionMax,
            1100556 => Self::ErrorShortMessageBubbleNoData,
            1100557 => Self::ErrorShortMessageBubbleNoConf,
            1100558 => Self::ErrorShortMessageChatBgNoData,
            1100559 => Self::ErrorShortMessageChatBgNoConf,
            1100560 => Self::ErrorNewTowerInstStateErr,
            1100561 => Self::ErrorNewTowerRoleWeaponPhantom,
            1100562 => Self::ErrorNewTowerNoTeamData,
            1100563 => Self::ErrorNewTowerNoCacheSet,
            1100564 => Self::ErrorNewTowerRoleWeaponNoMatch,
            1100565 => Self::ErrorNewTowerRolePhantomNoMatch,
            1100566 => Self::ShortMessageOptionNoConf2,
            1100567 => Self::ShortMessageOptionNoConf3,
            1100568 => Self::ErrorNewTowerEntityNoFind,
            1100569 => Self::ErrorNewTowerSeasonAwardNoConf,
            1100570 => Self::ErrorNewTowerSeasonNoConf,
            1100571 => Self::ErrorNewTowerSeasonNotOpen,
            1100572 => Self::ErrorNewTowerSeasonAwardRepeat,
            1100573 => Self::ErrorNewTowerSeasonAwardEmpty,
            1100574 => Self::NewTowerNoLastCycleIdReview,
            1100575 => Self::NewTowerNoActivityData,
            1100576 => Self::NewTowerNoHistoryData,
            1100577 => Self::ErrorNewTowerSeasonAwardTimeout,
            1100578 => Self::RogueInstConfNoFind,
            1100579 => Self::RoguePhantomLock,
            1100580 => Self::RogueRoleLock,
            1100581 => Self::RogueHotEntryLock,
            1100582 => Self::KurotatoNoActivityData,
            1100583 => Self::KurotatoNoActivityConf,
            1100584 => Self::KurotatoTaskLenghMax,
            1100585 => Self::KurotatoTaskNoConf,
            1100586 => Self::KurotatoTaskNoFinish,
            1100587 => Self::KurotatoTaskIsAward,
            1100588 => Self::KurotatoTaskRepeat,
            1100589 => Self::KurotatoTaskZero,
            1100590 => Self::KurotatoCanNotSaveRole,
            1100591 => Self::KurotatoIsSaveRole,
            1100592 => Self::KurotatoIsInInst,
            1100593 => Self::KurotatoNoLevelData,
            1100594 => Self::KurotatoNoRoleData,
            1100595 => Self::KurotatoNoRecord,
            1100596 => Self::KurotatoHasRecord,
            1100597 => Self::KurotatoRoleNoRecord,
            1100598 => Self::KurotatoRecordLevelNotMatch,
            1100599 => Self::KurotatoRoleNotMatch,
            1100600 => Self::HasRoguelikeProgressCanNotChangeSex,
            1100601 => Self::WeeklyFrameNoData,
            1100602 => Self::WeeklyFrameScoreAwardMax,
            1100603 => Self::WeeklyFrameNotInTime,
            1100604 => Self::WeeklyFrameNoConf,
            1100605 => Self::WeeklyFrameScoreAwardNoConf,
            1100606 => Self::WeeklyFrameScoreAwardGroupNotMatch,
            1100607 => Self::WeeklyFrameScoreAwardNotFinish,
            1100608 => Self::WeeklyFrameScoreAwardTaken,
            1100609 => Self::WeeklyFrameScoreAwardRepeat,
            1100610 => Self::WeeklyFrameScoreAwardZero,
            1100611 => Self::KurotatoItemLock,
            1100612 => Self::KurotatoWeaponLock,
            1100613 => Self::KurotatoLevelNotOpen,
            1100614 => Self::HasTempRogueArchive,
            1100615 => Self::NoTempRogueArchive,
            1100616 => Self::RogueArchiveSlotErr,
            1100617 => Self::NoRogueTowerTrial,
            1100618 => Self::RogueTowerTrialInstNoConf,
            1100619 => Self::RogueTowerTrialNoConf,
            1100620 => Self::RogueTowerTrialInstNoData,
            1100621 => Self::RogueTowerTrialInstInChallenge,
            1100622 => Self::RogueTowerTrialInstBossErr,
            1100623 => Self::NoRogueArchive,
            1100624 => Self::RogueArchiveInvalid,
            1100625 => Self::KurotatoStepErr1,
            1100626 => Self::KurotatoStepErr2,
            1100627 => Self::ErrorNewTowerAwardCantGet,
            1100628 => Self::KurotatoShopProductIsBought,
            1100629 => Self::KurotatoShopProductAllLock,
            1100630 => Self::ErrorShortMessageChatThemeNoData,
            1100631 => Self::ErrorShortMessageChatThemeNoConf,
            1100632 => Self::ErrorShortMessageChatCharmNoData,
            1100633 => Self::ErrorShortMessageChatCharmNoConf,
            1100505 => Self::RogueResLimitedRoles,
            1100634 => Self::RoleTowerNoActivityData,
            1100635 => Self::RoleTowerNoActivityConf,
            1100636 => Self::RoleTowerTaskLengthMax,
            1100637 => Self::RoleTowerTaskNoConf,
            1100638 => Self::RoleTowerTaskNoFinish,
            1100639 => Self::RoleTowerTaskIsAward,
            1100640 => Self::RoleTowerTaskRepeat,
            1100641 => Self::RoleTowerTaskZero,
            1100642 => Self::RoleTowerNotInLimitTime,
            1100643 => Self::RoleTowerContextEmpty,
            1100644 => Self::RoleTowerMultiModeNotSupported,
            1100645 => Self::RoleTowerLevelConfigNotExist,
            1100646 => Self::RoleTowerLevelNotOpen,
            1100647 => Self::RoleTowerNoLevelData,
            1100648 => Self::RoleTowerNoRecord,
            1100649 => Self::RoleTowerRecordLevelNotMatch,
            1100650 => Self::RoleTowerRoleCountInvalid,
            1100651 => Self::RoleTowerRoleNotMatch,
            1100652 => Self::RoleTowerRoleRepeat,
            1100653 => Self::RoleTowerRoleEnergyNotEnough,
            1100654 => Self::RoleTowerStageConfigNotExist,
            1100655 => Self::RoleTowerTokenConfigNotExist,
            1100656 => Self::RoleTowerTokenActivityNotMatch,
            1100666 => Self::RoleTowerConsumeItemNotInProgress,
            1100667 => Self::RoleTowerConsumeItemNotUsable,
            1100668 => Self::RoleTowerConsumeItemNotEnough,
            1100669 => Self::RoleTowerRankRequestInCd,
            1200000 => Self::ErrChatNotFriendNorOnline,
            1200001 => Self::ErrChatContentFilterFailed,
            1200002 => Self::ErrChatLockState,
            1200003 => Self::ErrChatEmojiNotValid,
            1200004 => Self::ErrChatSendTooFast,
            1200005 => Self::ErrChatMuteNotValidId,
            1200006 => Self::ErrBanChatDefault,
            1200007 => Self::ErrRoleQuestFuncNotOpen,
            1200008 => Self::ErrRoleQuestMaxCount,
            1200009 => Self::ErrRoleQuestUnlockPointNotEnough,
            1200010 => Self::ErrDailyQuestNotFoundArea,
            1200011 => Self::ErrDailyQuestNotFoundInfluence,
            1200012 => Self::ErrDailyQuestRewardAlreadyGet,
            1200013 => Self::ErrDailyQuestDataError,
            1200014 => Self::ErrDailyQuestCantGetReward,
            1200015 => Self::ErrEntityBuffProducerStateError,
            1200016 => Self::ErrEntityBuffProducerNotFound,
            1200017 => Self::ErrVoiceRemainChangeRoleNotInFormation,
            1200018 => Self::ErrVoiceRemainChangeRoleNotAlive,
            1300000 => Self::ErrApplyEffectFail,
            1300001 => Self::ErrOutofBattleTargetNotMonster,
            1300002 => Self::ErrMonsterBoomEntityNotExists,
            1300003 => Self::ErrMonsterBoomNotMonster,
            1300004 => Self::ErrMonsterBoomIsDead,
            1300005 => Self::ErrAnimationStateSpecialFuncException,
            1400000 => Self::ErrPayConfigNotFound,
            1400001 => Self::ErrPayCreateReceiptFail,
            1400002 => Self::ErrPayReceiptNotFound,
            1400003 => Self::ErrPayReceiptPlayerIdUnMatch,
            1400004 => Self::ErrPayReceiptCannotDealClose,
            1400005 => Self::ErrPayReceiptDealCloseFail,
            1400006 => Self::ErrPayNotEnable,
            1400007 => Self::ErrPayDataChanged,
            1400008 => Self::ErrPayUpdateReceiptFail,
            1400009 => Self::ErrGachaConfigNotFound,
            1400010 => Self::ErrGachaRuleGroupConfigNotFound,
            1400011 => Self::ErrGachaRulesNotFound,
            1400012 => Self::ErrGachaTypeKnowns,
            1400013 => Self::ErrGachaDailyTimesLimit,
            1400014 => Self::ErrGachaTotalTimesLimit,
            1400015 => Self::ErrGachaDailyTotalTimesLimit,
            1400016 => Self::ErrGachaIsNotOpen,
            1400017 => Self::ErrGachaIsNotInOpenTime,
            1400018 => Self::ErrGachaFuncIsNotOpen,
            1400019 => Self::ErrItemExchageConfigNotFound,
            1400020 => Self::ErrItemExchageDailyTimesLimit,
            1400021 => Self::ErrItemExchangeTotalTimesLimit,
            1400022 => Self::ErrGachaLimitNotFound,
            1400023 => Self::ErrGachaLimitsEmpty,
            1400024 => Self::ErrTextServerTimeout,
            1400025 => Self::ErrTextServerResFail,
            1400026 => Self::ErrTextServerResEmpty,
            1400027 => Self::ErrTextServerResException,
            1400028 => Self::ErrItemExchageParamError,
            1400029 => Self::ErrBattlePassFuncIsNotOpen,
            1400030 => Self::ErrPayShopFuncIsNotOpen,
            1400031 => Self::ErrGachaPoolConfigNotFound,
            1400032 => Self::ErrGachaPoolIsNotOpen,
            1400033 => Self::ErrGachaPoolIsNotInOpenTime,
            1400034 => Self::ErrGachaPoolLimitNotFound,
            1400035 => Self::ErrGachaPoolNotBelongToGacha,
            1400036 => Self::ErrGachaUsePoolIdNotSet,
            1400037 => Self::ErrGachaTimesNonsupport,
            1400038 => Self::ErrGachaFrontRuleGroupNotFinish,
            1400039 => Self::ErrGachaRuleGroupFinish,
            1400040 => Self::ErrPayGiftBuyLimit,
            1400041 => Self::ErrPayGiftTypeUnknown,
            1400042 => Self::ErrPayGiftNotInSellTime,
            1400043 => Self::ErrBattlePassBuyLevelLimit,
            1400044 => Self::ErrBattlePassBuyLevelError,
            1400045 => Self::ErrJsFileNotFound,
            1400046 => Self::ErrPayReceiptIsRefunded,
            1400047 => Self::ErrPayReceiptIsNotPay,
            1400048 => Self::ErrPayReceiptRefundFail,
            1400049 => Self::ErrPayGiftLocked,
            1400050 => Self::ErrPayGiftBuyConditionLimit,
            1400051 => Self::ErrMapMarkConfigIdNotExist,
            1400052 => Self::ErrTreasureSlotMarkNotExist,
            1400053 => Self::ErrTreasureBoxMarkNotExist,
            1400054 => Self::ErrTreasureSlotMarkExist,
            1400055 => Self::ErrMapMarkTypeNotCustom,
            1400056 => Self::ErrPayShopIsNotOpen,
            1400057 => Self::ErrConsumptiveActivityNotOpen,
            1400058 => Self::ErrConsumptiveTaskNotFound,
            1400059 => Self::ErrConsumptiveTaskNotFinish,
            1400060 => Self::ErrConsumptiveTaskRewarded,
            1400061 => Self::ErrConsumptiveActivityNotFound,
            1400062 => Self::ErrPayGiftPersonalNoPermission,
            1400063 => Self::ErrPayGiftPersonalPermissionExpired,
            1400064 => Self::ErrPayGiftNotPersonal,
            1400065 => Self::ErrPayGiftPersonalPermissionExist,
            1400066 => Self::ErrCouponId,
            1400067 => Self::ErrCouponConfigNotFound,
            1400068 => Self::ErrCouponNotEnough,
            1400069 => Self::ErrCouponTargetItemNotFound,
            1400070 => Self::ErrCouponTargetItemCount,
            1400071 => Self::ErrGachaTypeNotPersonal,
            1400072 => Self::ErrGachaPersonalPermissionExist,
            1400073 => Self::ErrGachaPersonalNoPermission,
            1400074 => Self::ErrGachaPersonalPermissionExpired,
            1400075 => Self::ErrPayGiftVersionConfigNotFound,
            1400076 => Self::ErrPayGiftSelfDefineVersionError,
            1500000 => Self::ErrInfoDisplayId,
            1500001 => Self::ErrItemAlreadyInCd,
            1500002 => Self::ErrCantFinAdventureConfig,
            1500003 => Self::ErrAdventureRewardReceived,
            1500004 => Self::ErrAdventureTaskCache,
            1500005 => Self::ErrAdventureState,
            1500006 => Self::ErrAdventureRewardOrder,
            1500007 => Self::ErrAdventureChapterState,
            1500008 => Self::ErrCantDetectRepeat,
            1500009 => Self::ErrNotInCurrentFollowList,
            1500010 => Self::ErrCantDetectOtherDetectionType,
            1500011 => Self::ErrNotSelectCurrentDetectionId,
            1500012 => Self::ErrDetectionConfigNotFound,
            1500013 => Self::ErrDetectionListCantBeEmpty,
            1500014 => Self::ErrCantFindAnyDetectionTarget,
            1500015 => Self::ErrCantFindTurntableComponentEntity,
            1500016 => Self::ErrHaveNoTurntableControlComponent,
            1500017 => Self::ErrCantFindLevitationMagnetComponentEntity,
            1500018 => Self::ErrHaveNoLevitaionMagnetComponent,
            1500019 => Self::ErrCantFindBoardEntity,
            1500020 => Self::ErrCantFindPlacementEntity,
            1500021 => Self::ErrHaveNoPlacementComponent,
            1500022 => Self::ErrCantFindBoardEntityComponent,
            1500023 => Self::ErrBoardHaveNoAnyPlacement,
            1500024 => Self::ErrBoardNotActiveAllGrid,
            1500025 => Self::ErrNeedBeControlledBefore,
            1500026 => Self::ErrPlaceFailOfAlreadyOnBoard,
            1500027 => Self::ErrInvalidBoardPosition,
            1500028 => Self::ErrNeedRemoveControlRelation,
            1500029 => Self::ErrNotOccupyOnBoard,
            1500030 => Self::ErrHaveNoFillRule,
            1500031 => Self::ErrGridPosAlreadyOccupied,
            1500032 => Self::ErrHaveNoJigsawFoundationConfig,
            1500033 => Self::ErrInvalidGridPos,
            1500034 => Self::ErrGridPosAlreadyActive,
            1500035 => Self::ErrCantPlaceItemOnBoard,
            1500036 => Self::ErrNeedJigsawFoundationComponentWhenBeControlled,
            1500037 => Self::ErrHaveNoBoardComponentConfig,
            1500038 => Self::ErrNeedJigsawItemComponentWhenBeControlled,
            1500039 => Self::ErrCantFindOriginBoardEntity,
            1500040 => Self::ErrDistanceNotInRangeBetweenEntity,
            1500041 => Self::ErrJigsawFoundationIsAlreadySilent,
            1500042 => Self::ErrCrystalEntityNotFound,
            1500043 => Self::ErrGachaBoardEntityNotFound,
            1500044 => Self::ErrNotCrystalEntity,
            1500045 => Self::ErrNotGachaFoundationEntity,
            1500046 => Self::ErrGachaHoleIsFull,
            1500047 => Self::ErrJigsawItemSilent,
            1500048 => Self::ErrThrowDamageConfigNotExists,
            1500049 => Self::ErrThrowDamageComponetNotExists,
            1500050 => Self::ErrEggNotMatchEggFoundation,
            1500051 => Self::ProgressBarEntityNotFound,
            1500052 => Self::NotProgressBarEntity,
            1500053 => Self::ProgressBarIsSilent,
            1500054 => Self::ScenePlayerInfoNotFound,
            1500055 => Self::PlayerNotInAnyScene,
            1500056 => Self::TeleportNotInValidDistance,
            1500057 => Self::AddMapMarkInfoLackOfTeleportParam,
            1500058 => Self::TemporaryTeleportNotExists,
            1500059 => Self::ErrNotHostPlayer,
            1500060 => Self::ErrMarkIdNotExists,
            1500061 => Self::ErrCantUpdateTemporaryTeleportMarkInfo,
            1500062 => Self::NotHostCantAddTemporaty,
            1500063 => Self::TemporaryTeleportPosIsNotWalkable,
            1500064 => Self::BadTemporaryTeleportConfig,
            1500065 => Self::HaveNoTemporaryTeleportComponent,
            1500066 => Self::ErrCantDetectAtInvalidPoint,
            1500067 => Self::ErrNotDetectionTreasureBoxBefore,
            1500068 => Self::ErrJigsawFoundationIsCompleteCantModifyGridState,
            1500069 => Self::GridIsActiveCantSwitchState,
            1500070 => Self::GridIsOccupiedCantSwitchState,
            1500071 => Self::ErrHaveNoBaseInfoComponent,
            1500072 => Self::ErrHaveNoParentEntity,
            1500073 => Self::NotRelationEntity,
            1500074 => Self::ErrLevelPlayNotRunning,
            1500075 => Self::ErrStateCantChangeWhenLifeCycleDestroy,
            1500076 => Self::ErrRangeEntityIdNotFoundWhenForbidTempTeleport,
            1500077 => Self::TemporaryTeleportIsForbidden,
            1500078 => Self::ErrGravityGearIsComplete,
            1500079 => Self::ErrInvalidRoleWhenUpdatePassiveSkill,
            1500080 => Self::ErrInvalidRolePassiveSkillId,
            1500081 => Self::ErrPassiveSkillNotAddBuff,
            1500082 => Self::ErrPassiveSkillCantSpecifyBuff,
            1500083 => Self::ErrPassiveSkillAddBuffFail,
            1500084 => Self::ErrPassiveSkillAddBulletFail,
            1500085 => Self::ErrBuffCreatePassiveSkillFail,
            1500086 => Self::ErrInvalidPreContext,
            1500087 => Self::ErrBadPassiveSkillId,
            1500088 => Self::ErrPassiveSkillComponentNotFound,
            1500089 => Self::ErrRepeatePassiveSkill,
            1500090 => Self::ErrBadPassiveSkillTriggerType,
            1500091 => Self::ErrAddPassiveSkillFailOfEntityNotFound,
            1500092 => Self::ErrEntityNotClientControlWhenAddPassiveSkill,
            1500093 => Self::ErrEntityNotClientControlWhenRemovePassiveSkill,
            1500094 => Self::ErrPassiveSkillNotFoundWhenRemovePassiveSkill,
            1500095 => Self::ErrRepeatedBattleContext,
            1500096 => Self::ErrPassiveSkillAddSkillFail,
            1500097 => Self::ErrCombatSendPackAbnormal,
            1500098 => Self::ErrContextCheckFail,
            1500099 => Self::ErrFsmComponentNotFound,
            1500100 => Self::ErrFsmCreateContextFail,
            1500101 => Self::ErrFsmStateBehaviorPreMessageCantBeZero,
            1500102 => Self::ErrFsmBehaviorCheckBattleContextFail,
            1500103 => Self::ErrFsmPlayMontageLackPreMessage,
            1500104 => Self::ErrFsmPlayMontageCheckContextFail,
            1500105 => Self::ErrFsmPlayMontageConfigCheckFail,
            1500106 => Self::ErrSkillFlowNotExist,
            1500107 => Self::ErrGetReportDataOverLimit,
            1500108 => Self::ErrGetReportDataTooFast,
            1500109 => Self::ErrNotInAnyScene,
            1500110 => Self::ErrAceLogDataNotFound,
            1500111 => Self::ErrAceLogDataRepeatReport,
            1500112 => Self::ErrAceInvalidLogId,
            1500113 => Self::ErrAceSceneGlobalObjNotFound,
            1500114 => Self::ErrAceBadParam,
            1500115 => Self::ErrS2CconfirmIdNotExists,
            1500116 => Self::ErrPassiveSkillConfigNotFound,
            1500117 => Self::ErrNotBehaviorController,
            1500118 => Self::ErrPlayMontageFail,
            1500119 => Self::ErrFightDataInConsistent,
            1500120 => Self::ErrNotInAoiSight,
            1500121 => Self::ErrPassiveSkillNotOwner,
            1500122 => Self::ErrReportStartFirstly,
            1500123 => Self::ErrBattleEntityNotFound,
            1500124 => Self::ErrBattleCampNotDefined,
            1500125 => Self::ErrOtherInternalError,
            1500126 => Self::ErrHaventBattleComponent,
            1500127 => Self::ErrLackCombinePartInfoParam,
            1500128 => Self::ErrCombinerEntityNotExists,
            1500129 => Self::ErrTargetEntityNotExists,
            1500130 => Self::ErrTargetPartNotExists,
            1500131 => Self::ErrCombineComponentNotExists,
            1500132 => Self::ErrAlreadyCombineToOtherEntity,
            1500133 => Self::ErrLackCombinerOffsetPos,
            1500134 => Self::ErrLackCombinerOffsetRotate,
            1500135 => Self::ErrCombineEntityNotFound,
            1500136 => Self::ErrDissolveCheckBattleContextFail,
            1500137 => Self::ErrRepeatedRole,
            1500138 => Self::ErrNotStateMachineBehavior,
            1500140 => Self::ErrDiscardMsgWhenChangeSceneMultiMode,
            1500139 => Self::ErrMayOccurDbAbnormal,
            1500141 => {
                Self::ErrEntityLivingStatusNotifyCheckFsmPlayMontageOfFsmGroupConfigNotExists
            }
            1500142 => Self::ErrEntityLivingStatusNotifyCheckFsmPlayMontageOfConfigNotExists,
            1500143 => Self::ErrEntityLivingStatusNotifyCheckFsmPlayMontageFail,
            1500144 => Self::ErrFsmActionCheckFsmPlayMontageFail,
            1500145 => Self::ErrFsmActionCheckSkillFail,
            1500146 => Self::ErrFsmActionCheckBufflFail,
            1500147 => Self::ErrFsmPlayMontageCheckAnParamError,
            1500148 => Self::ErrFsmPlayMontageCheckAnMontageConfigNotFound,
            1500149 => Self::ErrFsmPlayMontageCheckAnMontageFail,
            1500150 => Self::ErrMontageConfigNotFound,
            1500151 => Self::ErrAnconfigNotFound,
            1500152 => Self::ErrBulletConfigNotFound,
            1500153 => Self::ErrFsmVersion,
            1500154 => Self::ErrUpdateFightRoleRepeated,
            1500155 => Self::TrialRoleEntityOverLimit,
            1500156 => Self::ErrHaveNoRoleInfos,
            1500157 => Self::ErrTeamHaveNoAnyRoles,
            1500158 => Self::ErrSkillMontageNotNormalMontage,
            1500159 => Self::ErrMontageContextConfigNotFound,
            1500160 => Self::ErrMontageContextCheckAnFail,
            1500161 => Self::ErrLivingStatusContextCheckPlayMontageContextFail,
            1500162 => Self::ErrContextHaveNoFsmGroupConfig,
            1500163 => Self::ErrLivingStatusContextFsmGroupConfigNotFound,
            1500164 => Self::ErrFsmActionCheckPlayEntityMontageFail,
            1500165 => Self::ErrMontageContext1CheckPlayEntityMontageFail,
            1500166 => Self::ErrSkillGahaveNoModifyCommonEnemyPro,
            1500167 => Self::ErrNotSummonEntity,
            1500168 => Self::ErrSkillGahaveNoTagId,
            1600000 => Self::ErrCombatSkillGahandleGetEntityFailed,
            1600001 => Self::ErrCombatMaterialHandleGetEntityFailed,
            1600002 => Self::ErrCombatParticleHandleGetEntityFailed,
            1600003 => Self::ErrCombatPartLifeChangeEntityNotExisted,
            1600004 => Self::ErrCombatCreateBulletTargetNotExisted,
            1600005 => Self::ErrCombatDeleteBulletTargetNotExisted,
            1600006 => Self::ErrCombatDeleteBulletGetEntityFailed,
            1600007 => Self::ErrCombatBulletTargetNoExist,
            1600008 => Self::ErrPartEntityNotExisted,
            1600009 => Self::ErrNoAiControlRights,
            1600010 => Self::ErrAiHateComponent,
            1600011 => Self::ErrSummonerPlayerControl,
            1600012 => Self::ErrConfDamageNotFound,
            1600013 => Self::ErrProcessDamageFailed,
            1600014 => Self::ErrInjuryFreeLandingTag,
            1600015 => Self::ErrNotFindActiveGameplayEffect,
            1600016 => Self::NotClientControlBuff,
            1600017 => Self::ErrBuffNoEffectConf,
            1600018 => Self::ErrBuffCannotCreateBullet,
            1600019 => Self::ErrBuffCannotCreateBuff,
            1600020 => Self::ErrNoBuffConf,
            1600021 => Self::ErrStoppedAi,
            1600022 => Self::ErrEntityIsNotAlive,
            1600023 => Self::ErrSummonCannotSwitchAiControl,
            1600024 => Self::ErrAiControlNotChange,
            1600025 => Self::ErrPlayerCannotControlEntity,
            1600026 => Self::ErrNotFoundBuffEffect,
            1600027 => Self::ErrBuffEffectAuthority,
            1600028 => Self::ErrConcomitantDestroy,
            1600029 => Self::ErrPlayerFollowersComponent,
            1600030 => Self::ErrBuffComponentNotExist,
            1600031 => Self::ErrOrderApplyBuffFailed,
            1600032 => Self::ErrPlayerBuff,
            1600033 => Self::ErrFindPathNoEndPos,
            1600034 => Self::ErrNotGetCurRole,
            1600035 => Self::ErrFindPathFailed,
            1700000 => Self::ErrEntityFsmMachineNotExist,
            1700001 => Self::ErrEntityFsmStateIncorrect,
            1700002 => Self::ErrIsNotAiControler,
            1700003 => Self::ErrIentityFsmCantTransit,
            1700004 => Self::ErrIentityFsmTransitCondition,
            1700005 => Self::ErrIentityFsmTransitToState,
            1700006 => Self::ErrIentityFsmConfirmNotExist,
            1700007 => Self::ErrIentityFsmConfirmNotWait,
            1700008 => Self::ErrItest,
            1700009 => Self::ErrItest1,
            1700010 => Self::ErrItest2,
            1700011 => Self::ErrIentityFsmCondCantPass,
            1700012 => Self::ErrIentityFsmActionParamType,
            1700013 => Self::ErrIentityFsmActionParam,
            1700014 => Self::ErrIentityFsmActionExecuted,
            1700015 => Self::ErrIentityFsmActionNotMatchState,
            1700016 => Self::ErrSkillNotExecuting,
            1700017 => Self::ErrExecuteSkillNotMatch,
            1700018 => Self::ErrBlueprintPinNotSupport,
            1700019 => Self::ErrBlueprintPinNotMontage,
            1700020 => Self::ErrBlueprintPinMontageIndex,
            1700021 => Self::ErrConfSkillNotExist,
            1700022 => Self::ErrSkillGanotExist,
            1700023 => Self::ErrSkillGahaveNoBuff,
            1700024 => Self::ErrSkillGahaveNoBuffId,
            1700025 => Self::ErrSkillGahaveNoBullet,
            1700026 => Self::ErrSkillGahaveNoBulletId,
            1700027 => Self::ErrMontageNotMatchSkill,
            1700028 => Self::ErrMontageIndexError,
            1700029 => Self::ErrVisiionSkillNotEquip,
            1700030 => Self::ErrSkillCd,
            1700031 => Self::ErrHaveNoBattleContext,
            1700032 => Self::ErrContextFsmActionOnce,
            1700033 => Self::ErrPlayMontageButNoSkill,
            1700034 => Self::ErrMontageNotExist,
            1700035 => Self::ErrMontageNotContainBuff,
            1700036 => Self::ErrMontageNotContainBullet,
            1700037 => Self::ErrMontageCantBring,
            1700038 => Self::ErrSkillInfoParamError,
            1800000 => Self::ErrNoWorldTeam,
            1800001 => Self::ErrWorldTeamNoMember,
            1800002 => Self::ErrNoHostIs,
            1800003 => Self::ErrNoInstId,
            1800004 => Self::ErrNoTeamInfo,
            1800005 => Self::ErrHostNoTeamInfo,
            1800006 => Self::ErrHasInMatchTeam,
            1800007 => Self::ErrNotInMatchTeam,
            1800008 => Self::ErrHostIsParam,
            1800009 => Self::ErrMatchModeIparam,
            1800010 => Self::ErrMatchTeamFull,
            1800011 => Self::ErrLocalTeamCanNotOpt,
            1800012 => Self::ErrHostInLocalTeam,
            1800013 => Self::ErrNoMatchNodeId,
            1800014 => Self::ErrPlayerSceneIsNull,
            1800015 => Self::ErrPlayerSceneRolesNull,
            1800016 => Self::ErrInvalidMatchState,
            1800017 => Self::ErrRepeatedMatchState,
            1800018 => Self::ErrApplyrPlayerInMatchNotEnterMatchTeam,
            1800019 => Self::ErrOtherVersionLowNoOperate,
            1800020 => Self::ErrPlayerVersionLowNeedUpdate,
            1800021 => Self::ErrMultiGameModeNoWorldLevelDown,
            1800022 => Self::ErrMultiGameModeNoWorldLevelRegain,
            1800023 => Self::ErrOtherHasOnline,
            1800024 => Self::SwitchRoleNotInCurrentFormation,
            1800025 => Self::ErrNoChangeRoles,
            1800026 => Self::ErrExploreSkillPullGiantMultiGame,
            1800027 => Self::ErrExploreSkillPullGiantNotExist,
            1800028 => Self::ErrHttpRpcParam,
            1800029 => Self::ErrPlayerNotInGameNode,
            1800030 => Self::ErrApplyJoinPlayerCurRoleIsDead,
            1800031 => Self::ErrPlayerCurRoleIsDeadNoJoin,
            1800032 => Self::ErrPlayerCurRoleIsDead,
            1800033 => Self::ErrSwitchMultiverse,
            1800034 => Self::ErrSwitchNode,
            1800035 => Self::ErrMatchConfirmPlayerDead,
            1800036 => Self::ErrCheckPublicResourceVersionLower,
            1800037 => Self::ErrCheckPublicResourceVersionHigher,
            1800038 => Self::ErrCheckPublicResourceClientVersionErr,
            1800039 => Self::ErrCheckPublicResourceServerVersionErr,
            1800040 => Self::ErrCheckPublicResourceClientVersionParamErr,
            1800041 => Self::ErrCheckClientVersionNeedUpdate,
            1800042 => Self::ErrBranchNameNotMatch,
            1800043 => Self::ErrMatchRpcAlready,
            1800044 => Self::ErrOtherPlayerEnterHost,
            1800045 => Self::ErrPlayerEnterHost,
            1800046 => Self::ErrLevelPlayChangeSprotModeInMutile,
            1800047 => Self::ErrMatchingNotInvite,
            1800048 => Self::ErrEnableFunctionFb,
            1800049 => Self::ErrMatchSelectTrialRole,
            1800050 => Self::ErrNoFindLastBigScene,
            1800051 => Self::ErrExploreSkillCustomMultiGame,
            1800052 => Self::ErrExploreSkillCustomNotExist,
            1800053 => Self::ErrMatchInviteMemberDead,
            1800054 => Self::ErrMatchAcceptInviteMemberDead,
            1800055 => Self::ErrNoFishBoat,
            1800056 => Self::ErrTemplateNotExists,
            1800058 => Self::ErrDisableSubLevels,
            1800059 => Self::ErrSubLevelsClientNoPermission,
            1800057 => Self::ErrExploreSkillCustomNoActions,
            1800060 => Self::ErrMatchClientVersion,
            1800061 => Self::ErrCurNodeIsTainted,
            1800062 => Self::ErrTeamMateNodeIsTainted,
            1800063 => Self::ErrMapFuctionNotOpen,
            1800064 => Self::ErrNotSceneOwner,
            1800065 => Self::ErrLevelPlayUiListIsNull,
            1800066 => Self::ErrOpenLevelPlayNotClientUi,
            1800067 => Self::ErrOpenLevelPlayFailed,
            1800068 => Self::ErrBlackSwordChallengeId,
            1900000 => Self::ErrCalabashLevelRequest,
            1900001 => Self::ErrCalabashLevelRewardDone,
            1900002 => Self::ErrCalabashLevelConfig,
            1900003 => Self::ErrSkillTreeActiveConsume,
            1900004 => Self::ErrLoadFriendData,
            1900005 => Self::ErrNoLoadPrivateChatData,
            1900006 => Self::ErrNotInWolrd,
            1900007 => Self::ErrNotInGround,
            1900008 => Self::ErrInFighting,
            1900009 => Self::ErrNotHaveCountryAccess,
            1900010 => Self::ErrSkillIsEffect,
            1900011 => Self::ErrNoSoundBox,
            1900012 => Self::ErrConsumeNotEnough,
            1900013 => Self::ErrExploreSkillCountLimit,
            1900014 => Self::ErrLegalAreaNoTreasureBox,
            1900015 => Self::ErrTreasureBoxAllActive,
            1900016 => Self::ExploreProgressNoCountry,
            1900017 => Self::ExploreProgressNoScoreCfg,
            1900018 => Self::ExploreProgressLackProgress,
            1900019 => Self::ExploreProgressRewardDone,
            1900020 => Self::ExploreProgressNoArea,
            1900021 => Self::ExploreToolNotConfirm,
            1900022 => Self::ExploreToolNotOpen,
            1900023 => Self::ErrTreasureBoxPlaceFail,
            1900024 => Self::ErrTreasureBoxData,
            1900025 => Self::ErrPayShopBuyCondition,
            1900026 => Self::ErrGatherActivityData,
            1900027 => Self::ErrGatherTaskNoFinish,
            1900028 => Self::ErrHadGatherReward,
            1900029 => Self::ErrHadGetSharedReward,
            1900030 => Self::ErrSharedPlat,
            1900031 => Self::ErrTowerTargetComplete,
            1900032 => Self::ErrTowerGuideRewardHad,
            1900033 => Self::ErrTowerGuideNoOpen,
            1900034 => Self::ErrTowerGuideConfig,
            1900035 => Self::ErrNewBieCourseConfig,
            1900036 => Self::ErrNewBieCourseRewardHad,
            1900037 => Self::ErrNewBieCourseLevel,
            1900038 => Self::ErrDetectionTargetSilence,
            1900039 => Self::ErrRoleTrialNotInit,
            1900040 => Self::ErrRoleTrialNoFinish,
            1900041 => Self::ErrRoleTrialReward,
            1900042 => Self::ErrRoleTrialRewardDone,
            1900043 => Self::ErrAdventureTaskReward,
            1900044 => Self::ErrChapterReward,
            1900045 => Self::ErrSilentFirstPassStatus,
            1900046 => Self::ErrSilentFirstPassReward,
            1900047 => Self::ErrPayShopEchoRole,
            1900048 => Self::ErrPayShopEchoItemOver,
            1900049 => Self::ErrDailyAdventureActivityInit,
            1900050 => Self::ErrDailyAdventureActivityPtEnough,
            1900051 => Self::ErrDailyAdventureActivityRewardDone,
            1900052 => Self::ErrDailyAdventureActivityRewardTake,
            1900053 => Self::ErrDailyAdventureActivityTaskDone,
            1900054 => Self::ErrRoleTrialTimeOut,
            1900055 => Self::ErrFriendRemarkNull,
            1900056 => Self::ErrTrackMoonRoleUnLock,
            1900057 => Self::ErrTrackMoonTrigger,
            1900058 => Self::ErrTrackMoonBuildingUnLock,
            1900059 => Self::ErrTrackMoonBuildingCurve,
            1900060 => Self::ErrTrackMoonBuildingLock,
            1900061 => Self::ErrMoonEntrustCfg,
            1900062 => Self::ErrMoonRoleCfg,
            1900063 => Self::ErrMoonRoleTrailCurve,
            1900064 => Self::ErrMoonBuildingCfg,
            1900065 => Self::ErrMoonItemConsume,
            1900066 => Self::ErrDirtyWordDeserialize,
            1900067 => Self::ErrMoonTargetNoFinish,
            1900068 => Self::ErrMoonActivityReward,
            1900069 => Self::ErrMoonActivityOpen,
            1900070 => Self::ErrCircumDoReward,
            1900071 => Self::ErrRetrunRewardCfg,
            1900072 => Self::ErrRetrunRewardLevel,
            1900073 => Self::ErrRetrunHaddone,
            1900074 => Self::ErrSignRewardCfg,
            1900075 => Self::ErrCircumSignHadRwd,
            1900076 => Self::ErrCircumNoSign,
            1900077 => Self::ErrScoreRewardCfg,
            1900078 => Self::ErrCircumScoreHadRwd,
            1900079 => Self::ErrCircumScoreLack,
            1900080 => Self::ErrCircumTaskNoFinish,
            1900081 => Self::LoginServiceInvalidToken,
            1900082 => Self::LoginFusing,
            1900083 => Self::LoginRateLimiterRejected,
            1900084 => Self::LoginTimeoutRejected,
            1900085 => Self::AccountInputErr,
            1900086 => Self::DevInvalidLoginType,
            1900087 => Self::GarinvalidLoginType,
            1900088 => Self::GardevInvalidLoginType,
            1900089 => Self::SdkserverTimeOut,
            1900090 => Self::ReconnectInvalidOperation,
            1900091 => Self::PbMessageAppVersionNotMatch,
            1900092 => Self::ErrPluginReconnectIpWhiteList,
            1900093 => Self::NotInUserIdWhiteListWithChannel,
            1900094 => Self::ErrPluginReconnectChannelWhiteList,
            1900095 => Self::PluginPlayerLoggingIn,
            1900096 => Self::LoginFusing2,
            1900097 => Self::SoundBoxExploreFull,
            1900098 => Self::ErrMoonEntrustNoData,
            1900099 => Self::ErrMoonMoneyNotEnough,
            1900100 => Self::ErrCircumFluenceTimeIn,
            1900101 => Self::DragonPoolRewardWayErr,
            1900102 => Self::DragonPoolNoHandIn,
            1900103 => Self::WeaponSkinNoEquiped,
            1900104 => Self::WeaponSkinDataErr,
            1900105 => Self::WeaponSkinUnLockErr,
            1900106 => Self::WeaponSkinEquipDone,
            1900107 => Self::WeaponSkinTypeErr,
            1900108 => Self::FriendOfflineMsgErr,
            1900109 => Self::FindSpringSignConfigErr,
            1900110 => Self::SpringSignDataErr,
            1900111 => Self::SpringSignRewardDone,
            1900112 => Self::SpringSignRewardGetErr,
            1900113 => Self::SpringSignNoOpen,
            1900114 => Self::SpringSignNoTask,
            1900115 => Self::SpringSignInviteNum,
            1900116 => Self::SpringSignRolePool,
            1900117 => Self::SpringSignDrawPoolNull,
            1900118 => Self::FarmGoldActivityNotOpen,
            1900119 => Self::FarmGoldActivityPointReceived,
            1900120 => Self::FarmGoldActivityPointNotConfig,
            1900121 => Self::FarmGoldActivityPointNotEnough,
            1900122 => Self::FarmGoldActivityLevelNoData,
            1900123 => Self::FarmGoldActivityLevelReceived,
            1900124 => Self::FarmGoldActivityLevelNotConfig,
            1900125 => Self::FarmGoldActivityLevelDiffNotConfig,
            1900126 => Self::FarmGoldInstIdInValid,
            1900127 => Self::FarmGoldVarNotExist,
            1900128 => Self::FarmGoldResultCacheNotExist,
            1900129 => Self::FarmGoldInstNotOpen,
            1900130 => Self::FarmGoldActivityLimitDataNotFind,
            1900131 => Self::MapTravelDataErr,
            1900132 => Self::MapTravelConfigErr,
            1900133 => Self::MapTravelCannotReward,
            1900134 => Self::MapTravelRewardGet,
            1900135 => Self::MapTravelMaxLevel,
            1900136 => Self::MapTravelLackExp,
            1900137 => Self::FarmGoldLevelNotOpen,
            1900138 => Self::MapTravelAreaLock,
            1900139 => Self::MapTravelAreaConfigErr,
            1900142 => Self::SlashAndTowerCacheErr,
            1900143 => Self::SlashAndTowerConfigErr,
            1900144 => Self::SlashAndTowerDataErr,
            1900145 => Self::SlashAndTowerReceivedLevelAward,
            1900146 => Self::SlashAndTowerNotReward,
            1900147 => Self::SlashAndTowerRoleNum,
            1900148 => Self::SlashAndTowerBuffNum,
            1900149 => Self::SlashAndTowerBuffConfig,
            1900150 => Self::SlashAndTowerBuffAccess,
            1900151 => Self::SlashAndTowerRoleSame,
            1900152 => Self::SlashAndTowerSeasonErr,
            1900153 => Self::SlashAndTowerBuffLack,
            1900140 => Self::MapTravelLevelUpCfgErr,
            1900141 => Self::MapTravelLevelCfgErr,
            1900154 => Self::TeamParkOurTaskCfgErr,
            1900155 => Self::TeamParkOurDataErr,
            1900156 => Self::TeamParkOurCfgNoMatch,
            1900157 => Self::TeamParkOurTaskDoing,
            1900158 => Self::TeamParkOurTaskTaken,
            1900159 => Self::TeamParkOurLevelLock,
            1900160 => Self::TeamParkOurFindNoLevel,
            1900161 => Self::SlashAndTowerRewardErr,
            1900162 => Self::SlashAndTowerBuffSeasonErr,
            1900163 => Self::SlashAndTowerNotOpen,
            1900164 => Self::SlashAndTowerSeasonToCfgErr,
            1900165 => Self::SlashAndTowerSeasonNoUpdate,
            1900166 => Self::SlashAndTowerFirstNoPass,
            1900167 => Self::SlashAndTowerLevelErr,
            1900168 => Self::TeamParkMemberErr,
            1900172 => Self::AvignonNotOpen,
            1900173 => Self::AvignonNotConfig,
            1900174 => Self::AvignonTaskNotFinish,
            1900175 => Self::AvignonTaskNotData,
            1900176 => Self::AvignonHadReward,
            1900169 => Self::SlashAndTowerTeamErr,
            1900170 => Self::SlashAndTowerBuffErr,
            1900171 => Self::SeasonTowerNoMatch,
            1900177 => Self::TeamParkOurMemberErr,
            1900178 => Self::BattlePassRewardDone,
            1900179 => Self::SlashAndTowerAwardCfgErr,
            1900180 => Self::NoFlySkinItem,
            1900181 => Self::FlySkinHadWear,
            1900182 => Self::NoOldFlySkinItem,
            1900183 => Self::OldFlySkinNoWear,
            1900184 => Self::RoleWearNoFlySkin,
            1900185 => Self::FlySkinParaGliderNoOpen,
            1900186 => Self::FlySkinSoaringWingNoOpen,
            1900187 => Self::NoRoleWearFlySkinSucc,
            1900188 => Self::FlySkinItemNoConfig,
            1900189 => Self::SlashAndTowerLevelSettle,
            1900190 => Self::FlySkinTypeErr,
            1900191 => Self::FlySkinTrialRole,
            1900192 => Self::RegreeNotOpen,
            1900193 => Self::RegressNoConfig,
            1900194 => Self::RegressWrongId,
            1900195 => Self::NoRegressRoundData,
            1900196 => Self::NotInRegress,
            1900197 => Self::AskRewardNoFinish,
            1900198 => Self::BirthdayNotArrived,
            1900199 => Self::BirthdayRoleInvalid,
            1900200 => Self::BirthdayNoCfg,
            1900201 => Self::HadBirthDayReward,
            1900202 => Self::BirthdayNoReset,
            1900203 => Self::BirthDayRewardTimeInvalid,
            1900204 => Self::BirthdayMustNowYear,
            1900205 => Self::BirthDayNotOpen,
            1900206 => Self::BirthDayRewardNotFinish,
            1900207 => Self::BirthdayRoleDone,
            1900208 => Self::MoraleNoRewardGet,
            1900209 => Self::FloroNoRanchData,
            1900210 => Self::FloroNoRanchTech,
            1900211 => Self::FloroTechUnLock,
            1900212 => Self::FloroPreNodeLock,
            1900213 => Self::FloroTechPoint,
            1900214 => Self::FloroNoFindActivityTask,
            1900215 => Self::FloroNoFindTaskCfg,
            1900216 => Self::FloroTaskIsRunning,
            1900217 => Self::FloroTaskInValidTime,
            1900218 => Self::FloroRewardDone,
            1900219 => Self::FloroMilestoneNoFind,
            1900220 => Self::FloroDropNoFind,
            1900221 => Self::FloroNoReward,
            1900222 => Self::FloroActivityParamErr,
            1900223 => Self::ConditionTimeOut,
            1900224 => Self::ConditionModuleErr,
            1900225 => Self::ConditionGroupIdErr,
            1900226 => Self::ConditionRegisterFail,
            1900227 => Self::FloroRanchSetAgain,
            1900228 => Self::MoonPhaseNoOpen,
            1900229 => Self::MoonPhaseNoCount,
            1900230 => Self::ErrRandomDone,
            1900231 => Self::PhaseMoonRandomErr,
            1900232 => Self::PhaseMoonCfgErr,
            1900233 => Self::MoonLabelErr,
            1900234 => Self::MoonPhaseRewardErr,
            1900235 => Self::MoonPhaseRewardDone,
            1900236 => Self::PhaseMoonItemErr,
            1900237 => Self::MoonPhaseBuffFail,
            1900238 => Self::CoopRoleMaxLevelErr,
            1900239 => Self::CoopRoleNoUnLock,
            1900240 => Self::CoopRoleSpRewradErr,
            1900241 => Self::CoopRoleRewardErr,
            1900242 => Self::NoRewardCanGain,
            1900243 => Self::CoopRoleDataErr,
            1900244 => Self::CoopIdErr,
            1900245 => Self::MotorcycleMotorIpErr,
            1900246 => Self::MotorcycleRewardEmpty,
            1900247 => Self::RoadBookNotData,
            1900248 => Self::RoadBookNotInOpen,
            1900249 => Self::RoadBookTaskCannotReward,
            1900250 => Self::RoadBookRewardHad,
            1900251 => Self::RoadBookOverMaxLevel,
            1900252 => Self::RoadBookCfgErr,
            1900253 => Self::RoadBookTaskLock,
            1900254 => Self::RoadBookChallengeScoreErr,
            1900255 => Self::RoadBookDuplicateReward,
            1900256 => Self::RoadBookNoRewardGet,
            1900257 => Self::GuessJokerLevelErr,
            1900258 => Self::GuessJokerNotInScene,
            1900259 => Self::GuessJokerNotGameObj,
            1900260 => Self::GuessJokerPlayCardErr,
            1900261 => Self::GuessJokerActionPhaseErr,
            1900262 => Self::GuessJokerDeckErr,
            1900263 => Self::GuessJokerNoHandCard,
            1900264 => Self::GuessJokerBanCard,
            1900265 => Self::GuessJokerOnlyPair,
            1900266 => Self::GuessJokerPairAll,
            1900267 => Self::GuessJokerCardRefuse,
            1900268 => Self::GuessJokerDrawCardErr,
            1900269 => Self::JokerSkillTriggerFail,
            1900270 => Self::JokerSkillNoConfig,
            1900271 => Self::JokerSkillOwnerErr,
            1900272 => Self::GuessJokerPlayCardOver,
            1900273 => Self::GuessJokerPlayCardAgain,
            1900274 => Self::JokerGuessLevelLock,
            1900275 => Self::JokerGuessPreLevelLock,
            1900276 => Self::GuessJokerInitErr,
            1900277 => Self::NoDrinkRequireListConfig,
            1900278 => Self::DrinkBaseCountError,
            1900279 => Self::DrinkBaseNoConfig,
            1900280 => Self::DrinkBatchingCountErr,
            1900281 => Self::DrinkBatchingConfigErr,
            1900282 => Self::DrinkOrnaNoConfig,
            1900283 => Self::DrinkRoleLikeNoConfig,
            1900284 => Self::DrinkLikePointErr,
            1900285 => Self::GuessJokerScriptPlayErr,
            1900286 => Self::GuessJokerScriptDrawErr,
            1900287 => Self::GuessJokerNoPlayerWait,
            1900288 => Self::GuessJokerSkillUseOver,
            1900289 => Self::DrinkBatchingRepeate,
            1900290 => Self::SpringFestivalNoReward,
            1900291 => Self::SpringFestivalDropNoFind,
            1900292 => Self::SpringFestivalTaskRunning,
            1900293 => Self::SpringFestivalHadReward,
            1900294 => Self::SpringFestivalActivityIdErr,
            1900295 => Self::SpringFestivalTaskFindErr,
            1900296 => Self::SpringFestivalNoActivityCfg,
            1900297 => Self::SpringFestivalParamCountErr,
            1900298 => Self::SpringFestivalParamAgainErr,
            1900299 => Self::PayShopGoodsIdRepeatErr,
            1900300 => Self::PayShopBindActivityErr,
            1900301 => Self::PayShopActivityFromErr,
            1900302 => Self::PayShopFromErr,
            1900303 => Self::ErrPayShopGoodsTypeCountOverFlow,
            1900304 => Self::ErrPayShopTotalCountOverFlow,
            1900305 => Self::ErrRegressDevelopTask,
            1900306 => Self::ErrRegressVersion,
            1900307 => Self::DrinksLevelLock,
            1900308 => Self::DrinksRoleInviteErr,
            1900309 => Self::DrinksNoRoleData,
            1900310 => Self::GuessJokerTraceOver,
            1900311 => Self::GuessJokerDataErr,
            1900312 => Self::GuessJokerTaskRunning,
            1900313 => Self::GuessJokerTaskDone,
            1900314 => Self::DrinksRewardTaskRunning,
            1900315 => Self::DrinksRewardTaskDone,
            1900316 => Self::CoopReqLenthOver,
            1900317 => Self::CoopRewardIdErr,
            1900318 => Self::CoopRewardConfigNotFind,
            1900319 => Self::CoopRewardDiff,
            1900320 => Self::CoopActivityDataErr,
            1900321 => Self::CoopRewardDone,
            1900322 => Self::CoopRewardNoFinish,
            1900323 => Self::CoopItemDropErr,
            1900324 => Self::CoopItemDropErr2,
            1900325 => Self::PinballParamLimitOver,
            1900326 => Self::PinballTaskConfigLoss,
            1900327 => Self::PinballActivityErr,
            1900328 => Self::PinballActivityHadReward,
            1900329 => Self::PinballActivityTaskRunning,
            1900330 => Self::PinballActivityTaskDropNoFind,
            1900331 => Self::PinballActivityTaskParamAgainErr,
            1900332 => Self::PinballActivityTaskNoReward,
            1900333 => Self::PinballRoleAddLevelErr,
            1900334 => Self::PinballRoleConfigLoss,
            1900335 => Self::PinballRoleDataLoss,
            1900336 => Self::PinballRoleDataMaxLvLoss,
            1900337 => Self::PinballRoleMaxLvErr,
            1900338 => Self::PinballActivityConfigLoss,
            1900339 => Self::PinballRoleExpItemLack,
            1900340 => Self::PinballWeaponIsTrail,
            1900341 => Self::PinballWeaponNoExist,
            1900342 => Self::PinballWeaponConfigNoExist,
            1900343 => Self::PinballRoleNoExist,
            1900344 => Self::PinballRoleConfigNoExist,
            1900345 => Self::PinballWeaponRoleMismatch,
            1900346 => Self::PinballPersonWeaponChangeErr,
            1900347 => Self::PinballWearRepeat,
            1900348 => Self::PinballWeaponOverLimit,
            1900349 => Self::PinballWeaponRepeat,
            1900350 => Self::PinballWeaponLock,
            1900351 => Self::PinballWeaponIsEquip,
            1900352 => Self::PinballWeaponNoDel,
            1900353 => Self::ActivityRewardParamOver,
            1900354 => Self::ActivityNotSame,
            1900355 => Self::ActivityRewardDropErr,
            1900356 => Self::ActivityNoRewardCanGet,
            1900357 => Self::ActivityTaskTimeErr,
            1900358 => Self::ActivityTaskRewardRepeat,
            1900359 => Self::ActivityDataNotFind,
            1900360 => Self::PinballWeaponInitErr,
            1900361 => Self::PinballNoFriend,
            1900362 => Self::PinballTowerRankCd,
            1900363 => Self::PinballRankDataEmpty,
            1900364 => Self::PinballLevelLock,
            1900365 => Self::PinballDailyInstErr,
            1900366 => Self::PinballRoleNotOpen,
            1900367 => Self::PinballDailyNotOpen,
            1900368 => Self::PinballWeaponPersonMismatch,
            1900369 => Self::NoRoleVoiceDataSet,
            1900370 => Self::RoleVoiceSettingParamOver,
            1900371 => Self::RoleVoiceParamRepeat,
            1900372 => Self::PlayerUsingSkill,
            1900373 => Self::PlayerClimbing,
            1900374 => Self::PlayerInWater,
            1900375 => Self::PlayerInAir,
            1900376 => Self::OnlineMotorLevelLock,
            1900377 => Self::OnlineMotorMemberErr,
            1900378 => Self::OnlineMotorMemberActivityClose,
            1900379 => Self::RealmBetweenOverMaxLevel,
            1900380 => Self::ActivityTaskLock,
            1900381 => Self::ErrRecallItemBagItemTypeNotSupport,
            1900382 => Self::ErrRecallItemBagFull,
            1900383 => Self::ActivityPlayingCanNotDissolve,
            1900384 => Self::ErrRecallActionNotSupport,
            1900385 => Self::ErrAdventureSilentCtxNull,
            1900386 => Self::ErrAdventureSilentId,
            1900387 => Self::RoverRogueTalentAlreadyUnlock,
            1900388 => Self::RoverRoguePreTalentNotUnlock,
            1900389 => Self::RoverRogueTalentConditionNotFinish,
            1900390 => Self::ErrMailFavoriteCannotDelete,
            1900391 => Self::ErrMailFavoriteNeedRead,
            1900392 => Self::ErrMailFavoriteNeedTake,
            1900393 => Self::ErrMailFavoriteFull,
            1900394 => Self::ErrPhantomItemUse,
            1900395 => Self::ErrRoleTrailItemUse,
            1900396 => Self::ErrInFlowItemUse,
            1900397 => Self::ErrInVehicleItemUse,
            1900398 => Self::ErrZitherNoMainRoleInFormation,
            1900399 => Self::ErrTrialFollowShooterItemUse,
            1900400 => Self::ErrMainRoleVoiceRoleId,
            1900401 => Self::ErrTrackMoonNotInRecall,
            1900402 => Self::MatchThreeRoleLock,
            1900403 => Self::ErrMatchThreeScoreInvalid,
            1900404 => Self::ErrMatchThreeHourGlassInvalid,
            1900405 => Self::ErrMatchThreeBottleInvalid,
            1900406 => Self::ErrMatchThreeRecordTypeMismatch,
            1900407 => Self::CameraModeNotOpen,
            1900408 => Self::ErrItemUseBanned,
            1900409 => Self::ErrItemUseMainRoleDead,
            1900410 => Self::ErrWaitingOtherJoinBanUse,
            1900411 => Self::ErrPhantomSelfPortrait,
            1900412 => Self::ErrRoleTrailSelfPortrait,
            1900413 => Self::ErrPlayerStateSelfPortrait,
            1900414 => Self::ErrMainRoleDeadSelfPortrait,
            1900415 => Self::ErrInFlowSelfPortrait,
            1900416 => Self::ErrInVehicleSelfPortrait,
            1900417 => Self::ErrMainRoleActiveSelfPortrait,
            1900418 => Self::ErrZitherItemUse,
            1900419 => Self::InTeleportItemUse,
            1900420 => Self::IsMatchingItemUse,
            1900421 => Self::InTeleportSelfPortrait,
            1900422 => Self::IsMatchingSelfPortrait,
            1900460 => Self::ErrRecallItemUseFail,
            1900461 => Self::ErrRecallItemUseTypeNotSupport,
            1900462 => Self::ErrRecallItemTypeNotSupport,
            1900463 => Self::ErrRecallItemCountInvalid,
            1900464 => Self::ErrRecallItemCanNotUse,
            1900467 => Self::ErrWaitingOtherJoinSelfPortrait,
            2000000 => Self::ErrDoCommonRewardConfigError,
            2000001 => Self::InstPlayNotSettle,
            2000002 => Self::InstPlayNotFinishExecute,
            2000003 => Self::ErrResetItemEntityNotContain,
            2000004 => Self::InstPlayExchangeRewardNotExist,
            2000005 => Self::MapConfigNull,
            2000006 => Self::MapConfigError,
            2000007 => Self::InstPlayComponentNotExist,
            2000008 => Self::InstTeleportResetPlayerDead,
            2000009 => Self::DrownEndTeleportInBigWorld,
            2000010 => Self::ErrFightTrialRoleRoldIdsError,
            2000011 => Self::ErrFightTrialRoleFromationError,
            2000012 => Self::ErrInstSaveFail,
            2000013 => Self::ErrActiveFoundationControlPlayerError,
            2000014 => Self::ErrActiveFoundationOccupation,
            2000015 => Self::ErrSingleInstanceCanNotOnline,
            2000016 => Self::ErrInstanceRechallengeLimit,
            2000017 => Self::ErrTargetSame,
            2000018 => Self::ErrAttachTargetType,
            2000019 => Self::ErrAttachInfoNull,
            2000020 => Self::ErrLevelPlayChallengeFail,
            2000021 => Self::ErrGmtip,
            2000022 => Self::ErrPosSenderEntityNoExist,
            2000023 => Self::ErrPosSenderComponentNoExist,
            2000024 => Self::ErrPosSenderParamError,
            2000025 => Self::ErrPosSenderRemoveSenderNotSame,
            2000026 => Self::ErrConnectorEntityNoExist,
            2000027 => Self::ErrConnectorCompNoExist,
            2000028 => Self::ErrConnectorPreIdError,
            2000029 => Self::ErrConnectorCompleteState,
            2000030 => Self::ErrConnectorActiveStateError,
            2000031 => Self::ErrConnectorMatchErro,
            2000032 => Self::ErrActiveControlOccupation,
            2000033 => Self::ErrPortalCreatorActive,
            2000034 => Self::ErrComponentNull,
            2000035 => Self::ErrPortalCreatorConfigError,
            2000036 => Self::ErrPortalCreatorCreateFail,
            2000037 => Self::ErrTrialRoleEnterInst,
            2000038 => Self::ErrNpcInVehicle,
            2000039 => Self::ErrSceneItemBbnotChange,
            2000040 => Self::ErrInitMatchNotSuccess,
            2000041 => Self::ErrFightFormationSameRoleError,
            2000042 => Self::ErrFightMainRoleConflict,
            2000043 => Self::ErrVehicleItemConfigError,
            2000044 => Self::ErrVehicleCreateError,
            2000045 => Self::ErrVehicleEntityTypeError,
            2000046 => Self::ErrInstanceActivityExpire,
            2000047 => Self::ErrSlashAndTowerLevelUnlock,
            2000048 => Self::ErrInstLevelUnlock,
            2000049 => Self::ErrEnterInstConfigError,
            2000050 => Self::ErrMoveWithSplineConfigError,
            2000051 => Self::ErrMoveWithSplineIdError,
            2000052 => Self::ErrMoveWithSplineStop,
            2000053 => Self::ErrMoveWithSplineEntityConfigError,
            2000054 => Self::ErrMoveWithSplineControlPlayerError,
            2000055 => Self::ErrLeaveSceneStarted,
            2000056 => Self::ErrInstTeleportInstLimit,
            2000057 => Self::ErrGetOnVehicleNotInAoiSight,
            2000058 => Self::ErrHoldHandCharacterNotCurrent,
            2000059 => Self::ErrEnterInstPreInstNotComplete,
            2000060 => Self::ErrSetSystemVarError,
            2000061 => Self::ErrSetVarPermission,
            2000062 => Self::ErrSetVarCtxError,
            2000063 => Self::ErrSetVarGameCtxSceneError,
            2000064 => Self::ErrSetVarNotPublic,
            2000065 => Self::ErrSetVarComponent,
            2000066 => Self::ErrSetVarTargetNull,
            2000067 => Self::ErrVehicleNotMotor,
            2000068 => Self::ErrVehicleFormationError,
            2000069 => Self::ErrShareRideConflict,
            2000070 => Self::ErrShareRideNotExist,
            2000071 => Self::ErrNotInShareRide,
            2000072 => Self::ErrNotInVehicle,
            2000073 => Self::ErrSceneRoadGraphNotExist,
            2000074 => Self::ErrVehicleFlowRewardMax,
            2000075 => Self::ErrGetOnVehicleFailInFlow,
            2000076 => Self::ErrGetOnPlayerVechielGravityDifferent,
            2100000 => Self::ErrChangeFightState,
            2100001 => Self::ErrAddFragileFail,
            2100002 => Self::ErrStoreEnergyClose,
            2100003 => Self::ErrAttrOverMax,
            2100004 => Self::ErrBattleVersion,
            2200000 => Self::ErrGmkillEntityNotValid,
            2200001 => Self::ErrSplineConfigNotExist,
            2200002 => Self::BossRushActivityNotOpen,
            2200003 => Self::BossRushActivityScoreRewardNotExist,
            2200004 => Self::BossRushActivityLevelRewardNotExist,
            2200005 => Self::BossRushActivityScoreNotEnough,
            2200006 => Self::BossRushActivityLevelNotPass,
            2200007 => Self::BossRushActivityRewardClaimed,
            2200008 => Self::BossRushActivityBuffSelectionNotValid,
            2200009 => Self::BossRushActivityConfigNotExist,
            2200010 => Self::BossRushActivityCharacterSelectionNotValid,
            2200011 => Self::BossRushActivityComponentNotExist,
            2200012 => Self::BossRushActivityCharacterSelectionEmpty,
            2200013 => Self::BossRushActivityBuffSelectionEmpty,
            2200014 => Self::BossRushActivityLevelNotOpen,
            2200015 => Self::InRangeEntityDuplicate,
            2200016 => Self::InRangeEntityNotExist,
            2200017 => Self::NpcPerformComponentNotExist,
            2200018 => Self::NpcPerformStateNotInit,
            2200019 => Self::NpcPerformActionTargetEntityNotExist,
            2200020 => Self::ActionQueueTypeNotExist,
            2200021 => Self::ActionQueueCtxTypeNotExist,
            2200022 => Self::ExecuteQueueOwnerHasAction,
            2200023 => Self::ActionQueueExceedMaxCount,
            2200024 => Self::ActionQueueStartActionGroupFail,
            2200025 => Self::ActionQueueComponentNotExist,
            2200026 => Self::ActionQueueNotInit,
            2200027 => Self::ChangeBatchEntitiesStateError,
            2200028 => Self::EnableNearbyTrackingTargetEntityNotExist,
            2200029 => Self::EnableNearbyTrackingSelfNotEntity,
            2200030 => Self::EnableNearbyTrackingSelfComponentNotExist,
            2200031 => Self::SetTeleControlEntityNotExist,
            2200032 => Self::SetTeleControlTypeNotExist,
            2200033 => Self::SetTeleControlComponentNotExist,
            2200034 => Self::SetTeleControlCoordEntityNotExist,
            2200035 => Self::SceneItemAttributeIdNotInType,
            2200036 => Self::SceneItemAttributeIdNotExist,
            2200037 => Self::ModifySceneItemAttributeEntityNotExist,
            2200038 => Self::AddSceneItemAttributeTagDuplicate,
            2200039 => Self::RemoveSceneItemAttributeTagNotExist,
            2200040 => Self::AttributeEntityLock,
            2200041 => Self::AttributeEntitySilent,
            2200042 => Self::ModifySceneItemAttributeTagNotExist,
            2200043 => Self::ErrEnterInstCtx,
            2200044 => Self::ErrEnterInstBlackboardValueNotExist,
            2200045 => Self::TriggerLocked,
            2200046 => Self::TriggerIgnore,
            2200047 => Self::TriggerEntityNull,
            2200048 => Self::TriggerEntityNotMatch,
            2200049 => Self::TriggerMatchCountNotMet,
            2200050 => Self::TriggerActionEmpty,
            2200051 => Self::ExceedMaxTriggerCount,
            2200052 => Self::TriggerAlreadyLeaveWhenEnterCondFail,
            2200053 => Self::TriggerLeaveConfigEmpty,
            2200054 => Self::TriggerConditionNotMet,
            2200055 => Self::TriggerRangeRationalityFail,
            2200056 => Self::TrampleEntityNotMatch,
            2200057 => Self::TrampleConditionNotMet,
            2200058 => Self::TrampleMatchCountNotMet,
            2200059 => Self::HasDestroySelfActionInQueue,
            2200060 => Self::EntityWillDestroy,
            2200061 => Self::AddInRangePlayerDuplicate,
            2200062 => Self::AddInRangeEntityDuplicate,
            2200063 => Self::RemoveInRangePlayerNotExist,
            2200064 => Self::RemoveInRangeEntityNotExist,
            2200072 => Self::GravityDirectionNoChange,
            2200065 => Self::HookExitWayNotExist,
            2200066 => Self::HookLockPointLocked,
            2200067 => Self::HookLockAddPlayerDuplicate,
            2200068 => Self::HookLockRemovePlayerNotExist,
            2200069 => Self::KiteHookLockPointOnlyOnePlayer,
            2200073 => Self::EffectAreaAddBuffFail,
            2200074 => Self::ErrGravityInteractNoPermission,
            2200075 => Self::PlayerNotInAnyGravityRegion,
            2200070 => Self::ErrSceneEntityAlreadyExist,
            2200076 => Self::PlayerLeaveGravityRegionInAbnormalGravity,
            2200071 => Self::HookLockPointConditionNotMet,
            2200077 => Self::PlayerInAbnormalGravity,
            2200078 => Self::GravityFlipIndexNoChange,
            2200079 => Self::GravityFlipIndexNotExist,
            2200080 => Self::GravityFlipTypeNoChange,
            2200081 => Self::GravityFlipTypeNotExist,
            2200082 => Self::GravityFlipNotUpdateToTargetDirection,
            2200083 => Self::ReliablePosNotInGravityRegion,
            2200084 => Self::TargetPosNotInGravityRegion,
            2200085 => Self::PlayerCurGravityDirectionNotInOptions,
            2200086 => Self::WaterfallClimbingParticipatorCountErr,
            2200087 => Self::WaterfallClimbingParticipatorNoVehicle,
            2200088 => Self::WaterfallClimbingVehicleNoPassenger,
            2200089 => Self::WaterfallClimbingPlayerNotInPassenger,
            2200090 => Self::ErrVehicleEntityNotExist,
            2200091 => Self::GravityFlipLocked,
            2200092 => Self::ClientActionSkipped,
            2200093 => Self::AbnormalGravityCannotAddTemporary,
            2200094 => Self::ErrReliablePosEntityNotExist,
            2200095 => Self::UpdateReliablePosGravityDirNotMatch,
            2200096 => Self::UpdateReliablePosNotWalkable,
            2200097 => Self::UpdateReliablePosSelfNotEntity,
            2200098 => Self::GravityDirectionHasNaN,
            2200099 => Self::ErrHostInAbnormalGravity,
            2200100 => Self::MultiModeCannotTeleport,
            2200101 => Self::NotTrapDefenseInst,
            2200102 => Self::GridCellCreateFuncNotFound,
            2200103 => Self::GridSystemNotExist,
            2200104 => Self::ErrGridCellType,
            2200105 => Self::GridCellDirectionNotSpecified,
            2200106 => Self::NotGridObjectEntity,
            2200107 => Self::GridNotExist,
            2200108 => Self::GridCellNotExist,
            2200109 => Self::GridCellAlreadyOccupied,
            2200110 => Self::GridCellDisabled,
            2200111 => Self::GridCellPositionNotValid,
            2200112 => Self::SimpleCombatEntityNotMonster,
            2200113 => Self::GridObjectEntityNotFound,
            2200114 => Self::NotLowMemoryPlatform,
            2200115 => Self::ErrLowMemorySwitchingScene,
            2200116 => Self::SimpleCombatEntityBuffNotExist,
            2200117 => Self::SimpleCombatEntityBuffDuplicate,
            2200118 => Self::TrapDefenseRandomBdFail,
            2200119 => Self::TrapDefenseBdConfigNotExist,
            2200120 => Self::TrapDefenseBdGroupConfigOfBdNotExist,
            2200121 => Self::TrapDefenseRandomQualityFail,
            2200122 => Self::TrapDefenseRandomBdGroupFail,
            2200123 => Self::TrapDefenseBdGroupPoolEmpty,
            2200124 => Self::TrapDefenseNoAvailableBd,
            2200125 => Self::TrapDefenseNoAvailableQuality,
            2200126 => Self::TrapDefenseDrawActionConfigNotExist,
            2200127 => Self::TrapDefenseInstConfigNotExist,
            2200128 => Self::TrapDefenseWaveConfigNotExist,
            2200129 => Self::TrapDefenseSpawnMonsterWaveConfigNotExist,
            2200130 => Self::TrapDefenseMonsterConfigNotExist,
            2200131 => Self::TrapDefenseMonsterDataDuplicate,
            2200132 => Self::TrapDefenseTrapConfigNotExist,
            2200133 => Self::TrapDefenseTrapEntityNotFound,
            2200134 => Self::TrapDefenseNotPreviewStep,
            2200135 => Self::TrapDefenseBdGroupConfigNotExist,
            2200136 => Self::TrapDefenseBdPoolEmpty,
            2200137 => Self::TrapDefenseQualityPoolEmpty,
            2200138 => Self::ErrTrapDefenseRequestParam,
            2200139 => Self::ErrTrapDefenseRequestParamDuplicate,
            2200140 => Self::ErrTrapDefenseRewardConfig,
            2200141 => Self::ErrTrapDefenseRewardTime,
            2200142 => Self::ErrTrapDefenseRewardReceived,
            2200143 => Self::ErrTrapDefenseRewardNotComplete,
            2200144 => Self::ErrTrapDefenseSceneErr,
            2200145 => Self::ErrTrapDefenseContextEmpty,
            2200146 => Self::ErrTrapDefenseChallengeConfig,
            2200147 => Self::ErrTrapDefenseChallengeNotOpen,
            2200148 => Self::ErrTrapDefenseBuildingLock,
            2200149 => Self::ErrTrapDefenseAuxiliaryLock,
            2200150 => Self::ErrTrapDefenseSlotCount,
            2200151 => Self::ErrTrapDefenseSlotBuildingDuplicate,
            2200152 => Self::ErrTrapDefenseSlotDuplicate,
            2200153 => Self::ErrTrapDefenseBuildingConfig,
            2200154 => Self::ErrTrapDefenseAuxiliaryConfig,
            2200155 => Self::ErrTrapDefenseSlotAuxiliaryDuplicate,
            2200156 => Self::ErrTrapDefenseForceBuilding,
            2200157 => Self::ErrTrapDefenseForceAuxiliary,
            2200158 => Self::ErrTrapDefenseBuildingLevelConfig,
            2200159 => Self::ErrTrapDefenseBuildingMaxLevel,
            2200160 => Self::ErrTrapDefenseActivityConfig,
            2200161 => Self::ErrTrapDefenseAuxiliaryLevelConfig,
            2200162 => Self::ErrTrapDefenseAuxiliaryMaxLevel,
            2200163 => Self::ErrTrapDefenseAuxiliaryBranch,
            2200164 => Self::ErrTrapDefenseBuildingBranch,
            2200165 => Self::ErrTrapDefenseAuxiliaryReset,
            2200166 => Self::ErrTrapDefenseBuildingReset,
            2200167 => Self::ErrTrapDefenseTechConfig,
            2200168 => Self::ErrTrapDefenseTechUnlock,
            2200169 => Self::ErrTrapDefenseTechPreNodeConfig,
            2200170 => Self::ErrTrapDefenseTechPreNodeLock,
            2200171 => Self::ErrTrapDefenseTechCostNotEnough,
            2200172 => Self::TrapDefenseItemConfigNotExist,
            2200173 => Self::TrapDefenseItemExceedCarryLimit,
            2200174 => Self::TrapDefenseItemNotEnough,
            2200175 => Self::TrapDefenseBuyShopBdGroupNotExist,
            2200176 => Self::TrapDefenseBuyShopItemNotExist,
            2200177 => Self::TrapDefenseGoldNotEnough,
            2200178 => Self::TrapDefensePayGoldFail,
            2200179 => Self::TrapDefenseBuyShopItemNotEnough,
            2200180 => Self::TrapDefenseShopDataNotExist,
            2200181 => Self::TrapDefenseShopConfigNotExist,
            2200182 => Self::TrapDefenseShopRefreshCountNotEnough,
            2200183 => Self::TrapDefenseGoldenCoinConfigNotExist,
            2200184 => Self::TrapDefenseNotGoldenCoinEntity,
            2200185 => Self::TrapDefenseNotMonsterEntity,
            2200186 => Self::TrapDefenseNotBuildingEntity,
            2200187 => Self::TrapDefenseVarCannotUnderZero,
            2200188 => Self::TrapDefenseVarTypeNotExist,
            2200189 => Self::TrapDefenseBuyShopBdGroupSold,
            2200190 => Self::TrapDefenseTrapAmountExceedLimit,
            2200191 => Self::TrapDefenseNotRewardStep,
            2200192 => Self::TrapDefenseBdDrawDataNotExist,
            2200193 => Self::TrapDefenseBdDrawRefreshCountNotEnough,
            2200194 => Self::TrapDefenseSpecialCellConfigNotExist,
            2200195 => Self::TrapDefenseSpecialCellCertainLevelConfigNotExist,
            2200196 => Self::ErrActionTargetEntityNotExist,
            2200197 => Self::TrapDefenseCleanerNotEnough,
            2200198 => Self::TrapDefenseItemConfigError,
            2200199 => Self::TrapDefenseSpecialCellTypeNotSame,
            2200200 => Self::TrapDefenseCellNotSpecialCell,
            2200201 => Self::TrapDefenseExceedMaxTrapCount,
            2200202 => Self::GridSystemCannotGetWorldGridPos,
            2200203 => Self::TrapDefenseTrapCannotSelfDestruct,
            2200204 => Self::TrapDefenseBdDrawResultEmpty,
            2200205 => Self::TrapDefenseBdGroupMaxLevel,
            2200206 => Self::TrapDefenseCannotDeductHealth,
            2200207 => Self::SimpleCombatBuffConfigNotExist,
            2200208 => Self::SimpleCombatSubTypeNotExist,
            2200209 => Self::TrapDefenseBdGroupDisabled,
            2200210 => Self::TrapDefenseMonsterDisabled,
            2200211 => Self::TrapDefenseGainExceedLimit,
            2200212 => Self::OnlyOneRollBlockCanBeActivated,
            2200213 => Self::RollBlockGroupConfigNotExist,
            2200214 => Self::RollBlockDifficultyConfigNotExist,
            2200215 => Self::RollBlockNextDifficultyConfigNotExist,
            2200216 => Self::RollBlockGamePlayIdNotExist,
            2200217 => Self::RollBlockGamePlayNotExist,
            2200218 => Self::RollBlockMovementEntityNotExist,
            2200219 => Self::RollBlockMovementEntityNotBlock,
            2200220 => Self::RollBlockMainControlPlayerNotExist,
            2200221 => Self::RollBlockEntityNotMoving,
            2200222 => Self::RollBlockNoGridAfterRoll,
            2200223 => Self::RollBlockTemplateConfigNotExist,
            2200224 => Self::RollBlockNotMainControlPlayer,
            2200225 => Self::RollBlockEntityNotExist,
            2200226 => Self::RollBlockBlockWillChangeState,
            2200227 => Self::RollBlockBlockStateNoChange,
            2200228 => Self::RollBlockMainControlBlockNotExist,
            2200229 => Self::RollBlockDifficultyDataNotExist,
            2200230 => Self::RollBlockPositionNoFloor,
            2200231 => Self::RollBlockPositionItemCannotPass,
            2200232 => Self::RollBlockPositionBlockCannotPass,
            2200233 => Self::RollBlockInputCannotMove,
            2200234 => Self::RollBlockPositionFloorCannotPass,
            2200235 => Self::RollBlockHintAlreadyActive,
            2200236 => Self::RollBlockHintBlockNotExist,
            2200237 => Self::RollBlockHintEmpty,
            2200238 => Self::RollBlockAlreadyRewarded,
            2200239 => Self::RollBlockDifficultyNotPassed,
            2200240 => Self::RollBlockHintStepNotExecuting,
            2200241 => Self::RollBlockCurDifficultyConfigNotExist,
            2200242 => Self::RollBlockHintStepExecuting,
            2200243 => Self::RollBlockChildQuestGamePlayActived,
            2200244 => Self::RollBlockOnlyOneControllableBlock,
            2200245 => Self::RollBlockGamePlayStateNoChange,
            2200246 => Self::RollBlockGamePlayCannotReady,
            2200247 => Self::RollBlockGamePlayNotReady,
            2200248 => Self::RollBlockSendResetTooFrequently,
            2200249 => Self::EasterEggIdNotExist,
            2200250 => Self::TargetSceneNotBigWorldInst,
            2200251 => Self::EnterBigWorldInstCtxNotImplemented,
            2200252 => Self::EasterEggOwnerTypeNotImplemented,
            2200253 => Self::RollBlockCannotReset,
            2200254 => Self::CurSceneNotBigWorld,
            2200255 => Self::KurotatoCharacterConfigNotExist,
            2200256 => Self::KurotatoLevelInfoNotExist,
            2200257 => Self::KurotatoUpgradeRewardStepNotActive,
            2200258 => Self::KurotatoUpgradeRewardItemNotFound,
            2200259 => Self::KurotatoUpgradeRewardDataNotExist,
            2200260 => Self::KurotatoActivityConfigNotExist,
            2200261 => Self::KurotatoWeaponConfigNotExist,
            2200262 => Self::KurotatoWeaponNotExist,
            2200263 => Self::KurotatoItemConfigNotExist,
            2200264 => Self::KurotatoItemNotExist,
            2200265 => Self::KurotatoChestRewardStepNotActive,
            2200266 => Self::KurotatoChestRewardItemNotExist,
            2200267 => Self::KurotatoEffectConfigNotExist,
            2200268 => Self::KurotatoEffectNotExist,
            2200269 => Self::KurotatoDrawConfigNotExist,
            2200270 => Self::KurotatoPropertyConfigNotExist,
            2200271 => Self::KurotatoPropertyNotInSimpleCombatDefine,
            2200272 => Self::KurotatoShopStepNotActive,
            2200273 => Self::KurotatoShopProductNotFound,
            2200274 => Self::KurotatoShopGoldNotEnough,
            2200275 => Self::KurotatoVarNameNotExist,
            2200276 => Self::KurotatoVarNotExist,
            2200277 => Self::KurotatoRoleLevelConfigNotExist,
            2200278 => Self::KurotatoCharacterEntityNotExist,
            2200279 => Self::KurotatoGoldNotEnough,
            2200280 => Self::KurotatoMonsterConfigNotExist,
            2200281 => Self::KurotatoMultiModeNotSupported,
            2200282 => Self::KurotatoLevelConfigNotExist,
            2200283 => Self::KurotatoDropInfoNotExist,
            2200284 => Self::KurotatoDropConfigNotExist,
            2200285 => Self::KurotatoWaveConfigNotExist,
            2200286 => Self::KurotatoWeaponCountReachLimit,
            2200287 => Self::KurotatoWeaponRefineNotEnoughSameQuality,
            2200288 => Self::KurotatoItemStackFull,
            2200289 => Self::KurotatoCharacterUpgradeConfigNotExist,
            2200290 => Self::KurotatoUpgradeRewardDrawIdNotExist,
            2200291 => Self::KurotatoDrawResultEmpty,
            2200292 => Self::KurotatoLuckPropertyNotExist,
            2200293 => Self::KurotatoGoldGainPropertyNotExist,
            2200294 => Self::KurotatoCharacterAlreadyExist,
            2200295 => Self::KurotatoCombatStepNotActive,
            2200296 => Self::KurotatoNotInSpecialWave,
            2200297 => Self::KurotatoWeaponCountReachMin,
            2200298 => Self::KurotatoNotSupportedProductType,
            2200299 => Self::KurotatoStructureConfigNotExist,
            2200300 => Self::KurotatoStructureCannotCreate,
            2300000 => Self::NotQaAccountWithHiddenServer,
            2300001 => Self::DisabledFuncInHiddenServer,
            2400000 => Self::ErrActionExecutorFinishConditionNotSport,
            2400001 => Self::ErrAlreadyInSwitchNode,
            2500000 => Self::ErrCornActivityId,
            2500001 => Self::ErrCornActivityNoOpen,
            2500002 => Self::ErrCornNoActivityData,
            2500003 => Self::NoPlayIdCorniceReward,
            2500004 => Self::ActivityNoOpenCorniceReward,
            2500005 => Self::ScoreLimitCorniceReward,
            2500006 => Self::RewardedCorniceReward,
            2500007 => Self::NoUnlockCorniceReward,
            2500008 => Self::NoScoreCorniceReward,
            2500009 => Self::TrackMoonPhaseNoConfig,
            2500010 => Self::TrackMoonPhaseActivityNoOpen,
            2500011 => Self::TrackMoonPhaseNoPolulary,
            2500012 => Self::TrackMoonPhaseNoData,
            2500013 => Self::TrackMoonPhaseRewarded,
            2500014 => Self::TrackMoonPhaseDataNoConfig,
            2500015 => Self::TrackMoonPhaseDataNoOpen,
            2500016 => Self::TrackMoonPhaseDataNoData,
            2500017 => Self::BctrewardNoTconfig,
            2500018 => Self::BctrewardNoOpenActivity,
            2500019 => Self::BctrewardNoData,
            2500020 => Self::BctrewardNoUnlock,
            2500021 => Self::BctrewardNoComplete,
            2500022 => Self::BcarewardNoRconfig,
            2500023 => Self::BcarewardNoOpenActivity,
            2500024 => Self::BcarewardNoData,
            2500025 => Self::BcarewardNoActive,
            2500026 => Self::Bcarewarded,
            2500027 => Self::BcarewardNoActiveReward,
            2500028 => Self::CornTranNoPlayConfig,
            2500029 => Self::CornTranNoOpenPlay,
            2500030 => Self::CornTranNoOpenActivity,
            2500031 => Self::CornTranNoEntityConfig,
            2500032 => Self::BcarewardRepeatRewardId,
            2500033 => Self::BcarewardDifferActivityId,
            2500034 => Self::BctrewardNoActivity,
            2500035 => Self::BctrewardIllegalRewardNum,
            2500036 => Self::BctrewardNoUnlockStage,
            2500037 => Self::PreheatSignNodeNoConfig,
            2500038 => Self::PreheatSignNodeNoData,
            2500039 => Self::PreheatSignActivityOnOpen,
            2500040 => Self::PreheatSignNodeNoUnlock,
            2500041 => Self::PreheatSignNodeNoRewardStatus,
            2500042 => Self::PreheatSignNodeNoAnswer,
            2500043 => Self::ScratchCardNoRoundConfig,
            2500044 => Self::ScratchCardNoActivityConfig,
            2500045 => Self::ScratchCardNoDbData,
            2500046 => Self::ScratchCardIllegalIndex,
            2500047 => Self::ScratchCardIndexRewarded,
            2500048 => Self::ScratchCardNoTime,
            2500049 => Self::ScratchCardNoRandomReward,
            2500050 => Self::ScratchCardRoundNoUnlock,
            2500051 => Self::BossRushPlayerNoSceneData,
            2500052 => Self::BossRushPlayerCanNoChooseBuff,
            2500053 => Self::BossRushPlayerIllegalIndex,
            2500054 => Self::BossRushHadSameBuffId,
            2500055 => Self::BossRushBuffCountLimit,
            2500056 => Self::BossRushBuffIllegal,
            2500057 => Self::BossRushBuffNoConfig,
            2500058 => Self::ScratchCardActivityNoOpen,
            2500059 => Self::MowToweNoLevelConfig,
            2500060 => Self::MowTowerActivityNoOpen,
            2500061 => Self::MowToweNoCacheData,
            2500062 => Self::MowTowerNoPassFirstInst,
            2500063 => Self::MowTowerLevelsIdError,
            2500064 => Self::MowTowerRoleIdError,
            2500065 => Self::MowTowerBuffIdError,
            2500066 => Self::MowTowerNoComponent,
            2500067 => Self::MowTowerHadSameRole,
            2500068 => Self::MowTowerBuffCountError,
            2500069 => Self::MowTowerNoFirstInst,
            2500070 => Self::MowTowerNoActivityData,
            2500071 => Self::MowTowerScoreLimit,
            2500072 => Self::MowTowerScoreRewarded,
            2500073 => Self::MowTowerScoreRewardConfig,
            2500074 => Self::MowTowerNoInScene,
            2500075 => Self::MowTowerNoRewardConfig,
            2500076 => Self::MaterialReplaceNoTargetConfig,
            2500077 => Self::MaterialReplaceNoConsumeConfig,
            2500078 => Self::MaterialReplaceNoSameGroup,
            2500079 => Self::MaterialReplaceErrConsumeNum,
            2500080 => Self::ErrRoleSkinTrialNotInit,
            2500081 => Self::ErrRoleSkinTrialNoFinish,
            2500082 => Self::ErrRoleSkinTrialReward,
            2500083 => Self::ErrRoleSkinTrialRewardDone,
            2500084 => Self::ErrRoleSkinTrialTimeOut,
            2500085 => Self::PhantomEquipGroupNoEquipPhantom,
            2500086 => Self::PhantomEquipGroupCountLimit,
            2500087 => Self::PhantomEquipGroupHadInTop,
            2500088 => Self::PhantomEquipGroupNameEmpty,
            2500089 => Self::PhantomEquipGroupNameCountLimiy,
            2500090 => Self::PhantomRecommendFuncNoOpen,
            2500091 => Self::BossRushTaskNoFinish,
            2500092 => Self::PhantomGroupUseSame,
            2500093 => Self::PhantomGroupFunNoOpen,
            2500094 => Self::NoFishingActivityConfig,
            2500095 => Self::NoInFishingActivityTime,
            2500096 => Self::FishingActivityCanNoReward,
            2500097 => Self::NoFishingActivityMileConfig,
            2500098 => Self::FishingActivitySameMileId,
            2500099 => Self::FishingActivityRewarded,
            2500100 => Self::PlayerTitleFuncNoOpen,
            2500101 => Self::PlayerTitleHadUndress,
            2500102 => Self::PlayerTitleHadDress,
            2500103 => Self::PlayerTitleNoUnlock,
            2500104 => Self::BabelTowerActivityNoOpen,
            2500105 => Self::BabelTowerLevelNoOpen,
            2500106 => Self::BabelTowerDeEffectNoFind,
            2500107 => Self::BabelTowerDeEffectCanNoChoose,
            2500108 => Self::BabelTowerDeEffectMutex,
            2500109 => Self::BabelTowerNoDailyTask,
            2500110 => Self::BabelTowerTaskNoComplete,
            2500111 => Self::BabelTowerTaskRewarded,
            2500112 => Self::BabelTowerRoleLimit,
            2500113 => Self::BabelTowerNoBuffConfig,
            2500114 => Self::BabelTowerBuffCanNoChoose,
            2500115 => Self::BabelTowerBuffNumIllegal,
            2500116 => Self::BabelTowerBuffChooseCountLimit,
            2500117 => Self::NoBabelTowerInsComponent,
            2500118 => Self::BabelTowerNoSelectBuff,
            2500119 => Self::PhantomPolishFuncNoOpen,
            2500120 => Self::PhantomPolishHadLevelUp,
            2500121 => Self::PhantomPolishQualityLimit,
            2500122 => Self::PhantomPolishSamePro,
            2500123 => Self::ExploreActivityNoOpen,
            2500124 => Self::ExploreActivityTaskNoFinish,
            2500125 => Self::ExploreActivityTaskRewarded,
            2500126 => Self::BabelTowerDeTermNoUnlock,
            2500127 => Self::BabelTowerBuffNoUnlock,
            2500128 => Self::BabelTowerIsNoDifficult,
            2500129 => Self::PlayerTitleNoConfig,
            2500130 => Self::PlayerFixIndexIllegal,
            2500131 => Self::PlayerFixHadFlag,
            2500132 => Self::PlayerFixFrontNoFlag,
            2500133 => Self::BabelTowerNoSelectRoles,
            2500134 => Self::PhBaErrSelectNum,
            2500135 => Self::PhBaErrSelectTarget,
            2500136 => Self::PhBaNoFighterLogic,
            2500137 => Self::PhBaNoChallengeConf,
            2500138 => Self::PhBaSlotIllegal,
            2500139 => Self::PhBaNoEvolveNum,
            2500140 => Self::PhBaNoFighter,
            2500141 => Self::PhBaNoHandCard,
            2500142 => Self::PhBaNoCardConf,
            2500143 => Self::PhBaNoCardEvolveCountInValid,
            2500144 => Self::PhBaErrCostNoMatch,
            2500145 => Self::PhBaNoCardRoleConf,
            2500146 => Self::PhBaNoContainSkill,
            2500147 => Self::PhBaNoSkillConf,
            2500148 => Self::PhBaNoCardCanNoUse,
            2500149 => Self::PhBaIsWaitClient,
            2500150 => Self::PhBaNoDeployRound,
            2500151 => Self::PhBaNoClientParam,
            2500152 => Self::PhBaNoClientParamMatch,
            2500153 => Self::PhBaErrSelectCardNum,
            2500154 => Self::PhBaErrSelectReplace,
            2500155 => Self::PhBaErrPreStep,
            2500156 => Self::PhBaCanNoSelect,
            2500157 => Self::PhBaConditionLimit,
            2500158 => Self::PhBaNoOwnerFighter,
            2500159 => Self::PhBaNoTriggerFighter,
            2500160 => Self::PhBaSelectParamNull,
            2500161 => Self::PhBaNoBuffData,
            2500162 => Self::PhBaNoBuffConf,
            2500163 => Self::PhBaErrCardGroupIndex,
            2500164 => Self::PhBaCardRoleUnlock,
            2500165 => Self::PhBaNoCardGroupConf,
            2500166 => Self::PhBaSloFull,
            2500167 => Self::PhBaHandCardEmpty,
            2500168 => Self::PbSelectCardNotFound,
            2500169 => Self::PbSelectCardCount,
            2500170 => Self::PbNoNpcFighter,
            2500171 => Self::PbNoPlayerFighter,
            2500172 => Self::PbBteventDataIllegal,
            2500173 => Self::PbNoCardFighter,
            2500174 => Self::PbNoHandCardVarData,
            2500175 => Self::PbNoPosVarData,
            2500176 => Self::PbNoHandCard,
            2500177 => Self::PbNoCardFighterVarData,
            2500178 => Self::PbNoNpcConfig,
            2500179 => Self::PbNoCostPoint,
            2500180 => Self::PbNoActiveSkill,
            2500181 => Self::PbErrSelectTargets,
            2500182 => Self::PbErrHadFighter,
            2500183 => Self::PbErrCardUnmovable,
            2500184 => Self::PbNoCanSelectTarget,
            2500185 => Self::PbCanNotOpOtherUnit,
            2500186 => Self::PbCanNoBeEvolve,
            2500187 => Self::PbKeyCostCanNotOp,
            2500188 => Self::PbNoReChallenge,
            2500189 => Self::PbFuncNoOpen,
            2500190 => Self::PbNoRequestReChallenge,
            2500191 => Self::PbNoSameSlotIndex,
            2500192 => Self::PhBaHaCardEmpty,
            2500193 => Self::PhBaPassedChallenge,
            2500194 => Self::FunPlayNoConfig,
            2500195 => Self::FunPlayNoFinish,
            2500196 => Self::FunPlayHadRewarded,
            2500197 => Self::PbCardSkillCountLimit,
            2500248 => Self::BabelRoleIsSelected,
            2500249 => Self::BabelNoTalentConfig,
            2500250 => Self::BabelInnerCanNoOpe,
            2500251 => Self::BabelTalentIsLearned,
            2500252 => Self::BabelTalentNoFinish,
            2500253 => Self::BabelNoActivityConfig,
            2500254 => Self::BabelRankListInCd,
            2500255 => Self::FunPlayDayNoOpen,
            2500262 => Self::CheckShopConditionFail,
            2500263 => Self::HonamiStoryBagUpdateRepeat,
            2500264 => Self::HonamiStoryBagNoConfig,
            2500265 => Self::HonamiStoryNormalAddItemErr,
            2500266 => Self::HonamiStoryNoBagDb,
            2500267 => Self::HonamiStoryUpdateItemSizeIllegal,
            2500268 => Self::HonamiStoryItemUpdateRepeat,
            2500269 => Self::HonamiStoryBagHadItem,
            2500270 => Self::HonamiStoryBagPosIllegal,
            2500271 => Self::HonamiStoryItemUpdateTypeErr,
            2500272 => Self::HonamiStoryBagNoHadItem,
            2500273 => Self::HonamiStoryDropItemNoEntityId,
            2500274 => Self::HonamiStoryDropItemNoEntity,
            2500275 => Self::HonamiStoryNoDropEntity,
            2500276 => Self::HonamiStoryDropNoItemDb,
            2500277 => Self::HonamiStoryItemNoBalance,
            2500278 => Self::HonamiStoryNoUpdateItem,
            2500279 => Self::HonamiStoryCheckMarkNoItemConfig,
            2500280 => Self::HonamiStoryCheckMarkOverSize,
            2500281 => Self::HonamiStoryCheckWideOverSize,
            2500282 => Self::HonamiStoryCheckMarkRepeat,
            2500283 => Self::HonamiStoryActivityNoSame,
            2500284 => Self::HonamiStoryNoEquipRackData,
            2500285 => Self::HonamiStoryNoEquipType,
            2500286 => Self::HonamiStoryNoEquipConfig,
            2500287 => Self::HonamiStoryNoEquipTypeNoMatch,
            2500288 => Self::HonamiStoryActivityIdErr,
            2500289 => Self::HonamiStoryInnerCanNoModifyRole,
            2500290 => Self::HonamiStoryRoleNoValid,
            2500291 => Self::HonamiStoryNoChallengeConfig,
            2500292 => Self::NoHonamiStoryInstType,
            2500293 => Self::HonamiStoryNoItemConfig,
            2500294 => Self::HonamiStoryPickDistanceErr,
            2500295 => Self::HonamiStoryNoDressWeapon,
            2500296 => Self::HonamiStoryNoWeaponConfig,
            2500297 => Self::HonamiStoryWeaponPluginPosErr,
            2500298 => Self::HonamiStoryWeaponPluginTypeNoMatch,
            2500299 => Self::HonamiStoryWareHouseOverSize,
            2500300 => Self::HonamiStoryNoOpenSafeBag,
            2500301 => Self::HonamiStoryNoOpenSafeBagItem,
            2500302 => Self::HonamiStoryBagTypeNoMatch,
            2500303 => Self::HonamiStoryWeaponPluginNoUnDress,
            2500304 => Self::HonamiStoryChangeItemPosSame,
            2500305 => Self::HonamiStoryEquipPosError,
            2500306 => Self::HonamiStoryEquipPosRepeat,
            2500307 => Self::HonamiStoryOriPosErr,
            2500308 => Self::HonamiStoryInstCanNoOpe,
            2500309 => Self::HonamiStoryIsRewarded,
            2500310 => Self::HonamiStoryNoFinish,
            2500311 => Self::HonamiStoryNoConfig,
            2500312 => Self::HonamiStoryHadActivateTalent,
            2500313 => Self::HonamiStoryIsUnlocked,
            2500314 => Self::HonamiStoryItemCanNoSell,
            2500315 => Self::HonamiStoryRewardRepeat,
            2500316 => Self::HonamiStoryInstHadFinish,
            2500317 => Self::HonamiStoryLifeSupportFullLevel,
            2500318 => Self::HonamiStoryDangerLevelIllegal,
            2500319 => Self::HonamiStoryWeaponHadDress,
            2500320 => Self::HonamiStoryItemIsLock,
            2500321 => Self::HonamiStoryFunNoOpen,
            2500322 => Self::HonamiStoryHadSafeLeave,
            2500323 => Self::HonamiStoryInTop,
            2500324 => Self::HonamiStoryAreaNoUnlock,
            2500325 => Self::HonamiStoryPreSlotUnlock,
            2500326 => Self::TotalTopUpNoActivityConfig,
            2500327 => Self::TotalTopUpNoFinish,
            2500328 => Self::TotalTopUpRewarded,
            2500329 => Self::TotalTopUpRoleFullChain,
            2500330 => Self::WeekCardUseRepeat,
            2500331 => Self::WeekCardNoConfig,
            2500332 => Self::WeekCardNoActive,
            2500333 => Self::WeekCardCanNoReward,
            2500334 => Self::WeekCardRewarded,
            2500335 => Self::WeekCardNoEffect,
            2500336 => Self::FlagChallengeTaskNoFinish,
            2500337 => Self::FlagChallengeNoConfig,
            2500338 => Self::FlagChallengeTaskRewarded,
            2500339 => Self::FlagChallengeRewardRepeat,
            2500340 => Self::FlagChallengeNoUnlock,
            2500341 => Self::FeiXuePreheatNoFinish,
            2500342 => Self::FeiXuePreheatNoConfig,
            2500343 => Self::FeiXuePreheatRewarded,
            2500344 => Self::FeiXuePreheatRewardRepeat,
            2500345 => Self::ErrOrnamentConfig,
            2500346 => Self::ErrOrnamentLocked,
            2500347 => Self::ErrOrnamentDressed,
            2500348 => Self::ErrOrnamentCanNoDress,
            2500349 => Self::ErrOrnamentNoDress,
            2500350 => Self::BossPilingTaskNoFinish,
            2500351 => Self::BossPilingNoConfig,
            2500352 => Self::BossPilingTaskRewarded,
            2500353 => Self::BossPilingRewardRepeat,
            2500354 => Self::BossPilingNoUnlock,
            2500355 => Self::FlagChallengePassed,
            2500356 => Self::BossPilingFirstNoFinish,
            2500357 => Self::BossPilingRoleErr,
            2500358 => Self::GachaAccumulateNoConfig,
            2500359 => Self::GachaAccumulateNoInTime,
            2500360 => Self::GachaAccumulateRewarded,
            2500361 => Self::GachaAccumulateNoFinish,
            2500362 => Self::GachaAccumulateErrSelectParam,
            2500363 => Self::RequestParamRepeat,
            2500364 => Self::BossPilingActivityNoOpen,
            2500365 => Self::BossPilingErrMultiGame,
            2500366 => Self::SheriffNoConfig,
            2500367 => Self::SheriffActivityNoOpen,
            2500368 => Self::SheriffFuncNotOpen,
            2500369 => Self::SheriffClueNotUnlock,
            2500370 => Self::SheriffEndingAlreadyFinish,
            2500371 => Self::ErrGiftResonantChainOptionalLimit,
            2500372 => Self::DangoBrawlNoConfig,
            2500373 => Self::DangoBrawlErrMultiGame,
            2500374 => Self::DangoBrawlRoleErr,
            2500375 => Self::DangoBrawlNoComponent,
            2500376 => Self::DangoBrawlSceneStateErr,
            2500377 => Self::DangoBrawlCoinNotEnough,
            2500378 => Self::DangoBrawlShopSlotErr,
            2500379 => Self::DangoBrawlBagOpTypeErr,
            2500380 => Self::DangoBrawlBagItemNotFound,
            2500381 => Self::DangoBrawlBagSlotErr,
            2500382 => Self::DangoBrawlBagSlotOccupied,
            2500383 => Self::DangoBrawlBagOpDuplicateUid,
            2500384 => Self::DangoBrawlComposeNoRecipe,
            2500385 => Self::DangoBrawlSpeItemLimit,
            2500386 => Self::DangoBrawlCostOverLimit,
            2500387 => Self::DangoBrawlBenchFull,
            2500388 => Self::CreateCharacterNoContext,
            2500389 => Self::CreateCharacterStateErr,
            2500390 => Self::CreateCharacterFlagErr,
            2600000 => Self::ErrAlertAreaId,
            2600001 => Self::ErrAlertAreaEnable,
            2600002 => Self::ErrAlertAreaDisable,
            2600003 => Self::ErrAlertUiEnable,
            2600004 => Self::ErrAlertUiDisable,
            2600005 => Self::ErrAlertUiVisible,
            2600006 => Self::ErrAlertUiInvisible,
            2600007 => Self::ErrAlertValueError,
            2600008 => Self::ErrAlertSetAlertValueType,
            2600009 => Self::LevelPlayReportConfigNotExist,
            2600010 => Self::LevelPlayReportTypeError,
            2600011 => Self::LevelPlayReportVarsEmpty,
            2600012 => Self::LevelPlayReportPlayVarsError,
            2600013 => Self::CameraAlertHasAlert,
            2600014 => Self::CameraAlertHasNotAlert,
            2600015 => Self::CameraAlertTagIdNotExist,
            2600016 => Self::LevelPlayConfigNotExist,
            2600017 => Self::LevelPlayRepeateInstId,
            2600018 => Self::LevelPlayNotBelongInst,
            2600019 => Self::LevelPlayIdsNotExist,
            2600020 => Self::LevelPlayInstCountError,
            2600021 => Self::LevelPlayCountError,
            2600022 => Self::TimerHasPause,
            2600023 => Self::TimerHasNotPause,
            2600024 => Self::TimerHasFinish,
            2600025 => Self::CanNotContinueInst,
            2600026 => Self::ChapterNotExist,
            2600027 => Self::ChapterResultHasFinish,
            2600028 => Self::ChapterResultNotFinish,
            2600029 => Self::ChapterResultRewardCanNotTake,
            2600030 => Self::ChoiceHasUnlock,
            2600031 => Self::ChoiceNotUnlock,
            2600032 => Self::ChapterResultNotExist,
            2600033 => Self::ChapterResultRewardHasTake,
            2600034 => Self::ChoiceNotExist,
            2600035 => Self::InspirationNotEnough,
            2600036 => Self::ActivityRewardCanNotTake,
            2600037 => Self::ActivityResultNotExist,
            2600038 => Self::ScheduleRewardNotExist,
            2600039 => Self::ScheduleRewardHasTake,
            2600040 => Self::ScheduleRewardCanNotTake,
            2600041 => Self::ActivityRewardHasTake,
            2600042 => Self::QuestWaitConfirmResource,
            2600043 => Self::HasGuest,
            2600044 => Self::QuestConfigNotExist,
            2600045 => Self::QuestDataNotExist,
            2600046 => Self::ErrorQuestState,
            2600047 => Self::QuestCanNotSetFocus,
            2600048 => Self::ErrorQuestNotFocus,
            2600049 => Self::QuestIncDicNotExist,
            2600050 => Self::QuestFocusWaitAccept,
            2600051 => Self::ErrQuestResourceState,
            2600052 => Self::NoNeedDownloadQuestResource,
            2600053 => Self::HasFinishQuestResource,
            2600054 => Self::ErrInteracTreeSuspend,
            2600055 => Self::InQuestFocusMode,
            2600056 => Self::MultiModeCannotSetQuestFocus,
            2600057 => Self::MultiModeCannotCancelQuestFocus,
            2600058 => Self::InstanceCannotSetQuestFocus,
            2600059 => Self::InstanceCannotCancelQuestFocus,
            2600060 => Self::QuestFocusModeCannotAcceptQuest,
            2600061 => Self::ErrorNotFocusWaitQuest,
            2600062 => Self::ErrorBanInteractEntity,
            2600063 => Self::DisabledFocusMode,
            2600064 => Self::ErrorNotVaildGlobalSetting,
            2600065 => Self::ErrorSameGlobalSetting,
            2600066 => Self::ErrorNotVaildBtObjSetting,
            2600067 => Self::ErrorSameBtObjSetting,
            2600068 => Self::CopyUserLoginInvalidToken,
            2600069 => Self::LoginInvalidToken,
            2600070 => Self::AccessInvalidToken,
            2600071 => Self::ErrNotAtomicProcessChildQuest,
            2600072 => Self::ErrNotAtomicProcessStoveCoreFall,
            2600073 => Self::ErrSplineEntityIdNotExist,
            2600074 => Self::ErrSplineEntityNotExist,
            2600075 => Self::MotorFightInstNotLevelConfig,
            2600076 => Self::MotorFightSubLevelNotFind,
            2600077 => Self::MotorFightNextSubLevelNotExist,
            2600078 => Self::MotorFightErrorSubLevel,
            2600079 => Self::MotorFightHasEnterSubLevel,
            2600080 => Self::MotorFightCurSubLevelNotKillFinish,
            2600081 => Self::MotorFightHasNotEnterSubLevel,
            2600082 => Self::MotorFightCusSubLevelHasNotKillFinish,
            2600083 => Self::MotorFightCurWaveNotFound,
            2600084 => Self::MotorFightErrorKillParams,
            2600085 => Self::MotorFighttExceedRefreshMonster,
            2600086 => Self::MotorFighttExceedRefreshBoss,
            2600087 => Self::MotorFightNotKillBoss,
            2600088 => Self::MotorFightBossDropCollectionHasSelect,
            2600089 => Self::MotorFightSelectPosCollectionNotExist,
            2600090 => Self::MotorFightCollectionConfigNotExist,
            2600091 => Self::MotorFightWaveGroupNotExist,
            2600092 => Self::MotorFightBossMustKilled,
            2600093 => Self::MotorFightBossNotExist,
            2600094 => Self::MotorFightSelectBuffGateNotExist,
            2600095 => Self::MotorFightCurSubLevelCollectionNoSelectFinish,
            2600096 => Self::MotorFightSubLevelConfigNotFind,
            2600097 => Self::MotorFightSubLevelIndexError,
            2600098 => Self::MotorFightBossHasKilled,
            2600099 => Self::MotorFightBossHasNotRefreshTimes,
            2600100 => Self::MotorFightBossDropCollectionError,
            2600101 => Self::MotorFightHasNotKillBossInfo,
            2600102 => Self::MotorFightIsLastBoss,
            2600103 => Self::MotorFightBuffGateHasSelect,
            2600104 => Self::MotorFightSubLevelStateError,
            2600105 => Self::MotorFightSubLevelNotFound,
            2600106 => Self::MotorFightHasGameOver,
            2600107 => Self::MotorFightCurWaveNotExist,
            2600108 => Self::MotorFightCurWaveNotKillFinish,
            2600109 => Self::SpecificInstDbDataLoading,
            2600110 => Self::SpecificInstDbDataLoadError,
            2600111 => Self::RecallConfigNotExist,
            2600112 => Self::QuestNotFinish,
            2600113 => Self::HasInRecallScene,
            2600114 => Self::EntityConfigNotFound,
            2600115 => Self::InstConfigNotFound,
            2600116 => Self::FastReturnPosTypeNotSupport,
            2600117 => Self::FastReturnPosConfigError,
            2600118 => Self::TrackCustomBoardIsNull,
            2600119 => Self::BtNodeConfigNotExist,
            2600120 => Self::BtNodeObjNotExist,
            2600121 => Self::LevelPlayNotExist,
            2600122 => Self::LevelPlayDbDataNotExist,
            2600123 => Self::NotInRecallInst,
            2600124 => Self::SpecificRecallInfoNotExist,
            2600125 => Self::ElementalTypeNotMatch,
            2600126 => Self::ElementalMatchTypeIsEmpty,
            2600127 => Self::ErrComponentConfigNotExist,
            2600128 => Self::ErrHasHandCasketCore,
            2600129 => Self::ErrNotHandCasketCore,
            2600130 => Self::ErrFoundationHasCore,
            2600131 => Self::ErrFoundationHasNotCore,
            2600132 => Self::ErrFoundationInteractCondNotMatch,
            2600133 => Self::ErrElementalTypeNotMatch,
            2600134 => Self::ErrElementalHolderNotEnable,
            2600135 => Self::ElementalBoxNotInFoundation,
            2600136 => Self::ErrElementalBoxConfigNotExist,
            2600137 => Self::ErrMultigameQuestRecall,
            2600138 => Self::ErrActionGetItemHasNotFreeSize,
            2700001 => Self::IdCreateRuleChange2,
            2700002 => Self::RacingBetsActivityIdErr,
            2700003 => Self::RacingBetsActivityDataErr,
            2700004 => Self::RacingBetsRewardConfErr,
            2700005 => Self::RacingBetsTaskIdNotExist,
            2700006 => Self::RacingBetsTaskRewarded,
            2700007 => Self::RacingBetsTaskUndone,
            2700008 => Self::RacingBetsTaskRewardFail,
            2700009 => Self::RacingBetsSeasonConfErr,
            2700010 => Self::RacingBetsLegMatchIdNotExist,
            2700011 => Self::RacingBetsOddsTimesErr,
            2700012 => Self::RacingBetsOddsVersionErr,
            2700013 => Self::RacingBetsOddsVersionCodeErr,
            2700014 => Self::RacingBetsOddsConfErr,
            2700015 => Self::RacingBetsOddsDangoErr,
            2700016 => Self::RacingBetsOddsDangoConfErr,
            2700017 => Self::RacingBetsOddsTimeErr,
            2700018 => Self::RacingBetsOddsRetry,
            2700019 => Self::RacingBetsCostFundsErr,
            2700020 => Self::RacingBetsGearTimesLimit,
            2700021 => Self::RacingBetsGearInfoErr,
            2700022 => Self::RacingBetsGearRefundParamErr,
            2700023 => Self::RacingBetsGearRefundFail,
            2700024 => Self::RacingBetsGearNotRefund,
            2700025 => Self::RacingBetsActivityConfErr,
            2700026 => Self::RacingBetsOddsFundsLack,
            2700027 => Self::RacingBetsOddsLack,
            2700028 => Self::RacingBetsGroupNotExist,
            2700029 => Self::RacingBetsGroupConfNotExist,
            2700030 => Self::RacingBetsGroupConfError,
            2700031 => Self::RacingBetsNotInInst,
            2700032 => Self::RacingBetsFundsNotEnough,
            2700033 => Self::RacingBetsFundsCalcErr,
            2700034 => Self::RacingBetsLegMatchNotOpen,
            2700035 => Self::RacingBetsDangoActionTypeErr,
            2700036 => Self::RacingBetsDangoMatchParamErr,
            2700037 => Self::RacingBetsInstSubTypeErr,
            2700038 => Self::RacingBetsEntityErr,
            2700039 => Self::RacingBetsLegMatchIdErr,
            2700040 => Self::RacingBetsBulletScreenTableNotExist,
            2700041 => Self::RacingBetsBulletScreenIdErr,
            2700042 => Self::RacingBetsBulletActionIndexNotExist,
            2700043 => Self::RacingBetsBulletScreenLegMatchNotExist,
            2700044 => Self::RacingBetsBulletNotFundOpenRankCurTime,
            2700045 => Self::RacingBetsBulletGetRankErr,
            2700046 => Self::RacingBetsBulletCd,
            2700047 => Self::ActivityLinkageActivityIdConfErr,
            2700048 => Self::ActivityLinkageConfErr,
            2700049 => Self::ActivityLinkageRewardErr,
            2700050 => Self::ActivityLinkageDataErr,
            2700051 => Self::ActivityLinkageConfIndexErr,
            2700052 => Self::ActivityLinkageRewardStatusErr,
            2700053 => Self::ActivityLinkagePageTimeErr,
            2700054 => Self::RacingBetsdynamicOddsConfErr,
            2700055 => Self::RacingBetsdynamicOddsProportionErr,
            2700056 => Self::RacingBetsDangoDataNotExist,
            2700057 => Self::RacingBetsMatchDataErr,
            2700058 => Self::RacingBetsMatchRoundTimeErr,
            2700059 => Self::RacingBetsNotGear,
            2700060 => Self::RacingBetsLegSettle,
            2700061 => Self::RacingBetsRankNotOpen,
            2700062 => Self::RacingBetsActionIndexErr,
            2700063 => Self::RacingBetsMatchRoundNotFund,
            2700064 => Self::RacingBetsDangoConfNotExist,
            2700065 => Self::RacingBetsDangoTemplateConfNotExist,
            2700066 => Self::RacingBetsDangoEntityCreateFail,
            2700067 => Self::RacingBetsLegMatchConfNotExist,
            2700068 => Self::PhantomBattleCardUnlocked,
            2700069 => Self::PhantomBattleActDataNotExist,
            2700070 => Self::RacingBetsLegNextOddsErr,
            2700071 => Self::RacingBetsRankNotExist,
            2700072 => Self::RacingBetsPlayerNotExistInRank,
            2700073 => Self::PhantomBattleCardTableNotExist,
            2700074 => Self::PhantomBattleCardNumLimit,
            2700075 => Self::PhantomBattleActTableNotExist,
            2700076 => Self::PhantomBattleCardGroupLimit,
            2700077 => Self::PhantomBattleGroupLimit,
            2700078 => Self::PhantomBattleMainCostLimit,
            2700079 => Self::PhantomBattleCardLocked,
            2700080 => Self::PhantomBattleGroupNameLimit,
            2700081 => Self::PhantomBattleGroupNotExist,
            2700082 => Self::PhantomBattleRomoveReason,
            2700083 => Self::RacingBetsBulletIdErr,
            2700084 => Self::RacingBetsPlayerDataNotExist,
            2700085 => Self::RacingBetsTaskDataNotExist,
            2700086 => Self::PhantomBattleGroupElementLimit,
            2700087 => Self::PhantomBattleOutLookUped,
            2700088 => Self::PhantomBattleTaskConfNotExist,
            2700089 => Self::PhantomBattleConditionDataErr,
            2700090 => Self::PhantomBattleConditionConfNotExist,
            2700091 => Self::PhantomBattleConditionGroupConfNotExist,
            2700092 => Self::PhantomBattleTaskStutas,
            2700093 => Self::PhantomBattleTaskReward,
            2700094 => Self::PhantomBattleTaskNotFinish,
            2700095 => Self::PhantomBattleTaskProgressErr,
            2700096 => Self::PhantomBattleBadgeUnlocked,
            2700097 => Self::PhantomBattleLevelConfNotExist,
            2700098 => Self::PhantomBattBadgeRewardConfNotExist,
            2700099 => Self::PhantomBattCardRewardConfNotExist,
            2700100 => Self::PhantomBattBadgeUnlockNotEnough,
            2700101 => Self::PhantomBattBadgeConfErr,
            2700102 => Self::PhantomBattBadgeRewardErr,
            2700103 => Self::PhantomBattleLevelRewardConfNotExist,
            2700104 => Self::PhantomBattCardUnlockNotEnough,
            2700105 => Self::PhantomBattCardRewardConfErr,
            2700106 => Self::PhantomBattLvNotEnough,
            2700107 => Self::PhantomBattLvRewardConfErr,
            2700108 => Self::PhantomBattCardRewardErr,
            2700109 => Self::PhantomBattLvRewardErr,
            2700110 => Self::PhantomBattLvRewardConfNotExist,
            2700111 => Self::PhantomBattRoleLock,
            2700112 => Self::PhantomBattRoleReward,
            2700113 => Self::PhantomBattRoleRewardErr,
            2700114 => Self::PhantomBattPassRewardErr,
            2700115 => Self::PhantomBattFirstPassRewardErr,
            2700116 => Self::PhantomBattleChallengeConfNotFind,
            2700117 => Self::PhantomBattleChallengeOpenLimit,
            2700118 => Self::PhantomBattleParamLimit,
            2700119 => Self::PhantomBattleParamRepeat,
            2700120 => Self::PhantomBattleFuncOpenChallengeRepeat,
            2700121 => Self::PhantomBattleFuncOpenCardGroup,
            2700122 => Self::PhantomBattleFuncOpenCard,
            2700123 => Self::PhantomBattleFuncOpenCardOutlookUp,
            2700124 => Self::PhantomBattleCardNotAllowBuy,
            2700125 => Self::FloroRanchHasUnSettleIns,
            2700126 => Self::FloroRanchSubInsConfNotExist,
            2700127 => Self::FloroRanchRacesLimit,
            2700128 => Self::FloroRanchBaseTerrainLimit,
            2700129 => Self::FloroRanchBaseCardLimit,
            2700130 => Self::FloroRanchRaceRepeat,
            2700131 => Self::FloroRanchGamePlayNotExist,
            2700132 => Self::FloroRanchGamePlayGachaNotExist,
            2700133 => Self::FloroRanchCardNotExistInGacha,
            2700134 => Self::FloroRanchPhantomCreateFail,
            2700135 => Self::FloroRanchTributeCoinLimit,
            2700136 => Self::FloroRanchRaceNotEnough,
            2700137 => Self::FloroRanchStageNotSettle,
            2700138 => Self::FloroRanchActivitysNotFind,
            2700139 => Self::FloroRanchGamePlayShopNotExist,
            2700140 => Self::FloroRanchGamePlayShopItemNotExist,
            2700141 => Self::FloroRanchDiamondNotEnough,
            2700142 => Self::FloroRanchCardTableNotExist,
            2700143 => Self::FloroRanchToyTableNotExist,
            2700144 => Self::FloroRanchTerrainTableNotExist,
            2700145 => Self::FloroRanchToyNumLimit,
            2700146 => Self::FloroRanchNotEnoughToyIndex,
            2700147 => Self::FloroRanchToyCreateFail,
            2700148 => Self::FloroRanchCardGroupTableNotExist,
            2700149 => Self::FloroRanchCardGroupNotExist,
            2700150 => Self::FloroRanchBuffTableNotExist,
            2700151 => Self::FloroRanchRemoveUnitNotExist,
            2700152 => Self::PhantomBattleCardMaxLimit,
            2700153 => Self::FloroRanchRefreshTableNotExist,
            2700154 => Self::FloroRanchPlayTaskNotExist,
            2700155 => Self::FloroRanchSkillTableNotExist,
            2700156 => Self::FloroRanchSkillUseTimesLimit,
            2700157 => Self::FloroRanchSkillLogicNotExist,
            2700158 => Self::FloroRanchSkillLogicErr,
            2700159 => Self::FloroRanchSkillLogicFail,
            2700160 => Self::PhantomBattleNameLimitConfErr,
            2700161 => Self::FloroRanchNotEnableUnlimitedMode,
            2700162 => Self::PhantomBattleLimitTime,
            2700163 => Self::FloroRanchToyWeightConfErr,
            2700164 => Self::FloroRanchAnimalNumLimit,
            2700165 => Self::FloroRanchRaceTableNotFind,
            2700166 => Self::FloroRanchRaceLock,
            2700167 => Self::FloroRanchSkillLock,
            2700168 => Self::FloroRanchSubInsLock,
            2700169 => Self::RoleSelfBgmUnableOperate,
            2700170 => Self::RoleSelfBgmFlyTableNotExist,
            2700171 => Self::RoleSelfBgmUnableOp,
            2700172 => Self::FloroRanchInsLock,
            2700173 => Self::FloroRanchInsConfNotFind,
            2700174 => Self::FloroRanchInsIdNotFind,
            2700175 => Self::FloroRanchInsRaceNotInReq,
            2700176 => Self::FloroRanchUnEnableTribute,
            2700177 => Self::FloroRanchIsMulti,
            2700178 => Self::ButtonLockParamLimit,
            2700179 => Self::ButtonLockParamRepeat,
            2700180 => Self::PhantomBattlePlayerCardGroupBuildErr,
            2700181 => Self::PhantomBattleNpcCardGroupBuildErr,
            2700182 => Self::PhantomBattleNpcRecordLoadFail,
            2700183 => Self::PhantomBattleCardTypeErr,
            2700184 => Self::PhantomBattleActTypeErr,
            2700185 => Self::PhantomNoDurableSkill,
            2700186 => Self::PhantomBattleAreaCardNumLimit,
            2700187 => Self::PhantomBattleItemCardNumLimit,
            2700188 => Self::PhantomBattleCallTaskNumErr,
            2700189 => Self::PhantomBattleNotCanSoltIndex,
            2700190 => Self::PhantomBattleSoltMaxLimit,
            2700191 => Self::PhantomBattleTaskIdErr,
            2700192 => Self::PhantomBattleActIdErr,
            2700193 => Self::PhantomBattleConditionParamErr,
            2700194 => Self::PhantomBattleAreaCardConfigErr,
            2700195 => Self::PhantomBattleAreaLock,
            2700196 => Self::PhantomBattleGuideActTableNotExist,
            2700197 => Self::NewPlayerSupportTaskConfNotExist,
            2700198 => Self::NewPlayerSupportTaskRewardIndex,
            2700199 => Self::NewPlayerSupportTaskParamLimit,
            2700200 => Self::NewPlayerSupportTaskParamRepeat,
            2700201 => Self::NewPlayerSupportTaskReward,
            2700202 => Self::NewPlayerSupportTaskNotFinish,
            2700203 => Self::NewPlayerSupportDataNotEixst,
            2700204 => Self::NewPlayerSupportNotFindTrialRole,
            2700205 => Self::NewPlayerSupportRoleLock,
            2700206 => Self::NewPlayerSupportWorldLvErr,
            2700207 => Self::NewPlayerSupportRolePageErr,
            2700208 => Self::NewTrialRoleActivityRoleCheckErr,
            2700209 => Self::NewTrialRoleActivityInsNotFind,
            2700210 => Self::NewTrialRoleActivityInsUnableUse,
            2700211 => Self::NewTrialRoleActivityLogicNotFind,
            2700212 => Self::NewTrialRoleActivityDelegateNotFind,
            2700213 => Self::RegressNotFindTrialRole,
            2700214 => Self::RegressNotFindActivityTrialRole,
            2700215 => Self::RegressDisposableReward,
            2700216 => Self::RegressDisposableRewardConfigNotFind,
            2700217 => Self::RegressPayBonusLimit,
            2700218 => Self::GachaRoleDevelopInsNotOpen,
            2700219 => Self::PhantomBattleNoComponent,
            2700220 => Self::PhantomBattleUnableSkip,
            2700221 => Self::RegressMaxBonusItemNum,
            2700222 => Self::RegressBonusRewardNoConfig,
            2700223 => Self::RegressBonusBuyLvLimit,
            2700224 => Self::PhantomBattleCallCardLimt,
            2700225 => Self::PhantomBattleCallItemLimt,
            2700226 => Self::GachaRoleDevelopInsLimit,
            2700227 => Self::GachaRolesilentAreaIdNotFind,
            2700228 => Self::GachaRolesilentAreaUnlock,
            2700229 => Self::NewTrialRoleActivityNotFindTrialRoleConf,
            2700230 => Self::PhantomBattleSkipErr,
            2700231 => Self::NewTrialRoleGachaRoleDevelopNotFind,
            2700232 => Self::RegressTrialRoleLock,
            2700233 => Self::NewPlayerSupportTrialRoleLock,
            2700234 => Self::PhantomBattleCopyCardLimit,
            2700235 => Self::RegressTrialNoCanRewardTask,
            2700236 => Self::NewPlayerSupportNoCanRewardTask,
            2700237 => Self::FarmGoldPointReqLimit,
            2700238 => Self::FarmGoldPointReqErr,
            2700239 => Self::FarmGoldPointReqRe,
            2700240 => Self::FarmGoldLevelReqLimit,
            2700241 => Self::FarmGoldLevelReqErr,
            2700242 => Self::FarmGoldLevelReqRe,
            2700243 => Self::FarmGoldPointRewardDropErr,
            2700244 => Self::FarmGoldLevelRewardDropErr,
            2700245 => Self::NewTrialRoleInCurTeam,
            2700246 => Self::RegressEndTimeErr,
            2700247 => Self::NewTrialRoleMatchingUnableTrial,
            2700248 => Self::PhBaFindCd,
            2700249 => Self::PhBaTargetPlanNotExist,
            2700250 => Self::PhBaPlanUploadErr,
            2700251 => Self::PhBaPlanErr,
            2700252 => Self::PhBaSaveCd,
            2700253 => Self::PhBaAttrNotInCost,
            2700254 => Self::PhBaCostMainPropNotExist,
            2700255 => Self::PhBaFetterGroupLimit,
            2700256 => Self::PhBaAttrTypeLimit,
            2700257 => Self::PhBaCostTypeNumLimit,
            2700258 => Self::PhBaCodeLimit,
            2700259 => Self::PhBaCodeIllegal,
            2700260 => Self::PhBaCurUsePlanNotExist,
            2700261 => Self::PhBaTargetSuitNotExist,
            2700262 => Self::PhBaPlanFuncNotOpen,
            2700263 => Self::InsUnableMultiEnter,
            2700264 => Self::PhBaPlanSuitCountErr,
            2700265 => Self::PhBaPlanTargetSuitNotExist,
            2700266 => Self::NewTrialRoleNumLimit,
            2700267 => Self::PhBaPlanCostTypeRepeat,
            2700268 => Self::PhBaPlanSuitIdRepeat,
            2700269 => Self::RacingBetsNotFindOrderListDango,
            2700270 => Self::PhBaPlanBatchOperEnumErr,
            2700271 => Self::PhBaPlanRemotePlanNotExist,
            2700272 => Self::H5ViewActivityNotFindDb,
            2700273 => Self::ForoRanchWeekInsNotFind,
            2700274 => Self::PhBaPlanCodeCreateErr,
            2700275 => Self::RacingBetsOddsDangoIllegal,
            2700276 => Self::ForoRanchNotCurEvent,
            2700277 => Self::ForoRanchEventIncIdErr,
            2700278 => Self::ForoRanchEventChoiceErr,
            2700279 => Self::ForoRanchEventSubChoicesConfNotExist,
            2700280 => Self::ForoRanchEventChoicesConfNotExist,
            2700281 => Self::ForoRanchSubInsIdNoActivityId,
            2700282 => Self::ForoRanchGetSubWeekInsErr,
            2700283 => Self::ForoRanchSubInsNotOpen,
            2700284 => Self::ForoRanchVoidToyNotEnough,
            2700285 => Self::ForoRanchToyNotEnough,
            2700286 => Self::RoverRogueInsNotExist,
            2700287 => Self::RoverRogueCompCreateFail,
            2700288 => Self::RoverRogueNotFindRoomConf,
            2700289 => Self::RoverRogueNotFindBornEntityId,
            2700290 => Self::RoverRogueLootLock,
            2700291 => Self::RoverRogueComponentNotFind,
            2700292 => Self::RoverRogueGetNextRoomFail,
            2700293 => Self::RoverRogueExitNumLess,
            2700294 => Self::RoverRogueExitEntityConfNotFind,
            2700295 => Self::RoverRogueCreateExitEntityFail,
            2700296 => Self::RoverRogueNotFindRoomTypeConf,
            2700297 => Self::RoverRoguePortalEntityNotFind,
            2700298 => Self::RoverRogueNotFindEntityLogic,
            2700299 => Self::RoverRogueInsCtxNotExist,
            2700300 => Self::RoverRogueRoleConfNotExist,
            2700301 => Self::RoverRogueGainDataTypeErr,
            2700302 => Self::RoverRogueGainActiveErr,
            2700303 => Self::RoverRogueRewardDbNotExist,
            2700304 => Self::RoverRogueGainSelectNotExist,
            2700305 => Self::RoverRogueGainTagChangeErr,
            2700306 => Self::RoverRogueGainLogicNotFind,
            2700307 => Self::RoverRogueGainLogicErr,
            2700308 => Self::ForoRanchWeeklyCanNotMilestone,
            2700309 => Self::RoverRogueRewardDbToken,
            2700310 => Self::RoverRogueRoleTypeConfNotExist,
            2700311 => Self::RoverRogueRoomRewardNotInit,
            2700312 => Self::RoverRogueRoomTaskNotExist,
            2700313 => Self::RoverRogueRoomBlessGroupTaskNotExist,
            2700314 => Self::RoverRogueRoomTaskTypeErr,
            2700315 => Self::RoverRogueRoomTaskParamErr,
            2700316 => Self::RoverRogueRoomTaskIsToken,
            2700317 => Self::RoverRogueNotCompletedInsErr,
            2700318 => Self::RoverRogueNotFindRoomRewardConf,
            2700319 => Self::RoverRogueRoomNotFinish,
            2700320 => Self::RoverRogueRoomRewardNotActivate,
            2700321 => Self::RoverRogueRoomRewardCache,
            2700322 => Self::RoverRogueRoleListErr,
            2700323 => Self::RoverRogueRoleTypeErr,
            2700324 => Self::RoverRogueNotFindActConf,
            2700325 => Self::RoverRogueLootNumLimit,
            2700326 => Self::RoverRogueNotFindBlessRoleConf,
            2700327 => Self::RoverRogueNotFindBollToPortal,
            2700328 => Self::RoverRogueNotFindBollNotExist,
            2700329 => Self::RoverRogueNotFindRoomRoadConf,
            2700330 => Self::RoverRogueNotFindChaosConf,
            2700331 => Self::RoverRogueChaosExitNotEnough,
            2700332 => Self::RoverRoguePosNotFind,
            2700333 => Self::RoverRogueChangeSameLoot,
            2800000 => Self::GateUdpPortNotExist,
            2800001 => Self::GateKcpGetConvFail,
            2900000 => Self::ErrScreenActionExecutorNotFind,
            2900001 => Self::ErrScreenActionInfoNotFind,
            2900002 => Self::ErrScreenActionTypeNotMatch,
            2900003 => Self::ErrScreenActionNotFadeInScreen,
            2900004 => Self::ErrScreenActionNotFadeOutScreen,
            2900005 => Self::ErrSceneItemSequenceFrameRegister,
            2900006 => Self::ErrSceneItemSequenceFrameComponentConfig,
            2900007 => Self::ErrSceneItemSequenceFrameComponent,
            2900008 => Self::ErrSceneItemSequenceFrame,
            2900009 => Self::ErrSceneItemSequenceFrameMatchEventCbType,
            2900010 => Self::ErrSceneItemSequenceFrameNeedServerAction,
            2900011 => Self::ErrSceneItemSequenceFrameUnAnsEvent,
            2900027 => Self::ErrSceneBlockSplitNotBlock,
            2900012 => Self::ErrSunSpiritCollectOverNum,
            2900013 => Self::ErrSunSpiritInteractAction,
            2900014 => Self::ErrSunSpiritGetComponentConfig,
            2900015 => Self::ErrSunSpiritEntityRemove,
            2900016 => Self::ErrSunSpiritRepeateAdd,
            2900017 => Self::ErrSunSpiritNumUnEnough,
            2900018 => Self::ErrSunSpiritEnableEntityHistoryUse,
            2900019 => Self::ErrSunSpiritGetPlayerTempOccupySunSpiritInfo,
            2900020 => Self::ErrSunSpiritActionExecutorNotFind,
            2900021 => Self::ErrSunSpiritActionInfoNotFind,
            2900022 => Self::ErrSunSpiritActionIndexNotMatch,
            2900023 => Self::ErrSunSpiritActionTypeNotMatch,
            2900024 => Self::ErrSunSpiritActionParamOperationType,
            2900025 => Self::ErrSunSpiritAreaConfig,
            2900026 => Self::ErrSunSpiritNotHostPlayer,
            2900028 => Self::ErrResourcePackageSdkCheck,
            2900029 => Self::ErrResourcePackageFuncClose,
            2900030 => Self::ErrResourcePackageRedisWrite,
            2900031 => Self::ErrResourcePackageInvalidToken,
            2900032 => Self::ErrResourcePackageRateLimiterCircuit,
            2900033 => Self::ErrResourcePackageRateLimiterRejected,
            2900034 => Self::ErrResourcePackageException,
            2900035 => Self::ErrResourcePackageTimeoutRejected,
            2900036 => Self::ErrAvoidBlockGetInstComponent,
            3000000 => Self::ErrLoadCalabashDefault,
            3000001 => Self::CalabashSkinUnLockErr,
            3000002 => Self::RepeqtedRequest,
            3000003 => Self::ErrSynthesisBatchItemNotSupport,
            3000004 => Self::ErrSynthesisBatchItemDeadLoop,
            3000005 => Self::ErrSynthesisBatchItemCntNotMatch,
            3000006 => Self::ErrSynthesisBatchItemReapeat,
            3000007 => Self::ErrBatchSynthesisCoinOverflow,
            3000008 => Self::ErrPhantomSkinNotExist,
            3000009 => Self::ErrPhantomSkinNotUnlock,
            3000010 => Self::ErrPhantomInteractionNotUnlock,
            3000011 => Self::ErrIllustratedNotUnlock,
            3000012 => Self::ErrPhantomInteractionEquipCountNotMatch,
            3000013 => Self::ErrPhantomInteractionConfigNotFind,
            3000044 => Self::ErrPhantomSkinRepeatOperation,
            3000045 => Self::ErrPhantomInteractionNotShowable,
            3000014 => Self::ErrArtemisNodeNoConfig,
            3000015 => Self::ErrArtemisActivityNotOpen,
            3000016 => Self::ErrArtemisNodeNoData,
            3000017 => Self::ErrArtemisNodeNoUnlock,
            3000018 => Self::ErrArtemisNodeNotRewardStatus,
            3000019 => Self::ErrArtemisNodeAlreadyFixed,
            3000020 => Self::ErrArtemisNodeAlreadyFinished,
            3000021 => Self::ErrArtemisState,
            3000022 => Self::ErrArtemisInvalidUpdate,
            3000023 => Self::ErrIndexOutOfRange,
            3000024 => Self::ErrArtemisActivityNotExist,
            3000025 => Self::MotorParkourLevelNoOpen,
            3000026 => Self::MotorParkourNoConfig,
            3000027 => Self::MotorParkourInstComponentNotExist,
            3000028 => Self::MotorParkourGetTimeRecordFailed,
            3000029 => Self::ErrMotorParkourLevelRewardIndexLengthError,
            3000030 => Self::ErrMotorParkourActivityDataNotExist,
            3000031 => Self::ErrMotorParkourLevelRewardIndexInvalid,
            3000032 => Self::ErrMotorParkourRecordCfgNotExist,
            3000033 => Self::ErrMotorParkourGetRewardStatusFail,
            3000034 => Self::ErrMotorParkourRewardNotAvailable,
            3000035 => Self::ErrMotorParkourLevelRewardIndexDuplicate,
            3000043 => Self::ErrMotorParkourRewardCfgNotExist,
            3000046 => Self::ErrMotorParkourLevelInfoFailed,
            3000073 => Self::ErrMotorParkourPreLevelNotComplete,
            3000036 => Self::ErrAreaTerminalPinFull,
            3000037 => Self::ErrAreaTerminalPinExist,
            3000038 => Self::ErrAreaTerminalPinNotExist,
            3000039 => Self::ErrAreaTerminalParamError,
            3000040 => Self::ErrActivityLock,
            3000041 => Self::ErrFuncLock,
            3000042 => Self::ErrNoAreaTerminalCfg,
            3000068 => Self::ErrSpringFestivalActivityConfigNotFound,
            3000069 => Self::ErrSpringFestivalActivityDataNotFound,
            3000070 => Self::ErrSpringFestivalComponentNotFound,
            3000071 => Self::ErrSpringFestivalRuntimeInfoNotFound,
            3000083 => Self::ErrMultiModeCanNotRequest,
            3000084 => Self::ErrSpringFestivalActivityNotOpen,
            3000100 => Self::ErrSpringFestivalFuncNotExist,
            3000101 => Self::ErrSpringFestivalFuncLock,
            3000103 => Self::ErrSpringFestivalActivityIdNotMatch,
            3000047 => Self::ErrFurnitureDiyActivityConfigNotFound,
            3000048 => Self::ErrFurnitureAreaIdNotExistInActivity,
            3000049 => Self::ErrFurnitureAreaConfigNotFound,
            3000050 => Self::ErrFurnitureDiyActivityDataNotFound,
            3000051 => Self::ErrFurnitureSlotConfigNotExist,
            3000052 => Self::ErrFurnitureConfigNotExist,
            3000053 => Self::ErrFurnitureNotAvailable,
            3000054 => Self::ErrFurnitureAreaDataNotFound,
            3000055 => Self::ErrSpecificFurnitureCount,
            3000056 => Self::ErrFurnitureSlotNotSupportChildTag,
            3000057 => Self::ErrFurnitureSlotNotSupportFurniture,
            3000058 => Self::ErrFurnitureDiyComponentNotFound,
            3000059 => Self::ErrNotFurnitureSlotConfig,
            3000060 => Self::ErrFurnitureSlotConfigSlotCountError,
            3000061 => Self::ErrFurnitureSlotSubFurnitureCount,
            3000062 => Self::ErrFurnitureEntityConfigNotFound,
            3000063 => Self::ErrEntityCreateFailed,
            3000064 => Self::ErrFurnitureUnloadSubFurnitureNotEmpty,
            3000065 => Self::ErrEntitiesNull,
            3000066 => Self::ErrConvertShortIdToFurnitureIdFailed,
            3000067 => Self::ErrFurnitureSlotDuplicateInRequest,
            3000106 => Self::ErrFurnitureSlotNotBelongToArea,
            3000117 => Self::ErrSpringFestivalFurnitureHadUnlock,
            3000072 => Self::ErrExploreDegreeTypeNotFound,
            3000074 => Self::ErrActivityBrochureCfgNotFound,
            3000075 => Self::ErrBrochureRewardRequestEmptyBookItems,
            3000076 => Self::ErrBrochureCfgNotFound,
            3000077 => Self::ErrBrochureNotBelongToActivity,
            3000078 => Self::ErrBrochureNotUnlock,
            3000079 => Self::ErrBrochureRewardRequestDuplicateBookItem,
            3000080 => Self::ErrBookItemCfgNotFound,
            3000081 => Self::ErrBookItemNotFinish,
            3000082 => Self::ErrBookItemAlreadyRewarded,
            3000091 => Self::ErrBrochureRewardTimeInvalid,
            3000104 => Self::ErrBookItemNotBelongToBrochure,
            3000085 => Self::ErrExhibitionComponentCfgNotExist,
            3000086 => Self::ErrItemTypeNotMatchExhibitType,
            3000087 => Self::ErrExhibitConfigTypeInvalid,
            3000088 => Self::ErrExhibitionComponentNotExist,
            3000089 => Self::ErrExhibitionPhantomNotExist,
            3000092 => Self::ErrRepeatedEntity,
            3000093 => Self::ErrPhonographMusicNotExist,
            3000102 => Self::ErrMusicNotUnlocked,
            3000090 => Self::ErrInstanceConfigNotFound,
            3000094 => Self::ErrInvalidEnterContextCase,
            3000095 => Self::ErrTetrisLevelInfoDuplicate,
            3000096 => Self::ErrTetrisActivityDataNotFound,
            3000097 => Self::ErrTetrisLevelConfigNotExist,
            3000098 => Self::ErrTetrisLevelRewardDuplicate,
            3000099 => Self::ErrTetrisLevelStateInvalid,
            3000118 => Self::ErrActivityNotMatchTetrisLevel,
            3000119 => Self::ErrTetrisTargetScoreIdNotExist,
            3000120 => Self::ErrTetrisLevelHasRewarded,
            3000121 => Self::ErrTetrisLevelNotFinished,
            3000126 => Self::ErrTetrisLevelDifficultyIndexInvalid,
            3000127 => Self::ErrTetrisTargetScoreCountNotMatch,
            3000128 => Self::ErrTetrisScoreInvalid,
            3000129 => Self::ErrGetVarDefine,
            3000130 => Self::ErrDataPersistenceTetrisBoardGame,
            3000131 => Self::ErrTetrisTargetScoreNotEnough,
            3000132 => Self::ErrTetrisConfigInvalid,
            3000137 => Self::ErrTetrisPreLevelNotFinished,
            3000105 => Self::WlfTest,
            3000107 => Self::DropCatchNoConfig,
            3000108 => Self::DropCatchLevelNotOpen,
            3000109 => Self::ErrDropCatchLevelRewardIndexLengthError,
            3000110 => Self::ErrDropCatchActivityDataNotExist,
            3000111 => Self::ErrDropCatchLevelRewardIndexDuplicate,
            3000112 => Self::ErrDropCatchLevelRewardIndexInvalid,
            3000113 => Self::ErrDropCatchRewardCfgNotExist,
            3000114 => Self::ErrDropCatchRewardNotAvailable,
            3000115 => Self::ErrDropCatchLevelInfoFailed,
            3000116 => Self::ErrDropCatchLevelScoreInvalid,
            3000122 => Self::ErrDropCatchEntityComponentNotExist,
            3000123 => Self::ErrDropCatchComponentNotExist,
            3000124 => Self::ErrDropCatchRewardCfgNotActivity,
            3000125 => Self::ErrDropCatchDataNotExist,
            3000133 => Self::ErrGetSelfXboxOnlineId,
            3000134 => Self::ErrXboxPlayerInfoRequestLimit,
            3000135 => Self::ErrGetXboxUserPlayerErr,
            3000136 => Self::ErrXboxAccountBlocked,
            3000138 => Self::InfrV2LevelCfgNotFound,
            3000139 => Self::InfrV2ShopPhaseCfgNotFound,
            3000140 => Self::InfrV2ShopGoodsCfgNotFound,
            3000141 => Self::InfrV2FireAddExpNegative,
            3000142 => Self::InfrV2FireAlreadyMaxLevel,
            3000143 => Self::InfrV2FireConditionNotMet,
            3000144 => Self::InfrV2FireExpNotEnough,
            3000145 => Self::InfrV2TreeNotInProgress,
            3000146 => Self::InfrV2FireLevelNotInProgress,
            3000147 => Self::InfrV2TreeConditionNotMet,
            3000148 => Self::InfrV2NotOpen,
            3000149 => Self::InfrV2TreeBuildCfgNotFound,
            3000150 => Self::InfrV2TreeCannotTrace,
            3000151 => Self::InfrV2TreeSwitchTraceInMulti,
            3000152 => Self::InfrV2TaskFindErr,
            3000153 => Self::InfrV2HadReward,
            3000154 => Self::InfrV2TaskRunning,
            3000155 => Self::InfrV2DropNoFind,
            3000156 => Self::InfrV2ParamAgainErr,
            3000157 => Self::InfrV2NoReward,
            3000158 => Self::InfrV2ParamCountErr,
            3000159 => Self::SlashAndTowerParamErr,
            3000160 => Self::RogueWeeklyParamErr,
            3000161 => Self::ErrMatchRoleChangeSkillBranchCountNotEqualRoleCount,
            3000162 => Self::ErrActivityPreQuestNotFinished,
            3000163 => Self::ErrInstNotEdgeRunnerLordGym,
            3000164 => Self::ErrEdgeRunnerLordGymConfigNotExist,
            3000165 => Self::ErrEdgeRunnerDbDataNotExist,
            3000166 => Self::ErrEdgeRunnerPreLordGymNotPassed,
            3000167 => Self::EdgeRunnerParamCountErr,
            3000168 => Self::EdgeRunnerDropNoFind,
            3000169 => Self::EdgeRunnerNoReward,
            3000170 => Self::EdgeRunnerTaskFindErr,
            3000171 => Self::EdgeRunnerActivityIdErr,
            3000172 => Self::EdgeRunnerHadReward,
            3000173 => Self::EdgeRunnerTaskRunning,
            3000174 => Self::EdgeRunnerParamAgainErr,
            3000175 => Self::ErrEdgeRunnerActivityConfigNotFound,
            3000176 => Self::ErrEdgeRunnerActivityFuncUnlockConfigNotFound,
            3000177 => Self::ErrEdgeRunnerActivityFuncAlreadyUnlock,
            3000178 => Self::ErrEdgeRunnerActivityFuncUnlockConditionNotReached,
            3000179 => Self::ErrEdgeRunnerActivityFuncUnlockFailed,
            3000187 => Self::ErrEdgeRunnerLordGymLevelVarNotExist,
            3000188 => Self::EdgeRunnerLordGymRepeatChallenge,
            3000189 => Self::ErrEdgeRunnerLordGymBtTreeNotFound,
            3000190 => Self::ErrEdgeRunnerLordGymNotInPlay,
            3000180 => Self::ErrGolemCrackLevelConfigNotExist,
            3000181 => Self::ErrActivityNotMatchGolemCrackLevel,
            3000182 => Self::ErrGolemCrackActivityDataNotFound,
            3000183 => Self::ErrGolemCrackPreLevelNotFinished,
            3000184 => Self::ErrGolemCrackLevelRewardDuplicate,
            3000186 => Self::ErrGolemCrackPreLevelLocked,
            3000185 => Self::ErrPhantomRecommendConfigNotFound,
            3000191 => Self::ErrXboxGiftNotEnable,
            3000192 => Self::ErrXboxGiftDbError,
            3000193 => Self::ErrXboxGiftAlreadyClaimed,
            3000194 => Self::ErrXboxGiftNotXboxUser,
            3000195 => Self::ErrXboxGiftRequestErr,
            3000196 => Self::ErrXboxGiftRequestDataErr,
            3000197 => Self::ErrXboxGiftNotFound,
            3000198 => Self::ErrXboxGiftException,
            3000199 => Self::ErrXboxGiftHttpError,
            3000201 => Self::ErrXboxGiftXstsTokenInvalid,
            3000202 => Self::ErrXboxGiftXstsTokenMismatch,
            3000200 => Self::DangoBrawlActivityNotExist,
            3000218 => Self::DangoBrawlParamCountErr,
            3000219 => Self::DangoBrawlTaskFindErr,
            3000220 => Self::DangoBrawlActivityIdErr,
            3000221 => Self::DangoBrawlHadReward,
            3000222 => Self::DangoBrawlTaskRunning,
            3000223 => Self::DangoBrawlDropNoFind,
            3000224 => Self::DangoBrawlParamAgainErr,
            3000225 => Self::DangoBrawlNoReward,
            3000226 => Self::DangoBrawlPreChallengeNotPass,
            3000227 => Self::DangoBrawlWinError,
            3000203 => Self::QingXiaoParamCountErr,
            3000204 => Self::QingXiaoTaskFindErr,
            3000205 => Self::QingXiaoActivityIdErr,
            3000206 => Self::QingXiaoHadReward,
            3000207 => Self::QingXiaoTaskRunning,
            3000208 => Self::QingXiaoDropNoFind,
            3000209 => Self::QingXiaoParamAgainErr,
            3000210 => Self::QingXiaoNoReward,
            3000211 => Self::ErrVersionSignInNotOpen,
            3000212 => Self::ErrVersionSignInNoData,
            3000213 => Self::ErrVersionSignInRewardNotExist,
            3000214 => Self::ErrVersionSignInRewardAlreadyTaken,
            3000215 => Self::ErrVersionSignInDaysNotEnough,
            3000216 => Self::ErrVersionSignInParamError,
            3000217 => Self::ErrTimePointRewardActivityConditionNotFinish,
            3000228 => Self::MaterialReplaceTargetMismatch,
            3000229 => Self::MaterialReplaceNoExchangeParamConfig,
            3000230 => Self::ErrMotorParkourNotFound,
            3000231 => Self::ErrTetrisLevelDropIdNotExist,
            3100000 => Self::PhotoFightLevelNotFound,
            3100001 => Self::PhotoFightActivityNotOpen,
            3100002 => Self::PhotoFightSceneComponentInfoLost,
            3100003 => Self::PhotoFightGroupNotFound,
            3100004 => Self::PhotoFightLevelNotOpen,
            3100005 => Self::PhotoFightPreInstNotCleared,
            3100006 => Self::PhotoFightSceneComponentLost,
            3100007 => Self::PhotoFightTargetStatusHigher,
            3100008 => Self::PhotoFightNoTargetRole,
            3100009 => Self::PhotoFightRoleInvalid,
            3100010 => Self::PhotoFightRewardCfgNotFound,
            3100011 => Self::PhotoFightRewardInvalidInput,
            3100012 => Self::PhotoFightAlreadyRewarded,
            3100081 => Self::PhotoFightCannotRewarded,
            3100306 => Self::PhotoFightInvalidInstType,
            3100013 => Self::WuWuKujiQuestNotFound,
            3100014 => Self::WuWuKujiQuestNotOpenDay,
            3100015 => Self::WuWuKujiQuestAllFinished,
            3100016 => Self::WuWuKujiActivityNotOpen,
            3100017 => Self::WuWuKujiPreGuideNotFinished,
            3100018 => Self::WuWuKujiPreQuestNotFinished,
            3100019 => Self::WuWuKujiActivityConfigNotFound,
            3100020 => Self::WuWuKujiTrickTypeInvalid,
            3100021 => Self::WuWuKujiNoAvailableTrick,
            3100022 => Self::WuWuKujiAwardGroupCfgNotFound,
            3100023 => Self::WuWuKujiNoAvailableGroup,
            3100024 => Self::WuWuKujiGroupRewardTimesExceededMax,
            3100025 => Self::WuWuKujiGroupRemainTimesError,
            3100026 => Self::WeatherCtlInvalidInMulti,
            3100027 => Self::WeatherCtlMapDataNotFound,
            3100028 => Self::WeatherCtlSwitchCfgNotFound,
            3100029 => Self::WeatherCtlSwitchLocked,
            3100030 => Self::WeatherCtlAreaWeatherLocked,
            3100031 => Self::WeatherCtlAreaDateLocked,
            3100032 => Self::WeatherCtlAddHourNoNegative,
            3100052 => Self::WeatherCtlInMulti,
            3100053 => Self::WeatherCtlInBattle,
            3100054 => Self::WeatherCtlInQuest,
            3100082 => Self::WeatherCtlNotInBigWorld,
            3100162 => Self::WeatherCtlNotOpen,
            3100033 => Self::InfrArchiveTaskCfgNotFound,
            3100034 => Self::InfrLevelCfgNotFound,
            3100035 => Self::InfrShopPhaseCfgNotFound,
            3100036 => Self::InfrShopGoodsCfgNotFound,
            3100037 => Self::InfrFireAddExpNegative,
            3100038 => Self::InfrFireAlreadyMaxLevel,
            3100039 => Self::InfrFireConditionNotMet,
            3100040 => Self::InfrRoadBuildCfgNotFound,
            3100041 => Self::InfrRoadCannotTrace,
            3100042 => Self::InfrFireShopGoodsNotFound,
            3100043 => Self::InfrFireShopPhaseNotUnlocked,
            3100044 => Self::InfrFireShopBuyLimitExceeded,
            3100045 => Self::InfrFireShopBuyTimesInvalid,
            3100046 => Self::InfrArchiveTaskInvalidInput,
            3100047 => Self::InfrArchiveTaskCannotRewarded,
            3100048 => Self::InfrFireExpNotEnough,
            3100049 => Self::InfrRoadNotInProgress,
            3100050 => Self::InfrFireLevelNotInProgress,
            3100051 => Self::InfrRoadConditionNotMet,
            3100073 => Self::InfrNotOpen,
            3100083 => Self::InfrPhoneTaskInvalidInput,
            3100084 => Self::InfrPhoneTaskCannotRewarded,
            3100085 => Self::InfrPhoneTaskCfgNotFound,
            3100173 => Self::InfrRoadSwitchTraceInMulti,
            3100074 => Self::InfrThemeActivityTaskNotFound,
            3100075 => Self::InfrThemeActivityTaskRewarded,
            3100076 => Self::InfrThemeActivityTaskNotComplete,
            3100077 => Self::InfrThemeActivityInputInvalid,
            3100094 => Self::InfrThemeActivityIdMisMatch,
            3100078 => Self::InfrThemeActivityTaskCannotRewarded,
            3100055 => Self::MotorTechPreNodeLock,
            3100056 => Self::MotorTechPreNotUpgraded,
            3100057 => Self::MotorTechLevelLower,
            3100058 => Self::MotorTechNotUnlocked,
            3100059 => Self::MotorTechLevelMax,
            3100060 => Self::MotorTechCfgNotFound,
            3100061 => Self::MotorLvlConfigNotFound,
            3100062 => Self::MotorFuncNotOpen,
            3100063 => Self::MotorTaskCannotRewarded,
            3100064 => Self::MotorTaskInvalidParam,
            3100065 => Self::MotorTaskCfgNotFound,
            3100066 => Self::MotorTechTreeCfgNotFound,
            3100067 => Self::MotorTechTreeLocked,
            3100068 => Self::MotorPlayerUsingSkill,
            3100069 => Self::MotorPlayerClimbing,
            3100070 => Self::MotorPlayerInWater,
            3100071 => Self::MotorPlayerInAir,
            3100072 => Self::MotorPlayerOnMotor,
            3100079 => Self::MotorTaskNotInOneTree,
            3100080 => Self::MotorExpInvalid,
            3100086 => Self::MotorOutlookLocked,
            3100087 => Self::MotorFrameCfgNotFound,
            3100088 => Self::MotorStickerCfgNotFound,
            3100089 => Self::MotorOutlookBanInRegion,
            3100090 => Self::MotorOutlookConflict,
            3100091 => Self::MotorStickerPartCountError,
            3100092 => Self::MotorStickerPartIdError,
            3100093 => Self::MotorOutlookNotOwned,
            3100320 => Self::MotorOutlookNotShow,
            3100159 => Self::MotorLevelRewardAllClaimed,
            3100160 => Self::PlayerNotInMotorArea,
            3100164 => Self::MotorDecorationsCfgNotFound,
            3100165 => Self::MotorDecorationsPartCountError,
            3100166 => Self::MotorDecorationsPartIdError,
            3100175 => Self::MotorTechTreeInCd,
            3100095 => Self::ArrowSwordLevelCfgNotFound,
            3100096 => Self::ArrowSwordTalentCfgNotFound,
            3100097 => Self::ArrowSwordTaskCfgNotFound,
            3100098 => Self::ArrowSwordItemCfgNotFound,
            3100099 => Self::ArrowSwordRoleCfgNotFound,
            3100100 => Self::ArrowSwordPreNotCleared,
            3100101 => Self::ArrowSwordActivityTypeNotMatch,
            3100102 => Self::ArrowSwordTaskStatusHigher,
            3100103 => Self::ArrowSwordInvalidInput,
            3100104 => Self::ArrowSwordDataOwnerErr,
            3100105 => Self::ArrowSwordActivityIdMisMatch,
            3100106 => Self::ArrowSwordTalentLocked,
            3100107 => Self::ArrowSwordTalentAlreadyInUse,
            3100108 => Self::ArrowSwordTalentPreNodeNotInUse,
            3100109 => Self::ArrowSwordActivityCfgNotFound,
            3100110 => Self::ArrowSwordTalentNotInUse,
            3100111 => Self::ArrowSwordActivityNotOpen,
            3100112 => Self::ArrowSwordActivityEnd,
            3100113 => Self::ArrowSwordComponentNoFind,
            3100114 => Self::ArrowSwordSaveDataLost,
            3100115 => Self::ArrowSwordRoleNotUnlock,
            3100116 => Self::ArrowSwordRankListCd,
            3100117 => Self::ArrowSwordComponentInstLost,
            3100118 => Self::ArrowSwordInstNotInProgress,
            3100119 => Self::ArrowSwordInstNotInitialized,
            3100120 => Self::ArrowSwordHaveUnfinishedInst,
            3100121 => Self::MotorFightLevelCfgNotFound,
            3100122 => Self::MotorFightTalentCfgNotFound,
            3100123 => Self::MotorFightTaskCfgNotFound,
            3100124 => Self::MotorFightItemCfgNotFound,
            3100125 => Self::MotorFightRoleCfgNotFound,
            3100126 => Self::MotorFightPreNotCleared,
            3100127 => Self::MotorFightActivityTypeNotMatch,
            3100128 => Self::MotorFightTaskStatusHigher,
            3100129 => Self::MotorFightInvalidInput,
            3100130 => Self::MotorFightDataOwnerErr,
            3100131 => Self::MotorFightActivityIdMisMatch,
            3100132 => Self::MotorFightTalentLocked,
            3100133 => Self::MotorFightTalentAlreadyInUse,
            3100134 => Self::MotorFightTalentPreNodeNotInUse,
            3100135 => Self::MotorFightActivityCfgNotFound,
            3100136 => Self::MotorFightTalentNotInUse,
            3100137 => Self::MotorFightActivityNotOpen,
            3100138 => Self::MotorFightActivityEnd,
            3100139 => Self::MotorFightComponentNoFind,
            3100140 => Self::MotorFightSaveDataLost,
            3100141 => Self::MotorFightRoleNotUnlock,
            3100142 => Self::MotorFightRankListCd,
            3100143 => Self::MotorFightComponentInstLost,
            3100144 => Self::MotorFightInstNotInProgress,
            3100145 => Self::MotorFightInstNotInitialized,
            3100146 => Self::MotorFightHaveUnfinishedInst,
            3100147 => Self::MotorFightAlreadySettle,
            3100161 => Self::MotorFightNoInfiniteLevel,
            3100163 => Self::MotorFightNoRoundData,
            3100170 => Self::MotorFightLevelNotOpen,
            3100171 => Self::MotorFightFirstClearRewardFail,
            3100172 => Self::MotorFightSettleResultNull,
            3100148 => Self::EncircleChallengeCfgNotFound,
            3100149 => Self::EncircleChallengeNotOpen,
            3100150 => Self::EncirclePreChallengeNotPass,
            3100151 => Self::EncircleChallengeStepErr,
            3100174 => Self::EncircleActivityTypeNotMatch,
            3100177 => Self::EncircleChallengeNotStart,
            3100179 => Self::EncircleMultiGame,
            3100206 => Self::EncircleActivityNotOpen,
            3100152 => Self::FetterGroupNotExist,
            3100153 => Self::DirectRefiningQualityErr,
            3100154 => Self::FetterGroupConfigErr,
            3100155 => Self::FetterGroupDirectRefineLocked,
            3100156 => Self::PhantomDirectRefiningFetterNotFound,
            3100157 => Self::PhantomDirectRefiningWeekTimesLimit,
            3100158 => Self::PhantomMonsterConfNotFound,
            3100167 => Self::GivebackScoreCannotReward,
            3100168 => Self::GivebackScoreCfgNotFound,
            3100169 => Self::GivebackNotOpen,
            3100176 => Self::GivebackInvalidInput,
            3100178 => Self::GivebackParseDataError,
            3100180 => Self::RhythmNotInLimitTime,
            3100181 => Self::RhythmTaskCfgNotFound,
            3100182 => Self::RhythmActivityCfgNotFound,
            3100183 => Self::RhythmPlanetCfgNotFound,
            3100184 => Self::RhythmLevelCfgNotFound,
            3100185 => Self::RhythmSubLevelCfgNotFound,
            3100190 => Self::RhythmRoleCfgNotFound,
            3100186 => Self::RhythmInputInvalid,
            3100187 => Self::RhythmActivityDataNotExist,
            3100188 => Self::RhythmTaskCannotReward,
            3100189 => Self::RhythmInMulti,
            3100191 => Self::RhythmSubLevelNotOpen,
            3100192 => Self::RhythmPreSubLevelNotCompleted,
            3100193 => Self::RhythmLevelNotOpen,
            3100194 => Self::RhythmPlanetNotOpen,
            3100195 => Self::RhythmRoleLock,
            3100196 => Self::RhythmActivityIdMisMatch,
            3100197 => Self::RhythmInvalidPayload,
            3100198 => Self::RhythmActivityTypeNotMatch,
            3100199 => Self::RhythmNoPartnerRoleId,
            3100200 => Self::RhythmGetRankInCd,
            3100201 => Self::RhythmInstNotMatch,
            3100202 => Self::RhythmActivityEnd,
            3100203 => Self::RhythmPartnerNotSet,
            3100204 => Self::RhythmStartInfoLost,
            3100205 => Self::RhythmSubLevelNotMatch,
            3100207 => Self::PinballLevelCfgNotFound,
            3100208 => Self::PinballActivityNotOpen,
            3100209 => Self::PinballTrialLevelNotUseRole,
            3100210 => Self::PinballRoleCountInvalid,
            3100211 => Self::PinballRoleNotOwn,
            3100212 => Self::PinballSceneNotExists,
            3100213 => Self::PinballComponentNotExists,
            3100214 => Self::PinballEnterCtxInvalid,
            3100215 => Self::PinballCompStateInvalid,
            3100216 => Self::PinballGameAlreadySettled,
            3100217 => Self::PinballBonusScoreNotFull,
            3100218 => Self::PinballLevelTypeInvalid,
            3100219 => Self::PinballAlreadySettled,
            3100220 => Self::PinballNoReviveTimesLeft,
            3100221 => Self::PinballCompStateChangeReasonInvalid,
            3100222 => Self::PinballSettlePayloadInvalid,
            3100223 => Self::PinballRoleCfgNotFound,
            3100224 => Self::PinballTemplateEntityConfigNotFound,
            3100225 => Self::PinballLevelNotBonus,
            3100226 => Self::PinballLevelTargetCondTypeNotSupport,
            3100227 => Self::PinballEffectCfgNotFound,
            3100228 => Self::PinballEffectHandlerNotFound,
            3100229 => Self::PinballTrialLevelEnterWithRoles,
            3100230 => Self::PinballBtObjNotExists,
            3100231 => Self::PinballLevelEnterWithNoRole,
            3100238 => Self::PinballRoleLevelCfgNotFound,
            3100239 => Self::PinballDailyConfigInvalid,
            3100232 => Self::MotorOutlookPresetNameInvalid,
            3100233 => Self::MotorOutlookPresetNotOwn,
            3100234 => Self::MotorOutlookPresetMax,
            3100235 => Self::MotorOutlookPresetNotExist,
            3100236 => Self::MotorOutlookPresetInputInvalid,
            3100237 => Self::MotorOutlookPresetIdInvalid,
            3100243 => Self::MotorSkinCfgNotFound,
            3100244 => Self::MotorSkinNotEditable,
            3100289 => Self::MotorSceneCfgNotFound,
            3100240 => Self::CdkGiftCdkBatchAlreadyGot,
            3100241 => Self::CdkGiftCdkPayGiftAlreadyBought,
            3100242 => Self::CdkGiftCdkPayGiftTypeErr,
            3100287 => Self::CdkGiftCdkPayGiftOldVersion,
            3100245 => Self::NewBieMainlineActivityNotOpen,
            3100246 => Self::NewBieMainlineActivityEnd,
            3100247 => Self::NewBieMainlineInvalidInput,
            3100248 => Self::NewBieMainlineRewardActivityMismatch,
            3100249 => Self::NewBieMainlineTabTaskCfgNotFound,
            3100250 => Self::NewBieMainlineTabRewardCfgNotFound,
            3100251 => Self::NewBieMainlineRewardAlreadyTaken,
            3100252 => Self::NewBieMainlineScoreNotEnough,
            3100253 => Self::NewBieMainlineRewardDropFail,
            3100254 => Self::NewBieAdventureActivityNotOpen,
            3100255 => Self::NewBieAdventureActivityCfgNotFound,
            3100256 => Self::NewBieAdventureActivityEnd,
            3100257 => Self::NewBieAdventureInvalidInput,
            3100258 => Self::NewBieAdventureChapterV2CfgNotFound,
            3100259 => Self::NewBieAdventureTaskV2CfgNotFound,
            3100260 => Self::NewBieAdventureV2RewardAlreadyTaken,
            3100261 => Self::NewBieAdventureV2RewardDropFail,
            3100262 => Self::NewBieAdventureV2ChapterLocked,
            3100263 => Self::NewBieAdventureV2ChapterRewardNotReady,
            3100264 => Self::NewBieAdventureV2NoTaskConfig,
            3100265 => Self::NewBieAdventureV2RoleNotSet,
            3100266 => Self::NewBieAdventureV2RoleInvalid,
            3100267 => Self::NewBieAdventureV2TasksCrossChapter,
            3100268 => Self::NewBieAdventureV2TaskNotComplete,
            3100269 => Self::GeneralDailyResetCronNotFound,
            3100270 => Self::ActivitySignConfigNotFound,
            3100271 => Self::SignActivityFreeDropNoConfig,
            3100272 => Self::SignActivityFreeDropAlreadyTaken,
            3100273 => Self::NewBieV2NoLicense,
            3100274 => Self::NewBieV1NoLicense,
            3100275 => Self::WeekCardPersonalNoPermission,
            3100276 => Self::WeekCardPersonalPermissionExpired,
            3100277 => Self::WeekCardNotPersonal,
            3100286 => Self::WeekCardInputErr,
            3100278 => Self::WeekCardRecycleNoPermission,
            3100323 => Self::NewPlayerSupportSameParent,
            3100279 => Self::NewBieCourseV2ActivityNotOpen,
            3100280 => Self::NewBieCourseV2ActivityCfgNotFound,
            3100281 => Self::NewBieCourseV2ActivityEnd,
            3100282 => Self::NewBieCourseV2InvalidInput,
            3100283 => Self::NewBieCourseV2CfgNotFound,
            3100284 => Self::NewBieCourseV2LevelNotEnough,
            3100285 => Self::NewBieCourseV2RewardAlreadyTaken,
            3100324 => Self::SolarSchoolInputInvalid,
            3100325 => Self::SolarSchoolTaskCfgNotFound,
            3100326 => Self::SolarSchoolTaskCannotReward,
            3100327 => Self::SolarSchoolActivityDataNotExist,
            3100328 => Self::SolarSchoolNotInLimitTime,
            3100329 => Self::SolarSchoolActivityTypeNotMatch,
            3100330 => Self::SolarSchoolActivityEnd,
            3100331 => Self::SolarSchoolActivityCfgNotFound,
            3100332 => Self::SolarSchoolGalleryCfgNotFound,
            3100333 => Self::SolarSchoolGalleryLock,
            3100334 => Self::SolarSchoolGalleryCannotReward,
            3100335 => Self::SolarSchoolSaveInvalid,
            3100336 => Self::SolarSchoolSaveReasonInvalid,
            3100337 => Self::SolarSchoolSaveSlotInvalid,
            3100338 => Self::SolarSchoolSaveSlotNotFound,
            3100339 => Self::SolarSchoolSaveDataCorrupted,
            3100340 => Self::SolarSchoolInProgressNotInitialized,
            3100341 => Self::SolarSchoolBatchSyncCountInvalid,
            3100342 => Self::SolarSchoolSyncTypeInvalid,
            3100343 => Self::SolarSchoolSyncPayloadMismatch,
            3100344 => Self::SolarSchoolGameCfgNotFound,
            3100345 => Self::SolarSchoolDuplicateInput,
            3100346 => Self::SolarSchoolNotUnlocked,
            3100347 => Self::SolarSchoolAlreadyUnlocked,
            3100348 => Self::SolarSchoolAlreadyCompleted,
            3100349 => Self::SolarSchoolNpcFavorLevelNotIncreased,
            3100350 => Self::SolarSchoolValueOutOfRange,
            3100351 => Self::SolarSchoolItemInsufficient,
            3100352 => Self::SolarSchoolItemStackLimitExceeded,
            3100353 => Self::SolarSchoolShopGoodsInvalid,
            3100354 => Self::SolarSchoolShopBuyLimitExceeded,
            3100355 => Self::SolarSchoolEndingCfgNotFound,
            3100356 => Self::SolarSchoolActivityIdNotMatch,
            3100358 => Self::SolarSchoolLimitAccNotEnough,
            3100359 => Self::SolarSchoolLimitAccAlreadyRewarded,
            3100371 => Self::SolarSchoolQuestNotSavable,
            3100372 => Self::SolarSchoolRewardedBitInvalid,
            3100378 => Self::SolarSchoolSaveVersionInvalid,
            3100379 => Self::SolarSchoolShopRecordInvalid,
            3100380 => Self::SolarSchoolSummaryInvalid,
            3100386 => Self::SolarSchoolInfluenceLevelInvalid,
            3100387 => Self::SolarSchoolGiftCountInvalid,
            3200000 => Self::ErrNotGetFightInfoDtType,
            3200001 => Self::ErrFightInfoDtType,
            3200002 => Self::ErrSetFightInfoDtType,
            3200003 => Self::ErrChangeDtTypeParamError,
            3300000 => Self::ErrCurSceneNotMatch,
            3300001 => Self::ErrTargetNotForDeadEye,
            3300002 => Self::ErrDeadEyeCount,
            3300003 => Self::ErrNoMoveEvent,
            3300004 => Self::ErrEntityTypeNotSupport,
            3300005 => Self::ErrExploreSkillNoActions,
            3300006 => Self::ErrExploreSkillActionMultiGame,
            3300007 => Self::ErrExploreSkillActionNotExist,
            3300008 => Self::ErrMonsterNotGameplayTagComp,
            3300009 => Self::ErrDecalEffectNotFound,
            3300010 => Self::ErrEntityNotInCurMap,
            3300011 => Self::ErrEntityNotInCurScene,
            3300012 => Self::ErrEntityNotForNpcTimeSchedule,
            3300013 => Self::ErrEntityNotInAnySchedule,
            3300014 => Self::ErrSplineConfigNotFound,
            3300015 => Self::ErrSplineIdxOutofBound,
            3300016 => Self::ErrPositionConfig,
            3300017 => Self::ErrSplineNotForTimeSchedule,
            3300018 => Self::ErrHostPlayerNotSame,
            3300019 => Self::ErrInteractType,
            3300020 => Self::ErrEntityAlreadyDie,
            3300021 => Self::ErrEntityAlreadyDestroyed,
            3300022 => Self::ErrRequestLimitExceeded,
            3300023 => Self::ErrOptionOutofBound,
            3300024 => Self::ErrHoldHandInvalidActionType,
            3300025 => Self::ErrMultiPreconditionChainError,
            3400000 => Self::ErrBlackScreenIllegalSource,
            3400001 => Self::ErrBeamReceiveEntityNotConfig,
            3400002 => Self::ErrBeamReceiveCondition,
            3400003 => Self::ErrBeamReceiveEntityNotConfig2,
            3400004 => Self::ErrFlowerPollutionNotConfig,
            3400005 => Self::ErrorFlowerPollutionState,
            3400006 => Self::ErrDollGarbTimeOut,
            3400007 => Self::ErrGrabMachineEntityNotConfig,
            3400008 => Self::ErrGrabItemNotInConfig,
            3400009 => Self::ErrItemHasBeenClipped,
            3400010 => Self::ErrDollGrabNoCsvConfig,
            3400011 => Self::ErrPlayerNotStartedPlaying,
            3400012 => Self::ErrGrabShowcaseEntityNotConfig,
            3400013 => Self::ErrPlayerBagNotDoll,
            3400014 => Self::ErrDollIsDelivered,
            3400015 => Self::ErrPlayerAlreadyPlaying,
            3400016 => Self::ErrGrabItemTypeMismatch,
            3400017 => Self::ErrDollGrabAlreadyPaused,
            3400018 => Self::ErrDollGrabNotPaused,
            3400019 => Self::ErrDollMachineDropReward,
            1100670 => Self::RoleTowerPrepareStepTypeMismatch,
            1100671 => Self::RoleTowerPrepareStepAlreadyFinished,
            1100672 => Self::RoleTowerSettleStepTypeMismatch,
            1100673 => Self::RoleTowerTeamUpgradeStepTypeMismatch,
            1100674 => Self::RoleTowerLoadSubLevelStepTypeMismatch,
            1100675 => Self::RoleTowerUnsupportedStepRequest,
            1100676 => Self::RoleTowerRequestRepeated,
            1100687 => Self::RoleTowerRankEntranceClosed,
            2600139 => Self::ErrInForceClosedArea,
            2600140 => Self::DetailRecallInfoNotExist,
            2600141 => Self::RecallDungeonIdNotExist,
            3100416 => Self::SolarSchoolInMultiGame,
            3100419 => Self::SolarSchoolAttrCfgNotFound,
            3100420 => Self::SolarSchoolRoleCfgNotFound,
            3100421 => Self::SolarSchoolInfluenceCfgNotFound,
            3100422 => Self::SolarSchoolBehaviorCfgNotFound,
            3100423 => Self::SolarSchoolExamCfgNotFound,
            3100424 => Self::SolarSchoolItemCfgNotFound,
            3100425 => Self::SolarSchoolShopCfgNotFound,
            3100426 => Self::SolarSchoolShopGoodsCfgNotFound,
            3100427 => Self::SolarSchoolQuestCfgNotFound,
            3100428 => Self::SolarSchoolQuestRewardCfgNotFound,
            3100429 => Self::SolarSchoolQuestNodeCfgNotFound,
            3100463 => Self::SolarSchoolTraceIdNotMatch,
            3100464 => Self::SolarSchoolGameDayBackward,
            3100465 => Self::SolarSchoolGameDayExceedMax,
            3100466 => Self::SolarSchoolGameDaySkipInvalid,
            3100467 => Self::SolarSchoolSyncCountExceeded,
            3100468 => Self::SolarSchoolAttrGainExceeded,
            3100469 => Self::SolarSchoolItemDailyMaxExceeded,
            3100470 => Self::SolarSchoolSpecialItemCannotCost,
            3100471 => Self::SolarSchoolSaveCycleCountNotMatch,
            3100472 => Self::SolarSchoolSaveGameDayNotMatch,
            3100473 => Self::SolarSchoolSaveAttrNotMatch,
            3100474 => Self::SolarSchoolSaveItemNotMatch,
            3100475 => Self::SolarSchoolSaveGotItemBitNotMatch,
            3100476 => Self::SolarSchoolNewCycleAttrInvalid,
            3100477 => Self::SolarSchoolNewCycleItemInvalid,
            3100478 => Self::SolarSchoolPreviewAttrNotMatch,
            3100479 => Self::SolarSchoolAttrAddCostInvalid,
            3100480 => Self::SolarSchoolAttrDuplicate,
            3100481 => Self::SolarSchoolAttrFinalCfgNotFound,
            3100482 => Self::SolarSchoolAttrFinalOutOfRange,
            3100483 => Self::SolarSchoolMoneyAddExceeded,
            3100484 => Self::SolarSchoolClientResVersionTooLow,
            3100485 => Self::SolarSchoolParamCfgNotFound,
            _ => Self::Success,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ErrorCode {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EEntityType {
    Player = 0,
    Npc = 1,
    Monster = 2,
    SceneItem = 5,
    Custom = 6,
    Vision = 7,
    Animal = 8,
    ClientOnly = 9,
    Vehicle = 10,
    PlayerEntity = 11,
    SceneEntity = 12,
}

impl Default for EEntityType {
    fn default() -> Self {
        Self::Player
    }
}

impl crate::wire::PbEnum for EEntityType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Player,
            1 => Self::Npc,
            2 => Self::Monster,
            5 => Self::SceneItem,
            6 => Self::Custom,
            7 => Self::Vision,
            8 => Self::Animal,
            9 => Self::ClientOnly,
            10 => Self::Vehicle,
            11 => Self::PlayerEntity,
            12 => Self::SceneEntity,
            _ => Self::Player,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EEntityType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ERemoveEntityType {
    RemoveTypeForce = 0,
    HpIsZero = 1,
    Destructible = 2,
    RemoveTypeNormal = 3,
    RemoveTypeCapture = 4,
    RemoveTypeTreasureBox = 5,
    RemoveTypeDrop = 6,
    MonsterBoom = 7,
    RemoveTypeResetByModeChange = 8,
    RemoveTypSimpleCombat = 9,
    RbBlockDestroyed = 10,
    AllRoleDying = 11,
    DyingTimeOut = 12,
}

impl Default for ERemoveEntityType {
    fn default() -> Self {
        Self::RemoveTypeForce
    }
}

impl crate::wire::PbEnum for ERemoveEntityType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RemoveTypeForce,
            1 => Self::HpIsZero,
            2 => Self::Destructible,
            3 => Self::RemoveTypeNormal,
            4 => Self::RemoveTypeCapture,
            5 => Self::RemoveTypeTreasureBox,
            6 => Self::RemoveTypeDrop,
            7 => Self::MonsterBoom,
            8 => Self::RemoveTypeResetByModeChange,
            9 => Self::RemoveTypSimpleCombat,
            10 => Self::RbBlockDestroyed,
            11 => Self::AllRoleDying,
            12 => Self::DyingTimeOut,
            _ => Self::RemoveTypeForce,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ERemoveEntityType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EAttributeType {
    None = 0,
    Lv = 1,
    LifeMax = 2,
    Life = 3,
    Sheild = 4,
    SheildDamageChange = 5,
    SheildDamageReduce = 6,
    Atk = 7,
    Crit = 8,
    CritDamage = 9,
    Def = 10,
    EnergyEfficiency = 11,
    CdReduse = 12,
    ElementEfficiency = 13,
    DamageChangeNormalSkill = 14,
    DamageChange = 15,
    DamageReduce = 16,
    DamageChangeAuto = 17,
    DamageChangeCast = 18,
    DamageChangeUltra = 19,
    DamageChangeQte = 20,
    DamageChangePhys = 21,
    DamageChangeElement1 = 22,
    DamageChangeElement2 = 23,
    DamageChangeElement3 = 24,
    DamageChangeElement4 = 25,
    DamageChangeElement5 = 26,
    DamageChangeElement6 = 27,
    DamageResistancePhys = 28,
    DamageResistanceElement1 = 29,
    DamageResistanceElement2 = 30,
    DamageResistanceElement3 = 31,
    DamageResistanceElement4 = 32,
    DamageResistanceElement5 = 33,
    DamageResistanceElement6 = 34,
    HealChange = 35,
    HealedChange = 36,
    DamageReducePhys = 37,
    DamageReduceElement1 = 38,
    DamageReduceElement2 = 39,
    DamageReduceElement3 = 40,
    DamageReduceElement4 = 41,
    DamageReduceElement5 = 42,
    DamageReduceElement6 = 43,
    SpecialEnergy5Max = 44,
    SpecialEnergy5 = 45,
    ReactionChange3 = 46,
    ReactionChange4 = 47,
    ReactionChange5 = 48,
    ReactionChange6 = 49,
    ReactionChange7 = 50,
    ReactionChange8 = 51,
    ReactionChange9 = 52,
    ReactionChange10 = 53,
    ReactionChange11 = 54,
    ReactionChange12 = 55,
    ReactionChange13 = 56,
    ReactionChange14 = 57,
    ReactionChange15 = 58,
    EnergyMax = 59,
    Energy = 60,
    SpecialEnergy1Max = 61,
    SpecialEnergy1 = 62,
    SpecialEnergy2Max = 63,
    SpecialEnergy2 = 64,
    SpecialEnergy3Max = 65,
    SpecialEnergy3 = 66,
    SpecialEnergy4Max = 67,
    SpecialEnergy4 = 68,
    StrengthMax = 69,
    Strength = 70,
    StrengthRecover = 71,
    StrengthPunishTime = 72,
    StrengthRun = 73,
    StrengthSwim = 74,
    StrengthFastSwim = 75,
    ElementEnergyMax = 76,
    ElementEnergy = 77,
    HardnessMax = 78,
    Hardness = 79,
    HardnessRecover = 80,
    HardnessPunishTime = 81,
    HardnessChange = 82,
    HardnessReduce = 83,
    ToughMax = 84,
    Tough = 85,
    ToughRecover = 86,
    ToughChange = 87,
    ToughReduce = 88,
    ElementPower1 = 89,
    ElementPower2 = 90,
    ElementPower3 = 91,
    ElementPower4 = 92,
    ElementPower5 = 93,
    ElementPower6 = 94,
    SpecialDamageChange = 95,
    StrengthFastClimbCost = 96,
    ElementPropertyType = 97,
    WeakTime = 98,
    IgnoreDefRate = 99,
    IgnoreDamageResistancePhys = 100,
    IgnoreDamageResistanceElement1 = 101,
    IgnoreDamageResistanceElement2 = 102,
    IgnoreDamageResistanceElement3 = 103,
    IgnoreDamageResistanceElement4 = 104,
    IgnoreDamageResistanceElement5 = 105,
    IgnoreDamageResistanceElement6 = 106,
    SkillToughRatio = 107,
    StrengthClimbJump = 108,
    StrengthGliding = 109,
    Mass = 110,
    BrakingFrictionFactor = 111,
    GravityScale = 112,
    SpeedRatio = 113,
    DamageChangePhantom = 114,
    AutoAttackSpeed = 115,
    CastAttackSpeed = 116,
    StatusBuildUp1Max = 117,
    StatusBuildUp1 = 118,
    StatusBuildUp2Max = 119,
    StatusBuildUp2 = 120,
    StatusBuildUp3Max = 121,
    StatusBuildUp3 = 122,
    StatusBuildUp4Max = 123,
    StatusBuildUp4 = 124,
    StatusBuildUp5Max = 125,
    StatusBuildUp5 = 126,
    RageMax = 127,
    Rage = 128,
    RageRecover = 129,
    RagePunishTime = 130,
    RageChange = 131,
    RageReduce = 132,
    ToughRecoverDelayTime = 133,
    Jump = 134,
    ParalysisTimeMax = 135,
    ParalysisTime = 136,
    ParalysisTimeRecover = 137,
    WeaknessBuildUp = 138,
    WeaknessBuildUpMax = 139,
    WeaknessTotalBonus = 140,
    BreakWeaknessRatio = 141,
    WeaknessMastery = 142,
    MAX = 143,
}

impl Default for EAttributeType {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for EAttributeType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Lv,
            2 => Self::LifeMax,
            3 => Self::Life,
            4 => Self::Sheild,
            5 => Self::SheildDamageChange,
            6 => Self::SheildDamageReduce,
            7 => Self::Atk,
            8 => Self::Crit,
            9 => Self::CritDamage,
            10 => Self::Def,
            11 => Self::EnergyEfficiency,
            12 => Self::CdReduse,
            13 => Self::ElementEfficiency,
            14 => Self::DamageChangeNormalSkill,
            15 => Self::DamageChange,
            16 => Self::DamageReduce,
            17 => Self::DamageChangeAuto,
            18 => Self::DamageChangeCast,
            19 => Self::DamageChangeUltra,
            20 => Self::DamageChangeQte,
            21 => Self::DamageChangePhys,
            22 => Self::DamageChangeElement1,
            23 => Self::DamageChangeElement2,
            24 => Self::DamageChangeElement3,
            25 => Self::DamageChangeElement4,
            26 => Self::DamageChangeElement5,
            27 => Self::DamageChangeElement6,
            28 => Self::DamageResistancePhys,
            29 => Self::DamageResistanceElement1,
            30 => Self::DamageResistanceElement2,
            31 => Self::DamageResistanceElement3,
            32 => Self::DamageResistanceElement4,
            33 => Self::DamageResistanceElement5,
            34 => Self::DamageResistanceElement6,
            35 => Self::HealChange,
            36 => Self::HealedChange,
            37 => Self::DamageReducePhys,
            38 => Self::DamageReduceElement1,
            39 => Self::DamageReduceElement2,
            40 => Self::DamageReduceElement3,
            41 => Self::DamageReduceElement4,
            42 => Self::DamageReduceElement5,
            43 => Self::DamageReduceElement6,
            44 => Self::SpecialEnergy5Max,
            45 => Self::SpecialEnergy5,
            46 => Self::ReactionChange3,
            47 => Self::ReactionChange4,
            48 => Self::ReactionChange5,
            49 => Self::ReactionChange6,
            50 => Self::ReactionChange7,
            51 => Self::ReactionChange8,
            52 => Self::ReactionChange9,
            53 => Self::ReactionChange10,
            54 => Self::ReactionChange11,
            55 => Self::ReactionChange12,
            56 => Self::ReactionChange13,
            57 => Self::ReactionChange14,
            58 => Self::ReactionChange15,
            59 => Self::EnergyMax,
            60 => Self::Energy,
            61 => Self::SpecialEnergy1Max,
            62 => Self::SpecialEnergy1,
            63 => Self::SpecialEnergy2Max,
            64 => Self::SpecialEnergy2,
            65 => Self::SpecialEnergy3Max,
            66 => Self::SpecialEnergy3,
            67 => Self::SpecialEnergy4Max,
            68 => Self::SpecialEnergy4,
            69 => Self::StrengthMax,
            70 => Self::Strength,
            71 => Self::StrengthRecover,
            72 => Self::StrengthPunishTime,
            73 => Self::StrengthRun,
            74 => Self::StrengthSwim,
            75 => Self::StrengthFastSwim,
            76 => Self::ElementEnergyMax,
            77 => Self::ElementEnergy,
            78 => Self::HardnessMax,
            79 => Self::Hardness,
            80 => Self::HardnessRecover,
            81 => Self::HardnessPunishTime,
            82 => Self::HardnessChange,
            83 => Self::HardnessReduce,
            84 => Self::ToughMax,
            85 => Self::Tough,
            86 => Self::ToughRecover,
            87 => Self::ToughChange,
            88 => Self::ToughReduce,
            89 => Self::ElementPower1,
            90 => Self::ElementPower2,
            91 => Self::ElementPower3,
            92 => Self::ElementPower4,
            93 => Self::ElementPower5,
            94 => Self::ElementPower6,
            95 => Self::SpecialDamageChange,
            96 => Self::StrengthFastClimbCost,
            97 => Self::ElementPropertyType,
            98 => Self::WeakTime,
            99 => Self::IgnoreDefRate,
            100 => Self::IgnoreDamageResistancePhys,
            101 => Self::IgnoreDamageResistanceElement1,
            102 => Self::IgnoreDamageResistanceElement2,
            103 => Self::IgnoreDamageResistanceElement3,
            104 => Self::IgnoreDamageResistanceElement4,
            105 => Self::IgnoreDamageResistanceElement5,
            106 => Self::IgnoreDamageResistanceElement6,
            107 => Self::SkillToughRatio,
            108 => Self::StrengthClimbJump,
            109 => Self::StrengthGliding,
            110 => Self::Mass,
            111 => Self::BrakingFrictionFactor,
            112 => Self::GravityScale,
            113 => Self::SpeedRatio,
            114 => Self::DamageChangePhantom,
            115 => Self::AutoAttackSpeed,
            116 => Self::CastAttackSpeed,
            117 => Self::StatusBuildUp1Max,
            118 => Self::StatusBuildUp1,
            119 => Self::StatusBuildUp2Max,
            120 => Self::StatusBuildUp2,
            121 => Self::StatusBuildUp3Max,
            122 => Self::StatusBuildUp3,
            123 => Self::StatusBuildUp4Max,
            124 => Self::StatusBuildUp4,
            125 => Self::StatusBuildUp5Max,
            126 => Self::StatusBuildUp5,
            127 => Self::RageMax,
            128 => Self::Rage,
            129 => Self::RageRecover,
            130 => Self::RagePunishTime,
            131 => Self::RageChange,
            132 => Self::RageReduce,
            133 => Self::ToughRecoverDelayTime,
            134 => Self::Jump,
            135 => Self::ParalysisTimeMax,
            136 => Self::ParalysisTime,
            137 => Self::ParalysisTimeRecover,
            138 => Self::WeaknessBuildUp,
            139 => Self::WeaknessBuildUpMax,
            140 => Self::WeaknessTotalBonus,
            141 => Self::BreakWeaknessRatio,
            142 => Self::WeaknessMastery,
            143 => Self::MAX,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EAttributeType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BlockState {
    BStateAll = 0,
    BStateSimple = 1,
    BStateComplete = 2,
}

impl Default for BlockState {
    fn default() -> Self {
        Self::BStateAll
    }
}

impl crate::wire::PbEnum for BlockState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BStateAll,
            1 => Self::BStateSimple,
            2 => Self::BStateComplete,
            _ => Self::BStateAll,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BlockState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GameCtxType {
    BehaviorTree = 0,
    Entity = 1,
    NormalInteract = 2,
    DynamicInteract = 3,
    RandomInteract = 4,
    EntityStateChangeAction = 5,
    EntityGroupAction = 6,
    EntityTrigger = 7,
    EntityLeaveTrigger = 8,
    EntityDestructible = 9,
    EntityTimelineTrack = 10,
    LevelPlayOpenAction = 11,
    LevelPlayRewardAction = 12,
    QuestActiveAction = 13,
    QuestAcceptAction = 14,
    QuestFinishAction = 15,
    ChildQuestNodeEnterAction = 16,
    ChildQuestNodeFinishAction = 17,
    SuccessNodeAction = 18,
    FailedNodeAction = 19,
    CompositionEnterAction = 20,
    EntityConditionListeningAction = 21,
    PlayFlowChildQuestNode = 22,
    HandInItemChildQuestNode = 23,
    DoInteractChildQuestNode = 24,
    ActionGroupNodeAction = 25,
    ExploreSkillPullGiantAction = 26,
    LevelPlay = 27,
    GmLevelAction = 28,
    GmPlayFlow = 29,
    SceneItemLifeCycleComponentCreate = 30,
    SceneItemLifeCycleComponentDetroy = 31,
    GameCtxGm = 32,
    FlowActionCtx = 33,
    DailyQuestTerminateAction = 34,
    ChildQuestNodeFixAction = 35,
    ConditionNodeFixAction = 36,
    EntityFixAction = 37,
    ConditionNode = 38,
    EntityBeamReceiveAction = 39,
    EntityGroupFailureAction = 40,
    ChildQuestNodeCondition = 41,
    EntityStateChangeConditionAction = 42,
    RequestPlayerGameCurrStateBt = 43,
    RequestEntityCurrState = 44,
    TriggerConditionListeningAction = 45,
    FlowStartTeleport = 46,
    EntityVisibleCondition = 47,
    FailedNodeTeleport = 48,
    LeaveInstEscActionCtx = 49,
    TrampleActiveActionCtx = 50,
    TrampleDeActiveActionCtx = 51,
    DefaultGameCtx = 52,
    LevelPlayExploratoryCtx = 53,
    RenjuCompleteActionCtx = 54,
    JigsawFoundationMatchedActionCtx = 55,
    CompositionFixAction = 56,
    JigsawFoundationUnMatchedActionCtx = 57,
    HookLockPointActionCtx = 58,
    ClientTriggerActionCtx = 59,
    ExploreSkillCustomAction = 60,
    LevelSequenceFrameEventAction = 61,
    JigsawFoundationMatchedConditionActionCtx = 62,
    CameraAlertComponentCreate = 63,
    RenjuExitMatchedAction = 64,
    RenjuExitUnMatchedAction = 65,
    LevelPlayDestroyAction = 66,
    EffectAreaConditionListeningAction = 67,
    OccupationInfoAction = 68,
    EntityHeadInfoCondition = 69,
    TemplateSpawnerConditionListen = 70,
    TemplateSpawnerAction = 71,
    BatchRefresherConditionListen = 72,
    QuestDestroyAction = 73,
    RequestGameCurrState = 74,
    TemplateSpawnerStateConditionListen = 75,
    CompositionConditionEnterAction = 76,
    TrapDefenseSystem = 77,
    SceneItemSequenceFrameEventActionCtx = 78,
    TargetGearHitPart = 79,
    GlobalFixCtx = 80,
    ChildQuestNodeStuckCheckAction = 81,
    GameCurrFetchVar = 82,
    EntityAfterConditionActionCtx = 83,
    ChildQuestNodePreCondition = 84,
    BtNodePreCondition = 85,
    DynamicSpawnMonsterRefresherConditionListen = 86,
    BeamCastHitPlayerActionCtx = 87,
    MotorSliderCtx = 88,
    RollBlockGamePlayActionCtx = 89,
    MotorParkourSystem = 90,
    TransferCtx = 91,
    SystemModuleDataSyncComponent = 92,
    DynamicEntityRewardCtx = 93,
    ExploreSkillAction = 94,
    MotorFightActivity = 95,
    PasserByNpcSpawnerConditionListenCtx = 96,
    EffectAreaListeningAction = 97,
    SurvivorsSystem = 98,
    TimeScheduleConditionCtx = 99,
    EntityBeamReceiveConditionCtx = 100,
    FlowerPollutionActionCtx = 101,
    KurotatoSystem = 102,
    EntityQuickHackSkill = 103,
    RecallQuestActiveAction = 104,
    RecallQuestAcceptAction = 105,
    RecallQuestFinishAction = 106,
    RecallQuestDestroyAction = 107,
    EdDebugEnterAction = 3401,
    SetClientGlobalVarAction = 3402,
    CasketTrigger = 3501,
    FoundationCoreOperate = 3502,
    TargetGearActionByHitType = 3701,
    LoopTeleportVolume = 3702,
    LongPressInteract = 3703,
    LevelPlayActiveConditionCtx = 3801,
}

impl Default for GameCtxType {
    fn default() -> Self {
        Self::BehaviorTree
    }
}

impl crate::wire::PbEnum for GameCtxType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BehaviorTree,
            1 => Self::Entity,
            2 => Self::NormalInteract,
            3 => Self::DynamicInteract,
            4 => Self::RandomInteract,
            5 => Self::EntityStateChangeAction,
            6 => Self::EntityGroupAction,
            7 => Self::EntityTrigger,
            8 => Self::EntityLeaveTrigger,
            9 => Self::EntityDestructible,
            10 => Self::EntityTimelineTrack,
            11 => Self::LevelPlayOpenAction,
            12 => Self::LevelPlayRewardAction,
            13 => Self::QuestActiveAction,
            14 => Self::QuestAcceptAction,
            15 => Self::QuestFinishAction,
            16 => Self::ChildQuestNodeEnterAction,
            17 => Self::ChildQuestNodeFinishAction,
            18 => Self::SuccessNodeAction,
            19 => Self::FailedNodeAction,
            20 => Self::CompositionEnterAction,
            21 => Self::EntityConditionListeningAction,
            22 => Self::PlayFlowChildQuestNode,
            23 => Self::HandInItemChildQuestNode,
            24 => Self::DoInteractChildQuestNode,
            25 => Self::ActionGroupNodeAction,
            26 => Self::ExploreSkillPullGiantAction,
            27 => Self::LevelPlay,
            28 => Self::GmLevelAction,
            29 => Self::GmPlayFlow,
            30 => Self::SceneItemLifeCycleComponentCreate,
            31 => Self::SceneItemLifeCycleComponentDetroy,
            32 => Self::GameCtxGm,
            33 => Self::FlowActionCtx,
            34 => Self::DailyQuestTerminateAction,
            35 => Self::ChildQuestNodeFixAction,
            36 => Self::ConditionNodeFixAction,
            37 => Self::EntityFixAction,
            38 => Self::ConditionNode,
            39 => Self::EntityBeamReceiveAction,
            40 => Self::EntityGroupFailureAction,
            41 => Self::ChildQuestNodeCondition,
            42 => Self::EntityStateChangeConditionAction,
            43 => Self::RequestPlayerGameCurrStateBt,
            44 => Self::RequestEntityCurrState,
            45 => Self::TriggerConditionListeningAction,
            46 => Self::FlowStartTeleport,
            47 => Self::EntityVisibleCondition,
            48 => Self::FailedNodeTeleport,
            49 => Self::LeaveInstEscActionCtx,
            50 => Self::TrampleActiveActionCtx,
            51 => Self::TrampleDeActiveActionCtx,
            52 => Self::DefaultGameCtx,
            53 => Self::LevelPlayExploratoryCtx,
            54 => Self::RenjuCompleteActionCtx,
            55 => Self::JigsawFoundationMatchedActionCtx,
            56 => Self::CompositionFixAction,
            57 => Self::JigsawFoundationUnMatchedActionCtx,
            58 => Self::HookLockPointActionCtx,
            59 => Self::ClientTriggerActionCtx,
            60 => Self::ExploreSkillCustomAction,
            61 => Self::LevelSequenceFrameEventAction,
            62 => Self::JigsawFoundationMatchedConditionActionCtx,
            63 => Self::CameraAlertComponentCreate,
            64 => Self::RenjuExitMatchedAction,
            65 => Self::RenjuExitUnMatchedAction,
            66 => Self::LevelPlayDestroyAction,
            67 => Self::EffectAreaConditionListeningAction,
            68 => Self::OccupationInfoAction,
            69 => Self::EntityHeadInfoCondition,
            70 => Self::TemplateSpawnerConditionListen,
            71 => Self::TemplateSpawnerAction,
            72 => Self::BatchRefresherConditionListen,
            73 => Self::QuestDestroyAction,
            74 => Self::RequestGameCurrState,
            75 => Self::TemplateSpawnerStateConditionListen,
            76 => Self::CompositionConditionEnterAction,
            77 => Self::TrapDefenseSystem,
            78 => Self::SceneItemSequenceFrameEventActionCtx,
            79 => Self::TargetGearHitPart,
            80 => Self::GlobalFixCtx,
            81 => Self::ChildQuestNodeStuckCheckAction,
            82 => Self::GameCurrFetchVar,
            83 => Self::EntityAfterConditionActionCtx,
            84 => Self::ChildQuestNodePreCondition,
            85 => Self::BtNodePreCondition,
            86 => Self::DynamicSpawnMonsterRefresherConditionListen,
            87 => Self::BeamCastHitPlayerActionCtx,
            88 => Self::MotorSliderCtx,
            89 => Self::RollBlockGamePlayActionCtx,
            90 => Self::MotorParkourSystem,
            91 => Self::TransferCtx,
            92 => Self::SystemModuleDataSyncComponent,
            93 => Self::DynamicEntityRewardCtx,
            94 => Self::ExploreSkillAction,
            95 => Self::MotorFightActivity,
            96 => Self::PasserByNpcSpawnerConditionListenCtx,
            97 => Self::EffectAreaListeningAction,
            98 => Self::SurvivorsSystem,
            99 => Self::TimeScheduleConditionCtx,
            100 => Self::EntityBeamReceiveConditionCtx,
            101 => Self::FlowerPollutionActionCtx,
            102 => Self::KurotatoSystem,
            103 => Self::EntityQuickHackSkill,
            104 => Self::RecallQuestActiveAction,
            105 => Self::RecallQuestAcceptAction,
            106 => Self::RecallQuestFinishAction,
            107 => Self::RecallQuestDestroyAction,
            3401 => Self::EdDebugEnterAction,
            3402 => Self::SetClientGlobalVarAction,
            3501 => Self::CasketTrigger,
            3502 => Self::FoundationCoreOperate,
            3701 => Self::TargetGearActionByHitType,
            3702 => Self::LoopTeleportVolume,
            3703 => Self::LongPressInteract,
            3801 => Self::LevelPlayActiveConditionCtx,
            _ => Self::BehaviorTree,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GameCtxType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EntityTimelineEventType {
    LeftIn = 0,
    LeftOut = 1,
    RightIn = 2,
    RightOut = 3,
}

impl Default for EntityTimelineEventType {
    fn default() -> Self {
        Self::LeftIn
    }
}

impl crate::wire::PbEnum for EntityTimelineEventType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::LeftIn,
            1 => Self::LeftOut,
            2 => Self::RightIn,
            3 => Self::RightOut,
            _ => Self::LeftIn,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EntityTimelineEventType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BeamReceiveActionType {
    BeginAction = 0,
    CompleteAction = 1,
    StopAction = 2,
}

impl Default for BeamReceiveActionType {
    fn default() -> Self {
        Self::BeginAction
    }
}

impl crate::wire::PbEnum for BeamReceiveActionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BeginAction,
            1 => Self::CompleteAction,
            2 => Self::StopAction,
            _ => Self::BeginAction,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BeamReceiveActionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PlayEnterOrExitPollutionRangeType {
    EnterAction = 0,
    ExitAction = 1,
}

impl Default for PlayEnterOrExitPollutionRangeType {
    fn default() -> Self {
        Self::EnterAction
    }
}

impl crate::wire::PbEnum for PlayEnterOrExitPollutionRangeType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::EnterAction,
            1 => Self::ExitAction,
            _ => Self::EnterAction,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PlayEnterOrExitPollutionRangeType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum HookInteractActionType {
    Hooked = 0,
    ExitMidway = 1,
    ExitEndpoint = 2,
}

impl Default for HookInteractActionType {
    fn default() -> Self {
        Self::Hooked
    }
}

impl crate::wire::PbEnum for HookInteractActionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Hooked,
            1 => Self::ExitMidway,
            2 => Self::ExitEndpoint,
            _ => Self::Hooked,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for HookInteractActionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RollBlockGameActionType {
    RbMidWayExit = 0,
    RbPass = 1,
    RbEnter = 2,
}

impl Default for RollBlockGameActionType {
    fn default() -> Self {
        Self::RbMidWayExit
    }
}

impl crate::wire::PbEnum for RollBlockGameActionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RbMidWayExit,
            1 => Self::RbPass,
            2 => Self::RbEnter,
            _ => Self::RbMidWayExit,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RollBlockGameActionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TeleportReason {
    Transfer = 0,
    ApiTeleport = 1,
    BtRollbackFailed = 2,
    ParkourTrans = 3,
    Gm = 4,
    Rouge = 5,
    Fall = 6,
    Action = 7,
    UnOpenedAreaPullback = 8,
    TemporaryTransfer = 9,
    FlowAction = 10,
    Drown = 11,
    FlowStart = 12,
    CorniceTrans = 13,
    TeleportVehicle = 14,
    GetOff = 15,
    LeaveGravityRegion = 16,
    TeleportToBoat = 17,
    GravityFlip = 18,
    RogueRes = 19,
    AbyssTeleport = 20,
    GmForce = 21,
    CharacterMoveToPoint = 22,
    InstEntity = 23,
    InstRequestTeleportResetPoint = 24,
    StrongWindField = 25,
    FastReturn = 26,
    RoverRogueRoomInChaos = 27,
    RecallInstMark = 28,
}

impl Default for TeleportReason {
    fn default() -> Self {
        Self::Transfer
    }
}

impl crate::wire::PbEnum for TeleportReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Transfer,
            1 => Self::ApiTeleport,
            2 => Self::BtRollbackFailed,
            3 => Self::ParkourTrans,
            4 => Self::Gm,
            5 => Self::Rouge,
            6 => Self::Fall,
            7 => Self::Action,
            8 => Self::UnOpenedAreaPullback,
            9 => Self::TemporaryTransfer,
            10 => Self::FlowAction,
            11 => Self::Drown,
            12 => Self::FlowStart,
            13 => Self::CorniceTrans,
            14 => Self::TeleportVehicle,
            15 => Self::GetOff,
            16 => Self::LeaveGravityRegion,
            17 => Self::TeleportToBoat,
            18 => Self::GravityFlip,
            19 => Self::RogueRes,
            20 => Self::AbyssTeleport,
            21 => Self::GmForce,
            22 => Self::CharacterMoveToPoint,
            23 => Self::InstEntity,
            24 => Self::InstRequestTeleportResetPoint,
            25 => Self::StrongWindField,
            26 => Self::FastReturn,
            27 => Self::RoverRogueRoomInChaos,
            28 => Self::RecallInstMark,
            _ => Self::Transfer,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TeleportReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TransitionType {
    Empty = 0,
    PlayEffect = 1,
    PlayMp4 = 2,
    CenterText = 3,
    FadeInScreen = 4,
    Seamless = 5,
    WithCharacterDisplay = 6,
    WithCustomLoading = 7,
    WithSpine = 8,
    WithSpecialCustomLoading = 9,
    PlayFlow = 10,
}

impl Default for TransitionType {
    fn default() -> Self {
        Self::Empty
    }
}

impl crate::wire::PbEnum for TransitionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::PlayEffect,
            2 => Self::PlayMp4,
            3 => Self::CenterText,
            4 => Self::FadeInScreen,
            5 => Self::Seamless,
            6 => Self::WithCharacterDisplay,
            7 => Self::WithCustomLoading,
            8 => Self::WithSpine,
            9 => Self::WithSpecialCustomLoading,
            10 => Self::PlayFlow,
            _ => Self::Empty,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TransitionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Mp4BackgroundColor {
    Mp4BackgroundColorBlack = 0,
    Mp4BackgroundColorWhite = 1,
}

impl Default for Mp4BackgroundColor {
    fn default() -> Self {
        Self::Mp4BackgroundColorBlack
    }
}

impl crate::wire::PbEnum for Mp4BackgroundColor {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Mp4BackgroundColorBlack,
            1 => Self::Mp4BackgroundColorWhite,
            _ => Self::Mp4BackgroundColorBlack,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for Mp4BackgroundColor {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AfterTeleportScreenColor {
    AfterTeleportScreenColorBlack = 0,
    AfterTeleportScreenColorWhite = 1,
}

impl Default for AfterTeleportScreenColor {
    fn default() -> Self {
        Self::AfterTeleportScreenColorBlack
    }
}

impl crate::wire::PbEnum for AfterTeleportScreenColor {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::AfterTeleportScreenColorBlack,
            1 => Self::AfterTeleportScreenColorWhite,
            _ => Self::AfterTeleportScreenColorBlack,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for AfterTeleportScreenColor {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum KeepMovementState {
    KeepMovementStateInvalid = 0,
    Kite = 1,
    Soar = 2,
}

impl Default for KeepMovementState {
    fn default() -> Self {
        Self::KeepMovementStateInvalid
    }
}

impl crate::wire::PbEnum for KeepMovementState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::KeepMovementStateInvalid,
            1 => Self::Kite,
            2 => Self::Soar,
            _ => Self::KeepMovementStateInvalid,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for KeepMovementState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ActivityType {
    Parkour = 0,
    GatherActivity = 1,
    Sign = 2,
    TowerGuide = 3,
    NewBieCourse = 4,
    WorldNewJourney = 5,
    RougeActivity = 6,
    DoubleInstanceRewardActivity = 7,
    RoleTrialActivity = 8,
    Harvest = 9,
    NewRoleGuideActivity = 10,
    PhantomCollect = 11,
    DailyAdventureActivity = 12,
    LongShanMainActivity = 13,
    BossRushActivity = 14,
    TurnTableActivity = 15,
    PhotoMemoryActivity = 16,
    TrackMoonActivity = 17,
    CircumFluence = 18,
    TowerDefenceActivity = 19,
    TimePointRewardActivity = 20,
    TowerGuideNew = 21,
    TrackMoonPhase = 22,
    RiskHarvest = 23,
    CorniceMeeting = 24,
    BlackCoastTheme = 25,
    RogueWhiteCat = 26,
    ScratchCard = 27,
    PreheatSign = 28,
    MowTower = 29,
    ThroughTrain = 30,
    SprintSign = 31,
    MapTravelActivity = 32,
    FarmGold = 33,
    NewLordGym = 34,
    RoleSkinTrialActivity = 35,
    RogueWeekly = 36,
    FishingActivity = 37,
    TeamParkOurActivity = 38,
    SlashAndTowerLevelPlay = 39,
    BabelTower = 40,
    Avignon = 41,
    BetHorses = 42,
    Explore = 43,
    Abyss = 44,
    RogueRes = 45,
    H5CircumFluence = 46,
    DangoMonopoly = 47,
    CiacconaActivity = 48,
    ActivityLinkage = 49,
    RegressActivity = 50,
    ConsumptiveActivity = 51,
    PhantomBattle = 52,
    NewbieCarnival = 53,
    MoraleActivity = 54,
    FloroRanchActivity = 55,
    LifePointChallenge = 56,
    TrapDefense = 57,
    JinzhouFlyActivity = 58,
    FunPlay = 59,
    MoonPhase = 60,
    LineCross = 61,
    HonamiStory = 62,
    Survivors = 63,
    PhotoFight = 64,
    WuWuKuji = 65,
    FirstPersonParkour = 66,
    PreHeatTaskActivity = 67,
    InfrTheme = 68,
    AdvanceNoticeActivity = 69,
    PhantomBattleRecord = 70,
    CoopActivity = 71,
    PhantomBattleRecordGuide = 72,
    ArtemisActivity = 73,
    MotorcycleIpLink = 74,
    SkinRewardActivity = 75,
    RoadBookActivity = 76,
    NewTowerClimbing = 77,
    MotorFight = 78,
    Encircle = 79,
    NewPlayerSupportActivity = 80,
    SpringFestivalActivity = 81,
    MotorParkourActivity = 82,
    TotalTopUp = 83,
    H5View = 84,
    MotorDevelop = 85,
    FlagChallenge = 86,
    Rhythm = 87,
    FeiXue = 89,
    DropCatchActivity = 90,
    TetrisActivity = 91,
    InfrThemeV2 = 92,
    Kurotato = 93,
    PinballActivity = 94,
    WuWuWeekSign = 96,
    AnniversaryTheme = 97,
    BossPiling = 98,
    GolemCrackActivity = 99,
    EdgeRunnerActivity = 100,
    MotorDecalActivity = 101,
    MotorParkour = 102,
    LinkageCheckIn = 103,
    RoleGiftActivity = 104,
    RealmBetween = 105,
    SheriffActivity = 106,
    ThroughTrainSummary = 107,
    NewPlayerSupportActivityV2 = 108,
    NewbieAdventureV2 = 109,
    NewbieCourseV2 = 110,
    NewbieMain = 111,
    RoverRogue = 112,
    DangoBrawl = 113,
    QingXiaoPlayThePiano = 114,
    PurePreviewActivity = 115,
    MatchThreeActivity = 116,
    SolarSchool = 117,
    GiantSystemActivity = 118,
    RoleTower = 120,
    PureUiactivity = 200,
}

impl Default for ActivityType {
    fn default() -> Self {
        Self::Parkour
    }
}

impl crate::wire::PbEnum for ActivityType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Parkour,
            1 => Self::GatherActivity,
            2 => Self::Sign,
            3 => Self::TowerGuide,
            4 => Self::NewBieCourse,
            5 => Self::WorldNewJourney,
            6 => Self::RougeActivity,
            7 => Self::DoubleInstanceRewardActivity,
            8 => Self::RoleTrialActivity,
            9 => Self::Harvest,
            10 => Self::NewRoleGuideActivity,
            11 => Self::PhantomCollect,
            12 => Self::DailyAdventureActivity,
            13 => Self::LongShanMainActivity,
            14 => Self::BossRushActivity,
            15 => Self::TurnTableActivity,
            16 => Self::PhotoMemoryActivity,
            17 => Self::TrackMoonActivity,
            18 => Self::CircumFluence,
            19 => Self::TowerDefenceActivity,
            20 => Self::TimePointRewardActivity,
            21 => Self::TowerGuideNew,
            22 => Self::TrackMoonPhase,
            23 => Self::RiskHarvest,
            24 => Self::CorniceMeeting,
            25 => Self::BlackCoastTheme,
            26 => Self::RogueWhiteCat,
            27 => Self::ScratchCard,
            28 => Self::PreheatSign,
            29 => Self::MowTower,
            30 => Self::ThroughTrain,
            31 => Self::SprintSign,
            32 => Self::MapTravelActivity,
            33 => Self::FarmGold,
            34 => Self::NewLordGym,
            35 => Self::RoleSkinTrialActivity,
            36 => Self::RogueWeekly,
            37 => Self::FishingActivity,
            38 => Self::TeamParkOurActivity,
            39 => Self::SlashAndTowerLevelPlay,
            40 => Self::BabelTower,
            41 => Self::Avignon,
            42 => Self::BetHorses,
            43 => Self::Explore,
            44 => Self::Abyss,
            45 => Self::RogueRes,
            46 => Self::H5CircumFluence,
            47 => Self::DangoMonopoly,
            48 => Self::CiacconaActivity,
            49 => Self::ActivityLinkage,
            50 => Self::RegressActivity,
            51 => Self::ConsumptiveActivity,
            52 => Self::PhantomBattle,
            53 => Self::NewbieCarnival,
            54 => Self::MoraleActivity,
            55 => Self::FloroRanchActivity,
            56 => Self::LifePointChallenge,
            57 => Self::TrapDefense,
            58 => Self::JinzhouFlyActivity,
            59 => Self::FunPlay,
            60 => Self::MoonPhase,
            61 => Self::LineCross,
            62 => Self::HonamiStory,
            63 => Self::Survivors,
            64 => Self::PhotoFight,
            65 => Self::WuWuKuji,
            66 => Self::FirstPersonParkour,
            67 => Self::PreHeatTaskActivity,
            68 => Self::InfrTheme,
            69 => Self::AdvanceNoticeActivity,
            70 => Self::PhantomBattleRecord,
            71 => Self::CoopActivity,
            72 => Self::PhantomBattleRecordGuide,
            73 => Self::ArtemisActivity,
            74 => Self::MotorcycleIpLink,
            75 => Self::SkinRewardActivity,
            76 => Self::RoadBookActivity,
            77 => Self::NewTowerClimbing,
            78 => Self::MotorFight,
            79 => Self::Encircle,
            80 => Self::NewPlayerSupportActivity,
            81 => Self::SpringFestivalActivity,
            82 => Self::MotorParkourActivity,
            83 => Self::TotalTopUp,
            84 => Self::H5View,
            85 => Self::MotorDevelop,
            86 => Self::FlagChallenge,
            87 => Self::Rhythm,
            89 => Self::FeiXue,
            90 => Self::DropCatchActivity,
            91 => Self::TetrisActivity,
            92 => Self::InfrThemeV2,
            93 => Self::Kurotato,
            94 => Self::PinballActivity,
            96 => Self::WuWuWeekSign,
            97 => Self::AnniversaryTheme,
            98 => Self::BossPiling,
            99 => Self::GolemCrackActivity,
            100 => Self::EdgeRunnerActivity,
            101 => Self::MotorDecalActivity,
            102 => Self::MotorParkour,
            103 => Self::LinkageCheckIn,
            104 => Self::RoleGiftActivity,
            105 => Self::RealmBetween,
            106 => Self::SheriffActivity,
            107 => Self::ThroughTrainSummary,
            108 => Self::NewPlayerSupportActivityV2,
            109 => Self::NewbieAdventureV2,
            110 => Self::NewbieCourseV2,
            111 => Self::NewbieMain,
            112 => Self::RoverRogue,
            113 => Self::DangoBrawl,
            114 => Self::QingXiaoPlayThePiano,
            115 => Self::PurePreviewActivity,
            116 => Self::MatchThreeActivity,
            117 => Self::SolarSchool,
            118 => Self::GiantSystemActivity,
            120 => Self::RoleTower,
            200 => Self::PureUiactivity,
            _ => Self::Parkour,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ActivityType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GatherActivityTaskState {
    GatherLock = 0,
    GatherRunning = 1,
    GatherInComplete = 2,
    GatherDone = 3,
    GatherTakeReward = 4,
}

impl Default for GatherActivityTaskState {
    fn default() -> Self {
        Self::GatherLock
    }
}

impl crate::wire::PbEnum for GatherActivityTaskState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GatherLock,
            1 => Self::GatherRunning,
            2 => Self::GatherInComplete,
            3 => Self::GatherDone,
            4 => Self::GatherTakeReward,
            _ => Self::GatherLock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GatherActivityTaskState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SignState {
    Lock = 0,
    Unlock = 1,
    IsReceive = 2,
}

impl Default for SignState {
    fn default() -> Self {
        Self::Lock
    }
}

impl crate::wire::PbEnum for SignState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Lock,
            1 => Self::Unlock,
            2 => Self::IsReceive,
            _ => Self::Lock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SignState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhantomCollectRewardType {
    PhantomsCollect = 0,
    DataDock = 1,
    PhantomSideQuest = 2,
    Max = 3,
}

impl Default for PhantomCollectRewardType {
    fn default() -> Self {
        Self::PhantomsCollect
    }
}

impl crate::wire::PbEnum for PhantomCollectRewardType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::PhantomsCollect,
            1 => Self::DataDock,
            2 => Self::PhantomSideQuest,
            3 => Self::Max,
            _ => Self::PhantomsCollect,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhantomCollectRewardType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MowTowerRewardStatus {
    CanNoReward = 0,
    CanReward = 1,
    Rewarded = 2,
}

impl Default for MowTowerRewardStatus {
    fn default() -> Self {
        Self::CanNoReward
    }
}

impl crate::wire::PbEnum for MowTowerRewardStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::CanNoReward,
            1 => Self::CanReward,
            2 => Self::Rewarded,
            _ => Self::CanNoReward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MowTowerRewardStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DangoMonopolyTaskState {
    NotCompleted = 0,
    Completed = 1,
    HasGet = 2,
}

impl Default for DangoMonopolyTaskState {
    fn default() -> Self {
        Self::NotCompleted
    }
}

impl crate::wire::PbEnum for DangoMonopolyTaskState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotCompleted,
            1 => Self::Completed,
            2 => Self::HasGet,
            _ => Self::NotCompleted,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DangoMonopolyTaskState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FunPlayChallengeRewardStatus {
    FunPlayCanNoReward = 0,
    FunPlayCanReward = 1,
    FunPlayRewarded = 2,
}

impl Default for FunPlayChallengeRewardStatus {
    fn default() -> Self {
        Self::FunPlayCanNoReward
    }
}

impl crate::wire::PbEnum for FunPlayChallengeRewardStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::FunPlayCanNoReward,
            1 => Self::FunPlayCanReward,
            2 => Self::FunPlayRewarded,
            _ => Self::FunPlayCanNoReward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FunPlayChallengeRewardStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ChallengeState {
    Running = 0,
    WaitTakeReward = 1,
    Finish = 2,
}

impl Default for ChallengeState {
    fn default() -> Self {
        Self::Running
    }
}

impl crate::wire::PbEnum for ChallengeState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Running,
            1 => Self::WaitTakeReward,
            2 => Self::Finish,
            _ => Self::Running,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ChallengeState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DailyAdventureTaskState {
    DailyAdventureTaskRunning = 0,
    DailyAdventureTaskFinish = 1,
    DailyAdventureTaskTaken = 2,
}

impl Default for DailyAdventureTaskState {
    fn default() -> Self {
        Self::DailyAdventureTaskRunning
    }
}

impl crate::wire::PbEnum for DailyAdventureTaskState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::DailyAdventureTaskRunning,
            1 => Self::DailyAdventureTaskFinish,
            2 => Self::DailyAdventureTaskTaken,
            _ => Self::DailyAdventureTaskRunning,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DailyAdventureTaskState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BossRushRewardClaimStatus {
    Incomplete = 0,
    Claimable = 1,
    Claimed = 2,
}

impl Default for BossRushRewardClaimStatus {
    fn default() -> Self {
        Self::Incomplete
    }
}

impl crate::wire::PbEnum for BossRushRewardClaimStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Incomplete,
            1 => Self::Claimable,
            2 => Self::Claimed,
            _ => Self::Incomplete,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BossRushRewardClaimStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BossRushBuffSelectionStatus {
    BuffEmpty = 0,
    BuffSelected = 1,
    BuffLocked = 2,
    BuffInactive = 3,
}

impl Default for BossRushBuffSelectionStatus {
    fn default() -> Self {
        Self::BuffEmpty
    }
}

impl crate::wire::PbEnum for BossRushBuffSelectionStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BuffEmpty,
            1 => Self::BuffSelected,
            2 => Self::BuffLocked,
            3 => Self::BuffInactive,
            _ => Self::BuffEmpty,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BossRushBuffSelectionStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ActivityTaskState {
    ActivityTaskRunning = 0,
    ActivityTaskFinish = 1,
    ActivityTaskTaken = 2,
}

impl Default for ActivityTaskState {
    fn default() -> Self {
        Self::ActivityTaskRunning
    }
}

impl crate::wire::PbEnum for ActivityTaskState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ActivityTaskRunning,
            1 => Self::ActivityTaskFinish,
            2 => Self::ActivityTaskTaken,
            _ => Self::ActivityTaskRunning,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ActivityTaskState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ActivityOpenType {
    TimeLimited = 0,
    Permanent = 1,
    LimitToPermanent = 2,
}

impl Default for ActivityOpenType {
    fn default() -> Self {
        Self::TimeLimited
    }
}

impl crate::wire::PbEnum for ActivityOpenType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TimeLimited,
            1 => Self::Permanent,
            2 => Self::LimitToPermanent,
            _ => Self::TimeLimited,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ActivityOpenType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ConditionTaskState {
    ConditionTaskRunning = 0,
    ConditionTaskFinish = 1,
    ConditionTaskTaken = 2,
}

impl Default for ConditionTaskState {
    fn default() -> Self {
        Self::ConditionTaskRunning
    }
}

impl crate::wire::PbEnum for ConditionTaskState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ConditionTaskRunning,
            1 => Self::ConditionTaskFinish,
            2 => Self::ConditionTaskTaken,
            _ => Self::ConditionTaskRunning,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ConditionTaskState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum StarRewardState {
    RiskHarvestCanNoReward = 0,
    RiskHarvestCanReward = 1,
    RiskHarvestRewarded = 2,
}

impl Default for StarRewardState {
    fn default() -> Self {
        Self::RiskHarvestCanNoReward
    }
}

impl crate::wire::PbEnum for StarRewardState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RiskHarvestCanNoReward,
            1 => Self::RiskHarvestCanReward,
            2 => Self::RiskHarvestRewarded,
            _ => Self::RiskHarvestCanNoReward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for StarRewardState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ConditionTaskStatus {
    Undone = 0,
    TaskFinish = 1,
    Received = 2,
}

impl Default for ConditionTaskStatus {
    fn default() -> Self {
        Self::Undone
    }
}

impl crate::wire::PbEnum for ConditionTaskStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Undone,
            1 => Self::TaskFinish,
            2 => Self::Received,
            _ => Self::Undone,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ConditionTaskStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ConsumptiveTaskType {
    Unknown = 0,
    Single = 1,
    Cycle = 2,
}

impl Default for ConsumptiveTaskType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl crate::wire::PbEnum for ConsumptiveTaskType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Single,
            2 => Self::Cycle,
            _ => Self::Unknown,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ConsumptiveTaskType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MotorParkourRewardState {
    MotorParkourRewardLocked = 0,
    MotorParkourRewardAvailable = 1,
    MotorParkourRewardRewarded = 2,
}

impl Default for MotorParkourRewardState {
    fn default() -> Self {
        Self::MotorParkourRewardLocked
    }
}

impl crate::wire::PbEnum for MotorParkourRewardState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::MotorParkourRewardLocked,
            1 => Self::MotorParkourRewardAvailable,
            2 => Self::MotorParkourRewardRewarded,
            _ => Self::MotorParkourRewardLocked,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MotorParkourRewardState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BookItemState {
    BookItemLock = 0,
    BookItemUnlock = 1,
    BookItemRewarded = 2,
}

impl Default for BookItemState {
    fn default() -> Self {
        Self::BookItemLock
    }
}

impl crate::wire::PbEnum for BookItemState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BookItemLock,
            1 => Self::BookItemUnlock,
            2 => Self::BookItemRewarded,
            _ => Self::BookItemLock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BookItemState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SkinRewardActivityRewardState {
    InitState = 0,
    TaskComplete = 1,
    TaskRewarded = 2,
}

impl Default for SkinRewardActivityRewardState {
    fn default() -> Self {
        Self::InitState
    }
}

impl crate::wire::PbEnum for SkinRewardActivityRewardState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::InitState,
            1 => Self::TaskComplete,
            2 => Self::TaskRewarded,
            _ => Self::InitState,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SkinRewardActivityRewardState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RhythmShipRank {
    Undefined = 0,
    SSS = 1,
    SS = 2,
    S = 3,
    A = 4,
    B = 5,
}

impl Default for RhythmShipRank {
    fn default() -> Self {
        Self::Undefined
    }
}

impl crate::wire::PbEnum for RhythmShipRank {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Undefined,
            1 => Self::SSS,
            2 => Self::SS,
            3 => Self::S,
            4 => Self::A,
            5 => Self::B,
            _ => Self::Undefined,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RhythmShipRank {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RhythmTaskTypePb {
    Resident = 0,
    Limit = 1,
}

impl Default for RhythmTaskTypePb {
    fn default() -> Self {
        Self::Resident
    }
}

impl crate::wire::PbEnum for RhythmTaskTypePb {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Resident,
            1 => Self::Limit,
            _ => Self::Resident,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RhythmTaskTypePb {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TetrisState {
    TetrisLocked = 0,
    TetrisUnlocked = 1,
    TetrisFinished = 2,
}

impl Default for TetrisState {
    fn default() -> Self {
        Self::TetrisLocked
    }
}

impl crate::wire::PbEnum for TetrisState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TetrisLocked,
            1 => Self::TetrisUnlocked,
            2 => Self::TetrisFinished,
            _ => Self::TetrisLocked,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TetrisState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GolemCrackState {
    GolemCrackLocked = 0,
    GolemCrackUnlocked = 1,
    GolemCrackFinished = 2,
}

impl Default for GolemCrackState {
    fn default() -> Self {
        Self::GolemCrackLocked
    }
}

impl crate::wire::PbEnum for GolemCrackState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GolemCrackLocked,
            1 => Self::GolemCrackUnlocked,
            2 => Self::GolemCrackFinished,
            _ => Self::GolemCrackLocked,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GolemCrackState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoverRogueGrade {
    RoverRogueGradeB = 0,
    RoverRogueGradeA = 1,
    RoverRogueGradeS = 2,
    RoverRogueGradeSSS = 3,
}

impl Default for RoverRogueGrade {
    fn default() -> Self {
        Self::RoverRogueGradeB
    }
}

impl crate::wire::PbEnum for RoverRogueGrade {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RoverRogueGradeB,
            1 => Self::RoverRogueGradeA,
            2 => Self::RoverRogueGradeS,
            3 => Self::RoverRogueGradeSSS,
            _ => Self::RoverRogueGradeB,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoverRogueGrade {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DangoBrawlChallengeState {
    Locked = 0,
    Unlocked = 1,
    Passed = 2,
}

impl Default for DangoBrawlChallengeState {
    fn default() -> Self {
        Self::Locked
    }
}

impl crate::wire::PbEnum for DangoBrawlChallengeState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Locked,
            1 => Self::Unlocked,
            2 => Self::Passed,
            _ => Self::Locked,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DangoBrawlChallengeState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DetectionType {
    NormalMonster = 0,
    Dungeon = 1,
    SilentArea = 2,
    Material = 3,
}

impl Default for DetectionType {
    fn default() -> Self {
        Self::NormalMonster
    }
}

impl crate::wire::PbEnum for DetectionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NormalMonster,
            1 => Self::Dungeon,
            2 => Self::SilentArea,
            3 => Self::Material,
            _ => Self::NormalMonster,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DetectionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AdventreTaskState {
    UnFinish = 0,
    Finish = 1,
    Received = 2,
}

impl Default for AdventreTaskState {
    fn default() -> Self {
        Self::UnFinish
    }
}

impl crate::wire::PbEnum for AdventreTaskState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::UnFinish,
            1 => Self::Finish,
            2 => Self::Received,
            _ => Self::UnFinish,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for AdventreTaskState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SlientFirstAwardState {
    NotUnlock = 0,
    NotFinish = 1,
    IsFinish = 2,
    IsReceive = 3,
}

impl Default for SlientFirstAwardState {
    fn default() -> Self {
        Self::NotUnlock
    }
}

impl crate::wire::PbEnum for SlientFirstAwardState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotUnlock,
            1 => Self::NotFinish,
            2 => Self::IsFinish,
            3 => Self::IsReceive,
            _ => Self::NotUnlock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SlientFirstAwardState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PbAdviceContentType {
    Sentence = 0,
    Conjunction = 1,
    Expression = 2,
    Motion = 3,
}

impl Default for PbAdviceContentType {
    fn default() -> Self {
        Self::Sentence
    }
}

impl crate::wire::PbEnum for PbAdviceContentType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Sentence,
            1 => Self::Conjunction,
            2 => Self::Expression,
            3 => Self::Motion,
            _ => Self::Sentence,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PbAdviceContentType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BlackboardParamType {
    None = 0,
    Int = 1,
    IntArray = 2,
    Long = 3,
    LongArray = 4,
    Boolean = 5,
    String = 6,
    StringArray = 7,
    Float = 8,
    FloatArray = 9,
    Vector = 10,
    VectorArray = 11,
    Rotator = 12,
    RotatorArray = 13,
    Entity = 14,
    EntityArray = 15,
}

impl Default for BlackboardParamType {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for BlackboardParamType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Int,
            2 => Self::IntArray,
            3 => Self::Long,
            4 => Self::LongArray,
            5 => Self::Boolean,
            6 => Self::String,
            7 => Self::StringArray,
            8 => Self::Float,
            9 => Self::FloatArray,
            10 => Self::Vector,
            11 => Self::VectorArray,
            12 => Self::Rotator,
            13 => Self::RotatorArray,
            14 => Self::Entity,
            15 => Self::EntityArray,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BlackboardParamType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum NetStatusType {
    Wifi = 0,
    Stream = 1,
    Wired = 2,
    Other = 3,
}

impl Default for NetStatusType {
    fn default() -> Self {
        Self::Wifi
    }
}

impl crate::wire::PbEnum for NetStatusType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Wifi,
            1 => Self::Stream,
            2 => Self::Wired,
            3 => Self::Other,
            _ => Self::Wifi,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for NetStatusType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ClientDeviceLevel {
    Low = 0,
    Medium = 1,
    High = 2,
}

impl Default for ClientDeviceLevel {
    fn default() -> Self {
        Self::Low
    }
}

impl crate::wire::PbEnum for ClientDeviceLevel {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::Medium,
            2 => Self::High,
            _ => Self::Low,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ClientDeviceLevel {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EClientStorageSystemIdType {
    Default = 0,
    Activity = 1,
    VisionSkin = 2,
    LoopTowerIsClickSeason = 3,
    LoopTowerIsClickShop = 4,
    TowerOverLockArea = 5,
    WeaponSkinRedDot = 6,
    FlySkinRedDot = 7,
    CalabashSkinRedDot = 8,
    MailBindNextShowRedDotTime = 9,
    RedDotAdventureNewSoundAreaTabLastUpdateTime = 10,
    SuitWeaponFirstWearRecord = 11,
    RoleSkinRedDot = 12,
    FirstOpenVisionGroup = 13,
    ShipTowerSeason = 14,
    AdventrueShipTowerSeason = 15,
    AdventrueWeeklyRogue = 16,
    AdventrueTower = 17,
    IntroductionVersion = 18,
    DetectionRedDotRecord = 19,
    ExploreActivityFirstUnlock = 20,
    FilterRedPoint = 21,
    VisionRecoveryBatchTip = 22,
    VisionRecoveryBatchAimTip = 23,
    MotorDiyNewUnlockId = 24,
    MotorDevelopNewUnlockTree = 25,
    PhantomBattleConfigApplyPlanWhenSaved = 26,
    FeiXuePreheatSubViewRedDot = 27,
    DropCatchRoleClickDetail = 28,
    FlagChallengeNewlyUnlockedLevelIds = 29,
    FlagChallengeNewlyUnlockedBuff = 30,
    PeriodicActivityRedLogReport = 31,
    CoopRolePhotoRedDot = 32,
    CoopRoleNewLevelRedDot = 33,
    FlagChallengeNewlyUnlockedBuffIds = 34,
    RoguelikeNewPhantomUnlock = 35,
    RoguelikeNewCharacterUnlock = 36,
    RoguelikeNewEntriesGroupUnlock = 37,
    AnniversaryActivityFirstClick = 38,
    VisionRefineTip = 39,
    Ornament = 40,
    GetOrnament = 41,
    ItemBackpackFirstCheck = 42,
    QuestBranchSystem = 43,
    GachaAccumulateFirstOpen = 44,
    Photography = 45,
    PhoneMsgChatShowRedDot = 46,
    GolemCrack = 47,
    FirstSheriffInference = 48,
    ThroughTrainFirstCheck = 49,
    FightPhotoTab = 50,
    VersionPayGiftRedDot = 51,
    NormalPayGiftRedDot = 52,
    FirstOpenDailyActivityTab = 53,
    RoleLangCustomRecord = 54,
    ActivityRecommend = 55,
    WheelTowerSeasonReview = 56,
    MotorSceneHadCheck = 57,
    RoleLangCustomFuncClicked = 58,
    ActivityRecommend2 = 59,
    ActivityRecallSplashFirstShowTime = 60,
    ActivityRecallWatchFirstShowTime = 61,
    ActivityRegressSignLastDailyFirstShowTimeStamp = 62,
    ActivityRegressShopRedDotCheckedInPeriod = 63,
    ActivityRegressQuestionnaireRedDotCheckedInPeriod = 64,
    ActivityRegressSecondQuestionnaireRedDotCheckedInPeriod = 65,
    ActivityRegressDoubleDropReminderLastShowTime = 66,
    ActivityRegressDoubleDropFirstRedDotCheckedInPeriod = 67,
    ActivityRegressBpPayButtonRedDotCheckedInPeriod = 68,
    ActivityRegressTrialRoleRedDotCheckedInPeriod = 69,
    ActivityRegressDoubleDropRedDotCheckedInPeriod = 70,
    ActivityRegressRecommendRedDotCheckedInPeriod = 71,
    ActivityRegressAdventureRedDotCheckedInPeriod = 72,
    ShowPersonalTip = 73,
    FragmentMemoryOpened = 74,
    PlayerTitleUnlockRedDot = 76,
    RogueResDungeonNewest = 77,
    RogueResSkillTreeOpen = 78,
    RogueResEndingOpen = 79,
    RogueResTaskOpen = 80,
    RogueResTrialOpen = 81,
    RogueResNewRoleFlag = 82,
    RogueResDungeonSelected = 83,
    PhantomArenaEntranceShopRefresh = 84,
    PhantomArenaEntranceGymCheck = 85,
    PhantomArenaRepeatLastIndex = 86,
    PhantomArenaEntranceGymRedDotCheck = 87,
    TrapDefenseLevelReachOpenTime = 88,
    TrapDefenseRougeModeOpen = 89,
    TrapDefenseRougeModeOpenSub = 90,
    TrialRoleGroupUnlock = 91,
    PhantomArenaChallengeUnlockRedDotCheck = 92,
    PhantomArenaMapUnlockRedDotCheck = 93,
    TrapDefenseBdBuffUnlock = 94,
    TrapDefenseBuildingClicked = 95,
    TrapDefenseAuxiliaryClicked = 96,
    DangoMonopolyNewTask = 97,
    AbyssDangoRoleNew = 98,
    AbyssDangoFormationNew = 99,
    AbyssDangoLevelCheck = 100,
    AbyssDangoEnterNew = 101,
    InfrRecordObservatoryLevel = 102,
    InfrRoadMarkUnlockRecord = 103,
    InfrObservatoryLevelUpSeq = 104,
    WeatherCentralClicked = 105,
    RogueResShopRefresh = 106,
    RogueResShopNewGoods = 107,
    PhoneMsgLastOpenBigPanelTime = 108,
}

impl Default for EClientStorageSystemIdType {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for EClientStorageSystemIdType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Activity,
            2 => Self::VisionSkin,
            3 => Self::LoopTowerIsClickSeason,
            4 => Self::LoopTowerIsClickShop,
            5 => Self::TowerOverLockArea,
            6 => Self::WeaponSkinRedDot,
            7 => Self::FlySkinRedDot,
            8 => Self::CalabashSkinRedDot,
            9 => Self::MailBindNextShowRedDotTime,
            10 => Self::RedDotAdventureNewSoundAreaTabLastUpdateTime,
            11 => Self::SuitWeaponFirstWearRecord,
            12 => Self::RoleSkinRedDot,
            13 => Self::FirstOpenVisionGroup,
            14 => Self::ShipTowerSeason,
            15 => Self::AdventrueShipTowerSeason,
            16 => Self::AdventrueWeeklyRogue,
            17 => Self::AdventrueTower,
            18 => Self::IntroductionVersion,
            19 => Self::DetectionRedDotRecord,
            20 => Self::ExploreActivityFirstUnlock,
            21 => Self::FilterRedPoint,
            22 => Self::VisionRecoveryBatchTip,
            23 => Self::VisionRecoveryBatchAimTip,
            24 => Self::MotorDiyNewUnlockId,
            25 => Self::MotorDevelopNewUnlockTree,
            26 => Self::PhantomBattleConfigApplyPlanWhenSaved,
            27 => Self::FeiXuePreheatSubViewRedDot,
            28 => Self::DropCatchRoleClickDetail,
            29 => Self::FlagChallengeNewlyUnlockedLevelIds,
            30 => Self::FlagChallengeNewlyUnlockedBuff,
            31 => Self::PeriodicActivityRedLogReport,
            32 => Self::CoopRolePhotoRedDot,
            33 => Self::CoopRoleNewLevelRedDot,
            34 => Self::FlagChallengeNewlyUnlockedBuffIds,
            35 => Self::RoguelikeNewPhantomUnlock,
            36 => Self::RoguelikeNewCharacterUnlock,
            37 => Self::RoguelikeNewEntriesGroupUnlock,
            38 => Self::AnniversaryActivityFirstClick,
            39 => Self::VisionRefineTip,
            40 => Self::Ornament,
            41 => Self::GetOrnament,
            42 => Self::ItemBackpackFirstCheck,
            43 => Self::QuestBranchSystem,
            44 => Self::GachaAccumulateFirstOpen,
            45 => Self::Photography,
            46 => Self::PhoneMsgChatShowRedDot,
            47 => Self::GolemCrack,
            48 => Self::FirstSheriffInference,
            49 => Self::ThroughTrainFirstCheck,
            50 => Self::FightPhotoTab,
            51 => Self::VersionPayGiftRedDot,
            52 => Self::NormalPayGiftRedDot,
            53 => Self::FirstOpenDailyActivityTab,
            54 => Self::RoleLangCustomRecord,
            55 => Self::ActivityRecommend,
            56 => Self::WheelTowerSeasonReview,
            57 => Self::MotorSceneHadCheck,
            58 => Self::RoleLangCustomFuncClicked,
            59 => Self::ActivityRecommend2,
            60 => Self::ActivityRecallSplashFirstShowTime,
            61 => Self::ActivityRecallWatchFirstShowTime,
            62 => Self::ActivityRegressSignLastDailyFirstShowTimeStamp,
            63 => Self::ActivityRegressShopRedDotCheckedInPeriod,
            64 => Self::ActivityRegressQuestionnaireRedDotCheckedInPeriod,
            65 => Self::ActivityRegressSecondQuestionnaireRedDotCheckedInPeriod,
            66 => Self::ActivityRegressDoubleDropReminderLastShowTime,
            67 => Self::ActivityRegressDoubleDropFirstRedDotCheckedInPeriod,
            68 => Self::ActivityRegressBpPayButtonRedDotCheckedInPeriod,
            69 => Self::ActivityRegressTrialRoleRedDotCheckedInPeriod,
            70 => Self::ActivityRegressDoubleDropRedDotCheckedInPeriod,
            71 => Self::ActivityRegressRecommendRedDotCheckedInPeriod,
            72 => Self::ActivityRegressAdventureRedDotCheckedInPeriod,
            73 => Self::ShowPersonalTip,
            74 => Self::FragmentMemoryOpened,
            76 => Self::PlayerTitleUnlockRedDot,
            77 => Self::RogueResDungeonNewest,
            78 => Self::RogueResSkillTreeOpen,
            79 => Self::RogueResEndingOpen,
            80 => Self::RogueResTaskOpen,
            81 => Self::RogueResTrialOpen,
            82 => Self::RogueResNewRoleFlag,
            83 => Self::RogueResDungeonSelected,
            84 => Self::PhantomArenaEntranceShopRefresh,
            85 => Self::PhantomArenaEntranceGymCheck,
            86 => Self::PhantomArenaRepeatLastIndex,
            87 => Self::PhantomArenaEntranceGymRedDotCheck,
            88 => Self::TrapDefenseLevelReachOpenTime,
            89 => Self::TrapDefenseRougeModeOpen,
            90 => Self::TrapDefenseRougeModeOpenSub,
            91 => Self::TrialRoleGroupUnlock,
            92 => Self::PhantomArenaChallengeUnlockRedDotCheck,
            93 => Self::PhantomArenaMapUnlockRedDotCheck,
            94 => Self::TrapDefenseBdBuffUnlock,
            95 => Self::TrapDefenseBuildingClicked,
            96 => Self::TrapDefenseAuxiliaryClicked,
            97 => Self::DangoMonopolyNewTask,
            98 => Self::AbyssDangoRoleNew,
            99 => Self::AbyssDangoFormationNew,
            100 => Self::AbyssDangoLevelCheck,
            101 => Self::AbyssDangoEnterNew,
            102 => Self::InfrRecordObservatoryLevel,
            103 => Self::InfrRoadMarkUnlockRecord,
            104 => Self::InfrObservatoryLevelUpSeq,
            105 => Self::WeatherCentralClicked,
            106 => Self::RogueResShopRefresh,
            107 => Self::RogueResShopNewGoods,
            108 => Self::PhoneMsgLastOpenBigPanelTime,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EClientStorageSystemIdType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PlayerAttrType {
    Int32 = 0,
    String = 1,
}

impl Default for PlayerAttrType {
    fn default() -> Self {
        Self::Int32
    }
}

impl crate::wire::PbEnum for PlayerAttrType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Int32,
            1 => Self::String,
            _ => Self::Int32,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PlayerAttrType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PlayerAttrKey {
    Level = 0,
    Exp = 1,
    Coin = 2,
    RareCoin = 3,
    HeadPhoto = 4,
    HeadFrame = 5,
    AreaId = 6,
    Name = 7,
    Sign = 8,
    Sex = 9,
    OriginWorldLevel = 10,
    CurWorldLevel = 11,
    WorldLevelTimeStamp = 12,
    CashCoin = 13,
    WorldPermission = 14,
    PlayerTitle = 15,
}

impl Default for PlayerAttrKey {
    fn default() -> Self {
        Self::Level
    }
}

impl crate::wire::PbEnum for PlayerAttrKey {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Level,
            1 => Self::Exp,
            2 => Self::Coin,
            3 => Self::RareCoin,
            4 => Self::HeadPhoto,
            5 => Self::HeadFrame,
            6 => Self::AreaId,
            7 => Self::Name,
            8 => Self::Sign,
            9 => Self::Sex,
            10 => Self::OriginWorldLevel,
            11 => Self::CurWorldLevel,
            12 => Self::WorldLevelTimeStamp,
            13 => Self::CashCoin,
            14 => Self::WorldPermission,
            15 => Self::PlayerTitle,
            _ => Self::Level,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PlayerAttrKey {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BattlePassType {
    Free = 0,
    Pay = 1,
}

impl Default for BattlePassType {
    fn default() -> Self {
        Self::Free
    }
}

impl crate::wire::PbEnum for BattlePassType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Free,
            1 => Self::Pay,
            _ => Self::Free,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BattlePassType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BattlePassPayStatus {
    NoPaid = 0,
    Paid = 1,
    Advanced = 2,
}

impl Default for BattlePassPayStatus {
    fn default() -> Self {
        Self::NoPaid
    }
}

impl crate::wire::PbEnum for BattlePassPayStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NoPaid,
            1 => Self::Paid,
            2 => Self::Advanced,
            _ => Self::NoPaid,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BattlePassPayStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BtType {
    BtTypeInvalid = 0,
    BtTypeQuest = 1,
    BtTypeLevelPlay = 2,
    BtTypeInst = 3,
    BtTypeInstDecision = 4,
    BtTypeRecall = 5,
}

impl Default for BtType {
    fn default() -> Self {
        Self::BtTypeInvalid
    }
}

impl crate::wire::PbEnum for BtType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BtTypeInvalid,
            1 => Self::BtTypeQuest,
            2 => Self::BtTypeLevelPlay,
            3 => Self::BtTypeInst,
            4 => Self::BtTypeInstDecision,
            5 => Self::BtTypeRecall,
            _ => Self::BtTypeInvalid,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BtType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum NodeStatus {
    NotActive = 0,
    BeforeActivate = 1,
    Activate = 2,
    Completing = 3,
    CompletedSuccess = 4,
    CompletedFailed = 5,
    Suspend = 6,
    Destroy = 7,
}

impl Default for NodeStatus {
    fn default() -> Self {
        Self::NotActive
    }
}

impl crate::wire::PbEnum for NodeStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotActive,
            1 => Self::BeforeActivate,
            2 => Self::Activate,
            3 => Self::Completing,
            4 => Self::CompletedSuccess,
            5 => Self::CompletedFailed,
            6 => Self::Suspend,
            7 => Self::Destroy,
            _ => Self::NotActive,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for NodeStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ChildQuestNodeStatus {
    NotActive = 0,
    Enter = 1,
    EnterAction = 2,
    Progress = 3,
    Finished = 4,
    FinishAction = 5,
    Fail = 6,
}

impl Default for ChildQuestNodeStatus {
    fn default() -> Self {
        Self::NotActive
    }
}

impl crate::wire::PbEnum for ChildQuestNodeStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotActive,
            1 => Self::Enter,
            2 => Self::EnterAction,
            3 => Self::Progress,
            4 => Self::Finished,
            5 => Self::FinishAction,
            6 => Self::Fail,
            _ => Self::NotActive,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ChildQuestNodeStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ActionType {
    GetItem = 0,
    SendNpcMail = 1,
}

impl Default for ActionType {
    fn default() -> Self {
        Self::GetItem
    }
}

impl crate::wire::PbEnum for ActionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GetItem,
            1 => Self::SendNpcMail,
            _ => Self::GetItem,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ActionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SuspendType {
    Occupation = 0,
    Online = 1,
    ScreenOccupation = 2,
}

impl Default for SuspendType {
    fn default() -> Self {
        Self::Occupation
    }
}

impl crate::wire::PbEnum for SuspendType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Occupation,
            1 => Self::Online,
            2 => Self::ScreenOccupation,
            _ => Self::Occupation,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SuspendType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SourceType {
    UnDefine = 0,
    SourceEntity = 3,
    SourceQuest = 4,
}

impl Default for SourceType {
    fn default() -> Self {
        Self::UnDefine
    }
}

impl crate::wire::PbEnum for SourceType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::UnDefine,
            3 => Self::SourceEntity,
            4 => Self::SourceQuest,
            _ => Self::UnDefine,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SourceType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EntityConfigType {
    OldEntity = 0,
    Level = 1,
    Global = 2,
    Character = 3,
    Template = 4,
    Prefab = 5,
}

impl Default for EntityConfigType {
    fn default() -> Self {
        Self::OldEntity
    }
}

impl crate::wire::PbEnum for EntityConfigType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::OldEntity,
            1 => Self::Level,
            2 => Self::Global,
            3 => Self::Character,
            4 => Self::Template,
            5 => Self::Prefab,
            _ => Self::OldEntity,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EntityConfigType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DestroyType {
    NotDelay = 0,
    Delay = 1,
}

impl Default for DestroyType {
    fn default() -> Self {
        Self::NotDelay
    }
}

impl crate::wire::PbEnum for DestroyType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotDelay,
            1 => Self::Delay,
            _ => Self::NotDelay,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DestroyType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LivingStatus {
    Alive = 0,
    Dead = 1,
    Init = 2,
    Dying = 3,
}

impl Default for LivingStatus {
    fn default() -> Self {
        Self::Alive
    }
}

impl crate::wire::PbEnum for LivingStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Alive,
            1 => Self::Dead,
            2 => Self::Init,
            3 => Self::Dying,
            _ => Self::Alive,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for LivingStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EntityState {
    Default = 0,
    Sleep = 1,
    Born = 2,
    Other = 3,
}

impl Default for EntityState {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for EntityState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Sleep,
            2 => Self::Born,
            3 => Self::Other,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EntityState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum StaticHookMoveType {
    Hook = 0,
    Pull = 1,
}

impl Default for StaticHookMoveType {
    fn default() -> Self {
        Self::Hook
    }
}

impl crate::wire::PbEnum for StaticHookMoveType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Hook,
            1 => Self::Pull,
            _ => Self::Hook,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for StaticHookMoveType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ExitVehicleType {
    ExitVehicleTypeLaunch = 0,
    ExitVehicleTypeNormal = 1,
    ExitVehicleTypeDelayShow = 2,
    ExitVehicleTypeSeatStandUp = 3,
    ExitVehicleTypeAllRole = 4,
}

impl Default for ExitVehicleType {
    fn default() -> Self {
        Self::ExitVehicleTypeLaunch
    }
}

impl crate::wire::PbEnum for ExitVehicleType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ExitVehicleTypeLaunch,
            1 => Self::ExitVehicleTypeNormal,
            2 => Self::ExitVehicleTypeDelayShow,
            3 => Self::ExitVehicleTypeSeatStandUp,
            4 => Self::ExitVehicleTypeAllRole,
            _ => Self::ExitVehicleTypeLaunch,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ExitVehicleType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum VehicleShareReason {
    ClientRequest = 0,
    ChangeFormation = 1,
    GetOff = 2,
    RebackScene = 3,
    CharacterDie = 4,
    Command = 5,
}

impl Default for VehicleShareReason {
    fn default() -> Self {
        Self::ClientRequest
    }
}

impl crate::wire::PbEnum for VehicleShareReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ClientRequest,
            1 => Self::ChangeFormation,
            2 => Self::GetOff,
            3 => Self::RebackScene,
            4 => Self::CharacterDie,
            5 => Self::Command,
            _ => Self::ClientRequest,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for VehicleShareReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RideMode {
    ShareRideMode_MovieMotor = 0,
}

impl Default for RideMode {
    fn default() -> Self {
        Self::ShareRideMode_MovieMotor
    }
}

impl crate::wire::PbEnum for RideMode {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ShareRideMode_MovieMotor,
            _ => Self::ShareRideMode_MovieMotor,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RideMode {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TemplateSpawnerType {
    TemplateDefault = 0,
    TemplateMatrix = 1,
}

impl Default for TemplateSpawnerType {
    fn default() -> Self {
        Self::TemplateDefault
    }
}

impl crate::wire::PbEnum for TemplateSpawnerType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TemplateDefault,
            1 => Self::TemplateMatrix,
            _ => Self::TemplateDefault,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TemplateSpawnerType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GridPbDirection {
    GridForward = 0,
    GridBackward = 1,
    GridLeft = 2,
    GridRight = 3,
}

impl Default for GridPbDirection {
    fn default() -> Self {
        Self::GridForward
    }
}

impl crate::wire::PbEnum for GridPbDirection {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GridForward,
            1 => Self::GridBackward,
            2 => Self::GridLeft,
            3 => Self::GridRight,
            _ => Self::GridForward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GridPbDirection {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum VehicleSource {
    VehicleSourceNone = 0,
    VehicleSourceFishingShip = 1,
    VehicleSourceGongduolaSummon = 2,
}

impl Default for VehicleSource {
    fn default() -> Self {
        Self::VehicleSourceNone
    }
}

impl crate::wire::PbEnum for VehicleSource {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::VehicleSourceNone,
            1 => Self::VehicleSourceFishingShip,
            2 => Self::VehicleSourceGongduolaSummon,
            _ => Self::VehicleSourceNone,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for VehicleSource {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ApplyGEType {
    Common = 0,
    UseExtraTime = 1,
}

impl Default for ApplyGEType {
    fn default() -> Self {
        Self::Common
    }
}

impl crate::wire::PbEnum for ApplyGEType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Common,
            1 => Self::UseExtraTime,
            _ => Self::Common,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ApplyGEType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EFsmStateFlag {
    Changed = 0,
    Confirmed = 1,
}

impl Default for EFsmStateFlag {
    fn default() -> Self {
        Self::Changed
    }
}

impl crate::wire::PbEnum for EFsmStateFlag {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Changed,
            1 => Self::Confirmed,
            _ => Self::Changed,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EFsmStateFlag {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FsmStateBehaviorType {
    Enter = 0,
    Exit = 1,
    BindStart = 2,
    BindEnd = 3,
    Task = 4,
}

impl Default for FsmStateBehaviorType {
    fn default() -> Self {
        Self::Enter
    }
}

impl crate::wire::PbEnum for FsmStateBehaviorType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Enter,
            1 => Self::Exit,
            2 => Self::BindStart,
            3 => Self::BindEnd,
            4 => Self::Task,
            _ => Self::Enter,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FsmStateBehaviorType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ESummonType {
    Default = 0,
    ConcomitantVision = 1,
    ConcomitantCustom = 2,
    ConcomitantPhantomRole = 3,
    ConcomitantWeakVision = 4,
    ConcomitantMotorcycle = 5,
    ConcomitantSkillAvatar = 6,
}

impl Default for ESummonType {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for ESummonType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::ConcomitantVision,
            2 => Self::ConcomitantCustom,
            3 => Self::ConcomitantPhantomRole,
            4 => Self::ConcomitantWeakVision,
            5 => Self::ConcomitantMotorcycle,
            6 => Self::ConcomitantSkillAvatar,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ESummonType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FollowerType {
    EPlayerFollowerDefault = 0,
    EPlayerFollowerExploreSkill = 1,
    EPlayerFollowerAuxiliary = 2,
    EPlayerFollowerSpecialItem = 3,
    EPlayerFollowerMotor = 4,
    EPlayerFollowerMax = 5,
}

impl Default for FollowerType {
    fn default() -> Self {
        Self::EPlayerFollowerDefault
    }
}

impl crate::wire::PbEnum for FollowerType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::EPlayerFollowerDefault,
            1 => Self::EPlayerFollowerExploreSkill,
            2 => Self::EPlayerFollowerAuxiliary,
            3 => Self::EPlayerFollowerSpecialItem,
            4 => Self::EPlayerFollowerMotor,
            5 => Self::EPlayerFollowerMax,
            _ => Self::EPlayerFollowerDefault,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FollowerType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SceneItemBBKey {
    ManipulatableState = 0,
}

impl Default for SceneItemBBKey {
    fn default() -> Self {
        Self::ManipulatableState
    }
}

impl crate::wire::PbEnum for SceneItemBBKey {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ManipulatableState,
            _ => Self::ManipulatableState,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SceneItemBBKey {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DirectionType {
    GravityUp = 0,
    GravityDown = 1,
    GravityLeft = 2,
    GravityRight = 3,
}

impl Default for DirectionType {
    fn default() -> Self {
        Self::GravityUp
    }
}

impl crate::wire::PbEnum for DirectionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GravityUp,
            1 => Self::GravityDown,
            2 => Self::GravityLeft,
            3 => Self::GravityRight,
            _ => Self::GravityUp,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DirectionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RbGridDirection {
    RbForward = 0,
    RbBackward = 1,
    RbRight = 2,
    RbLeft = 3,
}

impl Default for RbGridDirection {
    fn default() -> Self {
        Self::RbForward
    }
}

impl crate::wire::PbEnum for RbGridDirection {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RbForward,
            1 => Self::RbBackward,
            2 => Self::RbRight,
            3 => Self::RbLeft,
            _ => Self::RbForward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RbGridDirection {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ButtonType {
    Home = 0,
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Home
    }
}

impl crate::wire::PbEnum for ButtonType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Home,
            _ => Self::Home,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ButtonType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SelfPortraitOpState {
    SelfPortraitEnter = 0,
    SelfPortraitExit = 1,
}

impl Default for SelfPortraitOpState {
    fn default() -> Self {
        Self::SelfPortraitEnter
    }
}

impl crate::wire::PbEnum for SelfPortraitOpState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SelfPortraitEnter,
            1 => Self::SelfPortraitExit,
            _ => Self::SelfPortraitEnter,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SelfPortraitOpState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ChatContentType {
    Text = 0,
    Emoji = 1,
}

impl Default for ChatContentType {
    fn default() -> Self {
        Self::Text
    }
}

impl crate::wire::PbEnum for ChatContentType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Text,
            1 => Self::Emoji,
            _ => Self::Text,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ChatContentType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrivateChatOperateType {
    CloseChat = 0,
    OpenChat = 1,
    ReadMsg = 2,
}

impl Default for PrivateChatOperateType {
    fn default() -> Self {
        Self::CloseChat
    }
}

impl crate::wire::PbEnum for PrivateChatOperateType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::CloseChat,
            1 => Self::OpenChat,
            2 => Self::ReadMsg,
            _ => Self::CloseChat,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PrivateChatOperateType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RewardType {
    TaskReward = 0,
    SignReward = 2,
    ScoreReward = 3,
}

impl Default for RewardType {
    fn default() -> Self {
        Self::TaskReward
    }
}

impl crate::wire::PbEnum for RewardType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TaskReward,
            2 => Self::SignReward,
            3 => Self::ScoreReward,
            _ => Self::TaskReward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RewardType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SceneMode {
    Single = 0,
    Multi = 1,
}

impl Default for SceneMode {
    fn default() -> Self {
        Self::Single
    }
}

impl crate::wire::PbEnum for SceneMode {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Single,
            1 => Self::Multi,
            _ => Self::Single,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SceneMode {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SceneDateUpdateReason {
    TimeFlowAuto = 0,
    LevelPlayAuto = 1,
    PlayerOperate = 2,
}

impl Default for SceneDateUpdateReason {
    fn default() -> Self {
        Self::TimeFlowAuto
    }
}

impl crate::wire::PbEnum for SceneDateUpdateReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TimeFlowAuto,
            1 => Self::LevelPlayAuto,
            2 => Self::PlayerOperate,
            _ => Self::TimeFlowAuto,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SceneDateUpdateReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BlackScreenSourceType {
    BehaviorTreeNode = 0,
    ClientAuto = 1,
}

impl Default for BlackScreenSourceType {
    fn default() -> Self {
        Self::BehaviorTreeNode
    }
}

impl crate::wire::PbEnum for BlackScreenSourceType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BehaviorTreeNode,
            1 => Self::ClientAuto,
            _ => Self::BehaviorTreeNode,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BlackScreenSourceType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EBulletCreateSource {
    NormalSource = 0,
    ReboundSource = 1,
    CollisionSpawnSource = 2,
}

impl Default for EBulletCreateSource {
    fn default() -> Self {
        Self::NormalSource
    }
}

impl crate::wire::PbEnum for EBulletCreateSource {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NormalSource,
            1 => Self::ReboundSource,
            2 => Self::CollisionSpawnSource,
            _ => Self::NormalSource,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EBulletCreateSource {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ENewLinkStage {
    NewLinkStageNone = 0,
    NewLinkStageLock = 1,
    Accumulate = 2,
    Ready = 3,
    Burst = 4,
}

impl Default for ENewLinkStage {
    fn default() -> Self {
        Self::NewLinkStageNone
    }
}

impl crate::wire::PbEnum for ENewLinkStage {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NewLinkStageNone,
            1 => Self::NewLinkStageLock,
            2 => Self::Accumulate,
            3 => Self::Ready,
            4 => Self::Burst,
            _ => Self::NewLinkStageNone,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ENewLinkStage {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EEndSkillReason {
    Default = 0,
    BeginOtherSkill = 1,
    BeHit = 2,
    BeCounter = 3,
}

impl Default for EEndSkillReason {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for EEndSkillReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::BeginOtherSkill,
            2 => Self::BeHit,
            3 => Self::BeCounter,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EEndSkillReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DamageSourceType {
    FromBullet = 0,
    FromEffect = 1,
}

impl Default for DamageSourceType {
    fn default() -> Self {
        Self::FromBullet
    }
}

impl crate::wire::PbEnum for DamageSourceType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::FromBullet,
            1 => Self::FromEffect,
            _ => Self::FromBullet,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DamageSourceType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BattleModule {
    Damage = 0,
    GameplayEffect = 1,
    Log = 2,
}

impl Default for BattleModule {
    fn default() -> Self {
        Self::Damage
    }
}

impl crate::wire::PbEnum for BattleModule {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Damage,
            1 => Self::GameplayEffect,
            2 => Self::Log,
            _ => Self::Damage,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BattleModule {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EDamageImmune {
    Default = 0,
    Invincible = 1,
    BuffEffectElement = 2,
    BulletCurNoCtrl = 3,
    VehiclePassenger = 4,
    FishBoat = 5,
    Revive = 6,
}

impl Default for EDamageImmune {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for EDamageImmune {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Invincible,
            2 => Self::BuffEffectElement,
            3 => Self::BulletCurNoCtrl,
            4 => Self::VehiclePassenger,
            5 => Self::FishBoat,
            6 => Self::Revive,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EDamageImmune {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EShieldUpdateType {
    EShieldUpdateTypeAdd = 0,
    EShieldUpdateTypeDel = 1,
    EShieldUpdateTypeModify = 2,
}

impl Default for EShieldUpdateType {
    fn default() -> Self {
        Self::EShieldUpdateTypeAdd
    }
}

impl crate::wire::PbEnum for EShieldUpdateType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::EShieldUpdateTypeAdd,
            1 => Self::EShieldUpdateTypeDel,
            2 => Self::EShieldUpdateTypeModify,
            _ => Self::EShieldUpdateTypeAdd,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EShieldUpdateType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SwitchRoleType {
    SignleWorld = 0,
    MultiWorld = 1,
    FbInstance = 2,
}

impl Default for SwitchRoleType {
    fn default() -> Self {
        Self::SignleWorld
    }
}

impl crate::wire::PbEnum for SwitchRoleType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SignleWorld,
            1 => Self::MultiWorld,
            2 => Self::FbInstance,
            _ => Self::SignleWorld,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SwitchRoleType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CombatValType {
    CvtNone = 0,
    CvtVirtualHp = 1,
}

impl Default for CombatValType {
    fn default() -> Self {
        Self::CvtNone
    }
}

impl crate::wire::PbEnum for CombatValType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::CvtNone,
            1 => Self::CvtVirtualHp,
            _ => Self::CvtNone,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for CombatValType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ControlType {
    Unknown = 0,
    TemporaryTeleportCtrl = 1,
}

impl Default for ControlType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl crate::wire::PbEnum for ControlType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::TemporaryTeleportCtrl,
            _ => Self::Unknown,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ControlType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RouletteType {
    Explore = 0,
    Function = 1,
    TrapDefense = 2,
    Motorcycle = 3,
}

impl Default for RouletteType {
    fn default() -> Self {
        Self::Explore
    }
}

impl crate::wire::PbEnum for RouletteType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Explore,
            1 => Self::Function,
            2 => Self::TrapDefense,
            3 => Self::Motorcycle,
            _ => Self::Explore,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RouletteType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FavorQuestStatus {
    Locked = 0,
    CanAccept = 1,
    Accepted = 2,
    Completed = 3,
}

impl Default for FavorQuestStatus {
    fn default() -> Self {
        Self::Locked
    }
}

impl crate::wire::PbEnum for FavorQuestStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Locked,
            1 => Self::CanAccept,
            2 => Self::Accepted,
            3 => Self::Completed,
            _ => Self::Locked,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FavorQuestStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FavorItemStatus {
    ItemLocked = 0,
    ItemCanUnLock = 1,
    ItemUnLocked = 2,
}

impl Default for FavorItemStatus {
    fn default() -> Self {
        Self::ItemLocked
    }
}

impl crate::wire::PbEnum for FavorItemStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ItemLocked,
            1 => Self::ItemCanUnLock,
            2 => Self::ItemUnLocked,
            _ => Self::ItemLocked,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FavorItemStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FishingEntrustStatus {
    Created = 0,
    Acceptable = 1,
    Accepted = 2,
}

impl Default for FishingEntrustStatus {
    fn default() -> Self {
        Self::Created
    }
}

impl crate::wire::PbEnum for FishingEntrustStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Created,
            1 => Self::Acceptable,
            2 => Self::Accepted,
            _ => Self::Created,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FishingEntrustStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FishingItemRotate {
    No = 0,
    DirectionDown = 1,
    DirectionLeft = 2,
    DirectionUp = 3,
}

impl Default for FishingItemRotate {
    fn default() -> Self {
        Self::No
    }
}

impl crate::wire::PbEnum for FishingItemRotate {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::No,
            1 => Self::DirectionDown,
            2 => Self::DirectionLeft,
            3 => Self::DirectionUp,
            _ => Self::No,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FishingItemRotate {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FishCup {
    SilverCup = 0,
    NormalCup = 1,
    GoldCup = 2,
}

impl Default for FishCup {
    fn default() -> Self {
        Self::SilverCup
    }
}

impl crate::wire::PbEnum for FishCup {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SilverCup,
            1 => Self::NormalCup,
            2 => Self::GoldCup,
            _ => Self::SilverCup,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FishCup {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IllustratedType {
    Monster = 0,
    VocalCorpse = 1,
    ViewPoint = 2,
    Weapon = 3,
    Animal = 4,
    Item = 5,
    Chip = 6,
    Photograph = 7,
    Noun = 8,
}

impl Default for IllustratedType {
    fn default() -> Self {
        Self::Monster
    }
}

impl crate::wire::PbEnum for IllustratedType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Monster,
            1 => Self::VocalCorpse,
            2 => Self::ViewPoint,
            3 => Self::Weapon,
            4 => Self::Animal,
            5 => Self::Item,
            6 => Self::Chip,
            7 => Self::Photograph,
            8 => Self::Noun,
            _ => Self::Monster,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for IllustratedType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhotographSubType {
    None = 0,
    PhotographSub = 7,
    Role = 8,
    Quest = 9,
}

impl Default for PhotographSubType {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for PhotographSubType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            7 => Self::PhotographSub,
            8 => Self::Role,
            9 => Self::Quest,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhotographSubType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum InfrTaskStatusPb {
    InfrTaskRunning = 0,
    InfrTaskFinish = 1,
    InfrTaskTaken = 2,
}

impl Default for InfrTaskStatusPb {
    fn default() -> Self {
        Self::InfrTaskRunning
    }
}

impl crate::wire::PbEnum for InfrTaskStatusPb {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::InfrTaskRunning,
            1 => Self::InfrTaskFinish,
            2 => Self::InfrTaskTaken,
            _ => Self::InfrTaskRunning,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for InfrTaskStatusPb {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum InfrStatusPb {
    InfrStatusLock = 0,
    InfrStatusProgress = 1,
    InfrStatusComplete = 2,
}

impl Default for InfrStatusPb {
    fn default() -> Self {
        Self::InfrStatusLock
    }
}

impl crate::wire::PbEnum for InfrStatusPb {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::InfrStatusLock,
            1 => Self::InfrStatusProgress,
            2 => Self::InfrStatusComplete,
            _ => Self::InfrStatusLock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for InfrStatusPb {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum InfrV2StatusPb {
    InfrV2StatusLock = 0,
    InfrV2StatusProgress = 1,
    InfrV2StatusComplete = 2,
}

impl Default for InfrV2StatusPb {
    fn default() -> Self {
        Self::InfrV2StatusLock
    }
}

impl crate::wire::PbEnum for InfrV2StatusPb {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::InfrV2StatusLock,
            1 => Self::InfrV2StatusProgress,
            2 => Self::InfrV2StatusComplete,
            _ => Self::InfrV2StatusLock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for InfrV2StatusPb {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EquipPos {
    Weapon = 0,
    WeaponSkin = 1,
    End = 2,
}

impl Default for EquipPos {
    fn default() -> Self {
        Self::Weapon
    }
}

impl crate::wire::PbEnum for EquipPos {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Weapon,
            1 => Self::WeaponSkin,
            2 => Self::End,
            _ => Self::Weapon,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EquipPos {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MailLevel {
    None = 0,
    General = 1,
    Important = 2,
    Administration = 3,
}

impl Default for MailLevel {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for MailLevel {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::General,
            2 => Self::Important,
            3 => Self::Administration,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MailLevel {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MarkPointState {
    MarkNormal = 0,
    MarkDisable = 1,
    MarkComplete = 2,
}

impl Default for MarkPointState {
    fn default() -> Self {
        Self::MarkNormal
    }
}

impl crate::wire::PbEnum for MarkPointState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::MarkNormal,
            1 => Self::MarkDisable,
            2 => Self::MarkComplete,
            _ => Self::MarkNormal,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MarkPointState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MotorTaskTypePb {
    Unknown = 0,
    Single = 1,
    Limited = 2,
    Cycle = 3,
}

impl Default for MotorTaskTypePb {
    fn default() -> Self {
        Self::Unknown
    }
}

impl crate::wire::PbEnum for MotorTaskTypePb {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Single,
            2 => Self::Limited,
            3 => Self::Cycle,
            _ => Self::Unknown,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MotorTaskTypePb {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PayUpdateType {
    None = 0,
    Daily = 1,
    Weekly = 2,
    Monthly = 3,
    Forever = 4,
}

impl Default for PayUpdateType {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for PayUpdateType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Daily,
            2 => Self::Weekly,
            3 => Self::Monthly,
            4 => Self::Forever,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PayUpdateType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PayShopItemType {
    Normal = 0,
    Direct = 1,
}

impl Default for PayShopItemType {
    fn default() -> Self {
        Self::Normal
    }
}

impl crate::wire::PbEnum for PayShopItemType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Direct,
            _ => Self::Normal,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PayShopItemType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhBaPlanAttrDeal {
    PhBaDefault = 0,
    PhBaLock = 1,
    PhBaDiscard = 2,
}

impl Default for PhBaPlanAttrDeal {
    fn default() -> Self {
        Self::PhBaDefault
    }
}

impl crate::wire::PbEnum for PhBaPlanAttrDeal {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::PhBaDefault,
            1 => Self::PhBaLock,
            2 => Self::PhBaDiscard,
            _ => Self::PhBaDefault,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhBaPlanAttrDeal {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhBaCostType {
    PhBaCostDefault = 0,
    PhBaCost1 = 1,
    PhBaCost3 = 3,
    PhBaCost4 = 4,
}

impl Default for PhBaCostType {
    fn default() -> Self {
        Self::PhBaCostDefault
    }
}

impl crate::wire::PbEnum for PhBaCostType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::PhBaCostDefault,
            1 => Self::PhBaCost1,
            3 => Self::PhBaCost3,
            4 => Self::PhBaCost4,
            _ => Self::PhBaCostDefault,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhBaCostType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhantomSettingType {
    AutoLock = 0,
    AutoDisuse = 1,
}

impl Default for PhantomSettingType {
    fn default() -> Self {
        Self::AutoLock
    }
}

impl crate::wire::PbEnum for PhantomSettingType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::AutoLock,
            1 => Self::AutoDisuse,
            _ => Self::AutoLock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhantomSettingType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhantomFuncValue {
    Locked = 0,
    Disuse = 1,
    Reset = 2,
}

impl Default for PhantomFuncValue {
    fn default() -> Self {
        Self::Locked
    }
}

impl crate::wire::PbEnum for PhantomFuncValue {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Locked,
            1 => Self::Disuse,
            2 => Self::Reset,
            _ => Self::Locked,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhantomFuncValue {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MotionType {
    Spurt = 0,
    Pullback = 1,
    BeLand = 2,
    MotionJump = 3,
    AirSprint = 4,
    BackFlip = 5,
    StepAcross = 6,
    ClimbTop = 7,
    LimitDodge = 8,
    CounterAttack = 9,
}

impl Default for MotionType {
    fn default() -> Self {
        Self::Spurt
    }
}

impl crate::wire::PbEnum for MotionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Spurt,
            1 => Self::Pullback,
            2 => Self::BeLand,
            3 => Self::MotionJump,
            4 => Self::AirSprint,
            5 => Self::BackFlip,
            6 => Self::StepAcross,
            7 => Self::ClimbTop,
            8 => Self::LimitDodge,
            9 => Self::CounterAttack,
            _ => Self::Spurt,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MotionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum QuestState {
    InActive = 0,
    Available = 1,
    InProgress = 2,
    Finish = 3,
    Delete = 4,
}

impl Default for QuestState {
    fn default() -> Self {
        Self::InActive
    }
}

impl crate::wire::PbEnum for QuestState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::InActive,
            1 => Self::Available,
            2 => Self::InProgress,
            3 => Self::Finish,
            4 => Self::Delete,
            _ => Self::InActive,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for QuestState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum QuestFocusReason {
    Request = 0,
    Inherit = 1,
}

impl Default for QuestFocusReason {
    fn default() -> Self {
        Self::Request
    }
}

impl crate::wire::PbEnum for QuestFocusReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Request,
            1 => Self::Inherit,
            _ => Self::Request,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for QuestFocusReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RangeType {
    RangeEnter = 0,
    RangeLeave = 1,
    RangeInit = 2,
    RangeInitOut = 3,
}

impl Default for RangeType {
    fn default() -> Self {
        Self::RangeEnter
    }
}

impl crate::wire::PbEnum for RangeType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RangeEnter,
            1 => Self::RangeLeave,
            2 => Self::RangeInit,
            3 => Self::RangeInitOut,
            _ => Self::RangeEnter,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RangeType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum InputSettingDevice {
    Mouse = 0,
    Handle = 1,
}

impl Default for InputSettingDevice {
    fn default() -> Self {
        Self::Mouse
    }
}

impl crate::wire::PbEnum for InputSettingDevice {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Mouse,
            1 => Self::Handle,
            _ => Self::Mouse,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for InputSettingDevice {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SettingInputType {
    Normal = 0,
    Motorcycle = 1,
}

impl Default for SettingInputType {
    fn default() -> Self {
        Self::Normal
    }
}

impl crate::wire::PbEnum for SettingInputType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Motorcycle,
            _ => Self::Normal,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SettingInputType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MapOpenType {
    HotKey = 0,
    Mouse = 1,
    Other = 2,
}

impl Default for MapOpenType {
    fn default() -> Self {
        Self::HotKey
    }
}

impl crate::wire::PbEnum for MapOpenType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::HotKey,
            1 => Self::Mouse,
            2 => Self::Other,
            _ => Self::HotKey,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MapOpenType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LeaveInstWay {
    Default = 0,
    Esc = 1,
}

impl Default for LeaveInstWay {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for LeaveInstWay {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Esc,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for LeaveInstWay {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TimerSetType {
    Add = 0,
    Sub = 1,
    Set = 2,
}

impl Default for TimerSetType {
    fn default() -> Self {
        Self::Add
    }
}

impl crate::wire::PbEnum for TimerSetType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Add,
            1 => Self::Sub,
            2 => Self::Set,
            _ => Self::Add,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TimerSetType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ChangeWeatherReason {
    ClientDefault = 0,
    ClientSkill = 1,
}

impl Default for ChangeWeatherReason {
    fn default() -> Self {
        Self::ClientDefault
    }
}

impl crate::wire::PbEnum for ChangeWeatherReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ClientDefault,
            1 => Self::ClientSkill,
            _ => Self::ClientDefault,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ChangeWeatherReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ChatChannelType {
    Default = 0,
    Private = 1,
    Team = 2,
}

impl Default for ChatChannelType {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for ChatChannelType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Private,
            2 => Self::Team,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ChatChannelType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SubChatChannelType {
    WorldTeam = 0,
    MatchTeam = 1,
}

impl Default for SubChatChannelType {
    fn default() -> Self {
        Self::WorldTeam
    }
}

impl crate::wire::PbEnum for SubChatChannelType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::WorldTeam,
            1 => Self::MatchTeam,
            _ => Self::WorldTeam,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SubChatChannelType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CabinType {
    ShipCabin = 0,
    TempCabin = 1,
    NetCabin = 2,
    Cage = 3,
    HandIn = 4,
}

impl Default for CabinType {
    fn default() -> Self {
        Self::ShipCabin
    }
}

impl crate::wire::PbEnum for CabinType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ShipCabin,
            1 => Self::TempCabin,
            2 => Self::NetCabin,
            3 => Self::Cage,
            4 => Self::HandIn,
            _ => Self::ShipCabin,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for CabinType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AccessRangeType {
    RangeEnter = 0,
    RangeLeave = 1,
    RangeInit = 2,
    RangeInitOut = 3,
}

impl Default for AccessRangeType {
    fn default() -> Self {
        Self::RangeEnter
    }
}

impl crate::wire::PbEnum for AccessRangeType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RangeEnter,
            1 => Self::RangeLeave,
            2 => Self::RangeInit,
            3 => Self::RangeInitOut,
            _ => Self::RangeEnter,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for AccessRangeType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AceBattleAntiCheatingLogType {
    LogTypeSecGetReportData2Flow = 0,
    LogTypeSecFbroundStartFlow = 1,
    LogTypeSecFbroundEndFlow = 2,
    LogTypeSecRoleFightFlowBigWorldStart = 3,
    LogTypeSecRoleFightFlowBigWorldEnd = 4,
    LogTypeSecRoleFightFlowInstStart = 5,
    LogTypeSecRoleFightFlowInstEnd = 6,
    LogTypeSecWorldInfoFlowStart = 7,
    LogTypeSecWorldInfoFlowEnd = 8,
    LogTypeSecWorldStartFlow = 9,
    LogTypeSecWorldSendFlow = 10,
}

impl Default for AceBattleAntiCheatingLogType {
    fn default() -> Self {
        Self::LogTypeSecGetReportData2Flow
    }
}

impl crate::wire::PbEnum for AceBattleAntiCheatingLogType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::LogTypeSecGetReportData2Flow,
            1 => Self::LogTypeSecFbroundStartFlow,
            2 => Self::LogTypeSecFbroundEndFlow,
            3 => Self::LogTypeSecRoleFightFlowBigWorldStart,
            4 => Self::LogTypeSecRoleFightFlowBigWorldEnd,
            5 => Self::LogTypeSecRoleFightFlowInstStart,
            6 => Self::LogTypeSecRoleFightFlowInstEnd,
            7 => Self::LogTypeSecWorldInfoFlowStart,
            8 => Self::LogTypeSecWorldInfoFlowEnd,
            9 => Self::LogTypeSecWorldStartFlow,
            10 => Self::LogTypeSecWorldSendFlow,
            _ => Self::LogTypeSecGetReportData2Flow,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for AceBattleAntiCheatingLogType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ActionInfrastructureItemDeliveryType {
    Observatory = 0,
    Road = 1,
}

impl Default for ActionInfrastructureItemDeliveryType {
    fn default() -> Self {
        Self::Observatory
    }
}

impl crate::wire::PbEnum for ActionInfrastructureItemDeliveryType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Observatory,
            1 => Self::Road,
            _ => Self::Observatory,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ActionInfrastructureItemDeliveryType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ActionInfrastructureV2ItemDeliveryType {
    Village = 0,
    Tree = 1,
}

impl Default for ActionInfrastructureV2ItemDeliveryType {
    fn default() -> Self {
        Self::Village
    }
}

impl crate::wire::PbEnum for ActionInfrastructureV2ItemDeliveryType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Village,
            1 => Self::Tree,
            _ => Self::Village,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ActionInfrastructureV2ItemDeliveryType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ActionOpenSystemType {
    None = 0,
    ActionOpenSystem = 1,
    SoaringChallenge = 2,
    FishingHandIn = 3,
    GreatSwordChallenge = 4,
    InfrHandIn = 5,
    MotorRaceChallenge = 6,
    InfrV2HandIn = 7,
    GiantSystem = 8,
}

impl Default for ActionOpenSystemType {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for ActionOpenSystemType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::ActionOpenSystem,
            2 => Self::SoaringChallenge,
            3 => Self::FishingHandIn,
            4 => Self::GreatSwordChallenge,
            5 => Self::InfrHandIn,
            6 => Self::MotorRaceChallenge,
            7 => Self::InfrV2HandIn,
            8 => Self::GiantSystem,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ActionOpenSystemType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ActionScreenOperationType {
    Close = 0,
    Open = 1,
}

impl Default for ActionScreenOperationType {
    fn default() -> Self {
        Self::Close
    }
}

impl crate::wire::PbEnum for ActionScreenOperationType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Close,
            1 => Self::Open,
            _ => Self::Close,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ActionScreenOperationType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ActionTime {
    Enter = 0,
    Complete = 1,
}

impl Default for ActionTime {
    fn default() -> Self {
        Self::Enter
    }
}

impl crate::wire::PbEnum for ActionTime {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Enter,
            1 => Self::Complete,
            _ => Self::Enter,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ActionTime {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AdsorbType {
    SceneAdsorb = 0,
    PlayerAdsorb = 1,
}

impl Default for AdsorbType {
    fn default() -> Self {
        Self::SceneAdsorb
    }
}

impl crate::wire::PbEnum for AdsorbType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SceneAdsorb,
            1 => Self::PlayerAdsorb,
            _ => Self::SceneAdsorb,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for AdsorbType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AgreeJoinWay {
    ClientActive = 0,
    ClientTimeOut = 1,
}

impl Default for AgreeJoinWay {
    fn default() -> Self {
        Self::ClientActive
    }
}

impl crate::wire::PbEnum for AgreeJoinWay {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ClientActive,
            1 => Self::ClientTimeOut,
            _ => Self::ClientActive,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for AgreeJoinWay {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AnimalCollectStateType {
    SlotCollected = 0,
    SlotRefreshed = 1,
}

impl Default for AnimalCollectStateType {
    fn default() -> Self {
        Self::SlotCollected
    }
}

impl crate::wire::PbEnum for AnimalCollectStateType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SlotCollected,
            1 => Self::SlotRefreshed,
            _ => Self::SlotCollected,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for AnimalCollectStateType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ApplyRechallengeReason {
    Settle = 0,
    Dead = 1,
}

impl Default for ApplyRechallengeReason {
    fn default() -> Self {
        Self::Settle
    }
}

impl crate::wire::PbEnum for ApplyRechallengeReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Settle,
            1 => Self::Dead,
            _ => Self::Settle,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ApplyRechallengeReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ArchiveType {
    NoneArchive = 0,
    NormalArchive = 1,
    OverdueVersion = 2,
    OverdueTime = 3,
    OverdueChangeSex = 4,
}

impl Default for ArchiveType {
    fn default() -> Self {
        Self::NoneArchive
    }
}

impl crate::wire::PbEnum for ArchiveType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NoneArchive,
            1 => Self::NormalArchive,
            2 => Self::OverdueVersion,
            3 => Self::OverdueTime,
            4 => Self::OverdueChangeSex,
            _ => Self::NoneArchive,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ArchiveType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AttachTargetType {
    AttachTargetNone = 0,
    AttachTargetEntity = 1,
    AttachTargetActorPath = 2,
}

impl Default for AttachTargetType {
    fn default() -> Self {
        Self::AttachTargetNone
    }
}

impl crate::wire::PbEnum for AttachTargetType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::AttachTargetNone,
            1 => Self::AttachTargetEntity,
            2 => Self::AttachTargetActorPath,
            _ => Self::AttachTargetNone,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for AttachTargetType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BanChatType {
    None = 0,
    All = 1,
    Friend = 2,
    Foreign = 3,
    Team = 4,
}

impl Default for BanChatType {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for BanChatType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::All,
            2 => Self::Friend,
            3 => Self::Foreign,
            4 => Self::Team,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BanChatType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BanItemUseType {
    BanItemUseNone = 0,
    BanItemUseAll = 1,
    BanItemUseZither = 2,
}

impl Default for BanItemUseType {
    fn default() -> Self {
        Self::BanItemUseNone
    }
}

impl crate::wire::PbEnum for BanItemUseType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BanItemUseNone,
            1 => Self::BanItemUseAll,
            2 => Self::BanItemUseZither,
            _ => Self::BanItemUseNone,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BanItemUseType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BathCameraType {
    Third = 0,
    First = 1,
}

impl Default for BathCameraType {
    fn default() -> Self {
        Self::Third
    }
}

impl crate::wire::PbEnum for BathCameraType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Third,
            1 => Self::First,
            _ => Self::Third,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BathCameraType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BattleAreaType {
    Retrieve = 0,
    Area = 1,
}

impl Default for BattleAreaType {
    fn default() -> Self {
        Self::Retrieve
    }
}

impl crate::wire::PbEnum for BattleAreaType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Retrieve,
            1 => Self::Area,
            _ => Self::Retrieve,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BattleAreaType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BossRushRewardType {
    Level = 0,
    Score = 1,
}

impl Default for BossRushRewardType {
    fn default() -> Self {
        Self::Level
    }
}

impl crate::wire::PbEnum for BossRushRewardType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Level,
            1 => Self::Score,
            _ => Self::Level,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BossRushRewardType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BuffOperateType {
    AddBuff = 0,
    RemoveBuff = 1,
    UndoBuff = 2,
}

impl Default for BuffOperateType {
    fn default() -> Self {
        Self::AddBuff
    }
}

impl crate::wire::PbEnum for BuffOperateType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::AddBuff,
            1 => Self::RemoveBuff,
            2 => Self::UndoBuff,
            _ => Self::AddBuff,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for BuffOperateType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ChangeReason {
    None = 0,
    QuestFix = 1,
    QuestFinish = 2,
    ActionFinish = 3,
    ActionRevert = 4,
    ConditionFinish = 5,
    Gm = 6,
    SpeakerChange = 7,
}

impl Default for ChangeReason {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for ChangeReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::QuestFix,
            2 => Self::QuestFinish,
            3 => Self::ActionFinish,
            4 => Self::ActionRevert,
            5 => Self::ConditionFinish,
            6 => Self::Gm,
            7 => Self::SpeakerChange,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ChangeReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ChatChannelNoticeType {
    None = 0,
    EnterTeam = 1,
    ExitTeam = 2,
    ClearMessages = 3,
}

impl Default for ChatChannelNoticeType {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for ChatChannelNoticeType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::EnterTeam,
            2 => Self::ExitTeam,
            3 => Self::ClearMessages,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ChatChannelNoticeType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CircumFluenceRewardType {
    TaskReward = 0,
    SignReward = 2,
    ScoreReward = 3,
}

impl Default for CircumFluenceRewardType {
    fn default() -> Self {
        Self::TaskReward
    }
}

impl crate::wire::PbEnum for CircumFluenceRewardType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TaskReward,
            2 => Self::SignReward,
            3 => Self::ScoreReward,
            _ => Self::TaskReward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for CircumFluenceRewardType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ClientStepState {
    NoStep = 0,
    SpecialEffect = 1,
    EventPloting = 2,
}

impl Default for ClientStepState {
    fn default() -> Self {
        Self::NoStep
    }
}

impl crate::wire::PbEnum for ClientStepState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NoStep,
            1 => Self::SpecialEffect,
            2 => Self::EventPloting,
            _ => Self::NoStep,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ClientStepState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DangoBrawlBagOpType {
    NormalChange = 0,
    ToBench = 1,
    BenchToNormal = 2,
}

impl Default for DangoBrawlBagOpType {
    fn default() -> Self {
        Self::NormalChange
    }
}

impl crate::wire::PbEnum for DangoBrawlBagOpType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NormalChange,
            1 => Self::ToBench,
            2 => Self::BenchToNormal,
            _ => Self::NormalChange,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DangoBrawlBagOpType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DangoBrawlDamageType {
    DangoBrawlDamageWeapon = 0,
    DangoBrawlDamageEffect = 1,
}

impl Default for DangoBrawlDamageType {
    fn default() -> Self {
        Self::DangoBrawlDamageWeapon
    }
}

impl crate::wire::PbEnum for DangoBrawlDamageType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::DangoBrawlDamageWeapon,
            1 => Self::DangoBrawlDamageEffect,
            _ => Self::DangoBrawlDamageWeapon,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DangoBrawlDamageType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DangoBrawlFightCamp {
    Player = 0,
    Opponent = 1,
}

impl Default for DangoBrawlFightCamp {
    fn default() -> Self {
        Self::Player
    }
}

impl crate::wire::PbEnum for DangoBrawlFightCamp {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Player,
            1 => Self::Opponent,
            _ => Self::Player,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DangoBrawlFightCamp {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DangoBrawlPhase {
    EventSelect = 0,
    Prepare = 1,
    Fighting = 2,
    FightRestore = 3,
}

impl Default for DangoBrawlPhase {
    fn default() -> Self {
        Self::EventSelect
    }
}

impl crate::wire::PbEnum for DangoBrawlPhase {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::EventSelect,
            1 => Self::Prepare,
            2 => Self::Fighting,
            3 => Self::FightRestore,
            _ => Self::EventSelect,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DangoBrawlPhase {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DetectionTargetChangeReason {
    Kill = 0,
    Visible = 1,
}

impl Default for DetectionTargetChangeReason {
    fn default() -> Self {
        Self::Kill
    }
}

impl crate::wire::PbEnum for DetectionTargetChangeReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Kill,
            1 => Self::Visible,
            _ => Self::Kill,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for DetectionTargetChangeReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EControlState {
    LetGo = 0,
    FreeThrowing = 1,
    LockBaseThrowing = 2,
    LockEntityThrowing = 3,
}

impl Default for EControlState {
    fn default() -> Self {
        Self::LetGo
    }
}

impl crate::wire::PbEnum for EControlState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::LetGo,
            1 => Self::FreeThrowing,
            2 => Self::LockBaseThrowing,
            3 => Self::LockEntityThrowing,
            _ => Self::LetGo,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EControlState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ELevelPlaySportMode {
    None = 0,
    Ski = 1,
    WingSuitFly = 2,
    RailSlide = 3,
}

impl Default for ELevelPlaySportMode {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for ELevelPlaySportMode {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Ski,
            2 => Self::WingSuitFly,
            3 => Self::RailSlide,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ELevelPlaySportMode {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ELinkExitReason {
    LinkExitNormal = 0,
    LinkExitTimeout = 1,
}

impl Default for ELinkExitReason {
    fn default() -> Self {
        Self::LinkExitNormal
    }
}

impl crate::wire::PbEnum for ELinkExitReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::LinkExitNormal,
            1 => Self::LinkExitTimeout,
            _ => Self::LinkExitNormal,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ELinkExitReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EMatchAcceptInviteResult {
    Accept = 0,
    ActiveRefuse = 1,
    TimeOutRefuse = 2,
}

impl Default for EMatchAcceptInviteResult {
    fn default() -> Self {
        Self::Accept
    }
}

impl crate::wire::PbEnum for EMatchAcceptInviteResult {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Accept,
            1 => Self::ActiveRefuse,
            2 => Self::TimeOutRefuse,
            _ => Self::Accept,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EMatchAcceptInviteResult {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EMatchPlayerUiState {
    Wait = 0,
    Confirm = 2,
    Matching = 3,
    Ready = 4,
    Selecting = 5,
    Teleporting = 6,
    Max = 7,
}

impl Default for EMatchPlayerUiState {
    fn default() -> Self {
        Self::Wait
    }
}

impl crate::wire::PbEnum for EMatchPlayerUiState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Wait,
            2 => Self::Confirm,
            3 => Self::Matching,
            4 => Self::Ready,
            5 => Self::Selecting,
            6 => Self::Teleporting,
            7 => Self::Max,
            _ => Self::Wait,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EMatchPlayerUiState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ENetPingState {
    Unknown = 0,
    Great = 1,
    Good = 2,
    Poor = 3,
    Max = 4,
}

impl Default for ENetPingState {
    fn default() -> Self {
        Self::Unknown
    }
}

impl crate::wire::PbEnum for ENetPingState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Great,
            2 => Self::Good,
            3 => Self::Poor,
            4 => Self::Max,
            _ => Self::Unknown,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ENetPingState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EPlayerTeleportState {
    Default = 0,
    Teleporting = 1,
    TeleportTimeOut = 2,
}

impl Default for EPlayerTeleportState {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for EPlayerTeleportState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Teleporting,
            2 => Self::TeleportTimeOut,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EPlayerTeleportState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ERechallengeResultResult {
    Accept = 0,
    ActiveRefuse = 1,
    TimeOutRefuse = 2,
}

impl Default for ERechallengeResultResult {
    fn default() -> Self {
        Self::Accept
    }
}

impl crate::wire::PbEnum for ERechallengeResultResult {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Accept,
            1 => Self::ActiveRefuse,
            2 => Self::TimeOutRefuse,
            _ => Self::Accept,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ERechallengeResultResult {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ESceneBlockSplitPopWindowType {
    Map = 0,
    Entity = 1,
}

impl Default for ESceneBlockSplitPopWindowType {
    fn default() -> Self {
        Self::Map
    }
}

impl crate::wire::PbEnum for ESceneBlockSplitPopWindowType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Map,
            1 => Self::Entity,
            _ => Self::Map,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ESceneBlockSplitPopWindowType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ESceneBlockSwithState {
    BstateAll = 0,
    BstateSimple = 1,
    BstateComplete = 2,
}

impl Default for ESceneBlockSwithState {
    fn default() -> Self {
        Self::BstateAll
    }
}

impl crate::wire::PbEnum for ESceneBlockSwithState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BstateAll,
            1 => Self::BstateSimple,
            2 => Self::BstateComplete,
            _ => Self::BstateAll,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ESceneBlockSwithState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ESceneRolesGroup {
    GroupNone = 0,
    Battle = 1,
    Phantom = 2,
    Plot = 3,
    Moto = 4,
    PianoPlaying = 5,
    SelfPortrait = 6,
    RoleGroupMax = 7,
}

impl Default for ESceneRolesGroup {
    fn default() -> Self {
        Self::GroupNone
    }
}

impl crate::wire::PbEnum for ESceneRolesGroup {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GroupNone,
            1 => Self::Battle,
            2 => Self::Phantom,
            3 => Self::Plot,
            4 => Self::Moto,
            5 => Self::PianoPlaying,
            6 => Self::SelfPortrait,
            7 => Self::RoleGroupMax,
            _ => Self::GroupNone,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ESceneRolesGroup {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ElevatorState {
    End = 0,
    Forward = 1,
    Reverse = 2,
}

impl Default for ElevatorState {
    fn default() -> Self {
        Self::End
    }
}

impl crate::wire::PbEnum for ElevatorState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::End,
            1 => Self::Forward,
            2 => Self::Reverse,
            _ => Self::End,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ElevatorState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EnableFunctionType {
    Invalid = 0,
    TeleportDungeon = 1,
    MotorRadioUi = 2,
    IinteractiveItems = 3,
}

impl Default for EnableFunctionType {
    fn default() -> Self {
        Self::Invalid
    }
}

impl crate::wire::PbEnum for EnableFunctionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Invalid,
            1 => Self::TeleportDungeon,
            2 => Self::MotorRadioUi,
            3 => Self::IinteractiveItems,
            _ => Self::Invalid,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EnableFunctionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EntityBeamReceiveType {
    BeginAction = 0,
    CompleteAction = 1,
    StopAction = 2,
}

impl Default for EntityBeamReceiveType {
    fn default() -> Self {
        Self::BeginAction
    }
}

impl crate::wire::PbEnum for EntityBeamReceiveType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BeginAction,
            1 => Self::CompleteAction,
            2 => Self::StopAction,
            _ => Self::BeginAction,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EntityBeamReceiveType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EntranceState {
    NotUnlock = 0,
    Unlockable = 1,
    Unlocked = 2,
    Closed = 3,
}

impl Default for EntranceState {
    fn default() -> Self {
        Self::NotUnlock
    }
}

impl crate::wire::PbEnum for EntranceState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotUnlock,
            1 => Self::Unlockable,
            2 => Self::Unlocked,
            3 => Self::Closed,
            _ => Self::NotUnlock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EntranceState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FRBuffOperate {
    BuffOpAdd = 0,
    BuffOpRemove = 1,
    BuffOpUpdate = 2,
}

impl Default for FRBuffOperate {
    fn default() -> Self {
        Self::BuffOpAdd
    }
}

impl crate::wire::PbEnum for FRBuffOperate {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BuffOpAdd,
            1 => Self::BuffOpRemove,
            2 => Self::BuffOpUpdate,
            _ => Self::BuffOpAdd,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FRBuffOperate {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FRUnitOperate {
    UnitOpAdd = 0,
    UnitOpRemove = 1,
    UnitOpUpdate = 2,
    UnitReplace = 3,
}

impl Default for FRUnitOperate {
    fn default() -> Self {
        Self::UnitOpAdd
    }
}

impl crate::wire::PbEnum for FRUnitOperate {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::UnitOpAdd,
            1 => Self::UnitOpRemove,
            2 => Self::UnitOpUpdate,
            3 => Self::UnitReplace,
            _ => Self::UnitOpAdd,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FRUnitOperate {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FRUnitResourcesChangeType {
    UnitBaseSalary = 0,
    UnitCoin = 1,
    UnitDiamond = 2,
}

impl Default for FRUnitResourcesChangeType {
    fn default() -> Self {
        Self::UnitBaseSalary
    }
}

impl crate::wire::PbEnum for FRUnitResourcesChangeType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::UnitBaseSalary,
            1 => Self::UnitCoin,
            2 => Self::UnitDiamond,
            _ => Self::UnitBaseSalary,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FRUnitResourcesChangeType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FailReason {
    DefaultFail = 0,
    AutoFail = 1,
    EntityStateFail = 2,
    TimeoutFail = 3,
    LeaveInstFail = 4,
    InstFail = 5,
    CharacterDieFail = 6,
    EntityDieFail = 7,
    OutRangeFail = 8,
    GiveUpFail = 9,
    SneakTimeoutFail = 10,
    TransferFail = 11,
    PlayerMotionState = 12,
    NpcTraceOutRange = 13,
    NpcTraceAlertFull = 14,
    DataLayerFail = 15,
    PlayerLevelPlayMoveStateFail = 16,
    CompareVarFail = 17,
    TimeRange = 18,
    ClearCabinFail = 19,
    YuXueModeChange = 20,
}

impl Default for FailReason {
    fn default() -> Self {
        Self::DefaultFail
    }
}

impl crate::wire::PbEnum for FailReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::DefaultFail,
            1 => Self::AutoFail,
            2 => Self::EntityStateFail,
            3 => Self::TimeoutFail,
            4 => Self::LeaveInstFail,
            5 => Self::InstFail,
            6 => Self::CharacterDieFail,
            7 => Self::EntityDieFail,
            8 => Self::OutRangeFail,
            9 => Self::GiveUpFail,
            10 => Self::SneakTimeoutFail,
            11 => Self::TransferFail,
            12 => Self::PlayerMotionState,
            13 => Self::NpcTraceOutRange,
            14 => Self::NpcTraceAlertFull,
            15 => Self::DataLayerFail,
            16 => Self::PlayerLevelPlayMoveStateFail,
            17 => Self::CompareVarFail,
            18 => Self::TimeRange,
            19 => Self::ClearCabinFail,
            20 => Self::YuXueModeChange,
            _ => Self::DefaultFail,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FailReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FavorItemType {
    Word = 0,
    Story = 1,
    Goods = 2,
}

impl Default for FavorItemType {
    fn default() -> Self {
        Self::Word
    }
}

impl crate::wire::PbEnum for FavorItemType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Word,
            1 => Self::Story,
            2 => Self::Goods,
            _ => Self::Word,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FavorItemType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FavorQuestType {
    Talk = 0,
    Favor = 1,
}

impl Default for FavorQuestType {
    fn default() -> Self {
        Self::Talk
    }
}

impl crate::wire::PbEnum for FavorQuestType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Talk,
            1 => Self::Favor,
            _ => Self::Talk,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FavorQuestType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FloroConfigType {
    FloroCard = 0,
    FloroRace = 1,
    FloroTask = 2,
    FloroSkill = 3,
    FloroToy = 4,
    FloroIns = 5,
    FloroSubIns = 6,
    SubInsHistory = 7,
}

impl Default for FloroConfigType {
    fn default() -> Self {
        Self::FloroCard
    }
}

impl crate::wire::PbEnum for FloroConfigType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::FloroCard,
            1 => Self::FloroRace,
            2 => Self::FloroTask,
            3 => Self::FloroSkill,
            4 => Self::FloroToy,
            5 => Self::FloroIns,
            6 => Self::FloroSubIns,
            7 => Self::SubInsHistory,
            _ => Self::FloroCard,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FloroConfigType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FloroRanchActionUnitType {
    Terrain = 0,
    Phantom = 1,
    Toy = 2,
    Dungeon = 3,
    Role = 4,
}

impl Default for FloroRanchActionUnitType {
    fn default() -> Self {
        Self::Terrain
    }
}

impl crate::wire::PbEnum for FloroRanchActionUnitType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Terrain,
            1 => Self::Phantom,
            2 => Self::Toy,
            3 => Self::Dungeon,
            4 => Self::Role,
            _ => Self::Terrain,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FloroRanchActionUnitType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FloroRanchPlayResourcesType {
    Coin = 0,
    Diamond = 1,
}

impl Default for FloroRanchPlayResourcesType {
    fn default() -> Self {
        Self::Coin
    }
}

impl crate::wire::PbEnum for FloroRanchPlayResourcesType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Coin,
            1 => Self::Diamond,
            _ => Self::Coin,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FloroRanchPlayResourcesType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FloroRanchPlayShopBuyType {
    BuyCard = 0,
    BuyGardGroup = 1,
    BuyToy = 2,
}

impl Default for FloroRanchPlayShopBuyType {
    fn default() -> Self {
        Self::BuyCard
    }
}

impl crate::wire::PbEnum for FloroRanchPlayShopBuyType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BuyCard,
            1 => Self::BuyGardGroup,
            2 => Self::BuyToy,
            _ => Self::BuyCard,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FloroRanchPlayShopBuyType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FloroRanchPlayTaskType {
    DayStart = 0,
    DaySettleActions = 1,
    Event = 2,
    Shop = 3,
    Gacha = 4,
    StageStart = 5,
    StageEnd = 6,
    WageSettleTask = 7,
}

impl Default for FloroRanchPlayTaskType {
    fn default() -> Self {
        Self::DayStart
    }
}

impl crate::wire::PbEnum for FloroRanchPlayTaskType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::DayStart,
            1 => Self::DaySettleActions,
            2 => Self::Event,
            3 => Self::Shop,
            4 => Self::Gacha,
            5 => Self::StageStart,
            6 => Self::StageEnd,
            7 => Self::WageSettleTask,
            _ => Self::DayStart,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FloroRanchPlayTaskType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FloroRanchShopItemType {
    Card = 0,
    CardGroup = 1,
    ShopToy = 2,
}

impl Default for FloroRanchShopItemType {
    fn default() -> Self {
        Self::Card
    }
}

impl crate::wire::PbEnum for FloroRanchShopItemType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Card,
            1 => Self::CardGroup,
            2 => Self::ShopToy,
            _ => Self::Card,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FloroRanchShopItemType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FloroRanchUnitActionType {
    OpBuff = 0,
    OpUnit = 1,
    UnitSettle = 2,
    UnitResourcesChange = 3,
    Eating = 4,
    EvolveUpdate = 5,
    Mix = 6,
    Sacrifice = 7,
    DebugActionInfo = 8,
    BeEat = 9,
    ChangePoint = 10,
    ActionStop = 11,
    ToyLvUp = 12,
    ChangeTag = 13,
    StageTributeChange = 14,
    SelfDefineValue = 15,
}

impl Default for FloroRanchUnitActionType {
    fn default() -> Self {
        Self::OpBuff
    }
}

impl crate::wire::PbEnum for FloroRanchUnitActionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::OpBuff,
            1 => Self::OpUnit,
            2 => Self::UnitSettle,
            3 => Self::UnitResourcesChange,
            4 => Self::Eating,
            5 => Self::EvolveUpdate,
            6 => Self::Mix,
            7 => Self::Sacrifice,
            8 => Self::DebugActionInfo,
            9 => Self::BeEat,
            10 => Self::ChangePoint,
            11 => Self::ActionStop,
            12 => Self::ToyLvUp,
            13 => Self::ChangeTag,
            14 => Self::StageTributeChange,
            15 => Self::SelfDefineValue,
            _ => Self::OpBuff,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FloroRanchUnitActionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FriendApplyOperator {
    Approve = 0,
    Reject = 1,
}

impl Default for FriendApplyOperator {
    fn default() -> Self {
        Self::Approve
    }
}

impl crate::wire::PbEnum for FriendApplyOperator {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Approve,
            1 => Self::Reject,
            _ => Self::Approve,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FriendApplyOperator {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FriendApplyWay {
    Search = 0,
    RecentlyTeam = 1,
}

impl Default for FriendApplyWay {
    fn default() -> Self {
        Self::Search
    }
}

impl crate::wire::PbEnum for FriendApplyWay {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Search,
            1 => Self::RecentlyTeam,
            _ => Self::Search,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for FriendApplyWay {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GainDataUpdateType {
    GainDataAdd = 0,
    GainDataDelete = 1,
    GainDataUpdate = 2,
}

impl Default for GainDataUpdateType {
    fn default() -> Self {
        Self::GainDataAdd
    }
}

impl crate::wire::PbEnum for GainDataUpdateType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GainDataAdd,
            1 => Self::GainDataDelete,
            2 => Self::GainDataUpdate,
            _ => Self::GainDataAdd,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GainDataUpdateType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GatherTaskState {
    Lock = 0,
    Running = 1,
    InComplete = 2,
    Done = 3,
    TakeReward = 4,
}

impl Default for GatherTaskState {
    fn default() -> Self {
        Self::Lock
    }
}

impl crate::wire::PbEnum for GatherTaskState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Lock,
            1 => Self::Running,
            2 => Self::InComplete,
            3 => Self::Done,
            4 => Self::TakeReward,
            _ => Self::Lock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GatherTaskState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GiantEcoNetStatus {
    Lock = 0,
    Noraml = 1,
    MaxLevel = 2,
    MaxStableValue = 3,
}

impl Default for GiantEcoNetStatus {
    fn default() -> Self {
        Self::Lock
    }
}

impl crate::wire::PbEnum for GiantEcoNetStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Lock,
            1 => Self::Noraml,
            2 => Self::MaxLevel,
            3 => Self::MaxStableValue,
            _ => Self::Lock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GiantEcoNetStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GiantEcoNetType {
    Fire = 0,
    Water = 1,
    Wood = 2,
}

impl Default for GiantEcoNetType {
    fn default() -> Self {
        Self::Fire
    }
}

impl crate::wire::PbEnum for GiantEcoNetType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Fire,
            1 => Self::Water,
            2 => Self::Wood,
            _ => Self::Fire,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GiantEcoNetType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GiantIdleNetType {
    Common = 0,
    Watch = 1,
    Akimbo = 2,
    Rest = 3,
}

impl Default for GiantIdleNetType {
    fn default() -> Self {
        Self::Common
    }
}

impl crate::wire::PbEnum for GiantIdleNetType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Common,
            1 => Self::Watch,
            2 => Self::Akimbo,
            3 => Self::Rest,
            _ => Self::Common,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GiantIdleNetType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GiantItemAccessStatus {
    AccessLock = 0,
    AccessNoraml = 1,
    AccessFinish = 2,
}

impl Default for GiantItemAccessStatus {
    fn default() -> Self {
        Self::AccessLock
    }
}

impl crate::wire::PbEnum for GiantItemAccessStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::AccessLock,
            1 => Self::AccessNoraml,
            2 => Self::AccessFinish,
            _ => Self::AccessLock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GiantItemAccessStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GiantResponseType {
    PlayAnimation = 0,
    PlayFlow = 1,
}

impl Default for GiantResponseType {
    fn default() -> Self {
        Self::PlayAnimation
    }
}

impl crate::wire::PbEnum for GiantResponseType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::PlayAnimation,
            1 => Self::PlayFlow,
            _ => Self::PlayAnimation,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GiantResponseType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GravityFlipType {
    GravityUp = 0,
    GravityDown = 1,
    GravityLeft = 2,
    GravityRight = 3,
}

impl Default for GravityFlipType {
    fn default() -> Self {
        Self::GravityUp
    }
}

impl crate::wire::PbEnum for GravityFlipType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GravityUp,
            1 => Self::GravityDown,
            2 => Self::GravityLeft,
            3 => Self::GravityRight,
            _ => Self::GravityUp,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GravityFlipType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GravityGearOperate {
    InActive = 0,
    Active = 1,
}

impl Default for GravityGearOperate {
    fn default() -> Self {
        Self::InActive
    }
}

impl crate::wire::PbEnum for GravityGearOperate {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::InActive,
            1 => Self::Active,
            _ => Self::InActive,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GravityGearOperate {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GridDataUpdateReason {
    GridUpdate = 0,
    ByMoveOp = 1,
    Teleport = 2,
    LightBlockByLocation = 4,
}

impl Default for GridDataUpdateReason {
    fn default() -> Self {
        Self::GridUpdate
    }
}

impl crate::wire::PbEnum for GridDataUpdateReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GridUpdate,
            1 => Self::ByMoveOp,
            2 => Self::Teleport,
            4 => Self::LightBlockByLocation,
            _ => Self::GridUpdate,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GridDataUpdateReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GridEventState {
    UnTrigger = 0,
    PreBeginPloting = 1,
    BeginPloting = 2,
    PreBeginPlotEffecting = 3,
    BeginPlotEffecting = 4,
    PreEndPloting = 5,
    EndPloting = 6,
    PreEndPlotEffecting = 7,
    EndPlotEffecting = 8,
    PreTakeGrid = 9,
    TakeGrid = 10,
    PreFinishAward = 11,
    FinishAward = 12,
    End = 13,
}

impl Default for GridEventState {
    fn default() -> Self {
        Self::UnTrigger
    }
}

impl crate::wire::PbEnum for GridEventState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::UnTrigger,
            1 => Self::PreBeginPloting,
            2 => Self::BeginPloting,
            3 => Self::PreBeginPlotEffecting,
            4 => Self::BeginPlotEffecting,
            5 => Self::PreEndPloting,
            6 => Self::EndPloting,
            7 => Self::PreEndPlotEffecting,
            8 => Self::EndPlotEffecting,
            9 => Self::PreTakeGrid,
            10 => Self::TakeGrid,
            11 => Self::PreFinishAward,
            12 => Self::FinishAward,
            13 => Self::End,
            _ => Self::UnTrigger,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for GridEventState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum HookLockPointExitWay {
    Midway = 0,
    Endpoint = 1,
}

impl Default for HookLockPointExitWay {
    fn default() -> Self {
        Self::Midway
    }
}

impl crate::wire::PbEnum for HookLockPointExitWay {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Midway,
            1 => Self::Endpoint,
            _ => Self::Midway,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for HookLockPointExitWay {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ItemRelateFoundationReason {
    Normal = 0,
    Init = 1,
}

impl Default for ItemRelateFoundationReason {
    fn default() -> Self {
        Self::Normal
    }
}

impl crate::wire::PbEnum for ItemRelateFoundationReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Init,
            _ => Self::Normal,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ItemRelateFoundationReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum JokerActionType {
    EnterNewRound = 0,
    DrawCard = 1,
    PlayCard = 2,
    TriggerSkill = 3,
    ExecSkill = 4,
    SettleResult = 5,
    HealthHpRefresh = 6,
    DisPlayCard = 7,
}

impl Default for JokerActionType {
    fn default() -> Self {
        Self::EnterNewRound
    }
}

impl crate::wire::PbEnum for JokerActionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::EnterNewRound,
            1 => Self::DrawCard,
            2 => Self::PlayCard,
            3 => Self::TriggerSkill,
            4 => Self::ExecSkill,
            5 => Self::SettleResult,
            6 => Self::HealthHpRefresh,
            7 => Self::DisPlayCard,
            _ => Self::EnterNewRound,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for JokerActionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum JokerGuessActor {
    Who = 0,
    Ai = 1,
}

impl Default for JokerGuessActor {
    fn default() -> Self {
        Self::Who
    }
}

impl crate::wire::PbEnum for JokerGuessActor {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Who,
            1 => Self::Ai,
            _ => Self::Who,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for JokerGuessActor {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum JokerGuessSkillOp {
    Use = 0,
    GiveUp = 1,
}

impl Default for JokerGuessSkillOp {
    fn default() -> Self {
        Self::Use
    }
}

impl crate::wire::PbEnum for JokerGuessSkillOp {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Use,
            1 => Self::GiveUp,
            _ => Self::Use,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for JokerGuessSkillOp {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum KurotatoNextWaveType {
    Default = 0,
    Special = 1,
    Infinite = 2,
}

impl Default for KurotatoNextWaveType {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for KurotatoNextWaveType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Special,
            2 => Self::Infinite,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for KurotatoNextWaveType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum KurotatoRoleSaveState {
    Undefine = 0,
    NotSaved = 1,
    ClientSaved = 2,
    ServerSaved = 3,
}

impl Default for KurotatoRoleSaveState {
    fn default() -> Self {
        Self::Undefine
    }
}

impl crate::wire::PbEnum for KurotatoRoleSaveState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Undefine,
            1 => Self::NotSaved,
            2 => Self::ClientSaved,
            3 => Self::ServerSaved,
            _ => Self::Undefine,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for KurotatoRoleSaveState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LiftMoveType {
    Default = 0,
    Teleport = 1,
    CustomSpeed = 2,
}

impl Default for LiftMoveType {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for LiftMoveType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Teleport,
            2 => Self::CustomSpeed,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for LiftMoveType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LinkageCheckInType {
    NormalCheckIn = 0,
    KeepCheckIn = 1,
}

impl Default for LinkageCheckInType {
    fn default() -> Self {
        Self::NormalCheckIn
    }
}

impl crate::wire::PbEnum for LinkageCheckInType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NormalCheckIn,
            1 => Self::KeepCheckIn,
            _ => Self::NormalCheckIn,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for LinkageCheckInType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MailAddReason {
    Gm = 0,
    BagFull = 1,
    Public = 2,
    OfflineEventAdd = 3,
    QuestNpcAction = 4,
    DailyQuestReward = 5,
    ReportPlayer = 6,
    MonthCardRemind = 7,
    BattlePassSettle = 8,
    PayRebate = 9,
    RemoveTemporaryTeleportBySceneDataLayerChange = 10,
    CdKey = 11,
    PayRefund = 12,
    Replenishment = 13,
    GatherActivitySettle = 14,
    HarvestActivitySettle = 15,
    RoleTrialActivitySettle = 16,
    KuroBbsFirstBind = 17,
    ParkourActivitySettle = 18,
    DailyAdventureActivitySettle = 19,
    LongShanActivitySettle = 20,
    SignActivitySettle = 21,
    BossRushActivitySettle = 22,
    PromotionActivity = 23,
    RogueActivitySettle = 24,
    TowerDefenceActivitySettle = 25,
    ExploreToolUnlock = 26,
    CalabashFixLevelExp = 27,
    ActivityNotice = 28,
    TrackMoonPhaseSettle = 29,
    TrackMoonSettle = 30,
    CircumFluenceSettle = 31,
    CorniceMeetingActivitySettle = 32,
    RiskHarvestSettle = 33,
    BlackCoastThemeSettle = 34,
    PreheatSignSettle = 35,
    MowTower = 36,
    RogueWhiteCatLimitRewardReissue = 37,
    SpecialItemUnlock = 38,
    SpringSignSettle = 39,
    RoleSkinTrialActivitySettle = 40,
    FishingActivityLimitSettleReward = 41,
    TeamParkOurSettleReward = 42,
    RogueWeeklyCycleAward = 43,
    BabelTowerDailyTaskResetReward = 44,
    BabelTowerCloseReward = 45,
    ExploreActivitySettle = 46,
    AvignonMailSettle = 47,
    CircumFluenceCloseNotice = 48,
    RogueResTaskAwardRecoup = 49,
    CiacconaActivitySettle = 50,
    DangoMonopolyGridReward = 51,
    DangoMonopolyBoardReward = 52,
    AbyssActivityRewardSettle = 53,
    RacingBetsActivitySettle = 54,
    ConsumptiveActivitySettle = 55,
    ConsumptiveShopSettle = 56,
    SlashAndTowerSettle = 57,
    TowerMailSettle = 58,
    BirthDaySettle = 59,
    RacingBetsActivityEnd = 60,
    NewbieCarnivalReissueAward = 61,
    PhantomBattleLimitSettleReward = 62,
    FloroMailSettle = 63,
    FixQuestTreasureBox = 64,
    TrapDefenseActivitySettle = 65,
    FunPlayActivityClose = 66,
    JinZhouFlySend = 67,
    PhotoFightActivityClose = 68,
    SurvivorsTaskReward = 69,
    SurvivorsScoreReward = 70,
    HonamiStoryLimitSettleReward = 71,
    RegressActivityDelayOpen = 72,
    ArtemisActivityReward = 73,
    MotorcycleIpLink = 74,
    InfrThemeActivityClose = 75,
    NewTowerScoreReward = 76,
    MotorLimitSettleReward = 77,
    NewPlayerSupportActTaskRewardReissue = 78,
    MotorParkourActivityReward = 79,
    TotalTopUpSettleReward = 80,
    MotorFightActivityClose = 81,
    WeekCardRewardSettle = 82,
    FarmGoldActivityRewardReissue = 83,
    SpringFestival31MailSettle = 84,
    MotorDevelopActivitySettle = 85,
    PhantomVicePolishNotice = 86,
    FlagChallengeTaskRewardSettle = 87,
    TotalTopUpReplenishment = 88,
    FeiXuePreheatRewardSettle = 89,
    RhythmLimitTaskReissue = 90,
    DropCatchActivityReward = 91,
    NewTowerSeasonAward = 92,
    NewTowerSeasonScoreAward = 93,
    BossPilingTaskRewardSettle = 94,
    ThemeCelebration = 95,
    ThemeCelebrationWuWu = 96,
    TotalTopUpCloseReplenishment = 97,
    MotorDecalActivity = 98,
    PinballMailSettle = 99,
    LinkageCheckInSettle = 100,
    RoleGiftSettle = 101,
    KurotatoLimitRewardReissue = 102,
    KurotatoScoreRewardReissue = 103,
    KurotatoResRewardReissue = 104,
    GachaAccumulateRewardSettle = 105,
    OnlineMotorSettle = 106,
    EdgeRunnerActivityMailSettle = 107,
    WeeklyFrameScoreRewardReissue = 108,
    XboxGift = 109,
    DangoBrawlActivityMailSettle = 110,
    RoverRogueTaskRewardSettle = 111,
    QingXiaoActivityMailSettle = 112,
    TimePointRewardActivityMailSettle = 113,
    SolarSchoolLimitTaskReissue = 114,
    RoleTowerLimitRewardReissue = 116,
    RoleTowerScoreRewardReissue = 117,
}

impl Default for MailAddReason {
    fn default() -> Self {
        Self::Gm
    }
}

impl crate::wire::PbEnum for MailAddReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Gm,
            1 => Self::BagFull,
            2 => Self::Public,
            3 => Self::OfflineEventAdd,
            4 => Self::QuestNpcAction,
            5 => Self::DailyQuestReward,
            6 => Self::ReportPlayer,
            7 => Self::MonthCardRemind,
            8 => Self::BattlePassSettle,
            9 => Self::PayRebate,
            10 => Self::RemoveTemporaryTeleportBySceneDataLayerChange,
            11 => Self::CdKey,
            12 => Self::PayRefund,
            13 => Self::Replenishment,
            14 => Self::GatherActivitySettle,
            15 => Self::HarvestActivitySettle,
            16 => Self::RoleTrialActivitySettle,
            17 => Self::KuroBbsFirstBind,
            18 => Self::ParkourActivitySettle,
            19 => Self::DailyAdventureActivitySettle,
            20 => Self::LongShanActivitySettle,
            21 => Self::SignActivitySettle,
            22 => Self::BossRushActivitySettle,
            23 => Self::PromotionActivity,
            24 => Self::RogueActivitySettle,
            25 => Self::TowerDefenceActivitySettle,
            26 => Self::ExploreToolUnlock,
            27 => Self::CalabashFixLevelExp,
            28 => Self::ActivityNotice,
            29 => Self::TrackMoonPhaseSettle,
            30 => Self::TrackMoonSettle,
            31 => Self::CircumFluenceSettle,
            32 => Self::CorniceMeetingActivitySettle,
            33 => Self::RiskHarvestSettle,
            34 => Self::BlackCoastThemeSettle,
            35 => Self::PreheatSignSettle,
            36 => Self::MowTower,
            37 => Self::RogueWhiteCatLimitRewardReissue,
            38 => Self::SpecialItemUnlock,
            39 => Self::SpringSignSettle,
            40 => Self::RoleSkinTrialActivitySettle,
            41 => Self::FishingActivityLimitSettleReward,
            42 => Self::TeamParkOurSettleReward,
            43 => Self::RogueWeeklyCycleAward,
            44 => Self::BabelTowerDailyTaskResetReward,
            45 => Self::BabelTowerCloseReward,
            46 => Self::ExploreActivitySettle,
            47 => Self::AvignonMailSettle,
            48 => Self::CircumFluenceCloseNotice,
            49 => Self::RogueResTaskAwardRecoup,
            50 => Self::CiacconaActivitySettle,
            51 => Self::DangoMonopolyGridReward,
            52 => Self::DangoMonopolyBoardReward,
            53 => Self::AbyssActivityRewardSettle,
            54 => Self::RacingBetsActivitySettle,
            55 => Self::ConsumptiveActivitySettle,
            56 => Self::ConsumptiveShopSettle,
            57 => Self::SlashAndTowerSettle,
            58 => Self::TowerMailSettle,
            59 => Self::BirthDaySettle,
            60 => Self::RacingBetsActivityEnd,
            61 => Self::NewbieCarnivalReissueAward,
            62 => Self::PhantomBattleLimitSettleReward,
            63 => Self::FloroMailSettle,
            64 => Self::FixQuestTreasureBox,
            65 => Self::TrapDefenseActivitySettle,
            66 => Self::FunPlayActivityClose,
            67 => Self::JinZhouFlySend,
            68 => Self::PhotoFightActivityClose,
            69 => Self::SurvivorsTaskReward,
            70 => Self::SurvivorsScoreReward,
            71 => Self::HonamiStoryLimitSettleReward,
            72 => Self::RegressActivityDelayOpen,
            73 => Self::ArtemisActivityReward,
            74 => Self::MotorcycleIpLink,
            75 => Self::InfrThemeActivityClose,
            76 => Self::NewTowerScoreReward,
            77 => Self::MotorLimitSettleReward,
            78 => Self::NewPlayerSupportActTaskRewardReissue,
            79 => Self::MotorParkourActivityReward,
            80 => Self::TotalTopUpSettleReward,
            81 => Self::MotorFightActivityClose,
            82 => Self::WeekCardRewardSettle,
            83 => Self::FarmGoldActivityRewardReissue,
            84 => Self::SpringFestival31MailSettle,
            85 => Self::MotorDevelopActivitySettle,
            86 => Self::PhantomVicePolishNotice,
            87 => Self::FlagChallengeTaskRewardSettle,
            88 => Self::TotalTopUpReplenishment,
            89 => Self::FeiXuePreheatRewardSettle,
            90 => Self::RhythmLimitTaskReissue,
            91 => Self::DropCatchActivityReward,
            92 => Self::NewTowerSeasonAward,
            93 => Self::NewTowerSeasonScoreAward,
            94 => Self::BossPilingTaskRewardSettle,
            95 => Self::ThemeCelebration,
            96 => Self::ThemeCelebrationWuWu,
            97 => Self::TotalTopUpCloseReplenishment,
            98 => Self::MotorDecalActivity,
            99 => Self::PinballMailSettle,
            100 => Self::LinkageCheckInSettle,
            101 => Self::RoleGiftSettle,
            102 => Self::KurotatoLimitRewardReissue,
            103 => Self::KurotatoScoreRewardReissue,
            104 => Self::KurotatoResRewardReissue,
            105 => Self::GachaAccumulateRewardSettle,
            106 => Self::OnlineMotorSettle,
            107 => Self::EdgeRunnerActivityMailSettle,
            108 => Self::WeeklyFrameScoreRewardReissue,
            109 => Self::XboxGift,
            110 => Self::DangoBrawlActivityMailSettle,
            111 => Self::RoverRogueTaskRewardSettle,
            112 => Self::QingXiaoActivityMailSettle,
            113 => Self::TimePointRewardActivityMailSettle,
            114 => Self::SolarSchoolLimitTaskReissue,
            116 => Self::RoleTowerLimitRewardReissue,
            117 => Self::RoleTowerScoreRewardReissue,
            _ => Self::Gm,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MailAddReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MailDeleteReason {
    OverTime = 0,
    OverSize = 1,
    OfflineEvent = 2,
    PublicCancelled = 3,
    PlayerDelete = 4,
}

impl Default for MailDeleteReason {
    fn default() -> Self {
        Self::OverTime
    }
}

impl crate::wire::PbEnum for MailDeleteReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::OverTime,
            1 => Self::OverSize,
            2 => Self::OfflineEvent,
            3 => Self::PublicCancelled,
            4 => Self::PlayerDelete,
            _ => Self::OverTime,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MailDeleteReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MarkState {
    MarkNormal = 0,
    MarkDisable = 1,
    MarkComplete = 2,
}

impl Default for MarkState {
    fn default() -> Self {
        Self::MarkNormal
    }
}

impl crate::wire::PbEnum for MarkState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::MarkNormal,
            1 => Self::MarkDisable,
            2 => Self::MarkComplete,
            _ => Self::MarkNormal,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MarkState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MatchFailReason {
    TimeOut = 0,
    NotAccept = 1,
    ServerError = 2,
}

impl Default for MatchFailReason {
    fn default() -> Self {
        Self::TimeOut
    }
}

impl crate::wire::PbEnum for MatchFailReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TimeOut,
            1 => Self::NotAccept,
            2 => Self::ServerError,
            _ => Self::TimeOut,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MatchFailReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MatchPlayerLeaveReason {
    NotConfirm = 0,
    Refuse = 1,
    Initiative = 2,
    BeKick = 3,
    HostLeave = 4,
    EnterInstance = 5,
    MatchServerDisable = 6,
    MatchGm = 7,
    EnterBigWorldInst = 8,
    EnableFunction = 9,
    DisabledOnlineNodeEvent = 10,
    DoCloseOnlineMatching = 11,
    PlayFlow = 12,
    MatchClientVersion = 13,
}

impl Default for MatchPlayerLeaveReason {
    fn default() -> Self {
        Self::NotConfirm
    }
}

impl crate::wire::PbEnum for MatchPlayerLeaveReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotConfirm,
            1 => Self::Refuse,
            2 => Self::Initiative,
            3 => Self::BeKick,
            4 => Self::HostLeave,
            5 => Self::EnterInstance,
            6 => Self::MatchServerDisable,
            7 => Self::MatchGm,
            8 => Self::EnterBigWorldInst,
            9 => Self::EnableFunction,
            10 => Self::DisabledOnlineNodeEvent,
            11 => Self::DoCloseOnlineMatching,
            12 => Self::PlayFlow,
            13 => Self::MatchClientVersion,
            _ => Self::NotConfirm,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MatchPlayerLeaveReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MatchTeamState {
    WaiteConfirm = 0,
    ReadyConfirm = 1,
    EnterInst = 2,
    EnterInstStart = 3,
}

impl Default for MatchTeamState {
    fn default() -> Self {
        Self::WaiteConfirm
    }
}

impl crate::wire::PbEnum for MatchTeamState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::WaiteConfirm,
            1 => Self::ReadyConfirm,
            2 => Self::EnterInst,
            3 => Self::EnterInstStart,
            _ => Self::WaiteConfirm,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MatchTeamState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MotorFightCollectionPos {
    First = 0,
    Second = 1,
    Third = 2,
}

impl Default for MotorFightCollectionPos {
    fn default() -> Self {
        Self::First
    }
}

impl crate::wire::PbEnum for MotorFightCollectionPos {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::First,
            1 => Self::Second,
            2 => Self::Third,
            _ => Self::First,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MotorFightCollectionPos {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MotorSliderInteractType {
    Enter = 0,
    NormalExit = 1,
}

impl Default for MotorSliderInteractType {
    fn default() -> Self {
        Self::Enter
    }
}

impl crate::wire::PbEnum for MotorSliderInteractType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Enter,
            1 => Self::NormalExit,
            _ => Self::Enter,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MotorSliderInteractType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MoveStatus {
    Stop = 0,
    Moving = 1,
    Interrupt = 2,
    Switch = 3,
}

impl Default for MoveStatus {
    fn default() -> Self {
        Self::Stop
    }
}

impl crate::wire::PbEnum for MoveStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Stop,
            1 => Self::Moving,
            2 => Self::Interrupt,
            3 => Self::Switch,
            _ => Self::Stop,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for MoveStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum NpcTimeScheduleSplineType {
    PathSpline = 0,
    AiSpline = 1,
}

impl Default for NpcTimeScheduleSplineType {
    fn default() -> Self {
        Self::PathSpline
    }
}

impl crate::wire::PbEnum for NpcTimeScheduleSplineType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::PathSpline,
            1 => Self::AiSpline,
            _ => Self::PathSpline,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for NpcTimeScheduleSplineType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum OnlineMotorRewardType {
    LevelReward = 0,
    GlobalReward = 1,
}

impl Default for OnlineMotorRewardType {
    fn default() -> Self {
        Self::LevelReward
    }
}

impl crate::wire::PbEnum for OnlineMotorRewardType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::LevelReward,
            1 => Self::GlobalReward,
            _ => Self::LevelReward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for OnlineMotorRewardType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PbAdviceVoteType {
    Up = 0,
    Cancel = 1,
}

impl Default for PbAdviceVoteType {
    fn default() -> Self {
        Self::Up
    }
}

impl crate::wire::PbEnum for PbAdviceVoteType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Up,
            1 => Self::Cancel,
            _ => Self::Up,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PbAdviceVoteType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PbHandInItemType {
    ItemIds = 0,
    ItemType = 1,
}

impl Default for PbHandInItemType {
    fn default() -> Self {
        Self::ItemIds
    }
}

impl crate::wire::PbEnum for PbHandInItemType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ItemIds,
            1 => Self::ItemType,
            _ => Self::ItemIds,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PbHandInItemType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhantomBatchOper {
    Lock = 0,
    Disuse = 1,
    Reset = 2,
}

impl Default for PhantomBatchOper {
    fn default() -> Self {
        Self::Lock
    }
}

impl crate::wire::PbEnum for PhantomBatchOper {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Lock,
            1 => Self::Disuse,
            2 => Self::Reset,
            _ => Self::Lock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhantomBatchOper {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhantomBattleBoardSettleType {
    CommonSettle = 0,
    RoundSettle = 1,
    LimitSettle = 2,
    Skip = 3,
}

impl Default for PhantomBattleBoardSettleType {
    fn default() -> Self {
        Self::CommonSettle
    }
}

impl crate::wire::PbEnum for PhantomBattleBoardSettleType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::CommonSettle,
            1 => Self::RoundSettle,
            2 => Self::LimitSettle,
            3 => Self::Skip,
            _ => Self::CommonSettle,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhantomBattleBoardSettleType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhantomBattleCardDurableDealType {
    BackHeap = 0,
    Destroy = 1,
}

impl Default for PhantomBattleCardDurableDealType {
    fn default() -> Self {
        Self::BackHeap
    }
}

impl crate::wire::PbEnum for PhantomBattleCardDurableDealType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::BackHeap,
            1 => Self::Destroy,
            _ => Self::BackHeap,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhantomBattleCardDurableDealType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhantomBattleEffectCardCamp {
    GamerFighterNpc = 0,
    GamerFighterPlayer = 1,
}

impl Default for PhantomBattleEffectCardCamp {
    fn default() -> Self {
        Self::GamerFighterNpc
    }
}

impl crate::wire::PbEnum for PhantomBattleEffectCardCamp {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GamerFighterNpc,
            1 => Self::GamerFighterPlayer,
            _ => Self::GamerFighterNpc,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhantomBattleEffectCardCamp {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhantomBattleSelectedCardFromType {
    HandCard = 0,
    Heap = 1,
}

impl Default for PhantomBattleSelectedCardFromType {
    fn default() -> Self {
        Self::HandCard
    }
}

impl crate::wire::PbEnum for PhantomBattleSelectedCardFromType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::HandCard,
            1 => Self::Heap,
            _ => Self::HandCard,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PhantomBattleSelectedCardFromType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PilotThrowOperationType {
    Ptothrow = 0,
    Ptocancel = 1,
}

impl Default for PilotThrowOperationType {
    fn default() -> Self {
        Self::Ptothrow
    }
}

impl crate::wire::PbEnum for PilotThrowOperationType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Ptothrow,
            1 => Self::Ptocancel,
            _ => Self::Ptothrow,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PilotThrowOperationType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PinballInGameStatePb {
    NotInitiated = 0,
    InProgress = 1,
    SettlePreSubmit = 2,
    SettleFinish = 3,
}

impl Default for PinballInGameStatePb {
    fn default() -> Self {
        Self::NotInitiated
    }
}

impl crate::wire::PbEnum for PinballInGameStatePb {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotInitiated,
            1 => Self::InProgress,
            2 => Self::SettlePreSubmit,
            3 => Self::SettleFinish,
            _ => Self::NotInitiated,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PinballInGameStatePb {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PinballStateChangeReasonPb {
    SettlePreSubmitAllTeamDeadNoRevive = 0,
    SettlePreSubmitAllTeamDeadHaveRevive = 1,
    SettlePreSubmitTimeZero = 2,
    SettlePreSubmitWaveClear = 3,
    InProgressFirstStart = 10,
    InProgressAllTeamRevive = 11,
    SettleFinishQuit = 21,
    SettleFinishLeaveScene = 22,
    SettleFinishBonusFullWin = 23,
    Gm = 15,
}

impl Default for PinballStateChangeReasonPb {
    fn default() -> Self {
        Self::SettlePreSubmitAllTeamDeadNoRevive
    }
}

impl crate::wire::PbEnum for PinballStateChangeReasonPb {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SettlePreSubmitAllTeamDeadNoRevive,
            1 => Self::SettlePreSubmitAllTeamDeadHaveRevive,
            2 => Self::SettlePreSubmitTimeZero,
            3 => Self::SettlePreSubmitWaveClear,
            10 => Self::InProgressFirstStart,
            11 => Self::InProgressAllTeamRevive,
            21 => Self::SettleFinishQuit,
            22 => Self::SettleFinishLeaveScene,
            23 => Self::SettleFinishBonusFullWin,
            15 => Self::Gm,
            _ => Self::SettlePreSubmitAllTeamDeadNoRevive,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for PinballStateChangeReasonPb {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum QuestLockType {
    QuestResource = 0,
    QuestResourceShow = 1,
    QuestFocus = 2,
}

impl Default for QuestLockType {
    fn default() -> Self {
        Self::QuestResource
    }
}

impl crate::wire::PbEnum for QuestLockType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::QuestResource,
            1 => Self::QuestResourceShow,
            2 => Self::QuestFocus,
            _ => Self::QuestResource,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for QuestLockType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RacingBetsDangoActionType {
    Move = 0,
    Skill = 1,
    Dice = 2,
    ChangeHigh = 3,
    DangoTransmit = 4,
    Organ = 5,
    DangoBlackHoleTransmit = 6,
}

impl Default for RacingBetsDangoActionType {
    fn default() -> Self {
        Self::Move
    }
}

impl crate::wire::PbEnum for RacingBetsDangoActionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Move,
            1 => Self::Skill,
            2 => Self::Dice,
            3 => Self::ChangeHigh,
            4 => Self::DangoTransmit,
            5 => Self::Organ,
            6 => Self::DangoBlackHoleTransmit,
            _ => Self::Move,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RacingBetsDangoActionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RacingBetsDangoMoveDirection {
    DangoForward = 0,
    DangoReverse = 1,
}

impl Default for RacingBetsDangoMoveDirection {
    fn default() -> Self {
        Self::DangoForward
    }
}

impl crate::wire::PbEnum for RacingBetsDangoMoveDirection {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::DangoForward,
            1 => Self::DangoReverse,
            _ => Self::DangoForward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RacingBetsDangoMoveDirection {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RacingBetsRankStatus {
    Rank = 0,
    Top1 = 1,
    Top50 = 2,
    UnRank = 3,
}

impl Default for RacingBetsRankStatus {
    fn default() -> Self {
        Self::Rank
    }
}

impl crate::wire::PbEnum for RacingBetsRankStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Rank,
            1 => Self::Top1,
            2 => Self::Top50,
            3 => Self::UnRank,
            _ => Self::Rank,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RacingBetsRankStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RacingBetsRoundActionType {
    MatchStart = 0,
    RoundStart = 1,
    MatchRoundDice = 3,
    DangoRoundStart = 2,
    DangoMove = 4,
    DangoSkill = 5,
    DangoChangeHigh = 6,
    CameraMove = 7,
    Destination = 8,
    UpdateRank = 9,
    Transmit = 10,
    OrganEffect = 11,
    BlackHoleTransmit = 12,
}

impl Default for RacingBetsRoundActionType {
    fn default() -> Self {
        Self::MatchStart
    }
}

impl crate::wire::PbEnum for RacingBetsRoundActionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::MatchStart,
            1 => Self::RoundStart,
            3 => Self::MatchRoundDice,
            2 => Self::DangoRoundStart,
            4 => Self::DangoMove,
            5 => Self::DangoSkill,
            6 => Self::DangoChangeHigh,
            7 => Self::CameraMove,
            8 => Self::Destination,
            9 => Self::UpdateRank,
            10 => Self::Transmit,
            11 => Self::OrganEffect,
            12 => Self::BlackHoleTransmit,
            _ => Self::MatchStart,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RacingBetsRoundActionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RecallQuestStatus {
    InActive = 0,
    Ready = 1,
    Progress = 2,
    Finish = 3,
    Delete = 4,
}

impl Default for RecallQuestStatus {
    fn default() -> Self {
        Self::InActive
    }
}

impl crate::wire::PbEnum for RecallQuestStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::InActive,
            1 => Self::Ready,
            2 => Self::Progress,
            3 => Self::Finish,
            4 => Self::Delete,
            _ => Self::InActive,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RecallQuestStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RecallState {
    NotStart = 0,
    Progress = 1,
    Finish = 2,
    Delete = 3,
}

impl Default for RecallState {
    fn default() -> Self {
        Self::NotStart
    }
}

impl crate::wire::PbEnum for RecallState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotStart,
            1 => Self::Progress,
            2 => Self::Finish,
            3 => Self::Delete,
            _ => Self::NotStart,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RecallState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ReconstructFromType {
    FromTypeHandCard = 0,
    FromTypeFighter = 1,
}

impl Default for ReconstructFromType {
    fn default() -> Self {
        Self::FromTypeHandCard
    }
}

impl crate::wire::PbEnum for ReconstructFromType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::FromTypeHandCard,
            1 => Self::FromTypeFighter,
            _ => Self::FromTypeHandCard,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ReconstructFromType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RegressBonusRewardType {
    Free = 0,
    Pay = 1,
    Both = 2,
}

impl Default for RegressBonusRewardType {
    fn default() -> Self {
        Self::Free
    }
}

impl crate::wire::PbEnum for RegressBonusRewardType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Free,
            1 => Self::Pay,
            2 => Self::Both,
            _ => Self::Free,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RegressBonusRewardType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RegressTaskTypeEnum {
    TaskReward = 0,
    Questionnaire = 1,
    SignReward = 2,
}

impl Default for RegressTaskTypeEnum {
    fn default() -> Self {
        Self::TaskReward
    }
}

impl crate::wire::PbEnum for RegressTaskTypeEnum {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TaskReward,
            1 => Self::Questionnaire,
            2 => Self::SignReward,
            _ => Self::TaskReward,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RegressTaskTypeEnum {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ResetQuestFocusModeReason {
    EnterSealedArea = 0,
    ForceOccupyFail = 1,
    EnterMultiMode = 2,
    BtSuspend = 3,
    EnterSpecialScene = 4,
    ClientRequest = 5,
    QuestFinish = 6,
    LoginEventCheck = 7,
    GmRequest = 8,
}

impl Default for ResetQuestFocusModeReason {
    fn default() -> Self {
        Self::EnterSealedArea
    }
}

impl crate::wire::PbEnum for ResetQuestFocusModeReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::EnterSealedArea,
            1 => Self::ForceOccupyFail,
            2 => Self::EnterMultiMode,
            3 => Self::BtSuspend,
            4 => Self::EnterSpecialScene,
            5 => Self::ClientRequest,
            6 => Self::QuestFinish,
            7 => Self::LoginEventCheck,
            8 => Self::GmRequest,
            _ => Self::EnterSealedArea,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ResetQuestFocusModeReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RhythmSettleReasonPb {
    Complete = 0,
    Quit = 1,
}

impl Default for RhythmSettleReasonPb {
    fn default() -> Self {
        Self::Complete
    }
}

impl crate::wire::PbEnum for RhythmSettleReasonPb {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Complete,
            1 => Self::Quit,
            _ => Self::Complete,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RhythmSettleReasonPb {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RogueArchiveState {
    CantSave = 0,
    CanSave = 1,
    Saved = 2,
}

impl Default for RogueArchiveState {
    fn default() -> Self {
        Self::CantSave
    }
}

impl crate::wire::PbEnum for RogueArchiveState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::CantSave,
            1 => Self::CanSave,
            2 => Self::Saved,
            _ => Self::CantSave,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RogueArchiveState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RogueChallengeModeType {
    None = 0,
    NormalInst = 1,
    TowerTrial = 2,
}

impl Default for RogueChallengeModeType {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for RogueChallengeModeType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::NormalInst,
            2 => Self::TowerTrial,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RogueChallengeModeType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RogueResDataType {
    NoGain = 0,
    Token = 1,
    TokenShop = 2,
    Phantom = 3,
    Role = 4,
    RoleBuff = 5,
    Event = 6,
    EventGrid = 7,
    MiracleCreation = 8,
    Effect = 9,
    Complex = 10,
}

impl Default for RogueResDataType {
    fn default() -> Self {
        Self::NoGain
    }
}

impl crate::wire::PbEnum for RogueResDataType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NoGain,
            1 => Self::Token,
            2 => Self::TokenShop,
            3 => Self::Phantom,
            4 => Self::Role,
            5 => Self::RoleBuff,
            6 => Self::Event,
            7 => Self::EventGrid,
            8 => Self::MiracleCreation,
            9 => Self::Effect,
            10 => Self::Complex,
            _ => Self::NoGain,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RogueResDataType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RogueResOpType {
    Unkonw = 0,
    Move = 1,
    GridEvent = 2,
    SelectView = 3,
    ShowView = 4,
    RogueGotoLevelPlay = 5,
    RollBuffBondLinkId = 6,
    FallbackView = 9,
    LightBlockByLocationEffect = 10,
    MapTeleportByLocationEffect = 11,
    ChangeEventByPos = 12,
}

impl Default for RogueResOpType {
    fn default() -> Self {
        Self::Unkonw
    }
}

impl crate::wire::PbEnum for RogueResOpType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Unkonw,
            1 => Self::Move,
            2 => Self::GridEvent,
            3 => Self::SelectView,
            4 => Self::ShowView,
            5 => Self::RogueGotoLevelPlay,
            6 => Self::RollBuffBondLinkId,
            9 => Self::FallbackView,
            10 => Self::LightBlockByLocationEffect,
            11 => Self::MapTeleportByLocationEffect,
            12 => Self::ChangeEventByPos,
            _ => Self::Unkonw,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RogueResOpType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RogueTowerTrialBossState {
    IsLock = 0,
    Selectable = 1,
    Completed = 2,
}

impl Default for RogueTowerTrialBossState {
    fn default() -> Self {
        Self::IsLock
    }
}

impl crate::wire::PbEnum for RogueTowerTrialBossState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::IsLock,
            1 => Self::Selectable,
            2 => Self::Completed,
            _ => Self::IsLock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RogueTowerTrialBossState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RogueWeeklyEntryType {
    Unknow = 0,
    Goods = 2,
    CommonBuff = 9,
}

impl Default for RogueWeeklyEntryType {
    fn default() -> Self {
        Self::Unknow
    }
}

impl crate::wire::PbEnum for RogueWeeklyEntryType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Unknow,
            2 => Self::Goods,
            9 => Self::CommonBuff,
            _ => Self::Unknow,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RogueWeeklyEntryType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoguelikeGainDataType {
    Unkonw = 0,
    Phantom = 1,
    Shop = 2,
    Role = 3,
    CommonBuff = 4,
    RoleBuff = 5,
    Event = 6,
    BranchBuff = 7,
    Miraclecreation = 8,
}

impl Default for RoguelikeGainDataType {
    fn default() -> Self {
        Self::Unkonw
    }
}

impl crate::wire::PbEnum for RoguelikeGainDataType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Unkonw,
            1 => Self::Phantom,
            2 => Self::Shop,
            3 => Self::Role,
            4 => Self::CommonBuff,
            5 => Self::RoleBuff,
            6 => Self::Event,
            7 => Self::BranchBuff,
            8 => Self::Miraclecreation,
            _ => Self::Unkonw,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoguelikeGainDataType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoleSettleResult {
    Bad = 0,
    Normal = 1,
    Good = 2,
}

impl Default for RoleSettleResult {
    fn default() -> Self {
        Self::Bad
    }
}

impl crate::wire::PbEnum for RoleSettleResult {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Bad,
            1 => Self::Normal,
            2 => Self::Good,
            _ => Self::Bad,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoleSettleResult {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoleTowerFloorType {
    Normal = 0,
    Verify = 1,
}

impl Default for RoleTowerFloorType {
    fn default() -> Self {
        Self::Normal
    }
}

impl crate::wire::PbEnum for RoleTowerFloorType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Verify,
            _ => Self::Normal,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoleTowerFloorType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoleTowerRankType {
    Friend = 0,
    Server = 1,
}

impl Default for RoleTowerRankType {
    fn default() -> Self {
        Self::Friend
    }
}

impl crate::wire::PbEnum for RoleTowerRankType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Friend,
            1 => Self::Server,
            _ => Self::Friend,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoleTowerRankType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoleTowerSettleReason {
    None = 0,
    AllPass = 1,
    TimeOut = 2,
    TeamDead = 3,
    Manual = 4,
}

impl Default for RoleTowerSettleReason {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for RoleTowerSettleReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::AllPass,
            2 => Self::TimeOut,
            3 => Self::TeamDead,
            4 => Self::Manual,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoleTowerSettleReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoleTowerStageState {
    NotPass = 0,
    Current = 1,
    Passed = 2,
    Locked = 3,
}

impl Default for RoleTowerStageState {
    fn default() -> Self {
        Self::NotPass
    }
}

impl crate::wire::PbEnum for RoleTowerStageState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotPass,
            1 => Self::Current,
            2 => Self::Passed,
            3 => Self::Locked,
            _ => Self::NotPass,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoleTowerStageState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoverRogueGainDataType {
    RoverRogueGainUnknown = 0,
    RoverRogueGainBless = 1,
    RoverRogueGainRoleEnhance = 2,
    RoverRogueGainLootItem = 3,
    RoverRogueGainItem = 4,
}

impl Default for RoverRogueGainDataType {
    fn default() -> Self {
        Self::RoverRogueGainUnknown
    }
}

impl crate::wire::PbEnum for RoverRogueGainDataType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RoverRogueGainUnknown,
            1 => Self::RoverRogueGainBless,
            2 => Self::RoverRogueGainRoleEnhance,
            3 => Self::RoverRogueGainLootItem,
            4 => Self::RoverRogueGainItem,
            _ => Self::RoverRogueGainUnknown,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoverRogueGainDataType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoverRogueGainOperateType {
    RoverRogueBlessSelect = 0,
    RoverRogueRoleEnhanceSelect = 1,
    RoverRogueBlessEnhanceSelect = 2,
    RoverRogueBlessGroupSelect = 3,
}

impl Default for RoverRogueGainOperateType {
    fn default() -> Self {
        Self::RoverRogueBlessSelect
    }
}

impl crate::wire::PbEnum for RoverRogueGainOperateType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RoverRogueBlessSelect,
            1 => Self::RoverRogueRoleEnhanceSelect,
            2 => Self::RoverRogueBlessEnhanceSelect,
            3 => Self::RoverRogueBlessGroupSelect,
            _ => Self::RoverRogueBlessSelect,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoverRogueGainOperateType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RoverRogueInstState {
    RoverRogueInstNotStart = 0,
    RoverRogueInstRunning = 1,
    RoverRogueInstFinished = 2,
    RoverRogueInstQuit = 3,
    RoverRogueInstDead = 4,
}

impl Default for RoverRogueInstState {
    fn default() -> Self {
        Self::RoverRogueInstNotStart
    }
}

impl crate::wire::PbEnum for RoverRogueInstState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::RoverRogueInstNotStart,
            1 => Self::RoverRogueInstRunning,
            2 => Self::RoverRogueInstFinished,
            3 => Self::RoverRogueInstQuit,
            4 => Self::RoverRogueInstDead,
            _ => Self::RoverRogueInstNotStart,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for RoverRogueInstState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SceneStepStatus {
    Accepted = 0,
    Finished = 1,
    Failed = 2,
}

impl Default for SceneStepStatus {
    fn default() -> Self {
        Self::Accepted
    }
}

impl crate::wire::PbEnum for SceneStepStatus {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Accepted,
            1 => Self::Finished,
            2 => Self::Failed,
            _ => Self::Accepted,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SceneStepStatus {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SeamlessTeleportType {
    ClientTriggerAction = 0,
    NoRenderPortalComponent = 1,
    LongPressInteractAction = 2,
}

impl Default for SeamlessTeleportType {
    fn default() -> Self {
        Self::ClientTriggerAction
    }
}

impl crate::wire::PbEnum for SeamlessTeleportType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ClientTriggerAction,
            1 => Self::NoRenderPortalComponent,
            2 => Self::LongPressInteractAction,
            _ => Self::ClientTriggerAction,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SeamlessTeleportType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ShareRideMode {
    MovieMotor = 0,
}

impl Default for ShareRideMode {
    fn default() -> Self {
        Self::MovieMotor
    }
}

impl crate::wire::PbEnum for ShareRideMode {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::MovieMotor,
            _ => Self::MovieMotor,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ShareRideMode {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ShowViewType {
    None = 0,
    GridTake = 1,
    RogueResAddRole = 2,
    PhantomEntryUnlock = 3,
    RoleBuffUnlock = 4,
    InstResult = 5,
    RoleBond = 6,
    AddToken = 7,
}

impl Default for ShowViewType {
    fn default() -> Self {
        Self::None
    }
}

impl crate::wire::PbEnum for ShowViewType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::GridTake,
            2 => Self::RogueResAddRole,
            3 => Self::PhantomEntryUnlock,
            4 => Self::RoleBuffUnlock,
            5 => Self::InstResult,
            6 => Self::RoleBond,
            7 => Self::AddToken,
            _ => Self::None,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ShowViewType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SneakFinishType {
    SneakSuccess = 0,
    SneakFail = 1,
}

impl Default for SneakFinishType {
    fn default() -> Self {
        Self::SneakSuccess
    }
}

impl crate::wire::PbEnum for SneakFinishType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SneakSuccess,
            1 => Self::SneakFail,
            _ => Self::SneakSuccess,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SneakFinishType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SneakState {
    Open = 0,
    Close = 1,
}

impl Default for SneakState {
    fn default() -> Self {
        Self::Open
    }
}

impl crate::wire::PbEnum for SneakState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Open,
            1 => Self::Close,
            _ => Self::Open,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SneakState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SolarSchoolAttrType {
    SolarSchoolAttrUnknown = 0,
    SolarSchoolAttrKnowledge = 1,
    SolarSchoolAttrThinking = 2,
    SolarSchoolAttrArt = 3,
    SolarSchoolAttrPhysical = 4,
    SolarSchoolAttrEmotion = 5,
    SolarSchoolAttrPopularity = 6,
    SolarSchoolAttrFatigue = 7,
    SolarSchoolAttrCurrency = 8,
}

impl Default for SolarSchoolAttrType {
    fn default() -> Self {
        Self::SolarSchoolAttrUnknown
    }
}

impl crate::wire::PbEnum for SolarSchoolAttrType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SolarSchoolAttrUnknown,
            1 => Self::SolarSchoolAttrKnowledge,
            2 => Self::SolarSchoolAttrThinking,
            3 => Self::SolarSchoolAttrArt,
            4 => Self::SolarSchoolAttrPhysical,
            5 => Self::SolarSchoolAttrEmotion,
            6 => Self::SolarSchoolAttrPopularity,
            7 => Self::SolarSchoolAttrFatigue,
            8 => Self::SolarSchoolAttrCurrency,
            _ => Self::SolarSchoolAttrUnknown,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SolarSchoolAttrType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SolarSchoolSaveReason {
    SolarSchoolReasonManual = 0,
    SolarSchoolReasonNewWeek = 1,
    SolarSchoolReasonNewCycle = 2,
    SolarSchoolReasonLoad = 3,
}

impl Default for SolarSchoolSaveReason {
    fn default() -> Self {
        Self::SolarSchoolReasonManual
    }
}

impl crate::wire::PbEnum for SolarSchoolSaveReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SolarSchoolReasonManual,
            1 => Self::SolarSchoolReasonNewWeek,
            2 => Self::SolarSchoolReasonNewCycle,
            3 => Self::SolarSchoolReasonLoad,
            _ => Self::SolarSchoolReasonManual,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SolarSchoolSaveReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SolarSchoolTaskType {
    SolarSchoolResident = 0,
    SolarSchoolLimited = 1,
}

impl Default for SolarSchoolTaskType {
    fn default() -> Self {
        Self::SolarSchoolResident
    }
}

impl crate::wire::PbEnum for SolarSchoolTaskType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::SolarSchoolResident,
            1 => Self::SolarSchoolLimited,
            _ => Self::SolarSchoolResident,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SolarSchoolTaskType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum StepState {
    NotInit = 0,
    WaitConfirm = 1,
    WaitEffect = 2,
    EffectInterrupt = 3,
}

impl Default for StepState {
    fn default() -> Self {
        Self::NotInit
    }
}

impl crate::wire::PbEnum for StepState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::NotInit,
            1 => Self::WaitConfirm,
            2 => Self::WaitEffect,
            3 => Self::EffectInterrupt,
            _ => Self::NotInit,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for StepState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SubInsSettleRewardType {
    FloroRanchSkill = 0,
    FloroRanchCard = 1,
    FloroRanchToy = 2,
}

impl Default for SubInsSettleRewardType {
    fn default() -> Self {
        Self::FloroRanchSkill
    }
}

impl crate::wire::PbEnum for SubInsSettleRewardType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::FloroRanchSkill,
            1 => Self::FloroRanchCard,
            2 => Self::FloroRanchToy,
            _ => Self::FloroRanchSkill,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SubInsSettleRewardType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Summon_EPlayerFollower {
    Default = 0,
    ExploreSkill = 1,
    Auxiliary = 2,
    SpecialItem = 3,
    Motor = 4,
    Max = 5,
}

impl Default for Summon_EPlayerFollower {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for Summon_EPlayerFollower {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::ExploreSkill,
            2 => Self::Auxiliary,
            3 => Self::SpecialItem,
            4 => Self::Motor,
            5 => Self::Max,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for Summon_EPlayerFollower {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Summon_ESummonType {
    Default = 0,
    ConcomitantVision = 1,
    ConcomitantCustom = 2,
    ConcomitantPhantomRole = 3,
    ConcomitantWeakVision = 4,
    ConcomitantMotorcycle = 5,
    ConcomitantSkillAvatar = 6,
}

impl Default for Summon_ESummonType {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for Summon_ESummonType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::ConcomitantVision,
            2 => Self::ConcomitantCustom,
            3 => Self::ConcomitantPhantomRole,
            4 => Self::ConcomitantWeakVision,
            5 => Self::ConcomitantMotorcycle,
            6 => Self::ConcomitantSkillAvatar,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for Summon_ESummonType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SunSpiritOperationType {
    Fly = 0,
    Back = 1,
}

impl Default for SunSpiritOperationType {
    fn default() -> Self {
        Self::Fly
    }
}

impl crate::wire::PbEnum for SunSpiritOperationType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Fly,
            1 => Self::Back,
            _ => Self::Fly,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for SunSpiritOperationType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TeamType {
    WorldTeam = 0,
    MatchTeam = 1,
}

impl Default for TeamType {
    fn default() -> Self {
        Self::WorldTeam
    }
}

impl crate::wire::PbEnum for TeamType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::WorldTeam,
            1 => Self::MatchTeam,
            _ => Self::WorldTeam,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TeamType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TimeHandleType {
    Lock = 0,
    Unlock = 1,
}

impl Default for TimeHandleType {
    fn default() -> Self {
        Self::Lock
    }
}

impl crate::wire::PbEnum for TimeHandleType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Lock,
            1 => Self::Unlock,
            _ => Self::Lock,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TimeHandleType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TrackMoonConditionType {
    TrackMoonEntrust = 0,
    TrackMoonRole = 1,
    TrackMoonBuild = 2,
}

impl Default for TrackMoonConditionType {
    fn default() -> Self {
        Self::TrackMoonEntrust
    }
}

impl crate::wire::PbEnum for TrackMoonConditionType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TrackMoonEntrust,
            1 => Self::TrackMoonRole,
            2 => Self::TrackMoonBuild,
            _ => Self::TrackMoonEntrust,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TrackMoonConditionType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TrackMoonTargetState {
    TrackMoonTargetRunning = 0,
    TrackMoonTargetFinish = 1,
    TrackMoonTargetTaken = 2,
}

impl Default for TrackMoonTargetState {
    fn default() -> Self {
        Self::TrackMoonTargetRunning
    }
}

impl crate::wire::PbEnum for TrackMoonTargetState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::TrackMoonTargetRunning,
            1 => Self::TrackMoonTargetFinish,
            2 => Self::TrackMoonTargetTaken,
            _ => Self::TrackMoonTargetRunning,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TrackMoonTargetState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TrampleGearTransition {
    Up = 0,
    Down = 1,
}

impl Default for TrampleGearTransition {
    fn default() -> Self {
        Self::Up
    }
}

impl crate::wire::PbEnum for TrampleGearTransition {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Up,
            1 => Self::Down,
            _ => Self::Up,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TrampleGearTransition {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TrapDefenseGainingUpdateReason {
    GmaddGroup = 0,
    ShopProduct = 1,
    RewardSelect = 2,
}

impl Default for TrapDefenseGainingUpdateReason {
    fn default() -> Self {
        Self::GmaddGroup
    }
}

impl crate::wire::PbEnum for TrapDefenseGainingUpdateReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::GmaddGroup,
            1 => Self::ShopProduct,
            2 => Self::RewardSelect,
            _ => Self::GmaddGroup,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TrapDefenseGainingUpdateReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TreasureChestCountTargetType {
    Instance = 0,
    Area = 1,
}

impl Default for TreasureChestCountTargetType {
    fn default() -> Self {
        Self::Instance
    }
}

impl crate::wire::PbEnum for TreasureChestCountTargetType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Instance,
            1 => Self::Area,
            _ => Self::Instance,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TreasureChestCountTargetType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TreasureChestStatType {
    OneTime = 0,
    HsinToken = 1,
}

impl Default for TreasureChestStatType {
    fn default() -> Self {
        Self::OneTime
    }
}

impl crate::wire::PbEnum for TreasureChestStatType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::OneTime,
            1 => Self::HsinToken,
            _ => Self::OneTime,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for TreasureChestStatType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum UiGamePlayType {
    Cipher = 0,
    SignalBreak = 1,
    SundialPuzzle = 2,
    MorseCode = 3,
    SignalDevice = 4,
    LifePoint = 5,
    Reigns = 6,
    TraceTracing = 7,
    GolemCrack = 8,
    SheriffInference = 9,
    WriteLetter = 10,
    InstrumentQte = 11,
}

impl Default for UiGamePlayType {
    fn default() -> Self {
        Self::Cipher
    }
}

impl crate::wire::PbEnum for UiGamePlayType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Cipher,
            1 => Self::SignalBreak,
            2 => Self::SundialPuzzle,
            3 => Self::MorseCode,
            4 => Self::SignalDevice,
            5 => Self::LifePoint,
            6 => Self::Reigns,
            7 => Self::TraceTracing,
            8 => Self::GolemCrack,
            9 => Self::SheriffInference,
            10 => Self::WriteLetter,
            11 => Self::InstrumentQte,
            _ => Self::Cipher,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for UiGamePlayType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum WorldEnterPermission {
    ConfirmJoin = 0,
    DirectJoin = 1,
    ForbidJoin = 2,
    OnlyFriendJoin = 3,
}

impl Default for WorldEnterPermission {
    fn default() -> Self {
        Self::ConfirmJoin
    }
}

impl crate::wire::PbEnum for WorldEnterPermission {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::ConfirmJoin,
            1 => Self::DirectJoin,
            2 => Self::ForbidJoin,
            3 => Self::OnlyFriendJoin,
            _ => Self::ConfirmJoin,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for WorldEnterPermission {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum WorldEnterWay {
    LobbyJoin = 0,
    QueryJoin = 1,
}

impl Default for WorldEnterWay {
    fn default() -> Self {
        Self::LobbyJoin
    }
}

impl crate::wire::PbEnum for WorldEnterWay {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::LobbyJoin,
            1 => Self::QueryJoin,
            _ => Self::LobbyJoin,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for WorldEnterWay {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum WorldTeamLeaveReason {
    Initiative = 0,
    BeKick = 1,
    Dissolve = 2,
    Logout = 3,
    ToOwnBigWorld = 4,
    PsnaccountLogin = 5,
    CheckClientVersionFailed = 6,
    SceneBlockSplitFail = 7,
    XboxaccountLogin = 8,
    InstPlayEnd = 9,
}

impl Default for WorldTeamLeaveReason {
    fn default() -> Self {
        Self::Initiative
    }
}

impl crate::wire::PbEnum for WorldTeamLeaveReason {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Initiative,
            1 => Self::BeKick,
            2 => Self::Dissolve,
            3 => Self::Logout,
            4 => Self::ToOwnBigWorld,
            5 => Self::PsnaccountLogin,
            6 => Self::CheckClientVersionFailed,
            7 => Self::SceneBlockSplitFail,
            8 => Self::XboxaccountLogin,
            9 => Self::InstPlayEnd,
            _ => Self::Initiative,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for WorldTeamLeaveReason {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum WorldTeamPlayerChangeType {
    Name = 0,
    Head = 1,
    Level = 2,
    Signature = 3,
    PlayerTitle = 4,
    Sex = 5,
}

impl Default for WorldTeamPlayerChangeType {
    fn default() -> Self {
        Self::Name
    }
}

impl crate::wire::PbEnum for WorldTeamPlayerChangeType {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Name,
            1 => Self::Head,
            2 => Self::Level,
            3 => Self::Signature,
            4 => Self::PlayerTitle,
            5 => Self::Sex,
            _ => Self::Name,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for WorldTeamPlayerChangeType {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ZitherOpState {
    OpEnter = 0,
    OpExit = 1,
}

impl Default for ZitherOpState {
    fn default() -> Self {
        Self::OpEnter
    }
}

impl crate::wire::PbEnum for ZitherOpState {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::OpEnter,
            1 => Self::OpExit,
            _ => Self::OpEnter,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ZitherOpState {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ECheatInput {
    Default = 0,
    Del = 1,
    End = 2,
}

impl Default for ECheatInput {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for ECheatInput {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Del,
            2 => Self::End,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for ECheatInput {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EPlayerFollower {
    Default = 0,
    ExploreSkill = 1,
    Auxiliary = 2,
    SpecialItem = 3,
    Motor = 4,
    Max = 5,
}

impl Default for EPlayerFollower {
    fn default() -> Self {
        Self::Default
    }
}

impl crate::wire::PbEnum for EPlayerFollower {
    fn from_i32_or_default(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::ExploreSkill,
            2 => Self::Auxiliary,
            3 => Self::SpecialItem,
            4 => Self::Motor,
            5 => Self::Max,
            _ => Self::Default,
        }
    }
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl crate::wire::MapCodec for EPlayerFollower {
    fn wire_type() -> crate::wire::WireType {
        crate::wire::WireType::Varint
    }
    fn is_default(&self) -> bool {
        (*self as i32) == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        crate::wire::write_varint_i32(out, *self as i32);
    }
    fn decode_value(
        _wt: crate::wire::WireType,
        r: &mut crate::wire::Reader<'_>,
    ) -> Result<Self, crate::wire::DecodeError> {
        Ok(<Self as crate::wire::PbEnum>::from_i32_or_default(
            r.read_varint_i32()?,
        ))
    }
}
