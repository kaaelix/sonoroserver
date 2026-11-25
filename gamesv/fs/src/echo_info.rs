use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::phantom_item::PhantomItem as PhantomItemConfig;
use sonoro_gamesv_data::tables::phantom_main_property::PhantomMainProperty;
use sonoro_gamesv_data::tables::phantom_main_prop_item::PhantomMainPropItem;
use sonoro_gamesv_data::tables::phantom_sub_property::PhantomSubProperty;
use sonoro_proto::pb::{
    PhantomItem as PbPhantomItem, PhantomPropInfo, RefreshVisionEquipGroupData,
    RolePhantomEquipInfo,
};

const FLAT_ATK_SUB_PROP_WEIGHTS: &[i32] = &[70, 540, 390, 30];
const FLAT_DEF_SUB_PROP_WEIGHTS: &[i32] = &[150, 460, 330, 90];
const CRITICAL_SUB_PROP_WEIGHTS: &[i32] = &[70, 70, 70, 24, 24, 24, 9, 9];
const SHARED_SUB_PROP_WEIGHTS: &[i32] = &[70, 80, 210, 250, 180, 150, 60, 30];
const FLAT_ATK_DEF_MULTIPLIERS: &[i32] = &[813, 1083, 1353, 1623];
const CRITICAL_MULTIPLIERS: &[i32] = &[1263, 1383, 1503, 1623, 1743, 1863, 1983, 2103];
const NORMAL_SUB_PROP_MULTIPLIERS: &[i32] = &[853, 953, 1053, 1153, 1253, 1353, 1453, 1553];

pub struct ZigDefaultPrng {
    s: [u64; 4],
}

impl ZigDefaultPrng {
    pub fn init(seed: u64) -> Self {
        let mut sm = seed;
        let mut next_sm = || {
            sm = sm.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = sm;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^ (z >> 31)
        };
        Self {
            s: [next_sm(), next_sm(), next_sm(), next_sm()],
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let r = self.s[0]
            .wrapping_add(self.s[3])
            .rotate_left(23)
            .wrapping_add(self.s[0]);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        r
    }

    pub fn less_than_u32(&mut self, bound: u32) -> u32 {
        if bound <= 1 {
            return 0;
        }
        let mut x = self.next_u64() as u32;
        let mut m = u64::from(x) * u64::from(bound);
        let mut l = m as u32;
        if l < bound {
            let mut t = bound.wrapping_neg();
            if t >= bound {
                t -= bound;
                if t >= bound {
                    t %= bound;
                }
            }
            while l < t {
                x = self.next_u64() as u32;
                m = u64::from(x) * u64::from(bound);
                l = m as u32;
            }
        }
        (m >> 32) as u32
    }

    pub fn less_than_usize(&mut self, bound: usize) -> usize {
        if bound <= 1 {
            return 0;
        }
        let bound_u64 = bound as u64;
        let mut x = self.next_u64();
        let mut m = u128::from(x) * u128::from(bound_u64);
        let mut l = m as u64;
        if l < bound_u64 {
            let mut t = bound_u64.wrapping_neg();
            if t >= bound_u64 {
                t -= bound_u64;
                if t >= bound_u64 {
                    t %= bound_u64;
                }
            }
            while l < t {
                x = self.next_u64();
                m = u128::from(x) * u128::from(bound_u64);
                l = m as u64;
            }
        }
        (m >> 64) as usize
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Prop {
    pub id: i32,
    pub value: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RoleEquip {
    pub role_id: i32,
    pub slots: [i32; 5],
}

impl RoleEquip {
    pub fn to_proto(&self) -> RolePhantomEquipInfo {
        RolePhantomEquipInfo {
            RoleId: self.role_id,
            PhantomItemIncrId: self.slots.to_vec(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct VisionGroup {
    pub incr_id: i32,
    pub name: String,
    pub slots: [i32; 5],
}

impl VisionGroup {
    pub fn to_proto(&self) -> RefreshVisionEquipGroupData {
        RefreshVisionEquipGroupData {
            IncId: self.slots.to_vec(),
            Name: self.name.as_bytes().to_vec(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PresetInfo {
    pub groups: Vec<VisionGroup>,
}

pub fn next_after_preset_groups(groups: &[VisionGroup]) -> i32 {
    groups.iter().fold(1i32, |acc, g| acc.max(g.incr_id + 1))
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct DiscoveredMonster {
    pub monster_id: i32,
    pub max_quality: i32,
    pub rewarded_quality: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ProjectorSkin {
    pub monster_id: i32,
    pub skin_id: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct CalabashInfo {
    pub rewarded_levels: Vec<i32>,
    pub projector_monster_ids: Vec<i32>,
    pub projector_skins: Vec<ProjectorSkin>,
    pub discovered_monsters: Vec<DiscoveredMonster>,
    pub direct_refine_week_times: i32,
}

impl CalabashInfo {
    pub fn highest_quality_for_monster(&self, monster_id: i32) -> i32 {
        self.discovered_monsters
            .iter()
            .find(|e| e.monster_id == monster_id)
            .map_or(0, |e| e.max_quality)
    }

    pub fn rewarded_quality_for_monster(&self, monster_id: i32) -> i32 {
        self.discovered_monsters
            .iter()
            .find(|e| e.monster_id == monster_id)
            .map_or(0, |e| e.rewarded_quality)
    }

    pub fn upsert_discovery(
        &mut self,
        monster_id: i32,
        quality: i32,
        mark_rewarded: bool,
    ) -> bool {
        if monster_id <= 0 || quality <= 0 {
            return false;
        }
        if let Some(entry) = self
            .discovered_monsters
            .iter_mut()
            .find(|e| e.monster_id == monster_id)
        {
            let mut changed = false;
            if quality > entry.max_quality {
                entry.max_quality = quality;
                changed = true;
            }
            if mark_rewarded && entry.rewarded_quality < entry.max_quality {
                entry.rewarded_quality = entry.max_quality;
                changed = true;
            }
            return changed;
        }
        self.discovered_monsters.push(DiscoveredMonster {
            monster_id,
            max_quality: quality,
            rewarded_quality: if mark_rewarded { quality } else { 0 },
        });
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct EchoInfo {
    pub id: i32,
    pub stack_count: i32,
    pub func_value: i32,
    pub level: i32,
    pub exp: i32,
    pub main_prop: Vec<Prop>,
    pub sub_prop: Vec<Prop>,
    pub fetter_group_id: i32,
    pub skin_id: i32,
    pub role_id: Option<i32>,
    pub equipped_pos: i32,
    pub unack_sub_prop: Vec<Prop>,
    pub lock_prop_index: Vec<i32>,
}

impl Default for EchoInfo {
    fn default() -> Self {
        Self {
            id: 0,
            stack_count: 1,
            func_value: 0,
            level: 0,
            exp: 0,
            main_prop: Vec::new(),
            sub_prop: Vec::new(),
            fetter_group_id: 0,
            skin_id: 0,
            role_id: None,
            equipped_pos: -1,
            unack_sub_prop: Vec::new(),
            lock_prop_index: Vec::new(),
        }
    }
}

impl EchoInfo {
    pub const DATA_DIR: &'static str = "echo";
    pub const PRESET_DATA_PATH: &'static str = "echo/presets";
    pub const CALABASH_DATA_PATH: &'static str = "echo/calabash";

    pub fn to_proto(&self, incr_id: i32) -> PbPhantomItem {
        PbPhantomItem {
            Id: self.id,
            IncrId: incr_id,
            StackCount: self.stack_count.max(1),
            FuncValue: self.func_value,
            PhantomLevel: self.level,
            PhantomExp: self.exp,
            PhantomMainProp: self
                .main_prop
                .iter()
                .map(|p| PhantomPropInfo {
                    PhantomPropId: p.id,
                    Value: p.value,
                })
                .collect(),
            PhantomSubProp: self
                .sub_prop
                .iter()
                .map(|p| PhantomPropInfo {
                    PhantomPropId: p.id,
                    Value: p.value,
                })
                .collect(),
            FetterGroupId: self.fetter_group_id,
            SkinId: self.skin_id,
            UnAckSubProp: self
                .unack_sub_prop
                .iter()
                .map(|p| PhantomPropInfo {
                    PhantomPropId: p.id,
                    Value: p.value,
                })
                .collect(),
            LockPropIndex: self.lock_prop_index.clone(),
            ..Default::default()
        }
    }

    pub fn add_defaults(
        assets: &Assets,
        echo_map: &mut BTreeMap<i32, EchoInfo>,
        start_incr_id: i32,
    ) {
        let mut next_incr_id = start_incr_id;
        for item in &assets.tables.phantom_item.items {
            if !valid_default_owned_item(assets, item) {
                continue;
            }
            for &fetter_group_id in &item.FetterGroup {
                echo_map.insert(next_incr_id, build_default_echo(assets, item, fetter_group_id));
                next_incr_id += 1;
            }
        }
    }

    pub fn rebuild_main_props(&mut self, assets: &Assets, rand_group_id: i32) {
        self.main_prop = build_main_props(assets, rand_group_id, self.level);
    }

    pub fn refresh_main_prop_values(&mut self, assets: &Assets) {
        for prop in &mut self.main_prop {
            if let Some(cfg) = assets.tables.phantom_main_prop_item.get_data_by_id(&prop.id) {
                prop.value = main_prop_value(assets, &cfg, self.level);
            }
        }
    }

    pub fn append_sub_props(
        &mut self,
        assets: &Assets,
        quality: i32,
        max_count: usize,
        count: usize,
        random_seed: u64,
    ) -> usize {
        if self.sub_prop.len() >= max_count
            || count == 0
            || assets.tables.phantom_sub_property.items.is_empty()
        {
            return 0;
        }
        let mut rng = ZigDefaultPrng::init(random_seed);
        let initial_len = self.sub_prop.len();
        while self.sub_prop.len() < max_count && self.sub_prop.len() < initial_len + count {
            let Some(prop) = pick_random_sub_property(assets, &self.sub_prop, &mut rng) else {
                break;
            };
            let value = random_sub_prop_value(quality, &prop, &mut rng);
            self.sub_prop.push(Prop { id: prop.Id, value });
        }
        self.sub_prop.len() - initial_len
    }
}

pub fn valid_default_owned_item(assets: &Assets, item: &PhantomItemConfig) -> bool {
    if item.PhantomType != 1 || item.ParentMonsterId != 0 {
        return false;
    }
    if !item.ShowInBag || !item.Destructible || item.FetterGroup.is_empty() {
        return false;
    }
    if assets.tables.phantom_rarity.get_data_by_id(&item.Rarity).is_none() {
        return false;
    }
    if assets.tables.phantom_quality.get_data_by_id(&item.QualityId).is_none() {
        return false;
    }
    find_main_property(assets, item.MainProp.RandGroupId).is_some()
}

pub fn build_default_echo(
    assets: &Assets,
    item: &PhantomItemConfig,
    fetter_group_id: i32,
) -> EchoInfo {
    EchoInfo {
        id: item.ItemId,
        main_prop: build_main_props(assets, item.MainProp.RandGroupId, 0),
        fetter_group_id,
        ..Default::default()
    }
}

fn build_main_props(assets: &Assets, rand_group_id: i32, level: i32) -> Vec<Prop> {
    let Some(main_property) = find_main_property(assets, rand_group_id) else {
        return Vec::new();
    };
    build_main_props_from_group(assets, &main_property.PropGroup, level)
}

pub fn build_main_props_from_group(assets: &Assets, prop_group: &[i32], level: i32) -> Vec<Prop> {
    prop_group
        .iter()
        .filter_map(|&pid| {
            let cfg = assets.tables.phantom_main_prop_item.get_data_by_id(&pid)?;
            Some(Prop {
                id: cfg.Id,
                value: main_prop_value(assets, &cfg, level),
            })
        })
        .collect()
}

pub fn max_unlocked_sub_prop_count(assets: &Assets, quality: i32) -> Option<usize> {
    assets
        .tables
        .phantom_quality
        .get_data_by_id(&quality)
        .map(|c| c.SlotUnlockLevel.len())
}

pub fn unlocked_sub_prop_count(assets: &Assets, quality: i32, level: i32) -> Option<usize> {
    let config = assets.tables.phantom_quality.get_data_by_id(&quality)?;
    Some(
        config
            .SlotUnlockLevel
            .iter()
            .filter(|&&unlock| level >= unlock)
            .count(),
    )
}

pub fn contains_sub_prop_role(
    assets: &Assets,
    props: &[Prop],
    candidate: &PhantomSubProperty,
) -> bool {
    props.iter().any(|prop| {
        assets
            .tables
            .phantom_sub_property
            .get_data_by_id(&prop.id)
            .is_some_and(|existing| same_sub_prop_type(&existing, candidate))
    })
}

pub fn same_sub_prop_type(a: &PhantomSubProperty, b: &PhantomSubProperty) -> bool {
    if is_flat_percent_sub_prop(a) && is_flat_percent_sub_prop(b) {
        return a.PropId == b.PropId && a.AddType == b.AddType;
    }
    a.PropId == b.PropId
}

const fn is_flat_percent_sub_prop(prop: &PhantomSubProperty) -> bool {
    prop.PropId == 10002 || prop.PropId == 10007 || prop.PropId == 10010
}

pub fn pick_random_sub_property(
    assets: &Assets,
    props: &[Prop],
    rng: &mut ZigDefaultPrng,
) -> Option<PhantomSubProperty> {
    let items = &assets.tables.phantom_sub_property.items;
    let available: Vec<&PhantomSubProperty> = items
        .iter()
        .enumerate()
        .filter(|&(index, prop)| {
            !contains_sub_prop_role(assets, props, prop)
                && !items[..index].iter().any(|p| same_sub_prop_type(p, prop))
        })
        .map(|(_, prop)| prop)
        .collect();
    if available.is_empty() {
        return None;
    }
    let chosen = rng.less_than_usize(available.len());
    Some(available[chosen].clone())
}

pub fn random_sub_prop_value(quality: i32, prop: &PhantomSubProperty, rng: &mut ZigDefaultPrng) -> i32 {
    let (weights, multipliers) = sub_prop_roll_profile(prop);
    let max_index = max_sub_prop_roll_index(quality, multipliers.len());
    let multiplier = multipliers[random_weighted_index(rng, weights, max_index)];
    scaled_sub_prop_value(prop.SubStandardProperty, multiplier)
}

pub fn max_sub_prop_tier(quality: i32, prop: &PhantomSubProperty) -> usize {
    let (_, multipliers) = sub_prop_roll_profile(prop);
    max_sub_prop_roll_index(quality, multipliers.len()) + 1
}

pub fn sub_prop_value_for_tier(
    quality: i32,
    prop: &PhantomSubProperty,
    tier: i32,
) -> Option<i32> {
    if tier <= 0 {
        return None;
    }
    let (_, multipliers) = sub_prop_roll_profile(prop);
    let tier_index = (tier - 1) as usize;
    if tier_index >= max_sub_prop_tier(quality, prop) {
        return None;
    }
    Some(scaled_sub_prop_value(
        prop.SubStandardProperty,
        multipliers[tier_index],
    ))
}

fn sub_prop_roll_profile(prop: &PhantomSubProperty) -> (&'static [i32], &'static [i32]) {
    if prop.PropId == 10007 && prop.AddType == 1 {
        (FLAT_ATK_SUB_PROP_WEIGHTS, FLAT_ATK_DEF_MULTIPLIERS)
    } else if prop.PropId == 10010 && prop.AddType == 1 {
        (FLAT_DEF_SUB_PROP_WEIGHTS, FLAT_ATK_DEF_MULTIPLIERS)
    } else if prop.PropId == 8 || prop.PropId == 9 {
        (CRITICAL_SUB_PROP_WEIGHTS, CRITICAL_MULTIPLIERS)
    } else {
        (SHARED_SUB_PROP_WEIGHTS, NORMAL_SUB_PROP_MULTIPLIERS)
    }
}

const fn max_sub_prop_roll_index(quality: i32, multiplier_count: usize) -> usize {
    if multiplier_count == FLAT_ATK_DEF_MULTIPLIERS.len() {
        match quality {
            3 => 1,
            4 => 2,
            _ => 3,
        }
    } else {
        match quality {
            3 => 3,
            4 => 5,
            _ => 7,
        }
    }
}

fn random_weighted_index(rng: &mut ZigDefaultPrng, weights: &[i32], max_index: usize) -> usize {
    let capped = max_index.min(weights.len() - 1);
    let total: u32 = weights[..=capped].iter().map(|&w| w as u32).sum();
    let mut roll = rng.less_than_u32(total);
    for (index, &w) in weights[..=capped].iter().enumerate() {
        let wv = w as u32;
        if roll < wv {
            return index;
        }
        roll -= wv;
    }
    capped
}

const fn scaled_sub_prop_value(value: i32, multiplier: i32) -> i32 {
    ((value * multiplier + 5000) / 10000) * 10
}

pub fn find_main_property(assets: &Assets, rand_group_id: i32) -> Option<&PhantomMainProperty> {
    assets
        .tables
        .phantom_main_property
        .items
        .iter()
        .find(|e| e.RandGroupId == rand_group_id)
}

pub fn main_prop_value(assets: &Assets, prop: &PhantomMainPropItem, level: i32) -> i32 {
    let growth = assets
        .tables
        .phantom_growth
        .items
        .iter()
        .find(|e| e.GrowthId == prop.GrowthId && e.Level == level)
        .map_or(10000, |e| e.Value);
    (prop.StandardProperty * growth) / 10000
}
