const std = @import("std");
const BaseInfo = @import("BaseInfo.zig");

pub const EmptyObj = struct {};

pub const HookInteractConfig = struct {
    Type: ?[]const u8 = null,
    GravityDirection: ?BaseInfo.GravityDirection = null,
    HookActions: ?[]const std.json.Value = null,
    FinishActions: ?[]const std.json.Value = null,
    ExitHookActions: ?[]const std.json.Value = null,
};

pub const AdsorbComponent = struct {
    Acceleration: ?i64 = null,
    AdsorbLimitedTime: ?i64 = null,
    Disabled: ?bool = null,
    Range: ?i64 = null,
    StartVelocity: ?i64 = null,
};

pub const AdviseItemComponent = struct {
};

pub const AiAlertNotifyComponent = struct {
    ExcludeEntities: ?[]const i64 = null,
    ExtraAiAlert: ?EmptyObj = null,
};

pub const AiGearStrategyComponent = struct {
    StrategyType: ?EmptyObj = null,
};

pub const AirPassageComponent = struct {
};

pub const AirWallSpawnerComponent = struct {
    BiasDirection: ?i64 = null,
    Disabled: ?bool = null,
    HasCover: ?bool = null,
    Height: ?i64 = null,
    SplineEntity: ?i64 = null,
    Thickness: ?i64 = null,
    WallName: ?[]const u8 = null,
};

pub const AnimalComponent = struct {
    AnimalAttackRange: ?i64 = null,
    CanKillSelf: ?bool = null,
    Disabled: ?bool = null,
    IsStare: ?bool = null,
    MoveRange: ?i64 = null,
    PlayDeathAnimOnDestroy: ?bool = null,
    SpecialAnimalConfig: ?EmptyObj = null,
    UseRangeComponentAsMoveRange: ?bool = null,
};

pub const AttachTargetComponent = struct {
    AttachTarget: ?EmptyObj = null,
    AttachTiming: ?EmptyObj = null,
    Disabled: ?bool = null,
    PosRule: ?[]const u8 = null,
    RotRule: ?[]const u8 = null,
};

pub const BatchBulletCasterComponent = struct {
    MovementType: ?i64 = null,
    WarningEffect: ?[]const u8 = null,
};

pub const BeamCastComponent = struct {
    BeamLayout: ?EmptyObj = null,
    DelayDestroyEffect: ?bool = null,
    DisableDynamicRange: ?bool = null,
    Disabled: ?bool = null,
    EffectPath: ?[]const u8 = null,
    HitEffectPath: ?[]const u8 = null,
    IgnoreMonsterCollision: ?bool = null,
    Range: ?EmptyObj = null,
    Reflection: ?EmptyObj = null,
    StartEffectPath: ?[]const u8 = null,
    TargetState: ?[]const u8 = null,
};

pub const BeamReceiveComponent = struct {
    Disabled: ?bool = null,
    Duration: ?f64 = null,
    ReceiveCondition: ?EmptyObj = null,
    ReceiveTarget: ?[]const u8 = null,
};

pub const BubbleComponent = struct {
    Disabled: ?bool = null,
    EnterRange: ?i64 = null,
    IsRangeFollowEntity: ?bool = null,
    LeaveRange: ?i64 = null,
    NpcIds: ?[]const i64 = null,
    Scale: ?f64 = null,
    TimberId: ?i64 = null,
    TriggerCylinderConfig: ?EmptyObj = null,
};

pub const BuffConsumerComponent = struct {
    BuffId: ?i64 = null,
    BulletId: ?i64 = null,
    ProducerEntityId: ?i64 = null,
};

pub const BuffProducerComponent = struct {
    AddBuffMode: ?EmptyObj = null,
    BuffId: ?i64 = null,
    Disabled: ?bool = null,
};

pub const CharacterConnectorComponent = struct {
    LogicType: ?EmptyObj = null,
};

pub const ChessmanComponent = struct {
};

pub const ClientConditionListenerComponent = struct {
    Disabled: ?bool = null,
};

pub const ClientTriggerComponent = struct {
    Disabled: ?bool = null,
    OnTriggerEnter: ?EmptyObj = null,
    OnTriggerExit: ?EmptyObj = null,
    TriggerMatch: ?EmptyObj = null,
};

pub const CombatComponent = struct {
    Disabled: ?bool = null,
};

pub const CombinedVisibleGroupComponent = struct {
    IncludeSubArea: ?bool = null,
};

pub const ConnectorComponent = struct {
    Disabled: ?bool = null,
    LogicType: ?EmptyObj = null,
};

pub const ConveyorBeltComponent = struct {
};

pub const CurveControlComponent = struct {
    CurveControlConfig: ?EmptyObj = null,
    Disabled: ?bool = null,
};

pub const DollGrabMachineComponent = struct {
    AkEvent: ?[]const u8 = null,
    BelongType: ?[]const u8 = null,
    InfiniteModeData: ?EmptyObj = null,
    PlayCost: ?i64 = null,
    TimeLimit: ?i64 = null,
};

pub const DollGrabShowcaseComponent = struct {
    BelongType: ?[]const u8 = null,
    BindInfiniteModeDollGrabEntityId: ?i64 = null,
    CameraConfig: ?EmptyObj = null,
};

pub const DragActorPlayComponent = struct {
    ExitSendSelfEvent: ?[]const u8 = null,
};

pub const DropComponent = struct {
    Disabled: ?bool = null,
};

pub const DungeonEntryComponent = struct {
    Disabled: ?bool = null,
};

pub const DynamicEntityRewardComponent = struct {
    ClaimTypes: ?[]const i64 = null,
    Disabled: ?bool = null,
    RewardType: ?EmptyObj = null,
};

pub const DynamicPortalCreatorComponent = struct {
    Disabled: ?bool = null,
    Model: ?EmptyObj = null,
};

pub const DynamicSpawnMonsterComponent = struct {
    Disabled: ?bool = null,
};

pub const DynamicSpawnMonsterPointComponent = struct {
    TagId: ?i64 = null,
};

pub const EditCustomAoiComponent = struct {
    Entities: ?[]const i64 = null,
};

pub const EffectAreaComponent = struct {
    Config: ?EmptyObj = null,
    Disabled: ?bool = null,
};

pub const ElementalBoxFoundationComponent = struct {
    ElementalType: ?[]const u8 = null,
    InitialActiveElementalBoxId: ?i64 = null,
    LoopTeleportMappingFoundationId: ?i64 = null,
    MaxCarryRange: ?i64 = null,
};

pub const ElementalEffectComponent = struct {
    Disabled: ?bool = null,
    MatchConfig: ?EmptyObj = null,
    Range: ?EmptyObj = null,
};

pub const ElementalHolderComponent = struct {
    AbilityConfig: ?EmptyObj = null,
    Disabled: ?bool = null,
};

pub const ElementalWellComponent = struct {
    LeaveRangeToState: ?[]const u8 = null,
    PrefabBindPointKey: ?[]const u8 = null,
    StreamWaitConfig: ?EmptyObj = null,
    StreamingFinishedToState: ?[]const u8 = null,
    StreamingRenderConfig: ?EmptyObj = null,
    WellType: ?[]const u8 = null,
};

pub const EnrichmentAreaComponent = struct {
    EnrichmentAreaType: ?i64 = null,
    FogIds: ?[]const i64 = null,
    RefEntityIds: ?[]const i64 = null,
};

pub const EntityBatchRefreshComponent = struct {
    EndCondition: ?EmptyObj = null,
    EntityBatchRefresh: ?EmptyObj = null,
    StartCondition: ?EmptyObj = null,
};

pub const EntityBundleComponent = struct {
    BundleId: ?i64 = null,
};

pub const EntityCustomAudioComponent = struct {
    AkEventType: ?EmptyObj = null,
    AudioControlType: ?EmptyObj = null,
    AudioRangeType: ?[]const u8 = null,
    Disabled: ?bool = null,
};

pub const EntityGroupComponent = struct {
    Disabled: ?bool = null,
    EntityIds: ?[]const i64 = null,
    FailureState: ?EmptyObj = null,
    FinishState: ?EmptyObj = null,
};

pub const EntityListComponent = struct {
};

pub const EntityPackageComponent = struct {
    BindTemplate: ?bool = null,
    PackageData: ?EmptyObj = null,
};

pub const EntityStateAudioComponent = struct {
    AkEventType: ?EmptyObj = null,
    AudioRangeType: ?[]const u8 = null,
    Disabled: ?bool = null,
};

pub const EntityVisibleComponent = struct {
    CustomVisibleRange: ?i64 = null,
    DelayChange: ?bool = null,
    Disabled: ?bool = null,
    TargetPlatform: ?i64 = null,
    UseCutEffect: ?bool = null,
    UseFadeEffect: ?bool = null,
    UseHolographicEffect: ?bool = null,
};

pub const ExhibitComponent = struct {
    ExhibitConfig: ?EmptyObj = null,
};

pub const ExploreSkillInteractComponent = struct {
    Condition: ?EmptyObj = null,
    Disabled: ?bool = null,
    ExploreSkillUiResource: ?[]const u8 = null,
    IgnoresCollisionCfg: ?EmptyObj = null,
    Option: ?EmptyObj = null,
    PlayerStateRestritionId: ?i64 = null,
    SearchTargetCfg: ?EmptyObj = null,
    SelfInteractBlockDetectionKeys: ?[]const []const u8 = null,
};

pub const ExploreSkillInteractGroupComponent = struct {
    Disabled: ?bool = null,
};

pub const ExtraRangeComponent = struct {
    Disabled: ?bool = null,
};

pub const FanComponent = struct {
    CirclePerRound: ?i64 = null,
    Condition: ?EmptyObj = null,
    EffectConfig: ?EmptyObj = null,
    GearType: ?[]const u8 = null,
    InitCircle: ?i64 = null,
    InteractType: ?EmptyObj = null,
    OffsetActor: ?[]const u8 = null,
    RotateActor: ?[]const u8 = null,
};

pub const FightInteractComponent = struct {
    Disabled: ?bool = null,
    LockControl: ?EmptyObj = null,
    LockOffset: ?EmptyObj = null,
    LockRange: ?i64 = null,
};

pub const FlowerPollutionComponent = struct {
    BakeData: ?[]const u8 = null,
    ObstacleEntityIds: ?[]const i64 = null,
    PointLightConfig: ?EmptyObj = null,
    PollutionSourceEntityIds: ?[]const i64 = null,
    SplineConfigs: ?EmptyObj = null,
};

pub const FollowDecorationComponent = struct {
    DecorationConfig: ?EmptyObj = null,
};

pub const FollowShooterComponent = struct {
    Config: ?[]const u8 = null,
};

pub const FollowTrackComponent = struct {
    Disabled: ?bool = null,
    EndType: ?EmptyObj = null,
    Range: ?i64 = null,
    SplineEntityId: ?i64 = null,
};

pub const FurnitureSlotComponent = struct {
    FurnitureCamera: ?EmptyObj = null,
};

pub const GameplayDataPersistenceComponent = struct {
    DataPersistenceGameplay: ?EmptyObj = null,
};

pub const GodKingFrequencyControllerComponent = struct {
    AttachSocket: ?[]const u8 = null,
    FlySpeed: ?i64 = null,
    GodKingFrequencyEntity: ?i64 = null,
    InitialAttachStatue: ?i64 = null,
    MaxFlyTime: ?i64 = null,
    StatueEntityIdList: ?[]const i64 = null,
};

pub const Graph3dComponent = struct {
    GraphConfig: ?EmptyObj = null,
};

pub const Graph3dEffectComponent = struct {
    Disabled: ?bool = null,
    EffectType: ?EmptyObj = null,
    Graph3dEntityId: ?i64 = null,
};

pub const GridObjectComponent = struct {
    CellSize: ?EmptyObj = null,
};

pub const GroupAiComponent = struct {
    Entities: ?[]const i64 = null,
    Option: ?EmptyObj = null,
    SyncTags: ?[]const []const u8 = null,
};

pub const GuideLineCreatorComponent = struct {
    ColorChangeOption: ?EmptyObj = null,
    ScanOption: ?EmptyObj = null,
    SplineEntityId: ?i64 = null,
};

pub const HackManagementComponent = struct {
    Disabled: ?bool = null,
    MaxHackingCount: ?i64 = null,
    ValidDistance: ?i64 = null,
};

pub const HitComponent = struct {
    Disabled: ?bool = null,
    HitBullet: ?EmptyObj = null,
    VictimHitTimeScaleRatio: ?EmptyObj = null,
};

pub const HookLockPoint = struct {
    CameraGaze: ?EmptyObj = null,
    DisableAtCreateStage: ?bool = null,
    Disabled: ?bool = null,
    GazeNextPointAfterInteract: ?EmptyObj = null,
    GravityConfig: ?BaseInfo.GravityConfigData = null,
    HookEnableCondition: ?EmptyObj = null,
    HookInteractConfig: ?HookInteractConfig = null,
    HookLockCd: ?i64 = null,
    IgnorePlayCollision: ?bool = null,
    InheritSpeed: ?bool = null,
    IsClimb: ?bool = null,
    IsDestroyedSelf: ?bool = null,
    IsHideSelf: ?bool = null,
    MotorHookConfig: ?EmptyObj = null,
    NonInteractiveInnerRadius: ?i64 = null,
    NormalEffect: ?[]const u8 = null,
    PlayerStateRestritionId: ?i64 = null,
    Range: ?EmptyObj = null,
    ScreenDetectType: ?EmptyObj = null,
    UseRangeComponent: ?bool = null,
};

pub const IndependentSystemRewardComponent = struct {
    Disabled: ?bool = null,
    RewardType: ?EmptyObj = null,
};

pub const InhalationAbilityComponent = struct {
};

pub const InhaledItemComponent = struct {
    InhaledPerResult: ?EmptyObj = null,
    InhaledPerformance: ?EmptyObj = null,
    InhaledStrength: ?i64 = null,
};

pub const InteractAudioComponent = struct {
    CollisionMaterial: ?[]const u8 = null,
    Disabled: ?bool = null,
    InteractEventConfig: ?EmptyObj = null,
};

pub const InteractGearComponent = struct {
    ActivePrepareTime: ?f64 = null,
    Disabled: ?bool = null,
    NormalPrepareTime: ?f64 = null,
};

pub const ItemFoundation2 = struct {
    Config: ?EmptyObj = null,
    Disabled: ?bool = null,
};

pub const JigsawFoundation = struct {
    CompleteCondition: ?EmptyObj = null,
    CompletedConfig: ?EmptyObj = null,
    JigsawConfig: ?EmptyObj = null,
    ModelId: ?i64 = null,
    PlaceOffset: ?EmptyObj = null,
};

pub const JigsawItem = struct {
    Disabled: ?bool = null,
    FillCfg: ?EmptyObj = null,
};

pub const LevelAiComponent = struct {
    BtTreeAsset: ?[]const u8 = null,
    Disabled: ?bool = null,
};

pub const LevelPrefabPerformComponent = struct {
    Disabled: ?bool = null,
    PrefabParams: ?EmptyObj = null,
};

pub const LevelQteComponent = struct {
    Disabled: ?bool = null,
    QteConfig: ?EmptyObj = null,
};

pub const LevelSequenceFrameEventComponent = struct {
    Disabled: ?bool = null,
};

pub const LevitateMagnetComponent = struct {
    MoveSpeed: ?i64 = null,
};

pub const LifePointCenterComponent = struct {
};

pub const LocationSafetyComponent = struct {
    DetectionFrequency: ?[]const u8 = null,
    SafeLocation: ?EmptyObj = null,
};

pub const MonitorComponent = struct {
    AttachPoint: ?[]const u8 = null,
    AvailableStates: ?[]const []const u8 = null,
    Disabled: ?bool = null,
    Range: ?EmptyObj = null,
    StateAfterLeaved: ?[]const u8 = null,
    StateAfterTriggered: ?[]const u8 = null,
};

pub const MonsterGachaBaseComponent = struct {
    MonsterEntityIds: ?[]const i64 = null,
};

pub const MonsterGachaItemComponent = struct {
    MaterialDataPath: ?[]const u8 = null,
    MonsterEntityIds: ?[]const i64 = null,
    MonsterType: ?i64 = null,
};

pub const MotorSlideComponent = struct {
    Disabled: ?bool = null,
    MaxAngle: ?i64 = null,
    MotorSlideDtName: ?[]const u8 = null,
};

pub const NearbyTrackingComponent = struct {
    Disabled: ?bool = null,
    IsEnable: ?bool = null,
    IsEnableWhileUnlock: ?bool = null,
    IsEnbaleWhileHoming: ?bool = null,
    TrackingType: ?EmptyObj = null,
};

pub const NpcPerformComponent = struct {
    DeathInteract: ?EmptyObj = null,
    DefaultAbpState: ?[]const u8 = null,
    DefaultFaceExpressionId: ?i64 = null,
    DefaultPerform: ?EmptyObj = null,
    Disabled: ?bool = null,
    FixedPosition: ?bool = null,
    IsShowStrike: ?bool = null,
    IsStare: ?bool = null,
    NpcBumpShow: ?EmptyObj = null,
    NpcHitShow: ?EmptyObj = null,
    NpcMonsterClosePerform: ?EmptyObj = null,
    NpcPerformState: ?EmptyObj = null,
    ShowOnAwake: ?EmptyObj = null,
    ShowOnInteract: ?EmptyObj = null,
    ShowOnStandby: ?EmptyObj = null,
    ShowOnUiInteract: ?EmptyObj = null,
    SpecialNpcPerformConfig: ?EmptyObj = null,
};

pub const PasserbyNpcSpawnComponent = struct {
    MoveConfig: ?EmptyObj = null,
    NpcMaterialDa: ?[]const u8 = null,
    SourceConfig: ?EmptyObj = null,
    SpawnConfig: ?EmptyObj = null,
    VisibleConditions: ?EmptyObj = null,
};

pub const PathDrivenActorSpawnerComponent = struct {
    Disabled: ?bool = null,
    SpawnRule: ?EmptyObj = null,
};

pub const PerformanceOptimizationComponent = struct {
};

pub const PhotoTargetComponent = struct {
    RayCastIgnoreEntities: ?[]const i64 = null,
    TargetCapturePromptUi: ?EmptyObj = null,
};

pub const PhysicsConstraintComponent = struct {
    AngularLimit: ?EmptyObj = null,
    AttachTarget: ?EmptyObj = null,
    DampingCoefficient: ?f64 = null,
    Disabled: ?bool = null,
    EffectPath: ?[]const u8 = null,
};

pub const PickInteractComponent = struct {
    Disabled: ?bool = null,
    PickInteractType: ?EmptyObj = null,
};

pub const ProceduralMaterialComponent = struct {
    Disabled: ?bool = null,
};

pub const ProceduralVisualComponent = struct {
    Disabled: ?bool = null,
};

pub const ProgressBarControlComponent = struct {
    Control: ?EmptyObj = null,
    Disabled: ?bool = null,
    HitBullet: ?EmptyObj = null,
};

pub const ProjectorComponent = struct {
    BackgroundEntityId: ?i64 = null,
    Disabled: ?bool = null,
    DragRotationCoefficient: ?i64 = null,
    MatchTolerance: ?i64 = null,
    ProjectorPatternId: ?i64 = null,
    RotationSpeed: ?i64 = null,
};

pub const PullingObjectFoundation = struct {
    Config: ?EmptyObj = null,
};

pub const QuickHackComponent = struct {
    AutoClose: ?[]const u8 = null,
    IconOffset: ?EmptyObj = null,
    LinkEffect: ?EmptyObj = null,
    Time: ?f64 = null,
};

pub const ReboundComponent = struct {
    BulletId: ?i64 = null,
    Disabled: ?bool = null,
    Option: ?EmptyObj = null,
};

pub const RenderSpecifiedRangeComponent = struct {
    Condition: ?EmptyObj = null,
    Disabled: ?bool = null,
    RenderConfig: ?EmptyObj = null,
};

pub const ResetEntitiesPosComponent = struct {
    Disabled: ?bool = null,
    EntityIds: ?[]const i64 = null,
    Range: ?EmptyObj = null,
};

pub const ResetSelfPosComponent = struct {
    Disabled: ?bool = null,
    IsDisableResetPosAfterThrow: ?bool = null,
    IsResetPosAfterThrow: ?bool = null,
    ResetPosDelayTime: ?f64 = null,
    ResetRadius: ?i64 = null,
};

pub const ResurrectionComponent = struct {
    GravityConfig: ?EmptyObj = null,
    ReviveId: ?i64 = null,
    TeleportPos: ?EmptyObj = null,
    TriggerShape: ?EmptyObj = null,
};

pub const RoadNetworkNavigationComponent = struct {
    BasicConfig: ?EmptyObj = null,
    ObstacleConfig: ?EmptyObj = null,
};

pub const RollBlockComponent = struct {
};

pub const RollBlockFloorComponent = struct {
};

pub const RollBlockItemComponent = struct {
};

pub const RotatorComponent2 = struct {
    Disabled: ?bool = null,
};

pub const SceneActorRefComponent = struct {
    Disabled: ?bool = null,
    LiftFloorSeqMarks: ?EmptyObj = null,
};

pub const SceneBulletComponent = struct {
    DisableGenerateByRange: ?bool = null,
    Disabled: ?bool = null,
};

pub const SceneItemAiComponent = struct {
    AiConfig: ?EmptyObj = null,
    Disabled: ?bool = null,
};

pub const SceneItemAttributeComponent = struct {
    AttributeType: ?[]const u8 = null,
    DefaultTags: ?[]const i64 = null,
    Disabled: ?bool = null,
};

pub const SceneItemEventListenerComponent = struct {
    Disabled: ?bool = null,
};

pub const SceneTextComponent = struct {
    DisplayState: ?[]const u8 = null,
    RelativeTransform: ?EmptyObj = null,
    TextConfig: ?EmptyObj = null,
};

pub const SimpleCombatComponent = struct {
    Id: ?i64 = null,
};

pub const SkyboxComponent = struct {
    Disabled: ?bool = null,
    FadeTime: ?f64 = null,
    PPTODDataAsset: ?[]const u8 = null,
    Priority: ?i64 = null,
    TriggerMode: ?EmptyObj = null,
    WeatherDataAsset: ?[]const u8 = null,
};

pub const SlidePerformComponent = struct {
    RailSplineEntityId: ?i64 = null,
    SlidePerformConfig: ?EmptyObj = null,
};

pub const SlideRailComponent = struct {
    DownSpeed: ?i64 = null,
    RailSplineEntityId: ?i64 = null,
    SlideSpeed: ?i64 = null,
    UpSpeed: ?i64 = null,
};

pub const SpawnMonsterComponent = struct {
    ActiveType: ?EmptyObj = null,
    Disabled: ?bool = null,
    IssharedHatred: ?bool = null,
    SpawnMonsterConstraintType: ?EmptyObj = null,
};

pub const StateHintComponent = struct {
    Disabled: ?bool = null,
};

pub const StreamingSourceTriggerPointComponent = struct {
    StreamingSource: ?EmptyObj = null,
};

pub const SunSpiritCollectComponent = struct {
    AreaId: ?i64 = null,
    Disabled: ?bool = null,
    PlayerBattleConfig: ?EmptyObj = null,
    SunSpiritScale: ?f64 = null,
};

pub const SunSpiritGearComponent = struct {
    Disabled: ?bool = null,
    PerformConfig: ?EmptyObj = null,
};

pub const SunSpiritLauncherComponent = struct {
    AreaId: ?i64 = null,
    NeedsCount: ?i64 = null,
    SunSpiritGearId: ?i64 = null,
};

pub const SystemModuleDataSyncComponent = struct {
    SystemModuleData: ?EmptyObj = null,
};

pub const TargetGearComponent = struct {
    CycleInterval: ?i64 = null,
    CycleStates: ?[]const []const u8 = null,
    Disabled: ?bool = null,
    HitBullet: ?EmptyObj = null,
    HitCd: ?f64 = null,
    HitLogicType: ?EmptyObj = null,
    IsCycle: ?bool = null,
    Patrol: ?EmptyObj = null,
    Type: ?[]const u8 = null,
    VictimHitTimeScaleRatio: ?EmptyObj = null,
};

pub const TargetGearGroupComponent = struct {
    Disabled: ?bool = null,
    Type: ?[]const u8 = null,
};

pub const TemplateEntitySpawnerComponent = struct {
    ClearConditions: ?EmptyObj = null,
    SpawnConditions: ?EmptyObj = null,
    SpawnConfig: ?EmptyObj = null,
};

pub const TimeScheduleComponent = struct {
};

pub const TimeStopComponent = struct {
    ActiveState: ?[]const u8 = null,
    StopTime: ?i64 = null,
    Target: ?EmptyObj = null,
};

pub const TimelineControlPoint = struct {
    State: ?[]const u8 = null,
};

pub const TimelineControlConfig = struct {
    Type: ?[]const u8 = null,
    EntityId: ?i64 = null,
    ControlPoints: []const TimelineControlPoint = &.{},
};

pub const TimelineControlGroup = struct {
    SegmentTime: ?f64 = null,
    ControlConfigs: []const TimelineControlConfig = &.{},
};

pub const TimelineTrackControlComponent = struct {
    Disabled: ?bool = null,
    ControlGroups: []const TimelineControlGroup = &.{},
};

pub const TrampleComponent = struct {
    Condition: ?EmptyObj = null,
    Disabled: ?bool = null,
    DownTime: ?f64 = null,
    IsResetGear: ?bool = null,
    Match: ?EmptyObj = null,
    ShowLandTipRadius: ?EmptyObj = null,
    StayTime: ?f64 = null,
    StopTeleControlMove: ?bool = null,
    TrampleTipOption: ?EmptyObj = null,
    UpTime: ?i64 = null,
};

pub const TurntableControlComponent = struct {
    Config: ?EmptyObj = null,
    Disabled: ?bool = null,
};

pub const VehicleComponent = struct {
    Config: ?[]const u8 = null,
    Disabled: ?bool = null,
    DriverSeat: ?i64 = null,
    RoleRiddingTag: ?[]const i64 = null,
    SeatCount: ?i64 = null,
    TeleportPlayersWhenDestroyed: ?EmptyObj = null,
    VehicleBornTag: ?[]const i64 = null,
    VehicleRiddenTag: ?[]const i64 = null,
    VehicleTypedConfig: ?EmptyObj = null,
};

pub const VisionComponent = struct {
    Disabled: ?bool = null,
    VisionId: ?i64 = null,
};

pub const VisionDisplayComponent = struct {
    Condition: ?EmptyObj = null,
    Disabled: ?bool = null,
    IsBanOcclusion: ?bool = null,
    IsBornInTargetPoint: ?bool = null,
    IsNeedInView: ?bool = null,
    ViewEntityId: ?i64 = null,
};

pub const VisionItemComponent = struct {
};

pub const WalkingPatternComponent = struct {
    Disabled: ?bool = null,
    EndEntityId: ?i64 = null,
    ReplaySpineEffect: ?[]const u8 = null,
    ScoreVar: ?EmptyObj = null,
    SpineEffectExistDuration: ?i64 = null,
    SplineEntityId: ?i64 = null,
    StayAwayFailConfig: ?EmptyObj = null,
};

pub const WeaponComponent = struct {
    WeaponId: ?i64 = null,
};

pub const WindSourceComponent = struct {
    WindSource: ?EmptyObj = null,
};

pub const WuWaGoComponent = struct {
    OpenAddClientTags: ?[]const i64 = null,
    WuWaGoEntity: ?EmptyObj = null,
};

pub const Entry = union(enum) {
    AdsorbComponent: *const AdsorbComponent,
    AdviseItemComponent: *const AdviseItemComponent,
    AiAlertNotifyComponent: *const AiAlertNotifyComponent,
    AiGearStrategyComponent: *const AiGearStrategyComponent,
    AirPassageComponent: *const AirPassageComponent,
    AirWallSpawnerComponent: *const AirWallSpawnerComponent,
    AnimalComponent: *const AnimalComponent,
    AttachTargetComponent: *const AttachTargetComponent,
    BatchBulletCasterComponent: *const BatchBulletCasterComponent,
    BeamCastComponent: *const BeamCastComponent,
    BeamReceiveComponent: *const BeamReceiveComponent,
    BubbleComponent: *const BubbleComponent,
    BuffConsumerComponent: *const BuffConsumerComponent,
    BuffProducerComponent: *const BuffProducerComponent,
    CharacterConnectorComponent: *const CharacterConnectorComponent,
    ChessmanComponent: *const ChessmanComponent,
    ClientConditionListenerComponent: *const ClientConditionListenerComponent,
    ClientTriggerComponent: *const ClientTriggerComponent,
    CombatComponent: *const CombatComponent,
    CombinedVisibleGroupComponent: *const CombinedVisibleGroupComponent,
    ConnectorComponent: *const ConnectorComponent,
    ConveyorBeltComponent: *const ConveyorBeltComponent,
    CurveControlComponent: *const CurveControlComponent,
    DollGrabMachineComponent: *const DollGrabMachineComponent,
    DollGrabShowcaseComponent: *const DollGrabShowcaseComponent,
    DragActorPlayComponent: *const DragActorPlayComponent,
    DropComponent: *const DropComponent,
    DungeonEntryComponent: *const DungeonEntryComponent,
    DynamicEntityRewardComponent: *const DynamicEntityRewardComponent,
    DynamicPortalCreatorComponent: *const DynamicPortalCreatorComponent,
    DynamicSpawnMonsterComponent: *const DynamicSpawnMonsterComponent,
    DynamicSpawnMonsterPointComponent: *const DynamicSpawnMonsterPointComponent,
    EditCustomAoiComponent: *const EditCustomAoiComponent,
    EffectAreaComponent: *const EffectAreaComponent,
    ElementalBoxFoundationComponent: *const ElementalBoxFoundationComponent,
    ElementalEffectComponent: *const ElementalEffectComponent,
    ElementalHolderComponent: *const ElementalHolderComponent,
    ElementalWellComponent: *const ElementalWellComponent,
    EnrichmentAreaComponent: *const EnrichmentAreaComponent,
    EntityBatchRefreshComponent: *const EntityBatchRefreshComponent,
    EntityBundleComponent: *const EntityBundleComponent,
    EntityCustomAudioComponent: *const EntityCustomAudioComponent,
    EntityGroupComponent: *const EntityGroupComponent,
    EntityListComponent: *const EntityListComponent,
    EntityPackageComponent: *const EntityPackageComponent,
    EntityStateAudioComponent: *const EntityStateAudioComponent,
    EntityVisibleComponent: *const EntityVisibleComponent,
    ExhibitComponent: *const ExhibitComponent,
    ExploreSkillInteractComponent: *const ExploreSkillInteractComponent,
    ExploreSkillInteractGroupComponent: *const ExploreSkillInteractGroupComponent,
    ExtraRangeComponent: *const ExtraRangeComponent,
    FanComponent: *const FanComponent,
    FightInteractComponent: *const FightInteractComponent,
    FlowerPollutionComponent: *const FlowerPollutionComponent,
    FollowDecorationComponent: *const FollowDecorationComponent,
    FollowShooterComponent: *const FollowShooterComponent,
    FollowTrackComponent: *const FollowTrackComponent,
    FurnitureSlotComponent: *const FurnitureSlotComponent,
    GameplayDataPersistenceComponent: *const GameplayDataPersistenceComponent,
    GodKingFrequencyControllerComponent: *const GodKingFrequencyControllerComponent,
    Graph3dComponent: *const Graph3dComponent,
    Graph3dEffectComponent: *const Graph3dEffectComponent,
    GridObjectComponent: *const GridObjectComponent,
    GroupAiComponent: *const GroupAiComponent,
    GuideLineCreatorComponent: *const GuideLineCreatorComponent,
    HackManagementComponent: *const HackManagementComponent,
    HitComponent: *const HitComponent,
    HookLockPoint: *const HookLockPoint,
    IndependentSystemRewardComponent: *const IndependentSystemRewardComponent,
    InhalationAbilityComponent: *const InhalationAbilityComponent,
    InhaledItemComponent: *const InhaledItemComponent,
    InteractAudioComponent: *const InteractAudioComponent,
    InteractGearComponent: *const InteractGearComponent,
    ItemFoundation2: *const ItemFoundation2,
    JigsawFoundation: *const JigsawFoundation,
    JigsawItem: *const JigsawItem,
    LevelAiComponent: *const LevelAiComponent,
    LevelPrefabPerformComponent: *const LevelPrefabPerformComponent,
    LevelQteComponent: *const LevelQteComponent,
    LevelSequenceFrameEventComponent: *const LevelSequenceFrameEventComponent,
    LevitateMagnetComponent: *const LevitateMagnetComponent,
    LifePointCenterComponent: *const LifePointCenterComponent,
    LocationSafetyComponent: *const LocationSafetyComponent,
    MonitorComponent: *const MonitorComponent,
    MonsterGachaBaseComponent: *const MonsterGachaBaseComponent,
    MonsterGachaItemComponent: *const MonsterGachaItemComponent,
    MotorSlideComponent: *const MotorSlideComponent,
    NearbyTrackingComponent: *const NearbyTrackingComponent,
    NpcPerformComponent: *const NpcPerformComponent,
    PasserbyNpcSpawnComponent: *const PasserbyNpcSpawnComponent,
    PathDrivenActorSpawnerComponent: *const PathDrivenActorSpawnerComponent,
    PerformanceOptimizationComponent: *const PerformanceOptimizationComponent,
    PhotoTargetComponent: *const PhotoTargetComponent,
    PhysicsConstraintComponent: *const PhysicsConstraintComponent,
    PickInteractComponent: *const PickInteractComponent,
    ProceduralMaterialComponent: *const ProceduralMaterialComponent,
    ProceduralVisualComponent: *const ProceduralVisualComponent,
    ProgressBarControlComponent: *const ProgressBarControlComponent,
    ProjectorComponent: *const ProjectorComponent,
    PullingObjectFoundation: *const PullingObjectFoundation,
    QuickHackComponent: *const QuickHackComponent,
    ReboundComponent: *const ReboundComponent,
    RenderSpecifiedRangeComponent: *const RenderSpecifiedRangeComponent,
    ResetEntitiesPosComponent: *const ResetEntitiesPosComponent,
    ResetSelfPosComponent: *const ResetSelfPosComponent,
    ResurrectionComponent: *const ResurrectionComponent,
    RoadNetworkNavigationComponent: *const RoadNetworkNavigationComponent,
    RollBlockComponent: *const RollBlockComponent,
    RollBlockFloorComponent: *const RollBlockFloorComponent,
    RollBlockItemComponent: *const RollBlockItemComponent,
    RotatorComponent2: *const RotatorComponent2,
    SceneActorRefComponent: *const SceneActorRefComponent,
    SceneBulletComponent: *const SceneBulletComponent,
    SceneItemAiComponent: *const SceneItemAiComponent,
    SceneItemAttributeComponent: *const SceneItemAttributeComponent,
    SceneItemEventListenerComponent: *const SceneItemEventListenerComponent,
    SceneTextComponent: *const SceneTextComponent,
    SimpleCombatComponent: *const SimpleCombatComponent,
    SkyboxComponent: *const SkyboxComponent,
    SlidePerformComponent: *const SlidePerformComponent,
    SlideRailComponent: *const SlideRailComponent,
    SpawnMonsterComponent: *const SpawnMonsterComponent,
    StateHintComponent: *const StateHintComponent,
    StreamingSourceTriggerPointComponent: *const StreamingSourceTriggerPointComponent,
    SunSpiritCollectComponent: *const SunSpiritCollectComponent,
    SunSpiritGearComponent: *const SunSpiritGearComponent,
    SunSpiritLauncherComponent: *const SunSpiritLauncherComponent,
    SystemModuleDataSyncComponent: *const SystemModuleDataSyncComponent,
    TargetGearComponent: *const TargetGearComponent,
    TargetGearGroupComponent: *const TargetGearGroupComponent,
    TemplateEntitySpawnerComponent: *const TemplateEntitySpawnerComponent,
    TimeScheduleComponent: *const TimeScheduleComponent,
    TimeStopComponent: *const TimeStopComponent,
    TimelineTrackControlComponent: *const TimelineTrackControlComponent,
    TrampleComponent: *const TrampleComponent,
    TurntableControlComponent: *const TurntableControlComponent,
    VehicleComponent: *const VehicleComponent,
    VisionComponent: *const VisionComponent,
    VisionDisplayComponent: *const VisionDisplayComponent,
    VisionItemComponent: *const VisionItemComponent,
    WalkingPatternComponent: *const WalkingPatternComponent,
    WeaponComponent: *const WeaponComponent,
    WindSourceComponent: *const WindSourceComponent,
    WuWaGoComponent: *const WuWaGoComponent,
};

test "HookLockPoint HookInteractConfig parses HookActions, FinishActions, and ExitHookActions" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const alloc = arena.allocator();
    const parsed = try std.json.parseFromSliceLeaky(HookLockPoint, alloc,
        \\{
        \\  "Type": {
        \\    "Type": "KiteHook",
        \\    "UseHookInteract": true
        \\  },
        \\  "HookInteractConfig": {
        \\    "HookActions": [{ "Name": "MoveWithSpline", "Params": { "SplineEntityId": 114900196 } }],
        \\    "FinishActions": [{ "Name": "LeisureInteract" }],
        \\    "ExitHookActions": [{ "Name": "StopNewMoveWithSpline" }]
        \\  }
        \\}
    , .{ .ignore_unknown_fields = true });
    const hic = parsed.HookInteractConfig.?;
    try std.testing.expectEqual(@as(usize, 1), hic.HookActions.?.len);
    try std.testing.expectEqual(@as(usize, 1), hic.FinishActions.?.len);
    try std.testing.expectEqual(@as(usize, 1), hic.ExitHookActions.?.len);
}
