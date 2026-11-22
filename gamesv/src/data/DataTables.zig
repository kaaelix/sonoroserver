const DataTables = @This();
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;
const pb = @import("proto").pb;
const WeaponItem = @import("../fs/WeaponItem.zig");
pub const Config = @import("zon/config.zon");
const ConcertoExceptions: []const i32 = @import("zon/concerto_exceptions.zon");

pub const RoleInfo = @import("tables/RoleInfo.zig");
pub const RoleBeHitMap = @import("tables/RoleBeHitMap.zig");
pub const MainRoleConfig = @import("tables/MainRoleConfig.zig");
pub const BaseProperty = @import("tables/BaseProperty.zig");
pub const FunctionCondition = @import("tables/FunctionCondition.zig");
pub const RolePropertyGrowth = @import("tables/RolePropertyGrowth.zig");
pub const MonsterPropertyGrowth = @import("tables/MonsterPropertyGrowth.zig");
pub const InstanceDungeon = @import("tables/InstanceDungeon.zig");
pub const Buff = @import("tables/Buff.zig");
pub const Shield = @import("tables/Shield.zig");
pub const SummonCfg = @import("tables/SummonCfg.zig");
pub const ModelConfigPreload = @import("tables/ModelConfigPreload.zig");
pub const CharacterPartConfig = @import("tables/CharacterPartConfig.zig");
pub const GameplayTagParent = @import("tables/GameplayTagParent.zig");
pub const GameplayTagRemap = @import("tables/GameplayTagRemap.zig");
pub const BlueprintConfig = @import("tables/BlueprintConfig.zig");
pub const RoleSkin = @import("tables/RoleSkin.zig");
pub const FlySkinConfig = @import("tables/FlySkinConfig.zig");
pub const WeaponSkin = @import("tables/WeaponSkin.zig");
pub const Ornament = @import("tables/Ornament.zig");
pub const TemplateConfig = @import("tables/TemplateConfig.zig");
pub const ExploreTools = @import("tables/ExploreTools.zig");
pub const WeaponConf = struct {
    ItemId: i32,
    QualityId: i32,
    WeaponType: i32 = 0,
    ResonId: i32,
    ResonLevelLimit: i32,
    LevelId: i32,
    BreachId: i32,
    FirstPropId: @import("tables/PropValue.zig"),
    FirstCurve: i32,
    SecondPropId: @import("tables/PropValue.zig"),
    SecondCurve: i32,
};
pub const WeaponBaseAttributes = @import("tables/WeaponBaseAttributes.zig");
pub const WeaponPropertyGrowth = @import("tables/WeaponPropertyGrowth.zig");
pub const WeaponReson = struct {
    Id: i32,
    ResonId: i32,
    Level: i32,
    Effect: []i64,
    Consume: i32 = 1,
    GoldConsume: i32 = 0,
    MaterialPlaceType: i32 = 0,
    AlternativeConsume: []const i32 = &.{},
};
pub const ItemExchangeContent = struct {
    ItemId: i32,
    Times: i32 = 1,
    GainCount: i32 = 1,
    Consume: @import("tables/int_map.zig").IntMap(i32, i32) = .{},
};
pub const DragonPool = struct {
    Id: i32,
    CoreId: i32 = 0,
    Goal: []const i32 = &.{},
    DropIds: []const i32 = &.{},
    DarkCoastDeliveryList: []const i32 = &.{},
};
pub const LoadingLevelArea = struct {
    Id: i32,
    BeginTime: i64 = 0,
    EndTime: i64 = 0,
    IsLimitShow: bool = false,
};
pub const WorldNewJourney = struct {
    Id: i32,
    ConditionId: i32 = 0,
};
pub const DarkCoastDelivery = struct {
    Id: i32,
    LevelPlayId: i32 = 0,
};
pub const NewTowerSeasonAward = struct {
    Id: i32,
    SeasonId: i32 = 0,
    CycleId: i32 = 0,
    Index: i32 = 0,
    DropId: i32 = 0,
};
pub const ExchangeReward = struct {
    Id: i32,
    SharedId: i32 = 0,
    MaxCount: i32 = 0,
};
pub const LoopTeleportVolumeFunction = struct {
    Id: i32,
    VolumeId: i32 = 0,
    Conditions: ?std.json.Value = null,
    Actions: ?std.json.Value = null,
};
pub const RecallConfig = struct {
    RecallId: i32,
    DungeonEntranceId: i32 = 0,
    DungeonIds: []const i32 = &.{},
};
pub const BossRushBuff = struct {
    Id: i32,
};
pub const InstanceDungeonEntrance = struct {
    Id: i32,
    InstanceDungeonList: []const i32 = &.{},
};
pub const ConsumptiveTask = struct {
    Id: i32,
    MaxFinishCount: i32 = 1,
    RewardScore: i32 = 0,
    TaskTab: i32 = 0,
};
pub const DollGrabItemsCsv = struct {
    Id: i32,
    ShowcaseBelongType: []const u8 = "",
    DropId: i32 = 0,
    BindingItemId: i32 = 0,
};
pub const TrackMoonEntrust = struct {
    Id: i32,
    Star: i32 = 1,
    InvestLimit: i32 = 500,
    Consume: @import("tables/int_map.zig").IntMap(i32, i32) = .{},
};
pub const MotorDecalQuest = struct {
    TaskId: i32,
    Ip: i32 = 0,
};
pub const MotorLinkageQuest = struct {
    TaskId: i32,
    Ip: i32 = 0,
};
pub const QuestRefVideoConfig = struct {
    Id: i32,
    QuestId: i32,
    GirlOrBoy: i32 = 0,
    PakName: []const u8 = "",
    OnlineBranch: []const u8 = "",
};

pub const QuestRefMapBlockConfig = struct {
    QuestId: i32,
    MapBlockId: []const i32 = &.{},
    OnlineBranch: []const u8 = "",
};

pub const QuestResourceWhite = struct {
    QuestId: i32,
};

pub const AbnormalDamageConfig = struct {
    Level: i32,
    Abnormal1001: i32 = 0,
    Abnormal1002: i32 = 0,
    Abnormal1003: i32 = 0,
    Abnormal1004: i32 = 0,
    Abnormal1005: i32 = 0,
    Abnormal1006: i32 = 0,
};
pub const BuffItem = struct {
    Id: i32,
    Buffs: []const i64 = &.{},
    Cd: i32 = 0,
};
pub const WeaponExpItem = @import("tables/WeaponExpItem.zig");
pub const WeaponLevel = @import("tables/WeaponLevel.zig");
pub const WeaponBreach = @import("tables/WeaponBreach.zig");
pub const RoleExpItem = @import("tables/RoleExpItem.zig");
pub const RoleLevelConsume = @import("tables/RoleLevelConsume.zig");
pub const RoleBreach = @import("tables/RoleBreach.zig");
pub const SkillLevel = @import("tables/SkillLevel.zig");
pub const LevelEntityConfig = @import("tables/LevelEntityConfig.zig");
pub const Damage = @import("tables/Damage.zig");
pub const Area = @import("tables/Area.zig");
pub const InfrV2TreeBuild = @import("tables/InfrV2TreeBuild.zig");
pub const InfrRoadBuild = @import("tables/InfrRoadBuild.zig");
pub const Teleporter = @import("tables/Teleporter.zig");
pub const MapMark = @import("tables/MapMark.zig");
pub const MonsterDetection = @import("tables/MonsterDetection.zig");
pub const DungeonDetection = @import("tables/DungeonDetection.zig");
pub const SilentAreaDetection = @import("tables/SilentAreaDetection.zig");
pub const LordGym = @import("tables/LordGym.zig");
pub const LordGymEntrance = @import("tables/LordGymEntrance.zig");
pub const AdventureTask = @import("tables/AdventureTask.zig");
pub const AdventureTaskChapter = @import("tables/AdventureTaskChapter.zig");
pub const GiftPackage = @import("tables/GiftPackage.zig");
pub const Activity = @import("tables/Activity.zig");
pub const AbyssLittleRole = @import("tables/AbyssLittleRole.zig");
pub const MapFog = @import("tables/MapFog.zig");
pub const MultiMap = @import("tables/MultiMap.zig");
pub const MapBlockInfo = @import("tables/MapBlockInfo.zig");
pub const ExploreProgressReward = @import("tables/ExploreProgressReward.zig");
pub const ExploreReward = @import("tables/ExploreReward.zig");
pub const ExploreScore = @import("tables/ExploreScore.zig");
pub const Flow = @import("tables/Flow.zig");
pub const Skill = @import("tables/Skill.zig");
pub const SkillTree = @import("tables/SkillTree.zig");
pub const ResonantChain = @import("tables/ResonantChain.zig");
pub const RogueResCharacterBuff = @import("tables/RogueResCharacterBuff.zig");
pub const HonamiStoryEffect = @import("tables/HonamiStoryEffect.zig");
pub const ComboTeaching = @import("tables/ComboTeaching.zig");
pub const EntitySkillPreload = @import("tables/EntitySkillPreload.zig");
pub const CommonSkillPreload = @import("tables/CommonSkillPreload.zig");
pub const PassiveSkill = @import("tables/PassiveSkill.zig");
pub const FavorWord = @import("tables/FavorWord.zig");
pub const FavorStory = @import("tables/FavorStory.zig");
pub const FavorGoods = @import("tables/FavorGoods.zig");
pub const Motion = @import("tables/Motion.zig");
pub const GuideGroup = @import("tables/GuideGroup.zig");
pub const CharacterInitInfo = @import("tables/CharacterInitInfo.zig");
pub const SkillBranchBuff = @import("tables/SkillBranchBuff.zig");
pub const PhonographAlbum = @import("tables/PhonographAlbum.zig");
pub const PhonographMusic = @import("tables/PhonographMusic.zig");
pub const BackgroundCard = @import("tables/BackgroundCard.zig");
pub const PlayerHeadRe = @import("tables/PlayerHeadRe.zig");
pub const PlayerTitle = @import("tables/PlayerTitle.zig");
pub const MotorTechTree = @import("tables/MotorTechTree.zig");
pub const MotorScene = @import("tables/MotorScene.zig");
pub const MotorLvl = @import("tables/MotorLvl.zig");
pub const MotorTech = @import("tables/MotorTech.zig");
pub const MotorTechLv = @import("tables/MotorTechLv.zig");
pub const MotorTask = @import("tables/MotorTask.zig");
pub const MotorSkin = @import("tables/MotorSkin.zig");
pub const MotorSticker = @import("tables/MotorSticker.zig");
pub const MotorStickerPart = @import("tables/MotorStickerPart.zig");
pub const MotorLoadProject = @import("tables/MotorLoadProject.zig");
pub const MotorGeneralPreview = @import("tables/MotorGeneralPreview.zig");
pub const MotorDecalIp = @import("tables/MotorDecalIp.zig");
pub const MotorLinkageIp = @import("tables/MotorLinkageIp.zig");
pub const MotorFrame = @import("tables/MotorFrame.zig");
pub const MotorDecorations = @import("tables/MotorDecorations.zig");
pub const MotorDecorationsPart = @import("tables/MotorDecorationsPart.zig");
pub const Gacha = @import("tables/Gacha.zig");
pub const GachaPool = @import("tables/GachaPool.zig");
pub const GachaViewInfo = @import("tables/GachaViewInfo.zig");
pub const GachaTextureInfo = @import("tables/GachaTextureInfo.zig");
pub const AiStateMachineConfig = @import("tables/AiStateMachineConfig.zig");
pub const AiBase = @import("tables/AiBase.zig");
pub const PhantomItem = @import("tables/PhantomItem.zig");
pub const PhantomSkill = @import("tables/PhantomSkill.zig");
pub const PhantomSummonGroup = @import("tables/PhantomSummonGroup.zig");
pub const PhantomFetter = @import("tables/PhantomFetter.zig");
pub const PhantomFetterGroup = @import("tables/PhantomFetterGroup.zig");
pub const PhantomMainProperty = @import("tables/PhantomMainProperty.zig");
pub const PhantomMainPropItem = @import("tables/PhantomMainPropItem.zig");
pub const PhantomSubProperty = @import("tables/PhantomSubProperty.zig");
pub const PhantomRarity = @import("tables/PhantomRarity.zig");
pub const PhantomQuality = @import("tables/PhantomQuality.zig");
pub const PhantomGrowth = @import("tables/PhantomGrowth.zig");
pub const PhantomLevel = @import("tables/PhantomLevel.zig");
pub const PhantomExpItem = @import("tables/PhantomExpItem.zig");
pub const PhantomVicePolishConfig = @import("tables/PhantomVicePolishConfig.zig");
pub const CalabashLevel = @import("tables/CalabashLevel.zig");
pub const CalabashDevelopCondition = @import("tables/CalabashDevelopCondition.zig");
pub const CalabashDevelopReward = @import("tables/CalabashDevelopReward.zig");
pub const CalabashSkin = @import("tables/CalabashSkin.zig");
pub const DropPackage = @import("tables/DropPackage.zig");
pub const DropGroup = @import("tables/DropGroup.zig");
pub const PlayerExp = @import("tables/PlayerExp.zig");
pub const ItemInfo = @import("tables/ItemInfo.zig");
pub const Condition = @import("tables/Condition.zig");
pub const ConditionGroup = @import("tables/ConditionGroup.zig");
pub const QuestData = @import("tables/QuestData.zig");
pub const LevelPlayData = @import("tables/LevelPlayData.zig");
pub const DailyTask = @import("tables/DailyTask.zig");
pub const DailyTaskGroup = @import("tables/DailyTaskGroup.zig");
pub const DataLayerConfig = @import("tables/DataLayerConfig.zig");
pub const Weather = @import("tables/Weather.zig");
pub const RoleTrialInfo = @import("tables/RoleTrialInfo.zig");
pub const InstanceTrialRoleConfig = @import("tables/InstanceTrialRoleConfig.zig");
pub const TrialRoleInfo = @import("tables/TrialRoleInfo.zig");
pub const TrialWeaponInfo = @import("tables/TrialWeaponInfo.zig");
pub const TrialPhantomPropItem = @import("tables/TrialPhantomPropItem.zig");
pub const TrailPhantomProp = @import("tables/TrailPhantomProp.zig");
pub const PropValue = @import("tables/PropValue.zig");
pub const PayShop = @import("tables/PayShop.zig");
pub const PayShopTab = @import("tables/PayShopTab.zig");
pub const PayShopRecommend = @import("tables/PayShopRecommend.zig");
pub const ShopInfo = @import("tables/ShopInfo.zig");
pub const ShopFixed = @import("tables/ShopFixed.zig");
pub const ForgeFormula = @import("tables/ForgeFormula.zig");
pub const SynthesisFormula = @import("tables/SynthesisFormula.zig");
pub const SynthesisLevel = @import("tables/SynthesisLevel.zig");
pub const CookFormula = @import("tables/CookFormula.zig");
pub const CookLevel = @import("tables/CookLevel.zig");
pub const CookProcessed = @import("tables/CookProcessed.zig");
pub const Achievement = @import("tables/Achievement.zig");
pub const AchievementGroup = @import("tables/AchievementGroup.zig");
pub const AchievementStarLevel = @import("tables/AchievementStarLevel.zig");
pub const Liveness = @import("tables/Liveness.zig");
pub const LivenessTask = @import("tables/LivenessTask.zig");
pub const TowerConfig = @import("tables/TowerConfig.zig");
pub const TowerDifficulty = @import("tables/TowerDifficulty.zig");
pub const TowerSeasonReward = @import("tables/TowerSeasonReward.zig");
pub const FishingTech = @import("tables/FishingTech.zig");
pub const FishingReputation = @import("tables/FishingReputation.zig");
pub const FishingItem = @import("tables/FishingItem.zig");
pub const ElementalBoxDefine = @import("tables/ElementalBoxDefine.zig");
pub const GiantItem = @import("tables/GiantItem.zig");
pub const GiantDeliveryReward = @import("tables/GiantDeliveryReward.zig");

pub const GameplayTagParentTable = Table(GameplayTagParent, "Id");
pub const GameplayTagRemapTable = Table(GameplayTagRemap, "RawId");

arena: ArenaAllocator,
role_info: Table(RoleInfo, "Id"),
role_be_hit_map: Table(RoleBeHitMap, "Id"),
main_role_config: Table(MainRoleConfig, "Id"),
base_property: Table(BaseProperty, "Id"),
function_condition: Table(FunctionCondition, "FunctionId"),
role_property_growth: Table(RolePropertyGrowth, "Id"),
monster_property_growth: Table(MonsterPropertyGrowth, "Id"),
instance_dungeon: Table(InstanceDungeon, "Id"),
buff: Table(Buff, "Id"),
shield: Table(Shield, "Id"),
summon_cfg: Table(SummonCfg, "Id"),
model_config_preload: Table(ModelConfigPreload, "Id"),
character_part_config: Table(CharacterPartConfig, "ModelId"),
gameplay_tag_parent: GameplayTagParentTable,
gameplay_tag_remap: GameplayTagRemapTable,
blueprint_config: Table(BlueprintConfig, "Id"),
role_skin: Table(RoleSkin, "Id"),
fly_skin_config: Table(FlySkinConfig, "Id"),
weapon_skin: Table(WeaponSkin, "Id"),
ornament: Table(Ornament, "Id"),
template_config: Table(TemplateConfig, "Id"),
explore_tools: Table(ExploreTools, "PhantomSkillId"),
weapon_conf: Table(WeaponConf, "ItemId"),
weapon_base_attributes: Table(WeaponBaseAttributes, "ItemId"),
weapon_property_growth: Table(WeaponPropertyGrowth, "Id"),
weapon_reson: Table(WeaponReson, "Id"),
weapon_exp_item: Table(WeaponExpItem, "Id"),
weapon_level: Table(WeaponLevel, "Id"),
weapon_breach: Table(WeaponBreach, "Id"),
role_exp_item: Table(RoleExpItem, "Id"),
role_level_consume: Table(RoleLevelConsume, "Id"),
role_breach: Table(RoleBreach, "Id"),
skill_level: Table(SkillLevel, "Id"),
level_entity_config: Table(LevelEntityConfig, "EntityId"),
damage: Table(Damage, "Id"),
area: Table(Area, "AreaId"),
infr_v2_tree_build: Table(InfrV2TreeBuild, "Id"),
infr_road_build: Table(InfrRoadBuild, "Id"),
teleporter: Table(Teleporter, "Id"),
activity: Table(Activity, "Id"),
map_fog: Table(MapFog, "Fog"),
multi_map: Table(MultiMap, "Id"),
map_block_info: Table(MapBlockInfo, "BlockId"),
explore_progress_reward: Table(ExploreProgressReward, "Id"),
explore_reward: Table(ExploreReward, "Id"),
explore_score: Table(ExploreScore, "Area"),
flow: Table(Flow, "Id"),
skill: Table(Skill, "Id"),
skill_tree: Table(SkillTree, "Id"),
resonant_chain: Table(ResonantChain, "Id"),
rogue_res_buff: Table(RogueResCharacterBuff, "Id"),
honami_story_effect: Table(HonamiStoryEffect, "Id"),
combo_teaching: Table(ComboTeaching, "Id"),
entity_skill_preload: Table(EntitySkillPreload, "Id"),
common_skill_preload: Table(CommonSkillPreload, "Id"),
passive_skill: Table(PassiveSkill, "Id"),
favor_word: Table(FavorWord, "Id"),
favor_story: Table(FavorStory, "Id"),
favor_goods: Table(FavorGoods, "Id"),
motion: Table(Motion, "Id"),
guide_group: Table(GuideGroup, "Id"),
char_init_info: Table(CharacterInitInfo, "RoleId"),
skill_branch_buff: Table(SkillBranchBuff, "BranchId"),
phonograph_album: Table(PhonographAlbum, "Id"),
phonograph_music: Table(PhonographMusic, "Id"),
background_card: Table(BackgroundCard, "Id"),
player_head_re: Table(PlayerHeadRe, "Id"),
player_title: Table(PlayerTitle, "Id"),
motor_tech_tree: Table(MotorTechTree, "Id"),
motor_scene: Table(MotorScene, "Id"),
motor_lvl: Table(MotorLvl, "Level"),
motor_tech: Table(MotorTech, "Id"),
motor_tech_lv: Table(MotorTechLv, "Id"),
motor_task: Table(MotorTask, "Id"),
motor_skin: Table(MotorSkin, "Id"),
motor_sticker: Table(MotorSticker, "Id"),
motor_sticker_part: Table(MotorStickerPart, "Id"),
motor_load_project: Table(MotorLoadProject, "Id"),
motor_general_preview: Table(MotorGeneralPreview, "Id"),
motor_decal_ip: Table(MotorDecalIp, "Id"),
motor_linkage_ip: Table(MotorLinkageIp, "Id"),
motor_frame: Table(MotorFrame, "Id"),
motor_decorations: Table(MotorDecorations, "Id"),
motor_decorations_part: Table(MotorDecorationsPart, "Id"),
gacha: Table(Gacha, "Id"),
gacha_pool: Table(GachaPool, "Id"),
gacha_view_info: Table(GachaViewInfo, "Id"),
gacha_texture_info: Table(GachaTextureInfo, "Id"),
ai_state_machine_config: Table(AiStateMachineConfig, "Id"),
ai_base: Table(AiBase, "Id"),
phantom_item: Table(PhantomItem, "ItemId"),
phantom_skill: Table(PhantomSkill, "PhantomSkillId"),
phantom_summon_group: Table(PhantomSummonGroup, "Id"),
phantom_fetter: Table(PhantomFetter, "Id"),
phantom_fetter_group: Table(PhantomFetterGroup, "Id"),
phantom_main_property: Table(PhantomMainProperty, "Id"),
phantom_main_prop_item: Table(PhantomMainPropItem, "Id"),
phantom_sub_property: Table(PhantomSubProperty, "Id"),
phantom_rarity: Table(PhantomRarity, "Rare"),
phantom_quality: Table(PhantomQuality, "Quality"),
phantom_growth: Table(PhantomGrowth, "Id"),
phantom_level: Table(PhantomLevel, "Id"),
phantom_exp_item: Table(PhantomExpItem, "ItemId"),
phantom_vice_polish_config: Table(PhantomVicePolishConfig, "PropCount"),
calabash_level: Table(CalabashLevel, "Level"),
calabash_develop_condition: Table(CalabashDevelopCondition, "Id"),
calabash_develop_reward: Table(CalabashDevelopReward, "MonsterId"),
calabash_skin: Table(CalabashSkin, "Id"),
drop_package: Table(DropPackage, "Id"),
drop_group: Table(DropGroup, "UnitId"),
player_exp: Table(PlayerExp, "PlayerLevel"),
item_info: Table(ItemInfo, "Id"),
gift_package: Table(GiftPackage, "Id"),
condition: Table(Condition, "Id"),
condition_group: Table(ConditionGroup, "Id"),
quest_data: Table(QuestData, "QuestId"),
level_play_data: Table(LevelPlayData, "LevelPlayId"),
map_mark: Table(MapMark, "MarkId"),
monster_detection: Table(MonsterDetection, "Id"),
dungeon_detection: Table(DungeonDetection, "Id"),
silent_area_detection: Table(SilentAreaDetection, "Id"),
lord_gym: Table(LordGym, "Id"),
lord_gym_entrance: Table(LordGymEntrance, "Id"),
adventure_task: Table(AdventureTask, "Id"),
adventure_task_chapter: Table(AdventureTaskChapter, "Id"),
daily_task: Table(DailyTask, "Id"),
daily_task_group: Table(DailyTaskGroup, "Id"),
data_layer_config: Table(DataLayerConfig, "Id"),
weather: Table(Weather, "Id"),
role_trial_info: Table(RoleTrialInfo, "Id"),
instance_trial_role_config: Table(InstanceTrialRoleConfig, "Id"),
trial_role_info: Table(TrialRoleInfo, "Id"),
trial_weapon_info: Table(TrialWeaponInfo, "Id"),
trial_phantom_prop_item: Table(TrialPhantomPropItem, "Id"),
trail_phantom_prop: Table(TrailPhantomProp, "Id"),
pay_shop: Table(PayShop, "Id"),
pay_shop_tab: Table(PayShopTab, "TabId"),
pay_shop_recommend: Table(PayShopRecommend, "Id"),
shop_info: Table(ShopInfo, "Id"),
shop_fixed: Table(ShopFixed, "Id"),
forge_formula: Table(ForgeFormula, "Id"),
synthesis_formula: Table(SynthesisFormula, "Id"),
synthesis_level: Table(SynthesisLevel, "Id"),
cook_formula: Table(CookFormula, "Id"),
cook_level: Table(CookLevel, "Id"),
cook_processed: Table(CookProcessed, "Id"),
achievement: Table(Achievement, "Id"),
achievement_group: Table(AchievementGroup, "Id"),
achievement_star_level: Table(AchievementStarLevel, "Level"),
liveness: Table(Liveness, "Id"),
liveness_task: Table(LivenessTask, "TaskId"),
tower_config: Table(TowerConfig, "Id"),
tower_difficulty: Table(TowerDifficulty, "Difficulty"),
tower_season_reward: Table(TowerSeasonReward, "Id"),
fishing_tech: Table(FishingTech, "Id"),
fishing_reputation: Table(FishingReputation, "Level"),
fishing_item: Table(FishingItem, "Id"),
elemental_box_define: Table(ElementalBoxDefine, "Id"),
giant_item: Table(GiantItem, "Id"),
giant_delivery_reward: Table(GiantDeliveryReward, "Id"),
abyss_little_role: Table(AbyssLittleRole, "Id"),
item_exchange_content: Table(ItemExchangeContent, "ItemId"),
dragon_pool: Table(DragonPool, "Id"),
loading_level_area: Table(LoadingLevelArea, "Id"),
world_new_journey: Table(WorldNewJourney, "Id"),
dark_coast_delivery: Table(DarkCoastDelivery, "Id"),
new_tower_season_award: Table(NewTowerSeasonAward, "Id"),
exchange_reward: Table(ExchangeReward, "Id"),
loop_teleport_volume_function: Table(LoopTeleportVolumeFunction, "Id"),
recall_config: Table(RecallConfig, "RecallId"),
boss_rush_buff: Table(BossRushBuff, "Id"),
instance_dungeon_entrance: Table(InstanceDungeonEntrance, "Id"),
consumptive_task: Table(ConsumptiveTask, "Id"),
doll_grab_items_csv: Table(DollGrabItemsCsv, "Id"),
track_moon_entrust: Table(TrackMoonEntrust, "Id"),
motor_decal_quest: Table(MotorDecalQuest, "TaskId"),
motor_linkage_quest: Table(MotorLinkageQuest, "TaskId"),
    abnormal_damage_config: Table(AbnormalDamageConfig, "Level"),
    buff_item: Table(BuffItem, "Id"),
    quest_ref_video_config: Table(QuestRefVideoConfig, "Id"),
    quest_ref_map_block_config: Table(QuestRefMapBlockConfig, "QuestId"),
    quest_resource_white: Table(QuestResourceWhite, "QuestId"),

    fn loadTableItems(
    comptime T: type,
    gpa: Allocator,
    arena: *ArenaAllocator,
    io: Io,
    base_path: []const u8,
) !T {
    if (Io.Dir.readFileAlloc(Io.Dir.cwd(), io, base_path, gpa, Io.Limit.unlimited)) |content| {
        defer gpa.free(content);
        return try std.json.parseFromSliceLeaky(
            T,
            arena.allocator(),
            content,
            .{ .ignore_unknown_fields = true, .allocate = .alloc_always },
        );
    } else |_| {}

    const base = base_path[0 .. base_path.len - ".json".len];

    const Item = std.meta.Child(T);
    var all_items: std.ArrayListUnmanaged(Item) = .empty;
    defer all_items.deinit(gpa);

    var chunk: usize = 1;
    while (true) : (chunk += 1) {
        var path_buf: [256]u8 = undefined;
        const path = std.fmt.bufPrint(
            &path_buf,
            "{s}_{d}.json",
            .{ base, chunk },
        ) catch return error.PathTooLong;

        const content = Io.Dir.readFileAlloc(Io.Dir.cwd(), io, path, gpa, Io.Limit.unlimited) catch |err| switch (err) {
            error.FileNotFound => break,
            else => return err,
        };
        defer gpa.free(content);

        const chunk_items = try std.json.parseFromSliceLeaky(
            T,
            arena.allocator(),
            content,
            .{ .ignore_unknown_fields = true, .allocate = .alloc_always },
        );
        try all_items.appendSlice(gpa, chunk_items);
    }

    if (chunk == 1) return error.FileNotFound;
    return try arena.allocator().dupe(Item, all_items.items);
}

pub fn load(gpa: Allocator, io: Io) !DataTables {
    var tables: DataTables = undefined;

    tables.arena = ArenaAllocator.init(gpa);
    errdefer tables.arena.deinit();

    inline for (std.meta.fields(DataTables)) |field| {
        if (field.type == ArenaAllocator) continue;

        var table: field.type = .init;
        const base_path = "assets/BinData/" ++ field.type.json;

        const items = try loadTableItems(
            @FieldType(field.type, "items"),
            gpa,
            &tables.arena,
            io,
            base_path,
        );
        table.items = items;

        for (table.items, 0..) |item, index| {
            const key = @field(item, field.type.key);
            try table.index.put(tables.arena.allocator(), key, index);
        }

        @field(tables, field.name) = table;
    }

    return tables;
}

pub fn deinit(tables: *DataTables) void {
    tables.arena.deinit();
}

pub fn Table(comptime T: type, comptime key_field: [:0]const u8) type {
    const KeyType = @FieldType(T, key_field);
    const IndexMap = switch (@typeInfo(KeyType)) {
        .int, .comptime_int => std.array_hash_map.Auto(KeyType, usize),
        .pointer => std.StringArrayHashMapUnmanaged(usize),
        else => @compileError("unsupported key type: " ++ @typeName(KeyType)),
    };

    return struct {
        pub const key = key_field;
        pub const init: @This() = .{ .items = &.{}, .index = .empty };
        pub const json = blk: {
            const t_name = @typeName(T);
            break :blk t_name[std.mem.findScalarLast(u8, t_name, '.').? + 1 ..] ++ ".json";
        };

        items: []const T,
        index: IndexMap,

        pub fn getDataById(table: @This(), id: KeyType) ?T {
            return if (table.index.get(id)) |i| table.items[i] else null;
        }
    };
}

pub fn getRolePropertyGrowth(tables: *const DataTables, level: i32, breach: i32) ?*const RolePropertyGrowth {
    for (tables.role_property_growth.items) |*config| {
        if (config.Level == level and config.BreachLevel == breach)
            return config;
    } else return null;
}

pub fn getMonsterPropertyGrowth(tables: *const DataTables, level: i32, curve_id: i32) ?*const MonsterPropertyGrowth {
    for (tables.monster_property_growth.items) |*config| {
        if (config.Level == level and config.CurveId == curve_id)
            return config;
    } else return null;
}

pub fn getProps(tables: *const DataTables, property_id: i32, gpa: Allocator) ![]i32 {
    const base_property = tables.base_property.getDataById(property_id) orelse return &.{};

    const props = try gpa.alloc(i32, @intFromEnum(pb.EAttributeType.MAX));
    @memset(props, 0);

    inline for (comptime std.meta.fields(DataTables.BaseProperty)) |field| {
        if (!@hasField(pb.EAttributeType, field.name)) continue;
        const attr_type = @field(pb.EAttributeType, field.name);

        props[@intFromEnum(attr_type)] = @field(base_property, field.name);
    }

    return props;
}

pub fn getMotorcycleConfigId(tables: *const DataTables) ?i32 {
    for (tables.explore_tools.items) |tool| {
        if (tool.SummonConfigId == 0) continue;

        const summon = tables.summon_cfg.getDataById(tool.SummonConfigId) orelse continue;
        const blueprint = blk: {
            for (tables.blueprint_config.items) |entry| {
                if (std.mem.eql(u8, entry.BlueprintType, summon.BlueprintType)) break :blk entry;
            }
            continue;
        };
        if (blueprint.EntityLogic != .Vehicle) continue;

        for (tables.template_config.items) |template| {
            if (std.mem.eql(u8, template.BlueprintType, summon.BlueprintType)) return template.Id;
        }
    }

    return null;
}

const BuffAdditionEntry = @import("../logic/events.zig").BuffAdditionEntry;

fn is_active_cond(buff: Buff) bool {
    // is_buff_active check from wicked
    return buff.ModifierMagnitude.len == 0 and
        buff.BuffAction.len == 0 and
        buff.OngoingTagRequirements.len == 0 and
        buff.OngoingTagIgnores.len == 0 and
        buff.RemovalTagRequirements.len == 0 and
        buff.RemovalTagIgnores.len == 0 and
        buff.ApplicationSourceTagIgnores.len == 0 and
        buff.ApplicationTagIgnores.len == 0 and
        buff.ExtraEffectRequirements.len == 0;
}

pub fn resolveRoleId(tables: *const DataTables, role_id: i32) i32 {
    if (tables.trial_role_info.getDataById(role_id)) |trial| {
        if (trial.ParentId != 0) return trial.ParentId;
    }
    return role_id;
}

pub fn resolveBaseRoleInfo(tables: *const DataTables, role_id: i32) ?RoleInfo {
    if (tables.role_info.getDataById(role_id)) |info| return info;
    if (tables.trial_role_info.getDataById(role_id)) |trial| {
        if (trial.ParentId != 0) return tables.role_info.getDataById(trial.ParentId);
    }
    return null;
}

pub fn resolveTrialRoleId(tables: *const DataTables, raw_id: i32, world_level: i32, gender: i32) ?i32 {
    var best_any_gender: ?TrialRoleInfo = null;
    var best_match_gender: ?TrialRoleInfo = null;

    for (tables.trial_role_info.items) |item| {
        if (item.GroupId != raw_id) continue;
        if (item.Gender == -1 or item.Gender == gender) {
            if (isBetterTrialMatch(best_match_gender, item, world_level, gender)) {
                best_match_gender = item;
            }
        }
        if (isBetterTrialMatch(best_any_gender, item, world_level, gender)) {
            best_any_gender = item;
        }
    }

    if (best_match_gender) |m| return m.Id;
    if (best_any_gender) |m| return m.Id;
    if (tables.trial_role_info.getDataById(raw_id)) |exact| return exact.Id;
    return null;
}

fn isBetterTrialMatch(current: ?TrialRoleInfo, candidate: TrialRoleInfo, world_level: i32, gender: i32) bool {
    const cur = current orelse return true;
    const cand_exact_gender = candidate.Gender == gender;
    const cur_exact_gender = cur.Gender == gender;
    if (cand_exact_gender != cur_exact_gender) return cand_exact_gender;

    const cand_le = candidate.WorldLevel <= world_level;
    const cur_le = cur.WorldLevel <= world_level;
    if (cand_le != cur_le) return cand_le;
    if (cand_le) return candidate.WorldLevel > cur.WorldLevel;
    return candidate.WorldLevel < cur.WorldLevel;
}

pub fn getRoleAutoBuffs(
    tables: *const DataTables,
    role_id: i32,
    weapon: WeaponItem,
    skill_branch: i32,
    gpa: Allocator,
) !std.ArrayListUnmanaged(BuffAdditionEntry) {
    const logger = std.log.scoped(.autobuff);
    const resolved_role_id = tables.resolveRoleId(role_id);
    const role_info = tables.resolveBaseRoleInfo(role_id) orelse return error.RoleNotFound;
    const weapon_conf = tables.weapon_conf.getDataById(weapon.id) orelse return error.WeaponConfigNotFound;
    const reson_conf = tables.getWeaponReson(weapon_conf.ResonId, weapon.reson_level);
    const additional_buffs = [_]i64{
        3003, // Remove wall run prohibition
        3004, // Remove gliding prohibition
        3127, // qte energy
        3134, // dodge energy
        1213, // Reduce stamina while flying
        1214, // Reduce stamina while flying in sprint
        1215, // Reduce stamina while flying up in sprint
        1216, // Reduce stamina while flying down in sprint
        1001000000, // Character initial buff mount
        70000077, // Role-IOU deduction control
    };
    const blacklisted_buff_id_combos = [_][]const u8{
        "666", "990", "777", "095", "402", "403", "302", "291", "98", "009", "020",
    };
    const blacklisted_tag_components = [_][]const u8{
        "Rogue",
        "肉鸽",
        "试用",
        "零能量",
        "59b4b0a5",
        "371b7a72",
        "角色.R2T1JiyanMd10011.技能标识.大招状态", // jiyan
        "23d0723c", // denia forms stuff
        "20b04084", // lynae roguelike
        "8c658780", // mornye roguelik
        "9296f07f", // amy roguelike
        "c79fde44", // qingxiao
        "fbcbee28", // qingxiao main fix
        "ed839a56", // jingran fix
        "功能.功能制作.禁止体力恢复", // qingxiao2
    };

    const maybe_init_info = tables.char_init_info.getDataById(resolved_role_id);
    var results: std.ArrayListUnmanaged(BuffAdditionEntry) = .empty;
    errdefer results.deinit(gpa);

    var do_autobuff = true;

    if (maybe_init_info) |init_info| {
        if (init_info.FightBuffs.len > 0) {
            do_autobuff = false;

            for (init_info.FightBuffs) |buff_id| {
                const buff = tables.buff.getDataById(buff_id) orelse continue;
                try results.append(gpa, .{ .id = buff_id, .is_active = is_active_cond(buff) });
            }
        }

        if (init_info.AdditionalFightBuffs) |add_role_buffs| {
            for (add_role_buffs) |buff_id| {
                const buff = tables.buff.getDataById(buff_id) orelse continue;
                try results.append(gpa, .{ .id = buff_id, .is_active = is_active_cond(buff) });
            }
        }
    }

    if (do_autobuff) {
        for (tables.buff.items) |*buff| {
            var id_buf: [20]u8 = undefined;
            var role_buf: [20]u8 = undefined;
            const id_str = std.fmt.bufPrint(&id_buf, "{d}", .{buff.Id}) catch continue;
            const role_str = std.fmt.bufPrint(&role_buf, "{d}", .{resolved_role_id}) catch continue;

            // autobuff
            if (!std.mem.startsWith(u8, id_str, role_str)) continue;
            if (id_str.len <= 4) continue;
            if (buff.DurationPolicy != .Infinite) continue;
            if (buff.ApplicationSourceTagRequirements.len != 0) continue;
            if (buff.ApplicationTagRequirements.len != 0) continue;
            if (buff.PrematureExpirationEffects.len != 0) continue;

            // roguelike buff removal
            const suffix = id_str[4..];
            const blacklisted_combo = blk: {
                for (blacklisted_buff_id_combos) |combo| {
                    if (std.mem.startsWith(u8, suffix, combo)) break :blk true;
                }
                break :blk false;
            };
            if (blacklisted_combo) continue;

            const blacklisted_tag = tag_cond: {
                for (buff.GrantedTags) |tag| {
                    for (blacklisted_tag_components) |component| {
                        if (std.mem.find(u8, tag, component) != null) break :tag_cond true;
                    }
                }
                break :tag_cond false;
            };
            if (blacklisted_tag) continue;

            if (buff.ExtraEffectID == 5) {
                const valid = buff.ExtraEffectParameters.len > 1 and calc_policy_cond: {
                    const param = buff.ExtraEffectParameters[1];
                    const hash_pos = std.mem.indexOfScalar(u8, param, '#') orelse param.len;
                    const ref_id = std.fmt.parseInt(i64, param[0..hash_pos], 10) catch break :calc_policy_cond true;
                    const ref_buff = tables.buff.getDataById(@intCast(ref_id)) orelse break :calc_policy_cond true;
                    break :calc_policy_cond ref_buff.CalculationPolicy.len > 0 and ref_buff.CalculationPolicy[0] != 0;
                };
                if (!valid) continue;
            }

            try results.append(gpa, .{ .id = buff.Id, .is_active = is_active_cond(buff.*) });
        }
    }

    if (do_autobuff) {
        var illegal_ids: std.AutoHashMapUnmanaged(i64, void) = .empty;
        defer illegal_ids.deinit(gpa);

        // resonant chain buff removal
        for (tables.resonant_chain.items) |chain| {
            if (chain.GroupId != role_info.ResonantChainGroupId) continue;
            for (chain.BuffIds) |buff_id| {
                try illegal_ids.put(gpa, buff_id, {});
            }
        }

        // further rogue buff removal
        for (tables.rogue_res_buff.items) |rogue_char_buffs| {
            var buff_id_buf: [20]u8 = undefined;
            var role_buf: [20]u8 = undefined;
            const role_str = std.fmt.bufPrint(&role_buf, "{d}", .{resolved_role_id}) catch continue;
            const buff_id_str = std.fmt.bufPrint(&buff_id_buf, "{d}", .{rogue_char_buffs.BuffId}) catch continue;
            if (std.mem.startsWith(u8, buff_id_str, role_str)) {
                try illegal_ids.put(gpa, rogue_char_buffs.BuffId, {});
            }
            for (rogue_char_buffs.BuffIds) |buff_id| {
                var bid_buf: [20]u8 = undefined;
                const bid_str = std.fmt.bufPrint(&bid_buf, "{d}", .{buff_id}) catch continue;
                if (std.mem.startsWith(u8, bid_str, role_str)) {
                    try illegal_ids.put(gpa, buff_id, {});
                }
            }
        }

        // honami
        for (tables.honami_story_effect.items) |honami| {
            for (honami.Param) |param| {
                if (param.len < 7) continue;
                const id = std.fmt.parseInt(i64, param, 10) catch continue;
                try illegal_ids.put(gpa, id, {});
            }
        }

        // combo teaching buff removal
        for (tables.combo_teaching.items) |combo| {
            for (combo.AddBuffID) |buff_id| {
                try illegal_ids.put(gpa, buff_id, {});
            }
        }

        // le denia (handle ee id 6 (attribute event))
        for (results.items) |entry| {
            const buff = tables.buff.getDataById(@intCast(entry.id)) orelse continue;
            if (buff.ExtraEffectID != 6) continue;
            const result_param = if (buff.ExtraEffectParameters.len > 3)
                buff.ExtraEffectParameters[3]
            else
                continue;
            var it = std.mem.splitScalar(u8, result_param, '#');
            while (it.next()) |id_str| {
                const id = std.fmt.parseInt(i64, id_str, 10) catch continue;
                try illegal_ids.put(gpa, id, {});
            }
        }

        // skill branch buffs
        for (tables.skill_branch_buff.items) |branch_data| {
            if (branch_data.RoleId != resolved_role_id) continue;
            for (branch_data.Buffs) |buff_id| {
                try illegal_ids.put(gpa, buff_id, {});
            }
        }

        if (illegal_ids.count() > 0) {
            var i: usize = 0;
            while (i < results.items.len) {
                if (illegal_ids.contains(results.items[i].id)) {
                    _ = results.swapRemove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    for (additional_buffs) |id| {
        try results.append(gpa, .{ .id = id, .is_active = true });
    }

    if (tables.skill_branch_buff.getDataById(skill_branch)) |branch_data| {
        for (branch_data.Buffs) |buff_id| {
            try results.append(gpa, .{ .id = buff_id, .is_active = true });
        }
    }

    if (tables.main_role_config.getDataById(role_id)) |main_role| {
        if (mainRoleTagBuffId(main_role.Gender)) |tag_buff_id| {
            try results.append(gpa, .{ .id = tag_buff_id, .is_active = true });
        }
    }

    if (std.mem.findScalar(i32, ConcertoExceptions, role_id) == null) {
        try results.append(gpa, .{ .id = 1101002011, .is_active = true });
    }

    if (reson_conf) |rc| {
        for (rc.Effect) |id| {
            try results.append(gpa, .{ .id = id, .is_active = true });
        }
    }

    try results.append(gpa, .{ .id = 1101007999 + role_info.ElementId, .is_active = true });
    try results.append(gpa, .{ .id = 1001001015 + (role_info.ElementId * 2), .is_active = true });
    try results.append(gpa, .{ .id = 3080 + role_info.ElementId, .is_active = true });
    // Grant "角色.Common.探索技能.准用翱翔" so Soar Wing (1015) works without blocked tag.
    try results.append(gpa, .{ .id = 640012051, .is_active = true });

    logger.info("autobuff results for: {d}, {any}", .{ role_id, results.items });

    return results;
}

fn mainRoleTagBuffId(gender: i32) ?i64 {
    return switch (gender) {
        1 => 291724064,
        0 => 291724065,
        else => null,
    };
}

pub fn createBuffInformation(
    handle_id: i32,
    buff_id: i64,
    instigator_id: i64,
    entity_id: i64,
    is_active: bool,
) pb.FightBuffInformation {
    return .{
        .HandleId = handle_id,
        .BuffId = buff_id,
        .Level = 1,
        .StackCount = 1,
        .InstigatorId = instigator_id,
        .EntityId = entity_id,
        .Duration = -1.0,
        .LeftDuration = -1.0,
        .IsActive = is_active,
        .ApplyType = .Common,
        .MessageId = -1,
        .ServerId = 0,
    };
}

pub fn getWeaponReson(tables: *const DataTables, reson_id: i32, level: i32) ?*const WeaponReson {
    for (tables.weapon_reson.items) |*reson| {
        if (reson.ResonId == reson_id and reson.Level == level) return reson;
    }
    return null;
}
