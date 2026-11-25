use serde::{Deserialize, Deserializer};
use sonoro_proto::enums::EAttributeType;
use sonoro_proto::wire::PbEnum;

#[derive(Debug, Clone, PartialEq)]
pub struct BuffAdditionEntry {
    pub id: i64,
    pub stack_count: i32,
    pub is_active: bool,
    pub duration_seconds: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum DurationPolicy {
    #[default]
    Instant = 0,
    Infinite = 1,
    HasDuration = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum FormationPolicy {
    #[default]
    None = 0,
    SharedByFormation = 1,
    InheritedWhenChangeRole = 2,
    InheritedWhenChangeRoleAndRemoveSelf = 3,
    SharedToSummoner = 4,
    FormationBuff = 5,
    Unknown = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PeriodicInhibitionPolicy {
    #[default]
    None = 0,
    Reset = 1,
    ResetAndExecute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum StackingType {
    #[default]
    None = 0,
    ByInstigator = 1,
    ByTarget = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum StackDurationRefreshPolicy {
    #[default]
    Refresh = 0,
    NoRefresh = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum StackPeriodResetPolicy {
    #[default]
    Refresh = 0,
    NoRefresh = 1,
}

macro_rules! impl_int_enum_deserialize {
    ($ty:ty, { $($num:literal => $variant:ident),+ $(,)? }) => {
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let raw = Option::<i64>::deserialize(deserializer)?.unwrap_or(0);
                match raw {
                    $($num => Ok(Self::$variant),)+
                    other => Err(serde::de::Error::custom(format!(
                        "invalid {} integer tag: {other}",
                        stringify!($ty)
                    ))),
                }
            }
        }
    };
}

impl_int_enum_deserialize!(DurationPolicy, {
    0 => Instant,
    1 => Infinite,
    2 => HasDuration,
});

impl_int_enum_deserialize!(FormationPolicy, {
    0 => None,
    1 => SharedByFormation,
    2 => InheritedWhenChangeRole,
    3 => InheritedWhenChangeRoleAndRemoveSelf,
    4 => SharedToSummoner,
    5 => FormationBuff,
    6 => Unknown,
});

impl_int_enum_deserialize!(PeriodicInhibitionPolicy, {
    0 => None,
    1 => Reset,
    2 => ResetAndExecute,
});

impl_int_enum_deserialize!(StackingType, {
    0 => None,
    1 => ByInstigator,
    2 => ByTarget,
});

impl_int_enum_deserialize!(StackDurationRefreshPolicy, {
    0 => Refresh,
    1 => NoRefresh,
});

impl_int_enum_deserialize!(StackPeriodResetPolicy, {
    0 => Refresh,
    1 => NoRefresh,
});

fn de_e_attribute_type<'de, D>(deserializer: D) -> Result<EAttributeType, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<i32>::deserialize(deserializer)?.unwrap_or(0);
    Ok(EAttributeType::from_i32_or_default(raw))
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Buff {
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub Id: i64,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub GeDesc: String,
    #[serde(default)]
    pub DurationPolicy: DurationPolicy,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub DurationCalculationPolicy: Vec<i32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub DurationMagnitude: Vec<f32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub DurationMagnitude2: Vec<f32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub bDurationAffectedByBulletTime: bool,
    #[serde(default)]
    pub FormationPolicy: FormationPolicy,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub Probability: u32,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub Period: f32,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub bExecutePeriodicEffectOnApplication: bool,
    #[serde(default)]
    pub PeriodicInhibitionPolicy: PeriodicInhibitionPolicy,
    #[serde(default, deserialize_with = "de_e_attribute_type")]
    pub GameAttributeID: EAttributeType,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub CalculationPolicy: Vec<i32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ModifierMagnitude: Vec<i32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ModifierMagnitude2: Vec<i32>,
    #[serde(default)]
    pub StackingType: StackingType,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub DefaultStackCount: u8,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub StackAppendCount: u8,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub StackLimitCount: u32,
    #[serde(default)]
    pub StackDurationRefreshPolicy: StackDurationRefreshPolicy,
    #[serde(default)]
    pub StackPeriodResetPolicy: StackPeriodResetPolicy,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub StackExpirationRemoveNumber: u8,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub bDenyOverflowApplication: bool,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub bClearStackOnOverflow: bool,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub bRequireModifierSuccessToTriggerCues: bool,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub bSuppressStackingCues: bool,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub GameplayCueIds: Vec<i64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub GrantedTags: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ApplicationSourceTagRequirements: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ApplicationSourceTagIgnores: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ApplicationTagRequirements: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ApplicationTagIgnores: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub OngoingTagRequirements: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub OngoingTagIgnores: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub RemovalTagRequirements: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub RemovalTagIgnores: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub RemoveBuffWithTags: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub GrantedApplicationImmunityTags: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub GrantedApplicationImmunityTagIgnores: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub PrematureExpirationEffects: Vec<i64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub RoutineExpirationEffects: Vec<i64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub OverflowEffects: Vec<i64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub RelatedExtraEffectBuffId: Vec<i64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectID: u32,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectParameters: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectParametersGrow1: Vec<i32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectParametersGrow2: Vec<i32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectRequirements: Vec<u32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectReqPara: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectReqSetting: u8,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectCD: Vec<f32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectCdForTarget: Vec<f32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectRemoveStackNum: u32,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ExtraEffectProbability: Vec<u32>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub BuffAction: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub TagLogic: Vec<String>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub DeadRemove: bool,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub bOnlyLocalAdd: bool,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ReplaceBuffIds: Vec<i64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ReplaceConditions: Vec<String>,
}

pub fn buff_list_from_ids(ids: &[i64]) -> Vec<BuffAdditionEntry> {
    ids.iter()
        .map(|&id| BuffAdditionEntry {
            id,
            stack_count: 0,
            is_active: true,
            duration_seconds: None,
        })
        .collect()
}
