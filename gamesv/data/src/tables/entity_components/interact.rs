use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ActionParams {
    #[serde(default, deserialize_with = "super::super::de_null_default")]
    pub EntityState: String,
    #[serde(default, deserialize_with = "super::super::de_null_default")]
    pub EntityIds: Vec<i64>,
    #[serde(default)]
    pub IsBlocked: Option<bool>,
    #[serde(default, deserialize_with = "super::super::de_opt_string_lenient")]
    pub Type: Option<String>,
    #[serde(default)]
    pub State: Option<String>,
    #[serde(default)]
    pub Flow: Option<FlowConfig>,
    #[serde(default)]
    pub Flows: Option<FlowConfig>,
    #[serde(default)]
    pub Option: Option<serde_json::Value>,
    #[serde(default)]
    pub QuestId: Option<i32>,
    #[serde(default)]
    pub VarName: Option<String>,
    #[serde(default)]
    pub Value: Option<serde_json::Value>,
    #[serde(default)]
    pub LevelPlayId: Option<i32>,
    #[serde(default)]
    pub TargetEntityId: Option<i64>,
    #[serde(default)]
    pub EntityId: Option<i64>,
    #[serde(default)]
    pub FlowListName: Option<String>,
    #[serde(default)]
    pub FlowId: Option<i32>,
    #[serde(default)]
    pub StateId: Option<i32>,
    #[serde(default)]
    pub DungeonId: Option<i32>,
    #[serde(default)]
    pub LocationEntityId: Option<i64>,
    #[serde(default)]
    pub WaitTime: Option<f64>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct FlowConfig {
    #[serde(default)]
    pub FlowId: Option<i32>,
    #[serde(default)]
    pub FlowListName: Option<String>,
    #[serde(default)]
    pub StateId: Option<i32>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ActionHelper {
    #[serde(default, deserialize_with = "super::super::de_null_default")]
    Name: String,
    #[serde(default)]
    Params: Option<ActionParams>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct Action {
    pub Name: String,
    pub Params: Option<ActionParams>,
    pub Raw: Option<serde_json::Value>,
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = serde_json::Value::deserialize(deserializer)?;
        let parsed: ActionHelper =
            serde_json::from_value(val.clone()).map_err(serde::de::Error::custom)?;
        Ok(Self {
            Name: parsed.Name,
            Params: parsed.Params,
            Raw: Some(val),
        })
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InteractType {
    #[serde(default, deserialize_with = "super::super::de_null_default")]
    pub Type: String,
    #[serde(default)]
    pub Flow: Option<FlowConfig>,
    #[serde(default)]
    pub CustomOptionType: Option<i32>,
    #[serde(default, deserialize_with = "super::super::de_null_default")]
    pub Actions: Vec<Action>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InteractOption {
    #[serde(default)]
    pub Type: Option<InteractType>,
    #[serde(default)]
    pub Condition: Option<serde_json::Value>,
    #[serde(default)]
    pub LockCondition: Option<serde_json::Value>,
    #[serde(default)]
    pub DoIntactType: Option<String>,
    #[serde(default)]
    pub Context: Option<serde_json::Value>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RandomInteractOption {
    #[serde(default, deserialize_with = "super::super::de_null_default")]
    pub Weight: i32,
    #[serde(default)]
    pub Option: Option<InteractOption>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RandomInteractConfig {
    #[serde(default, deserialize_with = "super::super::de_null_default")]
    pub RandomCount: i32,
    #[serde(default, deserialize_with = "super::super::de_null_default")]
    pub Options: Vec<RandomInteractOption>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InteractComponent {
    #[serde(default, deserialize_with = "super::super::de_present_bool")]
    pub Disabled: Option<bool>,
    #[serde(default)]
    pub DoIntactType: Option<String>,
    #[serde(default)]
    pub InteractPoint: Option<serde_json::Value>,
    #[serde(default)]
    pub InteractDefaultCount: Option<i32>,
    #[serde(default)]
    pub InteractIcon: Option<String>,
    #[serde(default)]
    pub TidContent: Option<String>,
    #[serde(default, deserialize_with = "super::super::de_null_default")]
    pub Options: Vec<InteractOption>,
    #[serde(default)]
    pub RandomInteract: Option<RandomInteractConfig>,
    #[serde(default)]
    pub Range: Option<i32>,
    #[serde(default)]
    pub InteractiveRange: Option<i32>,
    #[serde(default)]
    pub InteractiveRangeAngle: Option<i32>,
    #[serde(default)]
    pub MatchRoleOption: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub InteractAdditionalRange: Option<i32>,
    #[serde(default)]
    pub InterruptActions: Option<Vec<Action>>,
    #[serde(default)]
    pub InterruptBehavior: Option<Vec<Action>>,
    #[serde(default)]
    pub AutoTriggerOptions: Option<Vec<i32>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_params_accepts_non_string_type_and_defaults_to_none() {
        let p: ActionParams =
            serde_json::from_str(r#"{"EntityId":101,"Type":{"Kind":"Complex"}}"#).unwrap();
        assert_eq!(p.EntityId, Some(101));
        assert_eq!(p.Type, None);

        let s: ActionParams = serde_json::from_str(r#"{"Type":"Direct"}"#).unwrap();
        assert_eq!(s.Type.as_deref(), Some("Direct"));
    }
}
