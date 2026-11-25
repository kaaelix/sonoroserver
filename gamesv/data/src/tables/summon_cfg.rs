use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SummonCfg {
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub Id: i32,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub BlueprintType: String,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub BornBuffId: Vec<i64>,
}

impl SummonCfg {
    pub fn get_concom_count(role_id: i32) -> Option<usize> {
        match role_id {
            1105 => Some(3),
            1301 => Some(4),
            _ => None,
        }
    }
}
