//! Lazy-loaded typed gameplay tables from `assets.db` (`raw_files`) / JSON fallback.

use std::sync::OnceLock;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DirectTrainActivityEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub SkipQuestId: i32,
    #[serde(default)]
    pub SkipQuestCondition: i32,
    #[serde(default)]
    pub ForceRemindCond: i32,
    #[serde(default)]
    pub RecommendQuestId: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DirectTrainMainEntry {
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub SubActivityIds: Vec<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LevelPlayInfoMappingData {
    #[serde(default)]
    pub LevelPlayId: i32,
    #[serde(default)]
    pub Vars: Vec<String>,
    #[serde(default)]
    pub Chests: Vec<i32>,
    #[serde(default)]
    pub DefaultValues: Vec<i64>,
    #[serde(default)]
    pub Type: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LevelPlayInfoMappingEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub Type: i32,
    #[serde(default)]
    pub Data: LevelPlayInfoMappingData,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PunishReportEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub Cond1Key: String,
    #[serde(default)]
    pub Cond2ey: String,
    #[serde(default)]
    pub Cond3Key: String,
    #[serde(default)]
    pub GetBoxKey: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CoopRoleEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub UnlockQuestId: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CoopRoleLevelEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub CoopRoleId: i32,
    #[serde(default)]
    pub CoopLevel: i32,
    #[serde(default)]
    pub TaskGroupId: i32,
    #[serde(default)]
    pub PreUnLockCondition: i32,
    #[serde(default)]
    pub UnLockCondition: i32,
    #[serde(default)]
    pub Reward: i32,
    #[serde(default)]
    pub FinishGuideTextCondition1: String,
    #[serde(default)]
    pub FinishGuideTextConditionValue1: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CoopTaskConfigEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub CoopRoleId: i32,
    #[serde(default)]
    pub CoopLevelId: i32,
    #[serde(default)]
    pub CoopPreQuest: i32,
    #[serde(default)]
    pub ConditionGroup: i32,
    #[serde(default)]
    pub ItemId: i32,
    #[serde(default)]
    pub LevelId1: i32,
    #[serde(default)]
    pub LevelId2: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CoopSpRewardEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub Reward: i32,
    #[serde(default)]
    pub RoleLevelSum: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TeamParkourCfgEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub InstId: i32,
    #[serde(default)]
    pub OpenDay: i32,
    #[serde(default)]
    pub TaskList: Vec<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TeamParkourRewardEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub Reward: i32,
    #[serde(default)]
    pub RewardThreshold: i32,
    #[serde(default)]
    pub TaskTitle: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TimeScheduleEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub TimeScheduleTag: String,
    #[serde(default)]
    pub StartTime: String,
    #[serde(default)]
    pub Position: Vec<f32>,
    #[serde(default)]
    pub Rotation: Vec<f32>,
    #[serde(default)]
    pub BehaviorTree: i32,
    #[serde(default)]
    pub SplineEntityUid: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MultiHalfInstLevelEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub InstIds: Vec<i32>,
    #[serde(default)]
    pub PassScore: i32,
    #[serde(default)]
    pub LevelPassReward: i32,
    #[serde(default)]
    pub PreLevel: Vec<i32>,
    #[serde(default)]
    pub BuffGroup: Vec<i32>,
    #[serde(default)]
    pub RareBuffGroup: Vec<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RogueRoomPoolEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub BehaviorTree: i32,
    #[serde(default)]
    pub RoomsMusicState: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RogueRoomTypeEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub RoomType: String,
    #[serde(default)]
    pub RoomTipsType: i32,
    #[serde(default)]
    pub RoomsMusicState: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RogueSeasonRewardEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub SeasonId: i32,
    #[serde(default)]
    pub Index: i32,
    #[serde(default)]
    pub Point: i32,
    #[serde(default)]
    pub DropId: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AbyssInstEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub InstId: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub InstEntranceId: i32,
    #[serde(default)]
    pub RouteId: i32,
    #[serde(default)]
    pub Difficulty: i32,
    #[serde(default)]
    pub TotalTime: i32,
    #[serde(default)]
    pub TotalScore: i32,
    #[serde(default)]
    pub DropPreviewId: i32,
    #[serde(default)]
    pub UnlockLittleRole: Vec<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AbyssRoomEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub BehaviorTree: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct HonamiStoryItemEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub ItemType: i32,
    #[serde(default)]
    pub SubType: i32,
    #[serde(default)]
    pub QualityId: i32,
    #[serde(default)]
    pub SellPrice: i32,
    #[serde(default)]
    pub DropPlan: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct HonamiStoryBackPackEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub Type: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub Wide: i32,
    #[serde(default)]
    pub Size: Vec<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RoverRogueRoomEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub RoomTypeId: String,
    #[serde(default)]
    pub SubLevels: Vec<String>,
    #[serde(default)]
    pub BornEntityId: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RoverRogueGoodsEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub Type: i32,
    #[serde(default)]
    pub Param: i32,
    #[serde(default)]
    pub Price: i32,
    #[serde(default)]
    pub Discount: i32,
    #[serde(default)]
    pub ShowItemId: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RoverRogueBlessGroupEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub BlessRoleId: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RoverRogueEventGroupEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ChoicePool: Vec<i32>,
    #[serde(default)]
    pub Num: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RoverRogueLootEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub MaxLevel: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct QuestTreeNodeEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub QuestArray: Vec<i32>,
    #[serde(default)]
    pub CompleteQuestID: Vec<i32>,
    #[serde(default)]
    pub QuestType: i32,
    #[serde(default)]
    pub PreNode: Vec<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ShortMessageEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub QuestId: i32,
    #[serde(default)]
    pub ListenQuestId: i32,
}

#[derive(Default)]
pub struct GameplayTableCache {
    pub quest_tree_nodes: OnceLock<Vec<QuestTreeNodeEntry>>,
    pub short_messages: OnceLock<Vec<ShortMessageEntry>>,
    pub direct_train_activities: OnceLock<Vec<DirectTrainActivityEntry>>,
    pub direct_train_main: OnceLock<Vec<DirectTrainMainEntry>>,
    pub level_play_mappings: OnceLock<Vec<LevelPlayInfoMappingEntry>>,
    pub punish_reports: OnceLock<Vec<PunishReportEntry>>,
    pub coop_roles: OnceLock<Vec<CoopRoleEntry>>,
    pub coop_role_levels: OnceLock<Vec<CoopRoleLevelEntry>>,
    pub coop_tasks: OnceLock<Vec<CoopTaskConfigEntry>>,
    pub coop_sp_rewards: OnceLock<Vec<CoopSpRewardEntry>>,
    pub team_parkour_cfgs: OnceLock<Vec<TeamParkourCfgEntry>>,
    pub team_parkour_rewards: OnceLock<Vec<TeamParkourRewardEntry>>,
    pub time_schedules: OnceLock<Vec<TimeScheduleEntry>>,
    pub mow_tower_levels: OnceLock<Vec<MultiHalfInstLevelEntry>>,
    pub slash_and_tower_cfgs: OnceLock<Vec<MultiHalfInstLevelEntry>>,
    pub boss_piling_levels: OnceLock<Vec<MultiHalfInstLevelEntry>>,
    pub rogue_room_pools: OnceLock<Vec<RogueRoomPoolEntry>>,
    pub rogue_room_types: OnceLock<Vec<RogueRoomTypeEntry>>,
    pub rogue_season_rewards: OnceLock<Vec<RogueSeasonRewardEntry>>,
    pub abyss_insts: OnceLock<Vec<AbyssInstEntry>>,
    pub abyss_rooms: OnceLock<Vec<AbyssRoomEntry>>,
    pub honami_items: OnceLock<Vec<HonamiStoryItemEntry>>,
    pub honami_backpacks: OnceLock<Vec<HonamiStoryBackPackEntry>>,
    pub rover_rogue_rooms: OnceLock<Vec<RoverRogueRoomEntry>>,
    pub rover_rogue_goods: OnceLock<Vec<RoverRogueGoodsEntry>>,
    pub rover_rogue_bless_groups: OnceLock<Vec<RoverRogueBlessGroupEntry>>,
    pub rover_rogue_event_groups: OnceLock<Vec<RoverRogueEventGroupEntry>>,
    pub rover_rogue_loots: OnceLock<Vec<RoverRogueLootEntry>>,
    pub random_summons: OnceLock<Vec<serde_json::Value>>,
    pub raw_json_arrays:
        std::sync::Mutex<std::collections::HashMap<String, std::sync::Arc<Vec<serde_json::Value>>>>,
    pub world_controllers_by_map: std::sync::Mutex<
        std::collections::HashMap<
            i32,
            std::sync::Arc<Vec<std::sync::Arc<crate::tables::level_entity_config::LevelEntityConfig>>>,
        >,
    >,
}
