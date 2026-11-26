#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct HardnessMode {
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub Id: i32,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub ReductionData: Vec<f64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub AttackTypeData: Vec<f64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub CorrectData: Vec<f64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub WeaponReduction: Vec<f64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub SkillTypeParam: Vec<f64>,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub Percent: i32,
}
