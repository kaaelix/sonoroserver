use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

pub mod entity_components;

pub mod abyss_little_role;
pub mod achievement;
pub mod achievement_group;
pub mod achievement_star_level;
pub mod activity;
pub mod adventure_task;
pub mod adventure_task_chapter;
pub mod ai_base;
pub mod ai_sense_group;
pub mod ai_state_machine_config;
pub mod area;
pub mod background_card;
pub mod base_property;
pub mod blueprint_config;
pub mod buff;
pub mod calabash_develop_condition;
pub mod calabash_develop_reward;
pub mod calabash_level;
pub mod calabash_skin;
pub mod character_init_info;
pub mod character_part_config;
pub mod combo_teaching;
pub mod common_skill_preload;
pub mod condition;
pub mod condition_group;
pub mod cook_formula;
pub mod cook_level;
pub mod cook_processed;
pub mod daily_task;
pub mod daily_task_group;
pub mod damage;
pub mod data_layer_config;
pub mod drop_group;
pub mod drop_package;
pub mod dungeon_detection;
pub mod elemental_box_define;
pub mod entity_skill_preload;
pub mod explore_progress_reward;
pub mod explore_reward;
pub mod explore_score;
pub mod explore_tools;
pub mod favor_goods;
pub mod favor_story;
pub mod favor_word;
pub mod fishing_item;
pub mod fishing_reputation;
pub mod fishing_tech;
pub mod flow;
pub mod fly_skin_config;
pub mod forge_formula;
pub mod function_condition;
pub mod gacha;
pub mod gacha_pool;
pub mod gacha_texture_info;
pub mod gacha_view_info;
pub mod gameplay_tag_parent;
pub mod gameplay_tag_remap;
pub mod giant_delivery_reward;
pub mod giant_item;
pub mod gift_package;
pub mod guide_group;
pub mod hardness_mode;
pub mod honami_story_effect;
pub mod infr_road_build;
pub mod infr_v2_tree_build;
pub mod instance_dungeon;
pub mod instance_trial_role_config;
pub mod item_info;
pub mod level_entity_config;
pub mod level_play_data;
pub mod liveness;
pub mod liveness_task;
pub mod lord_gym;
pub mod lord_gym_entrance;
pub mod main_role_config;
pub mod map_block_info;
pub mod map_fog;
pub mod map_mark;
pub mod model_config_preload;
pub mod monster_detection;
pub mod monster_property_growth;
pub mod motion;
pub mod motor_decal_ip;
pub mod motor_decorations;
pub mod motor_decorations_part;
pub mod motor_frame;
pub mod motor_general_preview;
pub mod motor_linkage_ip;
pub mod motor_load_project;
pub mod motor_lvl;
pub mod motor_scene;
pub mod motor_skin;
pub mod motor_sticker;
pub mod motor_sticker_part;
pub mod motor_task;
pub mod motor_tech;
pub mod motor_tech_lv;
pub mod motor_tech_tree;
pub mod multi_map;
pub mod ornament;
pub mod passive_skill;
pub mod pay_shop;
pub mod pay_shop_recommend;
pub mod pay_shop_tab;
pub mod phantom_exp_item;
pub mod phantom_fetter;
pub mod phantom_fetter_group;
pub mod phantom_growth;
pub mod phantom_item;
pub mod phantom_level;
pub mod phantom_main_prop_item;
pub mod phantom_main_property;
pub mod phantom_quality;
pub mod phantom_rarity;
pub mod phantom_skill;
pub mod phantom_sub_property;
pub mod phantom_summon_group;
pub mod phantom_vice_polish_config;
pub mod phonograph_album;
pub mod phonograph_music;
pub mod player_exp;
pub mod player_head_re;
pub mod player_title;
pub mod quest_data;
pub mod resonant_chain;
pub mod rogue_res_character_buff;
pub mod role_be_hit_map;
pub mod role_breach;
pub mod role_exp_item;
pub mod role_info;
pub mod role_level_consume;
pub mod role_property_growth;
pub mod role_skin;
pub mod role_trial_info;
pub mod shield;
pub mod shop_fixed;
pub mod shop_info;
pub mod silent_area_detection;
pub mod skill;
pub mod skill_branch_buff;
pub mod skill_level;
pub mod skill_tree;
pub mod summon_cfg;
pub mod synthesis_formula;
pub mod synthesis_level;
pub mod teleporter;
pub mod template_config;
pub mod tower_config;
pub mod tower_difficulty;
pub mod tower_season_reward;
pub mod trail_phantom_prop;
pub mod trial_phantom_prop_item;
pub mod trial_role_info;
pub mod trial_weapon_info;
pub mod weapon_base_attributes;
pub mod weapon_breach;
pub mod weapon_conf;
pub mod weapon_exp_item;
pub mod weapon_level;
pub mod weapon_property_growth;
pub mod weapon_reson;
pub mod weapon_skin;
pub mod weather;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct EmptyObj {}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CostEntry {
    #[serde(default, deserialize_with = "de_null_default")]
    pub ItemId: i32,
    #[serde(default, deserialize_with = "de_null_default")]
    pub Count: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct IntPair {
    #[serde(default, deserialize_with = "de_null_default")]
    pub Item1: i32,
    #[serde(default, deserialize_with = "de_null_default")]
    pub Item2: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PropValue {
    #[serde(default, deserialize_with = "de_null_default")]
    pub Id: i32,
    #[serde(default, deserialize_with = "de_null_default")]
    pub Value: f64,
    #[serde(default, deserialize_with = "de_null_default")]
    pub IsRatio: bool,
}

/// Ordered integer-keyed map deserialized from JSON objects like `{"1": 2}` (or `[]`/`null`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntMap<K: Ord, V>(pub BTreeMap<K, V>);

impl<K: Ord, V> Default for IntMap<K, V> {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl<K: Ord, V> Deref for IntMap<K, V> {
    type Target = BTreeMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, K, V> Deserialize<'de> for IntMap<K, V>
where
    K: Ord + FromStr,
    K::Err: Display,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntMapVisitor<K, V>(std::marker::PhantomData<(K, V)>);

        impl<'de, K, V> Visitor<'de> for IntMapVisitor<K, V>
        where
            K: Ord + FromStr,
            K::Err: Display,
            V: Deserialize<'de>,
        {
            type Value = IntMap<K, V>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a map with integer string keys, null, or an empty array")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(IntMap(BTreeMap::new()))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(IntMap(BTreeMap::new()))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                while seq.next_element::<serde::de::IgnoredAny>()?.is_some() {}
                Ok(IntMap(BTreeMap::new()))
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut map = BTreeMap::new();
                while let Some((k_str, v)) = access.next_entry::<String, V>()? {
                    let key = k_str
                        .parse::<K>()
                        .map_err(|e| serde::de::Error::custom(format!("invalid IntMap key '{k_str}': {e}")))?;
                    map.insert(key, v);
                }
                Ok(IntMap(map))
            }
        }

        deserializer.deserialize_any(IntMapVisitor(std::marker::PhantomData))
    }
}

pub fn de_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

pub struct DefaultBool<const V: bool>;

impl<const V: bool> DefaultBool<V> {
    pub const fn get() -> bool {
        V
    }

    pub fn de<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<bool>::deserialize(deserializer)?.unwrap_or(V))
    }
}

pub struct DefaultI32<const V: i32>;

impl<const V: i32> DefaultI32<V> {
    pub const fn get() -> i32 {
        V
    }

    pub fn de<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<i32>::deserialize(deserializer)?.unwrap_or(V))
    }
}

/// Used when JSON explicit `null` means `Some(false)` (override template) while omission keeps `None`.
pub fn de_present_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(Option::<bool>::deserialize(deserializer)?.unwrap_or(false)))
}

/// Used when JSON explicit `null` means `Some(0)` (override template) while omission keeps `None`.
pub fn de_present_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(Option::<i32>::deserialize(deserializer)?.unwrap_or(0)))
}

/// Used when JSON explicit `null` means `Some(vec![])` (override template) while omission keeps `None`.
pub fn de_present_vec_i64<'de, D>(deserializer: D) -> Result<Option<Vec<i64>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(Option::<Vec<i64>>::deserialize(deserializer)?.unwrap_or_default()))
}

pub fn de_opt_string_lenient<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let val = serde_json::Value::deserialize(deserializer)?;
    Ok(match val {
        serde_json::Value::String(s) => Some(s),
        _ => None,
    })
}

pub trait FromJsonScalar: Default + Sized {
    fn from_json_value(v: &serde_json::Value) -> Result<Self, String>;
}

macro_rules! impl_from_json_int {
    ($ty:ty) => {
        impl FromJsonScalar for $ty {
            #[allow(unreachable_code)]
            fn from_json_value(v: &serde_json::Value) -> Result<Self, String> {
                match v {
                    serde_json::Value::Null => Ok(0),
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            Self::try_from(i).map_err(|e| e.to_string())
                        } else if let Some(u) = n.as_u64() {
                            Self::try_from(u).map_err(|e| e.to_string())
                        } else if let Some(f) = n.as_f64() {
                            Ok(f as Self)
                        } else {
                            Err(format!("invalid number {n}"))
                        }
                    }
                    serde_json::Value::String(s) => {
                        if s.is_empty() {
                            Ok(0)
                        } else {
                            s.trim().parse::<Self>().map_err(|e| format!("invalid '{}': {e}", s))
                        }
                    }
                    other => Err(format!("expected number or string, found {other}")),
                }
            }
        }
    };
}

impl_from_json_int!(i32);
impl_from_json_int!(i64);
impl_from_json_int!(u32);

impl FromJsonScalar for f32 {
    fn from_json_value(v: &serde_json::Value) -> Result<Self, String> {
        match v {
            serde_json::Value::Null => Ok(0.0),
            serde_json::Value::Number(n) => n
                .as_f64()
                .map(|f| f as f32)
                .ok_or_else(|| format!("invalid f32 {n}")),
            serde_json::Value::String(s) if s.is_empty() => Ok(0.0),
            serde_json::Value::String(s) => s.trim().parse::<f32>().map_err(|e| e.to_string()),
            other => Err(format!("expected float or string, found {other}")),
        }
    }
}

pub fn de_lenient_scalar_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromJsonScalar,
{
    let val = Option::<serde_json::Value>::deserialize(deserializer)?;
    match val {
        None | Some(serde_json::Value::Null) => Ok(T::default()),
        Some(v) => T::from_json_value(&v).map_err(serde::de::Error::custom),
    }
}

pub fn de_lenient_vec_default<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromJsonScalar,
{
    let val = Option::<serde_json::Value>::deserialize(deserializer)?;
    let Some(serde_json::Value::Array(arr)) = val else {
        return Ok(Vec::new());
    };
    arr.iter()
        .map(|el| T::from_json_value(el).map_err(serde::de::Error::custom))
        .collect()
}

pub fn de_lenient_opt_vec<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: FromJsonScalar,
{
    let val = Option::<serde_json::Value>::deserialize(deserializer)?;
    match val {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .map(|el| T::from_json_value(el).map_err(serde::de::Error::custom))
            .collect::<Result<Vec<T>, _>>()
            .map(Some),
        Some(other) => Err(serde::de::Error::custom(format!(
            "expected array or null, found {other}"
        ))),
    }
}

pub fn de_lenient_intmap_vec<'de, D, K, V>(deserializer: D) -> Result<IntMap<K, Vec<V>>, D::Error>
where
    D: Deserializer<'de>,
    K: Ord + FromStr,
    K::Err: Display,
    V: FromJsonScalar,
{
    let val = Option::<serde_json::Value>::deserialize(deserializer)?;
    let Some(serde_json::Value::Object(obj)) = val else {
        return Ok(IntMap::default());
    };
    let mut map = BTreeMap::new();
    for (k_str, v_val) in obj {
        let key = k_str
            .parse::<K>()
            .map_err(|e| serde::de::Error::custom(format!("invalid IntMap key '{k_str}': {e}")))?;
        let vec = match v_val {
            serde_json::Value::Array(arr) => arr
                .iter()
                .map(|el| V::from_json_value(el).map_err(serde::de::Error::custom))
                .collect::<Result<Vec<V>, _>>()?,
            serde_json::Value::Null => Vec::new(),
            other => {
                return Err(serde::de::Error::custom(format!(
                    "expected array in IntMap, found {other}"
                )));
            }
        };
        map.insert(key, vec);
    }
    Ok(IntMap(map))
}
