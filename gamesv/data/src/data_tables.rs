//! Strongly-typed BinData table container + combat/role helpers for `gamesv`.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use sonoro_proto::enums::{ApplyGEType, EAttributeType};
use sonoro_proto::pb::FightBuffInformation;

use crate::assets_db::AssetsDb;
use crate::inline_tables::*;
use crate::tables::base_property::BaseProperty;
use crate::tables::blueprint_config::EntityLogic;
use crate::tables::buff::{Buff, BuffAdditionEntry, DurationPolicy};
use crate::tables::monster_property_growth::MonsterPropertyGrowth;
use crate::tables::role_info::RoleInfo;
use crate::tables::role_property_growth::RolePropertyGrowth;
use crate::tables::trial_role_info::TrialRoleInfo;
use crate::zon_config;

#[derive(Debug, Clone, Copy, Default)]
pub struct WeaponEquipInfo {
    pub id: i32,
    pub reson_level: i32,
}

#[derive(Debug)]
pub enum TableRef<'a, T> {
    Borrowed(&'a T),
    Owned(Arc<T>),
}

impl<T> Clone for TableRef<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(r) => Self::Borrowed(r),
            Self::Owned(arc) => Self::Owned(Arc::clone(arc)),
        }
    }
}

impl<T> Deref for TableRef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(r) => r,
            Self::Owned(arc) => arc,
        }
    }
}

impl<T> AsRef<T> for TableRef<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: Clone> TableRef<'_, T> {
    #[inline]
    pub fn into_owned(self) -> T {
        match self {
            Self::Borrowed(r) => r.clone(),
            Self::Owned(arc) => Arc::unwrap_or_clone(arc),
        }
    }
}

pub type LazyLookupFn<T, K> = Arc<dyn Fn(&K) -> Option<Arc<T>> + Send + Sync>;

pub struct Table<T, K> {
    pub items: Vec<T>,
    index: HashMap<K, usize>,
    lazy_get: Option<LazyLookupFn<T, K>>,
}

impl<T, K> Default for Table<T, K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            items: Vec::new(),
            index: HashMap::new(),
            lazy_get: None,
        }
    }
}

impl<T: Clone, K: Eq + Hash> Table<T, K> {
    pub fn empty_lazy(lazy_get: Option<LazyLookupFn<T, K>>) -> Self {
        Self {
            items: Vec::new(),
            index: HashMap::new(),
            lazy_get,
        }
    }

    pub fn from_items(items: Vec<T>, key_fn: impl Fn(&T) -> K) -> Self {
        let mut index = HashMap::with_capacity(items.len());
        for (i, item) in items.iter().enumerate() {
            index.insert(key_fn(item), i);
        }
        Self {
            items,
            index,
            lazy_get: None,
        }
    }

    pub fn get_data_by_id(&self, id: &K) -> Option<TableRef<'_, T>> {
        if let Some(ref lazy) = self.lazy_get {
            return lazy(id).map(TableRef::Owned);
        }
        self.index
            .get(id)
            .and_then(|&idx| self.items.get(idx))
            .map(TableRef::Borrowed)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Default)]
pub struct DataTables {
    db: Option<Arc<AssetsDb>>,
    pub concerto_exceptions: Vec<i32>,
    pub role_info: Table<crate::tables::role_info::RoleInfo, i32>,
    pub role_be_hit_map: Table<crate::tables::role_be_hit_map::RoleBeHitMap, i32>,
    pub main_role_config: Table<crate::tables::main_role_config::MainRoleConfig, i32>,
    pub base_property: Table<crate::tables::base_property::BaseProperty, i32>,
    pub function_condition: Table<crate::tables::function_condition::FunctionCondition, i32>,
    pub role_property_growth: Table<crate::tables::role_property_growth::RolePropertyGrowth, i32>,
    pub monster_property_growth: Table<crate::tables::monster_property_growth::MonsterPropertyGrowth, i32>,
    pub instance_dungeon: Table<crate::tables::instance_dungeon::InstanceDungeon, i32>,
    pub buff: Table<crate::tables::buff::Buff, i64>,
    pub shield: Table<crate::tables::shield::Shield, i64>,
    pub summon_cfg: Table<crate::tables::summon_cfg::SummonCfg, i32>,
    pub model_config_preload: Table<crate::tables::model_config_preload::ModelConfigPreload, i32>,
    pub character_part_config: Table<crate::tables::character_part_config::CharacterPartConfig, i32>,
    pub gameplay_tag_parent: Table<crate::tables::gameplay_tag_parent::GameplayTagParent, i32>,
    pub gameplay_tag_remap: Table<crate::tables::gameplay_tag_remap::GameplayTagRemap, i32>,
    pub blueprint_config: Table<crate::tables::blueprint_config::BlueprintConfig, i32>,
    pub role_skin: Table<crate::tables::role_skin::RoleSkin, i32>,
    pub fly_skin_config: Table<crate::tables::fly_skin_config::FlySkinConfig, i32>,
    pub weapon_skin: Table<crate::tables::weapon_skin::WeaponSkin, i32>,
    pub ornament: Table<crate::tables::ornament::Ornament, i32>,
    pub template_config: Table<crate::tables::template_config::TemplateConfig, i32>,
    pub explore_tools: Table<crate::tables::explore_tools::ExploreTools, i32>,
    pub weapon_conf: Table<crate::inline_tables::WeaponConf, i32>,
    pub weapon_base_attributes: Table<crate::tables::weapon_base_attributes::WeaponBaseAttributes, i32>,
    pub weapon_property_growth: Table<crate::tables::weapon_property_growth::WeaponPropertyGrowth, i32>,
    pub weapon_reson: Table<crate::inline_tables::WeaponReson, i32>,
    pub weapon_exp_item: Table<crate::tables::weapon_exp_item::WeaponExpItem, i32>,
    pub weapon_level: Table<crate::tables::weapon_level::WeaponLevel, i32>,
    pub weapon_breach: Table<crate::tables::weapon_breach::WeaponBreach, i32>,
    pub role_exp_item: Table<crate::tables::role_exp_item::RoleExpItem, i32>,
    pub role_level_consume: Table<crate::tables::role_level_consume::RoleLevelConsume, i32>,
    pub role_breach: Table<crate::tables::role_breach::RoleBreach, i32>,
    pub skill_level: Table<crate::tables::skill_level::SkillLevel, i32>,
    pub level_entity_config: Table<crate::tables::level_entity_config::LevelEntityConfig, i64>,
    pub damage: Table<crate::tables::damage::Damage, i64>,
    pub area: Table<crate::tables::area::Area, i32>,
    pub infr_v2_tree_build: Table<crate::tables::infr_v2_tree_build::InfrV2TreeBuild, i32>,
    pub infr_road_build: Table<crate::tables::infr_road_build::InfrRoadBuild, i32>,
    pub teleporter: Table<crate::tables::teleporter::Teleporter, i32>,
    pub activity: Table<crate::tables::activity::Activity, i32>,
    pub map_fog: Table<crate::tables::map_fog::MapFog, i32>,
    pub multi_map: Table<crate::tables::multi_map::MultiMap, i32>,
    pub map_block_info: Table<crate::tables::map_block_info::MapBlockInfo, i32>,
    pub explore_progress_reward: Table<crate::tables::explore_progress_reward::ExploreProgressReward, i32>,
    pub explore_reward: Table<crate::tables::explore_reward::ExploreReward, i32>,
    pub explore_score: Table<crate::tables::explore_score::ExploreScore, i32>,
    pub flow: Table<crate::tables::flow::Flow, String>,
    pub skill: Table<crate::tables::skill::Skill, i32>,
    pub skill_tree: Table<crate::tables::skill_tree::SkillTree, i32>,
    pub resonant_chain: Table<crate::tables::resonant_chain::ResonantChain, i32>,
    pub rogue_res_buff: Table<crate::tables::rogue_res_character_buff::RogueResCharacterBuff, i32>,
    pub honami_story_effect: Table<crate::tables::honami_story_effect::HonamiStoryEffect, i32>,
    pub combo_teaching: Table<crate::tables::combo_teaching::ComboTeaching, i32>,
    pub entity_skill_preload: Table<crate::tables::entity_skill_preload::EntitySkillPreload, i32>,
    pub common_skill_preload: Table<crate::tables::common_skill_preload::CommonSkillPreload, i32>,
    pub passive_skill: Table<crate::tables::passive_skill::PassiveSkill, i64>,
    pub favor_word: Table<crate::tables::favor_word::FavorWord, i32>,
    pub favor_story: Table<crate::tables::favor_story::FavorStory, i32>,
    pub favor_goods: Table<crate::tables::favor_goods::FavorGoods, i32>,
    pub motion: Table<crate::tables::motion::Motion, i32>,
    pub guide_group: Table<crate::tables::guide_group::GuideGroup, i32>,
    pub char_init_info: Table<crate::tables::character_init_info::CharacterInitInfo, i32>,
    pub skill_branch_buff: Table<crate::tables::skill_branch_buff::SkillBranchBuff, i32>,
    pub phonograph_album: Table<crate::tables::phonograph_album::PhonographAlbum, i32>,
    pub phonograph_music: Table<crate::tables::phonograph_music::PhonographMusic, i32>,
    pub background_card: Table<crate::tables::background_card::BackgroundCard, i32>,
    pub player_head_re: Table<crate::tables::player_head_re::PlayerHeadRe, i32>,
    pub player_title: Table<crate::tables::player_title::PlayerTitle, i32>,
    pub motor_tech_tree: Table<crate::tables::motor_tech_tree::MotorTechTree, i32>,
    pub motor_scene: Table<crate::tables::motor_scene::MotorScene, i32>,
    pub motor_lvl: Table<crate::tables::motor_lvl::MotorLvl, i32>,
    pub motor_tech: Table<crate::tables::motor_tech::MotorTech, i32>,
    pub motor_tech_lv: Table<crate::tables::motor_tech_lv::MotorTechLv, i32>,
    pub motor_task: Table<crate::tables::motor_task::MotorTask, i32>,
    pub motor_skin: Table<crate::tables::motor_skin::MotorSkin, i32>,
    pub motor_sticker: Table<crate::tables::motor_sticker::MotorSticker, i32>,
    pub motor_sticker_part: Table<crate::tables::motor_sticker_part::MotorStickerPart, i32>,
    pub motor_load_project: Table<crate::tables::motor_load_project::MotorLoadProject, i32>,
    pub motor_general_preview: Table<crate::tables::motor_general_preview::MotorGeneralPreview, i32>,
    pub motor_decal_ip: Table<crate::tables::motor_decal_ip::MotorDecalIp, i32>,
    pub motor_linkage_ip: Table<crate::tables::motor_linkage_ip::MotorLinkageIp, i32>,
    pub motor_frame: Table<crate::tables::motor_frame::MotorFrame, i32>,
    pub motor_decorations: Table<crate::tables::motor_decorations::MotorDecorations, i32>,
    pub motor_decorations_part: Table<crate::tables::motor_decorations_part::MotorDecorationsPart, i32>,
    pub gacha: Table<crate::tables::gacha::Gacha, i32>,
    pub gacha_pool: Table<crate::tables::gacha_pool::GachaPool, i32>,
    pub gacha_view_info: Table<crate::tables::gacha_view_info::GachaViewInfo, i32>,
    pub gacha_texture_info: Table<crate::tables::gacha_texture_info::GachaTextureInfo, i32>,
    pub ai_state_machine_config: Table<crate::tables::ai_state_machine_config::AiStateMachineConfig, String>,
    pub ai_base: Table<crate::tables::ai_base::AiBase, i32>,
    pub hardness_mode: Table<crate::tables::hardness_mode::HardnessMode, i32>,
    pub ai_sense_group: Table<crate::tables::ai_sense_group::AiSenseGroup, i32>,
    pub phantom_item: Table<crate::tables::phantom_item::PhantomItem, i32>,
    pub phantom_skill: Table<crate::tables::phantom_skill::PhantomSkill, i32>,
    pub phantom_summon_group: Table<crate::tables::phantom_summon_group::PhantomSummonGroup, i32>,
    pub phantom_fetter: Table<crate::tables::phantom_fetter::PhantomFetter, i32>,
    pub phantom_fetter_group: Table<crate::tables::phantom_fetter_group::PhantomFetterGroup, i32>,
    pub phantom_main_property: Table<crate::tables::phantom_main_property::PhantomMainProperty, i32>,
    pub phantom_main_prop_item: Table<crate::tables::phantom_main_prop_item::PhantomMainPropItem, i32>,
    pub phantom_sub_property: Table<crate::tables::phantom_sub_property::PhantomSubProperty, i32>,
    pub phantom_rarity: Table<crate::tables::phantom_rarity::PhantomRarity, i32>,
    pub phantom_quality: Table<crate::tables::phantom_quality::PhantomQuality, i32>,
    pub phantom_growth: Table<crate::tables::phantom_growth::PhantomGrowth, i32>,
    pub phantom_level: Table<crate::tables::phantom_level::PhantomLevel, i32>,
    pub phantom_exp_item: Table<crate::tables::phantom_exp_item::PhantomExpItem, i32>,
    pub phantom_vice_polish_config: Table<crate::tables::phantom_vice_polish_config::PhantomVicePolishConfig, i32>,
    pub calabash_level: Table<crate::tables::calabash_level::CalabashLevel, i32>,
    pub calabash_develop_condition: Table<crate::tables::calabash_develop_condition::CalabashDevelopCondition, i32>,
    pub calabash_develop_reward: Table<crate::tables::calabash_develop_reward::CalabashDevelopReward, i32>,
    pub calabash_skin: Table<crate::tables::calabash_skin::CalabashSkin, i32>,
    pub drop_package: Table<crate::tables::drop_package::DropPackage, i32>,
    pub drop_group: Table<crate::tables::drop_group::DropGroup, i32>,
    pub player_exp: Table<crate::tables::player_exp::PlayerExp, i32>,
    pub item_info: Table<crate::tables::item_info::ItemInfo, i32>,
    pub gift_package: Table<crate::tables::gift_package::GiftPackage, i32>,
    pub condition: Table<crate::tables::condition::Condition, i32>,
    pub condition_group: Table<crate::tables::condition_group::ConditionGroup, i32>,
    pub quest_data: Table<crate::tables::quest_data::QuestData, i32>,
    pub level_play_data: Table<crate::tables::level_play_data::LevelPlayData, i32>,
    pub map_mark: Table<crate::tables::map_mark::MapMark, i32>,
    pub monster_detection: Table<crate::tables::monster_detection::MonsterDetection, i32>,
    pub dungeon_detection: Table<crate::tables::dungeon_detection::DungeonDetection, i32>,
    pub silent_area_detection: Table<crate::tables::silent_area_detection::SilentAreaDetection, i32>,
    pub lord_gym: Table<crate::tables::lord_gym::LordGym, i32>,
    pub lord_gym_entrance: Table<crate::tables::lord_gym_entrance::LordGymEntrance, i32>,
    pub adventure_task: Table<crate::tables::adventure_task::AdventureTask, i32>,
    pub adventure_task_chapter: Table<crate::tables::adventure_task_chapter::AdventureTaskChapter, i32>,
    pub daily_task: Table<crate::tables::daily_task::DailyTask, i32>,
    pub daily_task_group: Table<crate::tables::daily_task_group::DailyTaskGroup, i32>,
    pub data_layer_config: Table<crate::tables::data_layer_config::DataLayerConfig, i32>,
    pub weather: Table<crate::tables::weather::Weather, i32>,
    pub role_trial_info: Table<crate::tables::role_trial_info::RoleTrialInfo, i32>,
    pub instance_trial_role_config: Table<crate::tables::instance_trial_role_config::InstanceTrialRoleConfig, i32>,
    pub trial_role_info: Table<crate::tables::trial_role_info::TrialRoleInfo, i32>,
    pub trial_weapon_info: Table<crate::tables::trial_weapon_info::TrialWeaponInfo, i32>,
    pub trial_phantom_prop_item: Table<crate::tables::trial_phantom_prop_item::TrialPhantomPropItem, i32>,
    pub trail_phantom_prop: Table<crate::tables::trail_phantom_prop::TrailPhantomProp, i32>,
    pub pay_shop: Table<crate::tables::pay_shop::PayShop, i32>,
    pub pay_shop_tab: Table<crate::tables::pay_shop_tab::PayShopTab, i32>,
    pub pay_shop_recommend: Table<crate::tables::pay_shop_recommend::PayShopRecommend, i32>,
    pub shop_info: Table<crate::tables::shop_info::ShopInfo, i32>,
    pub shop_fixed: Table<crate::tables::shop_fixed::ShopFixed, i32>,
    pub forge_formula: Table<crate::tables::forge_formula::ForgeFormula, i32>,
    pub synthesis_formula: Table<crate::tables::synthesis_formula::SynthesisFormula, i32>,
    pub synthesis_level: Table<crate::tables::synthesis_level::SynthesisLevel, i32>,
    pub cook_formula: Table<crate::tables::cook_formula::CookFormula, i32>,
    pub cook_level: Table<crate::tables::cook_level::CookLevel, i32>,
    pub cook_processed: Table<crate::tables::cook_processed::CookProcessed, i32>,
    pub achievement: Table<crate::tables::achievement::Achievement, i32>,
    pub achievement_group: Table<crate::tables::achievement_group::AchievementGroup, i32>,
    pub achievement_star_level: Table<crate::tables::achievement_star_level::AchievementStarLevel, i32>,
    pub liveness: Table<crate::tables::liveness::Liveness, i32>,
    pub liveness_task: Table<crate::tables::liveness_task::LivenessTask, i32>,
    pub tower_config: Table<crate::tables::tower_config::TowerConfig, i32>,
    pub tower_difficulty: Table<crate::tables::tower_difficulty::TowerDifficulty, i32>,
    pub tower_season_reward: Table<crate::tables::tower_season_reward::TowerSeasonReward, i32>,
    pub fishing_tech: Table<crate::tables::fishing_tech::FishingTech, i32>,
    pub fishing_reputation: Table<crate::tables::fishing_reputation::FishingReputation, i32>,
    pub fishing_item: Table<crate::tables::fishing_item::FishingItem, i32>,
    pub elemental_box_define: Table<crate::tables::elemental_box_define::ElementalBoxDefine, i32>,
    pub giant_item: Table<crate::tables::giant_item::GiantItem, i32>,
    pub giant_delivery_reward: Table<crate::tables::giant_delivery_reward::GiantDeliveryReward, i32>,
    pub abyss_little_role: Table<crate::tables::abyss_little_role::AbyssLittleRole, i32>,
    pub item_exchange_content: Table<crate::inline_tables::ItemExchangeContent, i32>,
    pub dragon_pool: Table<crate::inline_tables::DragonPool, i32>,
    pub loading_level_area: Table<crate::inline_tables::LoadingLevelArea, i32>,
    pub world_new_journey: Table<crate::inline_tables::WorldNewJourney, i32>,
    pub dark_coast_delivery: Table<crate::inline_tables::DarkCoastDelivery, i32>,
    pub new_tower_season_award: Table<crate::inline_tables::NewTowerSeasonAward, i32>,
    pub exchange_reward: Table<crate::inline_tables::ExchangeReward, i32>,
    pub loop_teleport_volume_function: Table<crate::inline_tables::LoopTeleportVolumeFunction, i32>,
    pub recall_config: Table<crate::inline_tables::RecallConfig, i32>,
    pub boss_rush_buff: Table<crate::inline_tables::BossRushBuff, i32>,
    pub instance_dungeon_entrance: Table<crate::inline_tables::InstanceDungeonEntrance, i32>,
    pub consumptive_task: Table<crate::inline_tables::ConsumptiveTask, i32>,
    pub doll_grab_items_csv: Table<crate::inline_tables::DollGrabItemsCsv, i32>,
    pub track_moon_entrust: Table<crate::inline_tables::TrackMoonEntrust, i32>,
    pub motor_decal_quest: Table<crate::inline_tables::MotorDecalQuest, i32>,
    pub motor_linkage_quest: Table<crate::inline_tables::MotorLinkageQuest, i32>,
    pub abnormal_damage_config: Table<crate::inline_tables::AbnormalDamageConfig, i32>,
    pub buff_item: Table<crate::inline_tables::BuffItem, i32>,
    pub quest_ref_video_config: Table<crate::inline_tables::QuestRefVideoConfig, i32>,
    pub quest_ref_map_block_config: Table<crate::inline_tables::QuestRefMapBlockConfig, i32>,
    pub quest_resource_white: Table<crate::inline_tables::QuestResourceWhite, i32>,
    pub morale_keep_level: Table<crate::inline_tables::MoraleKeepLevel, i32>,
    pub morale_play: Table<crate::inline_tables::MoralePlay, i32>,
    pub battle_score_conf: Table<crate::inline_tables::BattleScoreConf, i32>,
    pub battle_score_level_conf: Table<crate::inline_tables::BattleScoreLevelConf, i32>,
    pub drop_show_plan: Table<crate::inline_tables::DropShowPlan, i32>,
    pub prefab_config: Table<crate::inline_tables::PrefabConfig, i32>,
    pub pay_item: Table<crate::inline_tables::PayItemCfg, i32>,
    pub pay: Table<crate::inline_tables::PayCfg, i32>,
    pub pay_gift: Table<crate::inline_tables::PayGiftCfg, i32>,
    pub battle_pass: Table<crate::inline_tables::BattlePassCfg, i32>,
    pub phantom_recommend: Table<crate::inline_tables::PhantomRecommendCfg, i32>,
    pub boss_rush_score: Table<crate::inline_tables::BossRushScoreCfg, i32>,
    pub gacha_accumulate_reward: Table<crate::inline_tables::GachaAccumulateRewardCfg, i32>,
}

impl DataTables {
    pub fn init(base_path: &str) -> Result<Self, String> {
        Self::init_with_db(base_path, None)
    }

    pub fn init_with_db(base_path: &str, db: Option<Arc<AssetsDb>>) -> Result<Self, String> {
        let concerto_exceptions: Vec<i32> = zon_config::cached_concerto_exceptions().to_vec();
        let role_info = load_table(db.as_deref(), base_path, "RoleInfo.json", "RoleInfo", |item: &crate::tables::role_info::RoleInfo| item.Id)?;
        let role_be_hit_map = load_table(db.as_deref(), base_path, "RoleBeHitMap.json", "RoleBeHitMap", |item: &crate::tables::role_be_hit_map::RoleBeHitMap| item.Id)?;
        let main_role_config = load_table(db.as_deref(), base_path, "MainRoleConfig.json", "MainRoleConfig", |item: &crate::tables::main_role_config::MainRoleConfig| item.Id)?;
        let base_property = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i32| {
                db_c.get_base_property(*k)
            })))
        } else {
            load_table_from_disk(base_path, "BaseProperty.json", |item: &crate::tables::base_property::BaseProperty| item.Id)?
        };
        let function_condition = load_table(db.as_deref(), base_path, "FunctionCondition.json", "FunctionCondition", |item: &crate::tables::function_condition::FunctionCondition| item.FunctionId)?;
        let role_property_growth = load_table(db.as_deref(), base_path, "RolePropertyGrowth.json", "RolePropertyGrowth", |item: &crate::tables::role_property_growth::RolePropertyGrowth| item.Id)?;
        let monster_property_growth = load_table(db.as_deref(), base_path, "MonsterPropertyGrowth.json", "MonsterPropertyGrowth", |item: &crate::tables::monster_property_growth::MonsterPropertyGrowth| item.Id)?;
        let instance_dungeon = load_table(db.as_deref(), base_path, "InstanceDungeon.json", "InstanceDungeon", |item: &crate::tables::instance_dungeon::InstanceDungeon| item.Id)?;
        let buff = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i64| {
                db_c.get_buff(*k)
            })))
        } else {
            load_table_from_disk(base_path, "Buff.json", |item: &crate::tables::buff::Buff| item.Id)?
        };
        let shield = load_table(db.as_deref(), base_path, "Shield.json", "Shield", |item: &crate::tables::shield::Shield| item.Id)?;
        let summon_cfg = load_table(db.as_deref(), base_path, "SummonCfg.json", "SummonCfg", |item: &crate::tables::summon_cfg::SummonCfg| item.Id)?;
        let model_config_preload = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i32| {
                db_c.get_model_config_preload(*k)
            })))
        } else {
            load_table_from_disk(base_path, "ModelConfigPreload.json", |item: &crate::tables::model_config_preload::ModelConfigPreload| item.Id)?
        };
        let character_part_config = load_table(db.as_deref(), base_path, "CharacterPartConfig.json", "CharacterPartConfig", |item: &crate::tables::character_part_config::CharacterPartConfig| item.ModelId)?;
        let gameplay_tag_parent = load_table(db.as_deref(), base_path, "GameplayTagParent.json", "GameplayTagParent", |item: &crate::tables::gameplay_tag_parent::GameplayTagParent| item.Id)?;
        let gameplay_tag_remap = load_table(db.as_deref(), base_path, "GameplayTagRemap.json", "GameplayTagRemap", |item: &crate::tables::gameplay_tag_remap::GameplayTagRemap| item.RawId)?;
        let blueprint_config = load_table(db.as_deref(), base_path, "BlueprintConfig.json", "BlueprintConfig", |item: &crate::tables::blueprint_config::BlueprintConfig| item.Id)?;
        let role_skin = load_table(db.as_deref(), base_path, "RoleSkin.json", "RoleSkin", |item: &crate::tables::role_skin::RoleSkin| item.Id)?;
        let fly_skin_config = load_table(db.as_deref(), base_path, "FlySkinConfig.json", "FlySkinConfig", |item: &crate::tables::fly_skin_config::FlySkinConfig| item.Id)?;
        let weapon_skin = load_table(db.as_deref(), base_path, "WeaponSkin.json", "WeaponSkin", |item: &crate::tables::weapon_skin::WeaponSkin| item.Id)?;
        let ornament = load_table(db.as_deref(), base_path, "Ornament.json", "Ornament", |item: &crate::tables::ornament::Ornament| item.Id)?;
        let template_config = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i32| {
                db_c.get_template_by_id(*k)
            })))
        } else {
            load_table_from_disk(base_path, "TemplateConfig.json", |item: &crate::tables::template_config::TemplateConfig| item.Id)?
        };
        let explore_tools = load_table(db.as_deref(), base_path, "ExploreTools.json", "ExploreTools", |item: &crate::tables::explore_tools::ExploreTools| item.PhantomSkillId)?;
        let weapon_conf = load_table(db.as_deref(), base_path, "WeaponConf.json", "WeaponConf", |item: &crate::inline_tables::WeaponConf| item.ItemId)?;
        let weapon_base_attributes = load_table(db.as_deref(), base_path, "WeaponBaseAttributes.json", "WeaponBaseAttributes", |item: &crate::tables::weapon_base_attributes::WeaponBaseAttributes| item.ItemId)?;
        let weapon_property_growth = load_table(db.as_deref(), base_path, "WeaponPropertyGrowth.json", "WeaponPropertyGrowth", |item: &crate::tables::weapon_property_growth::WeaponPropertyGrowth| item.Id)?;
        let weapon_reson = load_table(db.as_deref(), base_path, "WeaponReson.json", "WeaponReson", |item: &crate::inline_tables::WeaponReson| item.Id)?;
        let weapon_exp_item = load_table(db.as_deref(), base_path, "WeaponExpItem.json", "WeaponExpItem", |item: &crate::tables::weapon_exp_item::WeaponExpItem| item.Id)?;
        let weapon_level = load_table(db.as_deref(), base_path, "WeaponLevel.json", "WeaponLevel", |item: &crate::tables::weapon_level::WeaponLevel| item.Id)?;
        let weapon_breach = load_table(db.as_deref(), base_path, "WeaponBreach.json", "WeaponBreach", |item: &crate::tables::weapon_breach::WeaponBreach| item.Id)?;
        let role_exp_item = load_table(db.as_deref(), base_path, "RoleExpItem.json", "RoleExpItem", |item: &crate::tables::role_exp_item::RoleExpItem| item.Id)?;
        let role_level_consume = load_table(db.as_deref(), base_path, "RoleLevelConsume.json", "RoleLevelConsume", |item: &crate::tables::role_level_consume::RoleLevelConsume| item.Id)?;
        let role_breach = load_table(db.as_deref(), base_path, "RoleBreach.json", "RoleBreach", |item: &crate::tables::role_breach::RoleBreach| item.Id)?;
        let skill_level = load_table(db.as_deref(), base_path, "SkillLevel.json", "SkillLevel", |item: &crate::tables::skill_level::SkillLevel| item.Id)?;
        let level_entity_config = if db.is_some() {
            Table::empty_lazy(None)
        } else {
            load_table_from_disk(base_path, "LevelEntityConfig.json", |item: &crate::tables::level_entity_config::LevelEntityConfig| item.EntityId)?
        };
        let damage = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i64| {
                db_c.get_damage(*k)
            })))
        } else {
            load_table_from_disk(base_path, "Damage.json", |item: &crate::tables::damage::Damage| item.Id)?
        };
        let area = load_table(db.as_deref(), base_path, "Area.json", "Area", |item: &crate::tables::area::Area| item.AreaId)?;
        let infr_v2_tree_build = load_table(db.as_deref(), base_path, "InfrV2TreeBuild.json", "InfrV2TreeBuild", |item: &crate::tables::infr_v2_tree_build::InfrV2TreeBuild| item.Id)?;
        let infr_road_build = load_table(db.as_deref(), base_path, "InfrRoadBuild.json", "InfrRoadBuild", |item: &crate::tables::infr_road_build::InfrRoadBuild| item.Id)?;
        let teleporter = load_table(db.as_deref(), base_path, "Teleporter.json", "Teleporter", |item: &crate::tables::teleporter::Teleporter| item.Id)?;
        let activity = load_table(db.as_deref(), base_path, "Activity.json", "Activity", |item: &crate::tables::activity::Activity| item.Id)?;
        let map_fog = load_table(db.as_deref(), base_path, "MapFog.json", "MapFog", |item: &crate::tables::map_fog::MapFog| item.Fog)?;
        let multi_map = load_table(db.as_deref(), base_path, "MultiMap.json", "MultiMap", |item: &crate::tables::multi_map::MultiMap| item.Id)?;
        let map_block_info = load_table(db.as_deref(), base_path, "MapBlockInfo.json", "MapBlockInfo", |item: &crate::tables::map_block_info::MapBlockInfo| item.BlockId)?;
        let explore_progress_reward = load_table(db.as_deref(), base_path, "ExploreProgressReward.json", "ExploreProgressReward", |item: &crate::tables::explore_progress_reward::ExploreProgressReward| item.Id)?;
        let explore_reward = load_table(db.as_deref(), base_path, "ExploreReward.json", "ExploreReward", |item: &crate::tables::explore_reward::ExploreReward| item.Id)?;
        let explore_score = load_table(db.as_deref(), base_path, "ExploreScore.json", "ExploreScore", |item: &crate::tables::explore_score::ExploreScore| item.Area)?;
        let flow = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &String| {
                db_c.get_flow(k.as_str())
            })))
        } else {
            load_table_from_disk(base_path, "Flow.json", |item: &crate::tables::flow::Flow| item.Id.clone())?
        };
        let skill = load_table(db.as_deref(), base_path, "Skill.json", "Skill", |item: &crate::tables::skill::Skill| item.Id)?;
        let skill_tree = load_table(db.as_deref(), base_path, "SkillTree.json", "SkillTree", |item: &crate::tables::skill_tree::SkillTree| item.Id)?;
        let resonant_chain = load_table(db.as_deref(), base_path, "ResonantChain.json", "ResonantChain", |item: &crate::tables::resonant_chain::ResonantChain| item.Id)?;
        let rogue_res_buff = load_table(db.as_deref(), base_path, "RogueResCharacterBuff.json", "RogueResCharacterBuff", |item: &crate::tables::rogue_res_character_buff::RogueResCharacterBuff| item.Id)?;
        let honami_story_effect = load_table(db.as_deref(), base_path, "HonamiStoryEffect.json", "HonamiStoryEffect", |item: &crate::tables::honami_story_effect::HonamiStoryEffect| item.Id)?;
        let combo_teaching = load_table(db.as_deref(), base_path, "ComboTeaching.json", "ComboTeaching", |item: &crate::tables::combo_teaching::ComboTeaching| item.Id)?;
        let entity_skill_preload = if let Some(ref db) = db {
            let _ = db;
            Table::empty_lazy(None)
        } else {
            load_table_from_disk(base_path, "EntitySkillPreload.json", |item: &crate::tables::entity_skill_preload::EntitySkillPreload| item.Id)?
        };
        let common_skill_preload = load_table(db.as_deref(), base_path, "CommonSkillPreload.json", "CommonSkillPreload", |item: &crate::tables::common_skill_preload::CommonSkillPreload| item.Id)?;
        let passive_skill = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i64| {
                db_c.get_passive_skill(*k)
            })))
        } else {
            load_table_from_disk(base_path, "PassiveSkill.json", |item: &crate::tables::passive_skill::PassiveSkill| item.Id)?
        };
        let favor_word = load_table(db.as_deref(), base_path, "FavorWord.json", "FavorWord", |item: &crate::tables::favor_word::FavorWord| item.Id)?;
        let favor_story = load_table(db.as_deref(), base_path, "FavorStory.json", "FavorStory", |item: &crate::tables::favor_story::FavorStory| item.Id)?;
        let favor_goods = load_table(db.as_deref(), base_path, "FavorGoods.json", "FavorGoods", |item: &crate::tables::favor_goods::FavorGoods| item.Id)?;
        let motion = load_table(db.as_deref(), base_path, "Motion.json", "Motion", |item: &crate::tables::motion::Motion| item.Id)?;
        let guide_group = load_table(db.as_deref(), base_path, "GuideGroup.json", "GuideGroup", |item: &crate::tables::guide_group::GuideGroup| item.Id)?;
        let char_init_info = load_table(db.as_deref(), base_path, "CharacterInitInfo.json", "CharacterInitInfo", |item: &crate::tables::character_init_info::CharacterInitInfo| item.RoleId)?;
        let skill_branch_buff = load_table(db.as_deref(), base_path, "SkillBranchBuff.json", "SkillBranchBuff", |item: &crate::tables::skill_branch_buff::SkillBranchBuff| item.BranchId)?;
        let phonograph_album = load_table(db.as_deref(), base_path, "PhonographAlbum.json", "PhonographAlbum", |item: &crate::tables::phonograph_album::PhonographAlbum| item.Id)?;
        let phonograph_music = load_table(db.as_deref(), base_path, "PhonographMusic.json", "PhonographMusic", |item: &crate::tables::phonograph_music::PhonographMusic| item.Id)?;
        let background_card = load_table(db.as_deref(), base_path, "BackgroundCard.json", "BackgroundCard", |item: &crate::tables::background_card::BackgroundCard| item.Id)?;
        let player_head_re = load_table(db.as_deref(), base_path, "PlayerHeadRe.json", "PlayerHeadRe", |item: &crate::tables::player_head_re::PlayerHeadRe| item.Id)?;
        let player_title = load_table(db.as_deref(), base_path, "PlayerTitle.json", "PlayerTitle", |item: &crate::tables::player_title::PlayerTitle| item.Id)?;
        let motor_tech_tree = load_table(db.as_deref(), base_path, "MotorTechTree.json", "MotorTechTree", |item: &crate::tables::motor_tech_tree::MotorTechTree| item.Id)?;
        let motor_scene = load_table(db.as_deref(), base_path, "MotorScene.json", "MotorScene", |item: &crate::tables::motor_scene::MotorScene| item.Id)?;
        let motor_lvl = load_table(db.as_deref(), base_path, "MotorLvl.json", "MotorLvl", |item: &crate::tables::motor_lvl::MotorLvl| item.Level)?;
        let motor_tech = load_table(db.as_deref(), base_path, "MotorTech.json", "MotorTech", |item: &crate::tables::motor_tech::MotorTech| item.Id)?;
        let motor_tech_lv = load_table(db.as_deref(), base_path, "MotorTechLv.json", "MotorTechLv", |item: &crate::tables::motor_tech_lv::MotorTechLv| item.Id)?;
        let motor_task = load_table(db.as_deref(), base_path, "MotorTask.json", "MotorTask", |item: &crate::tables::motor_task::MotorTask| item.Id)?;
        let motor_skin = load_table(db.as_deref(), base_path, "MotorSkin.json", "MotorSkin", |item: &crate::tables::motor_skin::MotorSkin| item.Id)?;
        let motor_sticker = load_table(db.as_deref(), base_path, "MotorSticker.json", "MotorSticker", |item: &crate::tables::motor_sticker::MotorSticker| item.Id)?;
        let motor_sticker_part = load_table(db.as_deref(), base_path, "MotorStickerPart.json", "MotorStickerPart", |item: &crate::tables::motor_sticker_part::MotorStickerPart| item.Id)?;
        let motor_load_project = load_table(db.as_deref(), base_path, "MotorLoadProject.json", "MotorLoadProject", |item: &crate::tables::motor_load_project::MotorLoadProject| item.Id)?;
        let motor_general_preview = load_table(db.as_deref(), base_path, "MotorGeneralPreview.json", "MotorGeneralPreview", |item: &crate::tables::motor_general_preview::MotorGeneralPreview| item.Id)?;
        let motor_decal_ip = load_table(db.as_deref(), base_path, "MotorDecalIp.json", "MotorDecalIp", |item: &crate::tables::motor_decal_ip::MotorDecalIp| item.Id)?;
        let motor_linkage_ip = load_table(db.as_deref(), base_path, "MotorLinkageIp.json", "MotorLinkageIp", |item: &crate::tables::motor_linkage_ip::MotorLinkageIp| item.Id)?;
        let motor_frame = load_table(db.as_deref(), base_path, "MotorFrame.json", "MotorFrame", |item: &crate::tables::motor_frame::MotorFrame| item.Id)?;
        let motor_decorations = load_table(db.as_deref(), base_path, "MotorDecorations.json", "MotorDecorations", |item: &crate::tables::motor_decorations::MotorDecorations| item.Id)?;
        let motor_decorations_part = load_table(db.as_deref(), base_path, "MotorDecorationsPart.json", "MotorDecorationsPart", |item: &crate::tables::motor_decorations_part::MotorDecorationsPart| item.Id)?;
        let gacha = load_table(db.as_deref(), base_path, "Gacha.json", "Gacha", |item: &crate::tables::gacha::Gacha| item.Id)?;
        let gacha_pool = load_table(db.as_deref(), base_path, "GachaPool.json", "GachaPool", |item: &crate::tables::gacha_pool::GachaPool| item.Id)?;
        let gacha_view_info = load_table(db.as_deref(), base_path, "GachaViewInfo.json", "GachaViewInfo", |item: &crate::tables::gacha_view_info::GachaViewInfo| item.Id)?;
        let gacha_texture_info = load_table(db.as_deref(), base_path, "GachaTextureInfo.json", "GachaTextureInfo", |item: &crate::tables::gacha_texture_info::GachaTextureInfo| item.Id)?;
        let ai_state_machine_config = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &String| {
                db_c.get_ai_state_machine_config(k.as_str())
            })))
        } else {
            load_table_from_disk(base_path, "AiStateMachineConfig.json", |item: &crate::tables::ai_state_machine_config::AiStateMachineConfig| item.Id.clone())?
        };
        let ai_base = load_table(db.as_deref(), base_path, "AiBase.json", "AiBase", |item: &crate::tables::ai_base::AiBase| item.Id)?;
        let hardness_mode = load_table(db.as_deref(), base_path, "HardnessMode.json", "HardnessMode", |item: &crate::tables::hardness_mode::HardnessMode| item.Id)?;
        let ai_sense_group = load_table(db.as_deref(), base_path, "AiSenseGroup.json", "AiSenseGroup", |item: &crate::tables::ai_sense_group::AiSenseGroup| item.Id)?;
        let phantom_item = load_table(db.as_deref(), base_path, "PhantomItem.json", "PhantomItem", |item: &crate::tables::phantom_item::PhantomItem| item.ItemId)?;
        let phantom_skill = load_table(db.as_deref(), base_path, "PhantomSkill.json", "PhantomSkill", |item: &crate::tables::phantom_skill::PhantomSkill| item.PhantomSkillId)?;
        let phantom_summon_group = load_table(db.as_deref(), base_path, "PhantomSummonGroup.json", "PhantomSummonGroup", |item: &crate::tables::phantom_summon_group::PhantomSummonGroup| item.Id)?;
        let phantom_fetter = load_table(db.as_deref(), base_path, "PhantomFetter.json", "PhantomFetter", |item: &crate::tables::phantom_fetter::PhantomFetter| item.Id)?;
        let phantom_fetter_group = load_table(db.as_deref(), base_path, "PhantomFetterGroup.json", "PhantomFetterGroup", |item: &crate::tables::phantom_fetter_group::PhantomFetterGroup| item.Id)?;
        let phantom_main_property = load_table(db.as_deref(), base_path, "PhantomMainProperty.json", "PhantomMainProperty", |item: &crate::tables::phantom_main_property::PhantomMainProperty| item.Id)?;
        let phantom_main_prop_item = load_table(db.as_deref(), base_path, "PhantomMainPropItem.json", "PhantomMainPropItem", |item: &crate::tables::phantom_main_prop_item::PhantomMainPropItem| item.Id)?;
        let phantom_sub_property = load_table(db.as_deref(), base_path, "PhantomSubProperty.json", "PhantomSubProperty", |item: &crate::tables::phantom_sub_property::PhantomSubProperty| item.Id)?;
        let phantom_rarity = load_table(db.as_deref(), base_path, "PhantomRarity.json", "PhantomRarity", |item: &crate::tables::phantom_rarity::PhantomRarity| item.Rare)?;
        let phantom_quality = load_table(db.as_deref(), base_path, "PhantomQuality.json", "PhantomQuality", |item: &crate::tables::phantom_quality::PhantomQuality| item.Quality)?;
        let phantom_growth = load_table(db.as_deref(), base_path, "PhantomGrowth.json", "PhantomGrowth", |item: &crate::tables::phantom_growth::PhantomGrowth| item.Id)?;
        let phantom_level = load_table(db.as_deref(), base_path, "PhantomLevel.json", "PhantomLevel", |item: &crate::tables::phantom_level::PhantomLevel| item.Id)?;
        let phantom_exp_item = load_table(db.as_deref(), base_path, "PhantomExpItem.json", "PhantomExpItem", |item: &crate::tables::phantom_exp_item::PhantomExpItem| item.ItemId)?;
        let phantom_vice_polish_config = load_table(db.as_deref(), base_path, "PhantomVicePolishConfig.json", "PhantomVicePolishConfig", |item: &crate::tables::phantom_vice_polish_config::PhantomVicePolishConfig| item.PropCount)?;
        let calabash_level = load_table(db.as_deref(), base_path, "CalabashLevel.json", "CalabashLevel", |item: &crate::tables::calabash_level::CalabashLevel| item.Level)?;
        let calabash_develop_condition = load_table(db.as_deref(), base_path, "CalabashDevelopCondition.json", "CalabashDevelopCondition", |item: &crate::tables::calabash_develop_condition::CalabashDevelopCondition| item.Id)?;
        let calabash_develop_reward = load_table(db.as_deref(), base_path, "CalabashDevelopReward.json", "CalabashDevelopReward", |item: &crate::tables::calabash_develop_reward::CalabashDevelopReward| item.MonsterId)?;
        let calabash_skin = load_table(db.as_deref(), base_path, "CalabashSkin.json", "CalabashSkin", |item: &crate::tables::calabash_skin::CalabashSkin| item.Id)?;
        let drop_package = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i32| {
                db_c.get_drop_package(*k)
            })))
        } else {
            load_table_from_disk(base_path, "DropPackage.json", |item: &crate::tables::drop_package::DropPackage| item.Id)?
        };
        let drop_group = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i32| {
                db_c.get_drop_group(*k)
            })))
        } else {
            load_table_from_disk(base_path, "DropGroup.json", |item: &crate::tables::drop_group::DropGroup| item.UnitId)?
        };
        let player_exp = load_table(db.as_deref(), base_path, "PlayerExp.json", "PlayerExp", |item: &crate::tables::player_exp::PlayerExp| item.PlayerLevel)?;
        let item_info = load_table(db.as_deref(), base_path, "ItemInfo.json", "ItemInfo", |item: &crate::tables::item_info::ItemInfo| item.Id)?;
        let gift_package = load_table(db.as_deref(), base_path, "GiftPackage.json", "GiftPackage", |item: &crate::tables::gift_package::GiftPackage| item.Id)?;
        let condition = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i32| {
                db_c.get_condition(*k)
            })))
        } else {
            load_table_from_disk(base_path, "Condition.json", |item: &crate::tables::condition::Condition| item.Id)?
        };
        let condition_group = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i32| {
                db_c.get_condition_group(*k)
            })))
        } else {
            load_table_from_disk(base_path, "ConditionGroup.json", |item: &crate::tables::condition_group::ConditionGroup| item.Id)?
        };
        let quest_data = load_table(db.as_deref(), base_path, "QuestData.json", "QuestData", |item: &crate::tables::quest_data::QuestData| item.QuestId)?;
        let level_play_data = load_table(db.as_deref(), base_path, "LevelPlayData.json", "LevelPlayData", |item: &crate::tables::level_play_data::LevelPlayData| item.LevelPlayId)?;
        let map_mark = load_table(db.as_deref(), base_path, "MapMark.json", "MapMark", |item: &crate::tables::map_mark::MapMark| item.MarkId)?;
        let monster_detection = load_table(db.as_deref(), base_path, "MonsterDetection.json", "MonsterDetection", |item: &crate::tables::monster_detection::MonsterDetection| item.Id)?;
        let dungeon_detection = load_table(db.as_deref(), base_path, "DungeonDetection.json", "DungeonDetection", |item: &crate::tables::dungeon_detection::DungeonDetection| item.Id)?;
        let silent_area_detection = load_table(db.as_deref(), base_path, "SilentAreaDetection.json", "SilentAreaDetection", |item: &crate::tables::silent_area_detection::SilentAreaDetection| item.Id)?;
        let lord_gym = load_table(db.as_deref(), base_path, "LordGym.json", "LordGym", |item: &crate::tables::lord_gym::LordGym| item.Id)?;
        let lord_gym_entrance = load_table(db.as_deref(), base_path, "LordGymEntrance.json", "LordGymEntrance", |item: &crate::tables::lord_gym_entrance::LordGymEntrance| item.Id)?;
        let adventure_task = load_table(db.as_deref(), base_path, "AdventureTask.json", "AdventureTask", |item: &crate::tables::adventure_task::AdventureTask| item.Id)?;
        let adventure_task_chapter = load_table(db.as_deref(), base_path, "AdventureTaskChapter.json", "AdventureTaskChapter", |item: &crate::tables::adventure_task_chapter::AdventureTaskChapter| item.Id)?;
        let daily_task = load_table(db.as_deref(), base_path, "DailyTask.json", "DailyTask", |item: &crate::tables::daily_task::DailyTask| item.Id)?;
        let daily_task_group = load_table(db.as_deref(), base_path, "DailyTaskGroup.json", "DailyTaskGroup", |item: &crate::tables::daily_task_group::DailyTaskGroup| item.Id)?;
        let data_layer_config = load_table(db.as_deref(), base_path, "DataLayerConfig.json", "DataLayerConfig", |item: &crate::tables::data_layer_config::DataLayerConfig| item.Id)?;
        let weather = load_table(db.as_deref(), base_path, "Weather.json", "Weather", |item: &crate::tables::weather::Weather| item.Id)?;
        let role_trial_info = load_table(db.as_deref(), base_path, "RoleTrialInfo.json", "RoleTrialInfo", |item: &crate::tables::role_trial_info::RoleTrialInfo| item.Id)?;
        let instance_trial_role_config = load_table(db.as_deref(), base_path, "InstanceTrialRoleConfig.json", "InstanceTrialRoleConfig", |item: &crate::tables::instance_trial_role_config::InstanceTrialRoleConfig| item.Id)?;
        let trial_role_info = load_table(db.as_deref(), base_path, "TrialRoleInfo.json", "TrialRoleInfo", |item: &crate::tables::trial_role_info::TrialRoleInfo| item.Id)?;
        let trial_weapon_info = load_table(db.as_deref(), base_path, "TrialWeaponInfo.json", "TrialWeaponInfo", |item: &crate::tables::trial_weapon_info::TrialWeaponInfo| item.Id)?;
        let trial_phantom_prop_item = load_table(db.as_deref(), base_path, "TrialPhantomPropItem.json", "TrialPhantomPropItem", |item: &crate::tables::trial_phantom_prop_item::TrialPhantomPropItem| item.Id)?;
        let trail_phantom_prop = if let Some(ref db) = db {
            let db_c = Arc::clone(db);
            Table::empty_lazy(Some(Arc::new(move |k: &i32| {
                db_c.get_trail_phantom_prop(*k)
            })))
        } else {
            load_table_from_disk(base_path, "TrailPhantomProp.json", |item: &crate::tables::trail_phantom_prop::TrailPhantomProp| item.Id)?
        };
        let pay_shop = load_table(db.as_deref(), base_path, "PayShop.json", "PayShop", |item: &crate::tables::pay_shop::PayShop| item.Id)?;
        let pay_shop_tab = load_table(db.as_deref(), base_path, "PayShopTab.json", "PayShopTab", |item: &crate::tables::pay_shop_tab::PayShopTab| item.TabId)?;
        let pay_shop_recommend = load_table(db.as_deref(), base_path, "PayShopRecommend.json", "PayShopRecommend", |item: &crate::tables::pay_shop_recommend::PayShopRecommend| item.Id)?;
        let shop_info = load_table(db.as_deref(), base_path, "ShopInfo.json", "ShopInfo", |item: &crate::tables::shop_info::ShopInfo| item.Id)?;
        let shop_fixed = load_table(db.as_deref(), base_path, "ShopFixed.json", "ShopFixed", |item: &crate::tables::shop_fixed::ShopFixed| item.Id)?;
        let forge_formula = load_table(db.as_deref(), base_path, "ForgeFormula.json", "ForgeFormula", |item: &crate::tables::forge_formula::ForgeFormula| item.Id)?;
        let synthesis_formula = load_table(db.as_deref(), base_path, "SynthesisFormula.json", "SynthesisFormula", |item: &crate::tables::synthesis_formula::SynthesisFormula| item.Id)?;
        let synthesis_level = load_table(db.as_deref(), base_path, "SynthesisLevel.json", "SynthesisLevel", |item: &crate::tables::synthesis_level::SynthesisLevel| item.Id)?;
        let cook_formula = load_table(db.as_deref(), base_path, "CookFormula.json", "CookFormula", |item: &crate::tables::cook_formula::CookFormula| item.Id)?;
        let cook_level = load_table(db.as_deref(), base_path, "CookLevel.json", "CookLevel", |item: &crate::tables::cook_level::CookLevel| item.Id)?;
        let cook_processed = load_table(db.as_deref(), base_path, "CookProcessed.json", "CookProcessed", |item: &crate::tables::cook_processed::CookProcessed| item.Id)?;
        let achievement = load_table(db.as_deref(), base_path, "Achievement.json", "Achievement", |item: &crate::tables::achievement::Achievement| item.Id)?;
        let achievement_group = load_table(db.as_deref(), base_path, "AchievementGroup.json", "AchievementGroup", |item: &crate::tables::achievement_group::AchievementGroup| item.Id)?;
        let achievement_star_level = load_table(db.as_deref(), base_path, "AchievementStarLevel.json", "AchievementStarLevel", |item: &crate::tables::achievement_star_level::AchievementStarLevel| item.Level)?;
        let liveness = load_table(db.as_deref(), base_path, "Liveness.json", "Liveness", |item: &crate::tables::liveness::Liveness| item.Id)?;
        let liveness_task = load_table(db.as_deref(), base_path, "LivenessTask.json", "LivenessTask", |item: &crate::tables::liveness_task::LivenessTask| item.TaskId)?;
        let tower_config = load_table(db.as_deref(), base_path, "TowerConfig.json", "TowerConfig", |item: &crate::tables::tower_config::TowerConfig| item.Id)?;
        let tower_difficulty = load_table(db.as_deref(), base_path, "TowerDifficulty.json", "TowerDifficulty", |item: &crate::tables::tower_difficulty::TowerDifficulty| item.Difficulty)?;
        let tower_season_reward = load_table(db.as_deref(), base_path, "TowerSeasonReward.json", "TowerSeasonReward", |item: &crate::tables::tower_season_reward::TowerSeasonReward| item.Id)?;
        let fishing_tech = load_table(db.as_deref(), base_path, "FishingTech.json", "FishingTech", |item: &crate::tables::fishing_tech::FishingTech| item.Id)?;
        let fishing_reputation = load_table(db.as_deref(), base_path, "FishingReputation.json", "FishingReputation", |item: &crate::tables::fishing_reputation::FishingReputation| item.Level)?;
        let fishing_item = load_table(db.as_deref(), base_path, "FishingItem.json", "FishingItem", |item: &crate::tables::fishing_item::FishingItem| item.Id)?;
        let elemental_box_define = load_table(db.as_deref(), base_path, "ElementalBoxDefine.json", "ElementalBoxDefine", |item: &crate::tables::elemental_box_define::ElementalBoxDefine| item.Id)?;
        let giant_item = load_table(db.as_deref(), base_path, "GiantItem.json", "GiantItem", |item: &crate::tables::giant_item::GiantItem| item.Id)?;
        let giant_delivery_reward = load_table(db.as_deref(), base_path, "GiantDeliveryReward.json", "GiantDeliveryReward", |item: &crate::tables::giant_delivery_reward::GiantDeliveryReward| item.Id)?;
        let abyss_little_role = load_table(db.as_deref(), base_path, "AbyssLittleRole.json", "AbyssLittleRole", |item: &crate::tables::abyss_little_role::AbyssLittleRole| item.Id)?;
        let item_exchange_content = load_table(db.as_deref(), base_path, "ItemExchangeContent.json", "ItemExchangeContent", |item: &crate::inline_tables::ItemExchangeContent| item.ItemId)?;
        let dragon_pool = load_table(db.as_deref(), base_path, "DragonPool.json", "DragonPool", |item: &crate::inline_tables::DragonPool| item.Id)?;
        let loading_level_area = load_table(db.as_deref(), base_path, "LoadingLevelArea.json", "LoadingLevelArea", |item: &crate::inline_tables::LoadingLevelArea| item.Id)?;
        let world_new_journey = load_table(db.as_deref(), base_path, "WorldNewJourney.json", "WorldNewJourney", |item: &crate::inline_tables::WorldNewJourney| item.Id)?;
        let dark_coast_delivery = load_table(db.as_deref(), base_path, "DarkCoastDelivery.json", "DarkCoastDelivery", |item: &crate::inline_tables::DarkCoastDelivery| item.Id)?;
        let new_tower_season_award = load_table(db.as_deref(), base_path, "NewTowerSeasonAward.json", "NewTowerSeasonAward", |item: &crate::inline_tables::NewTowerSeasonAward| item.Id)?;
        let exchange_reward = load_table(db.as_deref(), base_path, "ExchangeReward.json", "ExchangeReward", |item: &crate::inline_tables::ExchangeReward| item.Id)?;
        let loop_teleport_volume_function = load_table(db.as_deref(), base_path, "LoopTeleportVolumeFunction.json", "LoopTeleportVolumeFunction", |item: &crate::inline_tables::LoopTeleportVolumeFunction| item.Id)?;
        let recall_config = load_table(db.as_deref(), base_path, "RecallConfig.json", "RecallConfig", |item: &crate::inline_tables::RecallConfig| item.RecallId)?;
        let boss_rush_buff = load_table(db.as_deref(), base_path, "BossRushBuff.json", "BossRushBuff", |item: &crate::inline_tables::BossRushBuff| item.Id)?;
        let instance_dungeon_entrance = load_table(db.as_deref(), base_path, "InstanceDungeonEntrance.json", "InstanceDungeonEntrance", |item: &crate::inline_tables::InstanceDungeonEntrance| item.Id)?;
        let consumptive_task = load_table(db.as_deref(), base_path, "ConsumptiveTask.json", "ConsumptiveTask", |item: &crate::inline_tables::ConsumptiveTask| item.Id)?;
        let doll_grab_items_csv = load_table(db.as_deref(), base_path, "DollGrabItemsCsv.json", "DollGrabItemsCsv", |item: &crate::inline_tables::DollGrabItemsCsv| item.Id)?;
        let track_moon_entrust = load_table(db.as_deref(), base_path, "TrackMoonEntrust.json", "TrackMoonEntrust", |item: &crate::inline_tables::TrackMoonEntrust| item.Id)?;
        let motor_decal_quest = load_table(db.as_deref(), base_path, "MotorDecalQuest.json", "MotorDecalQuest", |item: &crate::inline_tables::MotorDecalQuest| item.TaskId)?;
        let motor_linkage_quest = load_table(db.as_deref(), base_path, "MotorLinkageQuest.json", "MotorLinkageQuest", |item: &crate::inline_tables::MotorLinkageQuest| item.TaskId)?;
        let abnormal_damage_config = load_table(db.as_deref(), base_path, "AbnormalDamageConfig.json", "AbnormalDamageConfig", |item: &crate::inline_tables::AbnormalDamageConfig| item.Level)?;
        let buff_item = load_table(db.as_deref(), base_path, "BuffItem.json", "BuffItem", |item: &crate::inline_tables::BuffItem| item.Id)?;
        let quest_ref_video_config = load_table(db.as_deref(), base_path, "QuestRefVideoConfig.json", "QuestRefVideoConfig", |item: &crate::inline_tables::QuestRefVideoConfig| item.Id)?;
        let quest_ref_map_block_config = load_table(db.as_deref(), base_path, "QuestRefMapBlockConfig.json", "QuestRefMapBlockConfig", |item: &crate::inline_tables::QuestRefMapBlockConfig| item.QuestId)?;
        let quest_resource_white = load_table(db.as_deref(), base_path, "QuestResourceWhite.json", "QuestResourceWhite", |item: &crate::inline_tables::QuestResourceWhite| item.QuestId)?;
        let morale_keep_level = load_table(db.as_deref(), base_path, "MoraleKeepLevel.json", "MoraleKeepLevel", |item: &crate::inline_tables::MoraleKeepLevel| item.Level)?;
        let morale_play = load_table(db.as_deref(), base_path, "MoralePlay.json", "MoralePlay", |item: &crate::inline_tables::MoralePlay| item.Id)?;
        let battle_score_conf = load_table(db.as_deref(), base_path, "BattleScoreConf.json", "BattleScoreConf", |item: &crate::inline_tables::BattleScoreConf| item.Id)?;
        let battle_score_level_conf = load_table(db.as_deref(), base_path, "BattleScoreLevelConf.json", "BattleScoreLevelConf", |item: &crate::inline_tables::BattleScoreLevelConf| item.Id)?;
        let drop_show_plan = load_table(db.as_deref(), base_path, "DropShowPlan.json", "DropShowPlan", |item: &crate::inline_tables::DropShowPlan| item.Id)?;
        let prefab_config = load_table(db.as_deref(), base_path, "PrefabConfig.json", "PrefabConfig", |item: &crate::inline_tables::PrefabConfig| item.Id)?;
        let pay_item = load_table(db.as_deref(), base_path, "PayItem.json", "PayItem", |item: &crate::inline_tables::PayItemCfg| item.Id)?;
        let pay = load_table(db.as_deref(), base_path, "Pay.json", "Pay", |item: &crate::inline_tables::PayCfg| item.Id)?;
        let pay_gift = load_table(db.as_deref(), base_path, "PayGift.json", "PayGift", |item: &crate::inline_tables::PayGiftCfg| item.Id)?;
        let battle_pass = load_table(db.as_deref(), base_path, "BattlePass.json", "BattlePass", |item: &crate::inline_tables::BattlePassCfg| item.Id)?;
        let phantom_recommend = load_table(db.as_deref(), base_path, "PhantomRecommend.json", "PhantomRecommend", |item: &crate::inline_tables::PhantomRecommendCfg| item.RoleId)?;
        let boss_rush_score = load_table(db.as_deref(), base_path, "BossRushScore.json", "BossRushScore", |item: &crate::inline_tables::BossRushScoreCfg| item.Id)?;
        let gacha_accumulate_reward = load_table(db.as_deref(), base_path, "GachaAccumulateReward.json", "GachaAccumulateReward", |item: &crate::inline_tables::GachaAccumulateRewardCfg| item.Id)?;

        Ok(Self {
            db,
            concerto_exceptions,
            role_info,
            role_be_hit_map,
            main_role_config,
            base_property,
            function_condition,
            role_property_growth,
            monster_property_growth,
            instance_dungeon,
            buff,
            shield,
            summon_cfg,
            model_config_preload,
            character_part_config,
            gameplay_tag_parent,
            gameplay_tag_remap,
            blueprint_config,
            role_skin,
            fly_skin_config,
            weapon_skin,
            ornament,
            template_config,
            explore_tools,
            weapon_conf,
            weapon_base_attributes,
            weapon_property_growth,
            weapon_reson,
            weapon_exp_item,
            weapon_level,
            weapon_breach,
            role_exp_item,
            role_level_consume,
            role_breach,
            skill_level,
            level_entity_config,
            damage,
            area,
            infr_v2_tree_build,
            infr_road_build,
            teleporter,
            activity,
            map_fog,
            multi_map,
            map_block_info,
            explore_progress_reward,
            explore_reward,
            explore_score,
            flow,
            skill,
            skill_tree,
            resonant_chain,
            rogue_res_buff,
            honami_story_effect,
            combo_teaching,
            entity_skill_preload,
            common_skill_preload,
            passive_skill,
            favor_word,
            favor_story,
            favor_goods,
            motion,
            guide_group,
            char_init_info,
            skill_branch_buff,
            phonograph_album,
            phonograph_music,
            background_card,
            player_head_re,
            player_title,
            motor_tech_tree,
            motor_scene,
            motor_lvl,
            motor_tech,
            motor_tech_lv,
            motor_task,
            motor_skin,
            motor_sticker,
            motor_sticker_part,
            motor_load_project,
            motor_general_preview,
            motor_decal_ip,
            motor_linkage_ip,
            motor_frame,
            motor_decorations,
            motor_decorations_part,
            gacha,
            gacha_pool,
            gacha_view_info,
            gacha_texture_info,
            ai_state_machine_config,
            ai_base,
            hardness_mode,
            ai_sense_group,
            phantom_item,
            phantom_skill,
            phantom_summon_group,
            phantom_fetter,
            phantom_fetter_group,
            phantom_main_property,
            phantom_main_prop_item,
            phantom_sub_property,
            phantom_rarity,
            phantom_quality,
            phantom_growth,
            phantom_level,
            phantom_exp_item,
            phantom_vice_polish_config,
            calabash_level,
            calabash_develop_condition,
            calabash_develop_reward,
            calabash_skin,
            drop_package,
            drop_group,
            player_exp,
            item_info,
            gift_package,
            condition,
            condition_group,
            quest_data,
            level_play_data,
            map_mark,
            monster_detection,
            dungeon_detection,
            silent_area_detection,
            lord_gym,
            lord_gym_entrance,
            adventure_task,
            adventure_task_chapter,
            daily_task,
            daily_task_group,
            data_layer_config,
            weather,
            role_trial_info,
            instance_trial_role_config,
            trial_role_info,
            trial_weapon_info,
            trial_phantom_prop_item,
            trail_phantom_prop,
            pay_shop,
            pay_shop_tab,
            pay_shop_recommend,
            shop_info,
            shop_fixed,
            forge_formula,
            synthesis_formula,
            synthesis_level,
            cook_formula,
            cook_level,
            cook_processed,
            achievement,
            achievement_group,
            achievement_star_level,
            liveness,
            liveness_task,
            tower_config,
            tower_difficulty,
            tower_season_reward,
            fishing_tech,
            fishing_reputation,
            fishing_item,
            elemental_box_define,
            giant_item,
            giant_delivery_reward,
            abyss_little_role,
            item_exchange_content,
            dragon_pool,
            loading_level_area,
            world_new_journey,
            dark_coast_delivery,
            new_tower_season_award,
            exchange_reward,
            loop_teleport_volume_function,
            recall_config,
            boss_rush_buff,
            instance_dungeon_entrance,
            consumptive_task,
            doll_grab_items_csv,
            track_moon_entrust,
            motor_decal_quest,
            motor_linkage_quest,
            abnormal_damage_config,
            buff_item,
            quest_ref_video_config,
            quest_ref_map_block_config,
            quest_resource_white,
            morale_keep_level,
            morale_play,
            battle_score_conf,
            battle_score_level_conf,
            drop_show_plan,
            prefab_config,
            pay_item,
            pay,
            pay_gift,
            battle_pass,
            phantom_recommend,
            boss_rush_score,
            gacha_accumulate_reward,
        })
    }

    pub fn assets_db(&self) -> Option<&Arc<AssetsDb>> {
        self.db.as_ref()
    }

    pub fn get_role_property_growth(&self, level: i32, breach: i32) -> Option<&RolePropertyGrowth> {
        self.role_property_growth
            .items
            .iter()
            .find(|c| c.Level == level && c.BreachLevel == breach)
    }

    pub fn get_monster_property_growth(
        &self,
        level: i32,
        curve_id: i32,
    ) -> Option<&MonsterPropertyGrowth> {
        self.monster_property_growth
            .items
            .iter()
            .find(|c| c.Level == level && c.CurveId == curve_id)
    }

    pub fn get_props(&self, property_id: i32) -> Vec<i32> {
        let Some(base_property) = self.base_property.get_data_by_id(&property_id) else {
            return Vec::new();
        };
        base_property_to_props(&base_property)
    }

    pub fn get_motorcycle_config_id(&self) -> Option<i32> {
        for tool in &self.explore_tools.items {
            if tool.SummonConfigId == 0 {
                continue;
            }
            let Some(summon) = self.summon_cfg.get_data_by_id(&tool.SummonConfigId) else {
                continue;
            };
            let Some(bp) = self
                .blueprint_config
                .items
                .iter()
                .find(|e| e.BlueprintType == summon.BlueprintType)
            else {
                continue;
            };
            if bp.EntityLogic != EntityLogic::Vehicle {
                continue;
            }
            if let Some(ref db) = self.db {
                if let Some(tpl) = db.find_template_by_blueprint(&summon.BlueprintType) {
                    return Some(tpl.Id);
                }
                continue;
            }
            if let Some(tpl) = self
                .template_config
                .items
                .iter()
                .find(|t| t.BlueprintType == summon.BlueprintType)
            {
                return Some(tpl.Id);
            }
        }
        None
    }

    pub fn resolve_role_id(&self, role_id: i32) -> i32 {
        if let Some(trial) = self.trial_role_info.get_data_by_id(&role_id)
            && trial.ParentId != 0
        {
            return trial.ParentId;
        }
        role_id
    }

    pub fn resolve_base_role_info(&self, role_id: i32) -> Option<RoleInfo> {
        if let Some(info) = self.role_info.get_data_by_id(&role_id) {
            return Some((*info).clone());
        }
        if let Some(trial) = self.trial_role_info.get_data_by_id(&role_id)
            && trial.ParentId != 0
        {
            return self
                .role_info
                .get_data_by_id(&trial.ParentId)
                .map(|c| (*c).clone());
        }
        None
    }

    pub fn resolve_trial_role_id(
        &self,
        raw_id: i32,
        world_level: i32,
        gender: i32,
    ) -> Option<i32> {
        let mut best_any_gender: Option<&TrialRoleInfo> = None;
        let mut best_match_gender: Option<&TrialRoleInfo> = None;

        for item in &self.trial_role_info.items {
            if item.GroupId != raw_id {
                continue;
            }
            if (item.Gender == -1 || item.Gender == gender)
                && is_better_trial_match(best_match_gender, item, world_level, gender)
            {
                best_match_gender = Some(item);
            }
            if is_better_trial_match(best_any_gender, item, world_level, gender) {
                best_any_gender = Some(item);
            }
        }

        if let Some(m) = best_match_gender {
            return Some(m.Id);
        }
        if let Some(m) = best_any_gender {
            return Some(m.Id);
        }
        self.trial_role_info.get_data_by_id(&raw_id).map(|e| e.Id)
    }

    pub fn get_weapon_reson(&self, reson_id: i32, level: i32) -> Option<&WeaponReson> {
        self.weapon_reson
            .items
            .iter()
            .find(|r| r.ResonId == reson_id && r.Level == level)
    }

    pub fn get_role_auto_buffs(
        &self,
        role_id: i32,
        weapon: WeaponEquipInfo,
        skill_branch: i32,
    ) -> Result<Vec<BuffAdditionEntry>, &'static str> {
        let resolved_role_id = self.resolve_role_id(role_id);
        let role_info = self
            .resolve_base_role_info(role_id)
            .ok_or("RoleNotFound")?;
        let weapon_conf = self
            .weapon_conf
            .get_data_by_id(&weapon.id)
            .ok_or("WeaponConfigNotFound")?;
        let reson_conf = self.get_weapon_reson(weapon_conf.ResonId, weapon.reson_level);

        const ADDITIONAL_BUFFS: &[i64] = &[
            3003,
            3004,
            3127,
            3134,
            1213,
            1214,
            1215,
            1216,
            1001000000,
            70000077,
        ];
        const BLACKLISTED_BUFF_COMBOS: &[&str] = &[
            "666", "990", "777", "095", "402", "403", "302", "291", "98", "009", "020",
        ];
        const BLACKLISTED_TAG_COMPONENTS: &[&str] = &[
            "Rogue",
            "肉鸽",
            "试用",
            "零能量",
            "59b4b0a5",
            "371b7a72",
            "角色.R2T1JiyanMd10011.技能标识.大招状态",
            "23d0723c",
            "20b04084",
            "8c658780",
            "9296f07f",
            "c79fde44",
            "fbcbee28",
            "ed839a56",
            "功能.功能制作.禁止体力恢复",
        ];

        let mut results = Vec::new();
        let mut do_autobuff = true;

        if let Some(init_info) = self.char_init_info.get_data_by_id(&resolved_role_id) {
            if !init_info.FightBuffs.is_empty() {
                do_autobuff = false;
                for &buff_id in &init_info.FightBuffs {
                    if let Some(buff) = self.buff.get_data_by_id(&buff_id) {
                        results.push(BuffAdditionEntry {
                            id: buff_id,
                            stack_count: 0,
                            is_active: is_active_cond(&buff),
                            duration_seconds: None,
                        });
                    }
                }
            }
            if let Some(ref add_role_buffs) = init_info.AdditionalFightBuffs {
                for &buff_id in add_role_buffs {
                    if let Some(buff) = self.buff.get_data_by_id(&buff_id) {
                        results.push(BuffAdditionEntry {
                            id: buff_id,
                            stack_count: 0,
                            is_active: is_active_cond(&buff),
                            duration_seconds: None,
                        });
                    }
                }
            }
        }

        if do_autobuff {
            let owned_candidates;
            let candidates: &[Buff] = if let Some(ref db) = self.db {
                owned_candidates = db
                    .get_auto_buff_candidates(resolved_role_id)
                    .map_err(|_| "AutoBuffQueryFailed")?;
                &owned_candidates
            } else {
                &self.buff.items
            };

            let role_str = resolved_role_id.to_string();
            for buff in candidates {
                let id_str = buff.Id.to_string();
                if !id_str.starts_with(&role_str) || id_str.len() <= 4 {
                    continue;
                }
                if buff.DurationPolicy != DurationPolicy::Infinite {
                    continue;
                }
                if !buff.ApplicationSourceTagRequirements.is_empty()
                    || !buff.ApplicationTagRequirements.is_empty()
                    || !buff.PrematureExpirationEffects.is_empty()
                {
                    continue;
                }
                let suffix = &id_str[4..];
                if BLACKLISTED_BUFF_COMBOS
                    .iter()
                    .any(|combo| suffix.starts_with(combo))
                {
                    continue;
                }
                if buff.GrantedTags.iter().any(|tag| {
                    BLACKLISTED_TAG_COMPONENTS
                        .iter()
                        .any(|comp| tag.contains(comp))
                }) {
                    continue;
                }
                if buff.ExtraEffectID == 5 {
                    let valid = buff.ExtraEffectParameters.get(1).is_some_and(|param| {
                        let prefix = param.split('#').next().unwrap_or(param);
                        let Ok(ref_id) = prefix.parse::<i64>() else {
                            return true;
                        };
                        let Some(ref_buff) = self.buff.get_data_by_id(&ref_id) else {
                            return true;
                        };
                        !ref_buff.CalculationPolicy.is_empty() && ref_buff.CalculationPolicy[0] != 0
                    });
                    if !valid {
                        continue;
                    }
                }
                results.push(BuffAdditionEntry {
                    id: buff.Id,
                    stack_count: 0,
                    is_active: is_active_cond(buff),
                    duration_seconds: None,
                });
            }

            let mut illegal_ids: HashSet<i64> = HashSet::new();
            for chain in &self.resonant_chain.items {
                if chain.GroupId == role_info.ResonantChainGroupId {
                    illegal_ids.extend(&chain.BuffIds);
                }
            }
            for rogue in &self.rogue_res_buff.items {
                if rogue.BuffId.to_string().starts_with(&role_str) {
                    illegal_ids.insert(rogue.BuffId);
                }
                for &bid in &rogue.BuffIds {
                    if bid.to_string().starts_with(&role_str) {
                        illegal_ids.insert(bid);
                    }
                }
            }
            for honami in &self.honami_story_effect.items {
                for param in &honami.Param {
                    if param.len() >= 7
                        && let Ok(id) = param.parse::<i64>()
                    {
                        illegal_ids.insert(id);
                    }
                }
            }
            for combo in &self.combo_teaching.items {
                illegal_ids.extend(&combo.AddBuffID);
            }
            for entry in &results {
                let Some(buff) = self.buff.get_data_by_id(&entry.id) else {
                    continue;
                };
                if buff.ExtraEffectID == 6
                    && let Some(param) = buff.ExtraEffectParameters.get(3)
                {
                    for part in param.split('#') {
                        if let Ok(id) = part.parse::<i64>() {
                            illegal_ids.insert(id);
                        }
                    }
                }
            }
            for branch_data in &self.skill_branch_buff.items {
                if branch_data.RoleId == resolved_role_id {
                    illegal_ids.extend(&branch_data.Buffs);
                }
            }
            if !illegal_ids.is_empty() {
                results.retain(|e| !illegal_ids.contains(&e.id));
            }
        }

        for &id in ADDITIONAL_BUFFS {
            results.push(BuffAdditionEntry {
                id,
                stack_count: 0,
                is_active: true,
                duration_seconds: None,
            });
        }
        if let Some(branch_data) = self.skill_branch_buff.get_data_by_id(&skill_branch) {
            for &id in &branch_data.Buffs {
                results.push(BuffAdditionEntry {
                    id,
                    stack_count: 0,
                    is_active: true,
                    duration_seconds: None,
                });
            }
        }
        if let Some(main_role) = self.main_role_config.get_data_by_id(&role_id)
            && let Some(tag_buff_id) = main_role_tag_buff_id(main_role.Gender)
        {
            results.push(BuffAdditionEntry {
                id: tag_buff_id,
                stack_count: 0,
                is_active: true,
                duration_seconds: None,
            });
        }
        let concerto_exceptions = if self.concerto_exceptions.is_empty() {
            zon_config::cached_concerto_exceptions()
        } else {
            &self.concerto_exceptions
        };
        if !concerto_exceptions.contains(&role_id) {
            results.push(BuffAdditionEntry {
                id: 1101002011,
                stack_count: 0,
                is_active: true,
                duration_seconds: None,
            });
        }
        if let Some(rc) = reson_conf {
            for &id in &rc.Effect {
                results.push(BuffAdditionEntry {
                    id,
                    stack_count: 0,
                    is_active: true,
                    duration_seconds: None,
                });
            }
        }
        let elem = i64::from(role_info.ElementId);
        results.push(BuffAdditionEntry {
            id: 1101007999 + elem,
            stack_count: 0,
            is_active: true,
            duration_seconds: None,
        });
        results.push(BuffAdditionEntry {
            id: 1001001015 + (elem * 2),
            stack_count: 0,
            is_active: true,
            duration_seconds: None,
        });
        results.push(BuffAdditionEntry {
            id: 3080 + elem,
            stack_count: 0,
            is_active: true,
            duration_seconds: None,
        });
        results.push(BuffAdditionEntry {
            id: 640012051,
            stack_count: 0,
            is_active: true,
            duration_seconds: None,
        });

        Ok(results)
    }
}

fn load_table<T, K>(
    db: Option<&AssetsDb>,
    base_path: &str,
    json_name: &str,
    stem: &str,
    key_fn: impl Fn(&T) -> K,
) -> Result<Table<T, K>, String>
where
    T: DeserializeOwned + Clone,
    K: Eq + Hash,
{
    if let Some(db) = db
        && let Some(raw) = db.read_raw_file(stem)
    {
        let items: Vec<T> = serde_json::from_str(&raw)
            .map_err(|e| format!("failed parsing raw table {stem}: {e}"))?;
        return Ok(Table::from_items(items, key_fn));
    }
    load_table_from_disk(base_path, json_name, key_fn)
}

fn load_table_from_disk<T, K>(
    base_path: &str,
    json_name: &str,
    key_fn: impl Fn(&T) -> K,
) -> Result<Table<T, K>, String>
where
    T: DeserializeOwned + Clone,
    K: Eq + Hash,
{
    let full = format!("{base_path}/{json_name}");
    if Path::new(&full).exists() {
        let raw = std::fs::read_to_string(&full)
            .map_err(|e| format!("failed reading {full}: {e}"))?;
        let items: Vec<T> = serde_json::from_str(&raw)
            .map_err(|e| format!("failed parsing {full}: {e}"))?;
        return Ok(Table::from_items(items, key_fn));
    }

    let stem = json_name.strip_suffix(".json").unwrap_or(json_name);
    let mut all_items = Vec::new();
    let mut chunk = 1usize;
    loop {
        let chunk_path = format!("{base_path}/{stem}_{chunk}.json");
        if !Path::new(&chunk_path).exists() {
            break;
        }
        let raw = std::fs::read_to_string(&chunk_path)
            .map_err(|e| format!("failed reading {chunk_path}: {e}"))?;
        let mut chunk_items: Vec<T> = serde_json::from_str(&raw)
            .map_err(|e| format!("failed parsing {chunk_path}: {e}"))?;
        all_items.append(&mut chunk_items);
        chunk += 1;
    }

    if chunk == 1 {
        return Err(format!("missing table file {full}"));
    }
    Ok(Table::from_items(all_items, key_fn))
}

pub fn base_property_to_props(bp: &BaseProperty) -> Vec<i32> {
    let mut props = vec![0i32; EAttributeType::MAX as usize];
    props[EAttributeType::Lv as usize] = bp.Lv;
    props[EAttributeType::LifeMax as usize] = bp.LifeMax;
    props[EAttributeType::Life as usize] = bp.Life;
    props[EAttributeType::Sheild as usize] = bp.Sheild;
    props[EAttributeType::SheildDamageChange as usize] = bp.SheildDamageChange;
    props[EAttributeType::SheildDamageReduce as usize] = bp.SheildDamageReduce;
    props[EAttributeType::Atk as usize] = bp.Atk;
    props[EAttributeType::Crit as usize] = bp.Crit;
    props[EAttributeType::CritDamage as usize] = bp.CritDamage;
    props[EAttributeType::Def as usize] = bp.Def;
    props[EAttributeType::EnergyEfficiency as usize] = bp.EnergyEfficiency;
    props[EAttributeType::CdReduse as usize] = bp.CdReduse;
    props[EAttributeType::DamageChangeNormalSkill as usize] = bp.DamageChangeNormalSkill;
    props[EAttributeType::DamageChange as usize] = bp.DamageChange;
    props[EAttributeType::DamageReduce as usize] = bp.DamageReduce;
    props[EAttributeType::DamageChangeAuto as usize] = bp.DamageChangeAuto;
    props[EAttributeType::DamageChangeCast as usize] = bp.DamageChangeCast;
    props[EAttributeType::DamageChangeUltra as usize] = bp.DamageChangeUltra;
    props[EAttributeType::DamageChangeQte as usize] = bp.DamageChangeQte;
    props[EAttributeType::DamageChangePhys as usize] = bp.DamageChangePhys;
    props[EAttributeType::DamageChangeElement1 as usize] = bp.DamageChangeElement1;
    props[EAttributeType::DamageChangeElement2 as usize] = bp.DamageChangeElement2;
    props[EAttributeType::DamageChangeElement3 as usize] = bp.DamageChangeElement3;
    props[EAttributeType::DamageChangeElement4 as usize] = bp.DamageChangeElement4;
    props[EAttributeType::DamageChangeElement5 as usize] = bp.DamageChangeElement5;
    props[EAttributeType::DamageChangeElement6 as usize] = bp.DamageChangeElement6;
    props[EAttributeType::DamageResistancePhys as usize] = bp.DamageResistancePhys;
    props[EAttributeType::DamageResistanceElement1 as usize] = bp.DamageResistanceElement1;
    props[EAttributeType::DamageResistanceElement2 as usize] = bp.DamageResistanceElement2;
    props[EAttributeType::DamageResistanceElement3 as usize] = bp.DamageResistanceElement3;
    props[EAttributeType::DamageResistanceElement4 as usize] = bp.DamageResistanceElement4;
    props[EAttributeType::DamageResistanceElement5 as usize] = bp.DamageResistanceElement5;
    props[EAttributeType::DamageResistanceElement6 as usize] = bp.DamageResistanceElement6;
    props[EAttributeType::HealChange as usize] = bp.HealChange;
    props[EAttributeType::HealedChange as usize] = bp.HealedChange;
    props[EAttributeType::DamageReducePhys as usize] = bp.DamageReducePhys;
    props[EAttributeType::DamageReduceElement1 as usize] = bp.DamageReduceElement1;
    props[EAttributeType::DamageReduceElement2 as usize] = bp.DamageReduceElement2;
    props[EAttributeType::DamageReduceElement3 as usize] = bp.DamageReduceElement3;
    props[EAttributeType::DamageReduceElement4 as usize] = bp.DamageReduceElement4;
    props[EAttributeType::DamageReduceElement5 as usize] = bp.DamageReduceElement5;
    props[EAttributeType::DamageReduceElement6 as usize] = bp.DamageReduceElement6;
    props[EAttributeType::EnergyMax as usize] = bp.EnergyMax;
    props[EAttributeType::Energy as usize] = bp.Energy;
    props[EAttributeType::SpecialEnergy1Max as usize] = bp.SpecialEnergy1Max;
    props[EAttributeType::SpecialEnergy1 as usize] = bp.SpecialEnergy1;
    props[EAttributeType::SpecialEnergy2Max as usize] = bp.SpecialEnergy2Max;
    props[EAttributeType::SpecialEnergy2 as usize] = bp.SpecialEnergy2;
    props[EAttributeType::SpecialEnergy3Max as usize] = bp.SpecialEnergy3Max;
    props[EAttributeType::SpecialEnergy3 as usize] = bp.SpecialEnergy3;
    props[EAttributeType::SpecialEnergy4Max as usize] = bp.SpecialEnergy4Max;
    props[EAttributeType::SpecialEnergy4 as usize] = bp.SpecialEnergy4;
    props[EAttributeType::SpecialEnergy5Max as usize] = bp.SpecialEnergy5Max;
    props[EAttributeType::SpecialEnergy5 as usize] = bp.SpecialEnergy5;
    props[EAttributeType::StrengthMax as usize] = bp.StrengthMax;
    props[EAttributeType::Strength as usize] = bp.Strength;
    props[EAttributeType::StrengthRecover as usize] = bp.StrengthRecover;
    props[EAttributeType::StrengthPunishTime as usize] = bp.StrengthPunishTime;
    props[EAttributeType::StrengthRun as usize] = bp.StrengthRun;
    props[EAttributeType::StrengthSwim as usize] = bp.StrengthSwim;
    props[EAttributeType::StrengthFastSwim as usize] = bp.StrengthFastSwim;
    props[EAttributeType::HardnessMax as usize] = bp.HardnessMax;
    props[EAttributeType::Hardness as usize] = bp.Hardness;
    props[EAttributeType::HardnessRecover as usize] = bp.HardnessRecover;
    props[EAttributeType::HardnessPunishTime as usize] = bp.HardnessPunishTime;
    props[EAttributeType::HardnessChange as usize] = bp.HardnessChange;
    props[EAttributeType::HardnessReduce as usize] = bp.HardnessReduce;
    props[EAttributeType::RageMax as usize] = bp.RageMax;
    props[EAttributeType::Rage as usize] = bp.Rage;
    props[EAttributeType::RageRecover as usize] = bp.RageRecover;
    props[EAttributeType::RagePunishTime as usize] = bp.RagePunishTime;
    props[EAttributeType::RageChange as usize] = bp.RageChange;
    props[EAttributeType::RageReduce as usize] = bp.RageReduce;
    props[EAttributeType::ToughMax as usize] = bp.ToughMax;
    props[EAttributeType::Tough as usize] = bp.Tough;
    props[EAttributeType::ToughRecover as usize] = bp.ToughRecover;
    props[EAttributeType::ToughChange as usize] = bp.ToughChange;
    props[EAttributeType::ToughReduce as usize] = bp.ToughReduce;
    props[EAttributeType::ToughRecoverDelayTime as usize] = bp.ToughRecoverDelayTime;
    props[EAttributeType::ElementPower1 as usize] = bp.ElementPower1;
    props[EAttributeType::ElementPower2 as usize] = bp.ElementPower2;
    props[EAttributeType::ElementPower3 as usize] = bp.ElementPower3;
    props[EAttributeType::ElementPower4 as usize] = bp.ElementPower4;
    props[EAttributeType::ElementPower5 as usize] = bp.ElementPower5;
    props[EAttributeType::ElementPower6 as usize] = bp.ElementPower6;
    props[EAttributeType::SpecialDamageChange as usize] = bp.SpecialDamageChange;
    props[EAttributeType::StrengthFastClimbCost as usize] = bp.StrengthFastClimbCost;
    props[EAttributeType::ElementPropertyType as usize] = bp.ElementPropertyType;
    props[EAttributeType::WeakTime as usize] = bp.WeakTime;
    props[EAttributeType::IgnoreDefRate as usize] = bp.IgnoreDefRate;
    props[EAttributeType::IgnoreDamageResistancePhys as usize] = bp.IgnoreDamageResistancePhys;
    props[EAttributeType::IgnoreDamageResistanceElement1 as usize] = bp.IgnoreDamageResistanceElement1;
    props[EAttributeType::IgnoreDamageResistanceElement2 as usize] = bp.IgnoreDamageResistanceElement2;
    props[EAttributeType::IgnoreDamageResistanceElement3 as usize] = bp.IgnoreDamageResistanceElement3;
    props[EAttributeType::IgnoreDamageResistanceElement4 as usize] = bp.IgnoreDamageResistanceElement4;
    props[EAttributeType::IgnoreDamageResistanceElement5 as usize] = bp.IgnoreDamageResistanceElement5;
    props[EAttributeType::IgnoreDamageResistanceElement6 as usize] = bp.IgnoreDamageResistanceElement6;
    props[EAttributeType::SkillToughRatio as usize] = bp.SkillToughRatio;
    props[EAttributeType::StrengthClimbJump as usize] = bp.StrengthClimbJump;
    props[EAttributeType::StrengthGliding as usize] = bp.StrengthGliding;
    props[EAttributeType::Mass as usize] = bp.Mass;
    props[EAttributeType::BrakingFrictionFactor as usize] = bp.BrakingFrictionFactor;
    props[EAttributeType::GravityScale as usize] = bp.GravityScale;
    props[EAttributeType::SpeedRatio as usize] = bp.SpeedRatio;
    props[EAttributeType::DamageChangePhantom as usize] = bp.DamageChangePhantom;
    props[EAttributeType::AutoAttackSpeed as usize] = bp.AutoAttackSpeed;
    props[EAttributeType::CastAttackSpeed as usize] = bp.CastAttackSpeed;
    props[EAttributeType::StatusBuildUp1Max as usize] = bp.StatusBuildUp1Max;
    props[EAttributeType::StatusBuildUp1 as usize] = bp.StatusBuildUp1;
    props[EAttributeType::StatusBuildUp2Max as usize] = bp.StatusBuildUp2Max;
    props[EAttributeType::StatusBuildUp2 as usize] = bp.StatusBuildUp2;
    props[EAttributeType::StatusBuildUp3Max as usize] = bp.StatusBuildUp3Max;
    props[EAttributeType::StatusBuildUp3 as usize] = bp.StatusBuildUp3;
    props[EAttributeType::StatusBuildUp4Max as usize] = bp.StatusBuildUp4Max;
    props[EAttributeType::StatusBuildUp4 as usize] = bp.StatusBuildUp4;
    props[EAttributeType::StatusBuildUp5Max as usize] = bp.StatusBuildUp5Max;
    props[EAttributeType::StatusBuildUp5 as usize] = bp.StatusBuildUp5;
    props[EAttributeType::ParalysisTimeMax as usize] = bp.ParalysisTimeMax;
    props[EAttributeType::ParalysisTime as usize] = bp.ParalysisTime;
    props[EAttributeType::ParalysisTimeRecover as usize] = bp.ParalysisTimeRecover;
    props[EAttributeType::ElementEnergyMax as usize] = bp.ElementEnergyMax;
    props[EAttributeType::ElementEnergy as usize] = bp.ElementEnergy;
    props[EAttributeType::ElementEfficiency as usize] = bp.ElementEfficiency;
    props[EAttributeType::WeaknessBuildUp as usize] = bp.WeaknessBuildUp;
    props[EAttributeType::WeaknessBuildUpMax as usize] = bp.WeaknessBuildUpMax;
    props[EAttributeType::WeaknessTotalBonus as usize] = bp.WeaknessTotalBonus;
    props[EAttributeType::BreakWeaknessRatio as usize] = bp.BreakWeaknessRatio;
    props[EAttributeType::WeaknessMastery as usize] = bp.WeaknessMastery;
    props
}

fn is_active_cond(buff: &Buff) -> bool {
    buff.ModifierMagnitude.is_empty()
        && buff.BuffAction.is_empty()
        && buff.OngoingTagRequirements.is_empty()
        && buff.OngoingTagIgnores.is_empty()
        && buff.RemovalTagRequirements.is_empty()
        && buff.RemovalTagIgnores.is_empty()
        && buff.ApplicationSourceTagIgnores.is_empty()
        && buff.ApplicationTagIgnores.is_empty()
        && buff.ExtraEffectRequirements.is_empty()
}

fn is_better_trial_match(
    current: Option<&TrialRoleInfo>,
    candidate: &TrialRoleInfo,
    world_level: i32,
    gender: i32,
) -> bool {
    let Some(cur) = current else {
        return true;
    };
    let cand_exact_gender = candidate.Gender == gender;
    let cur_exact_gender = cur.Gender == gender;
    if cand_exact_gender != cur_exact_gender {
        return cand_exact_gender;
    }
    let cand_le = candidate.WorldLevel <= world_level;
    let cur_le = cur.WorldLevel <= world_level;
    if cand_le != cur_le {
        return cand_le;
    }
    if cand_le {
        candidate.WorldLevel > cur.WorldLevel
    } else {
        candidate.WorldLevel < cur.WorldLevel
    }
}

const fn main_role_tag_buff_id(gender: i32) -> Option<i64> {
    match gender {
        1 => Some(291724064),
        0 => Some(291724065),
        _ => None,
    }
}

pub fn create_buff_information(
    handle_id: i32,
    buff_id: i64,
    instigator_id: i64,
    entity_id: i64,
    is_active: bool,
) -> FightBuffInformation {
    FightBuffInformation {
        HandleId: handle_id,
        BuffId: buff_id,
        Level: 1,
        StackCount: 1,
        InstigatorId: instigator_id,
        EntityId: entity_id,
        Duration: -1.0,
        LeftDuration: -1.0,
        IsActive: is_active,
        ApplyType: Some(ApplyGEType::Common),
        MessageId: -1,
        ServerId: 0,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[derive(Debug)]
    struct CloneCounter {
        id: i32,
        clones: Arc<AtomicUsize>,
    }

    impl Clone for CloneCounter {
        fn clone(&self) -> Self {
            self.clones.fetch_add(1, Ordering::SeqCst);
            Self {
                id: self.id,
                clones: Arc::clone(&self.clones),
            }
        }
    }

    #[test]
    fn table_ref_lazy_lookup_avoids_cloning_payload() {
        let clones = Arc::new(AtomicUsize::new(0));
        let shared = Arc::new(CloneCounter {
            id: 42,
            clones: Arc::clone(&clones),
        });
        let table: Table<CloneCounter, i32> = Table::empty_lazy(Some(Arc::new(move |k| {
            (*k == 42).then(|| Arc::clone(&shared))
        })));

        let r1 = table.get_data_by_id(&42).unwrap();
        let r2 = r1.clone();
        assert_eq!(r1.id, 42);
        assert_eq!(r2.id, 42);
        assert_eq!(clones.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn concerto_exceptions_cache_matches_loader() {
        assert_eq!(
            zon_config::cached_concerto_exceptions(),
            zon_config::load_concerto_exceptions().as_slice()
        );
    }
}
