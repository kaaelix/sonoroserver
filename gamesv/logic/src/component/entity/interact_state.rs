use sonoro_gamesv_data::tables::entity_components::interact::InteractComponent as InteractConfig;
use sonoro_proto::pb::InteractComponentPb;

#[derive(Debug, Clone, Default)]
pub struct InteractStateComponent {
    pub random_option_indices: Vec<i32>,
}

impl InteractStateComponent {
    pub const TRANSIENT: bool = true;

    pub fn from_config(
        config: &InteractConfig,
        player_id: i32,
        scene_id: i32,
        config_id: i32,
    ) -> Self {
        if config.Disabled.unwrap_or(false) {
            return Self::default();
        }
        let Some(random) = &config.RandomInteract else {
            return Self::default();
        };
        if random.RandomCount <= 0 || random.Options.is_empty() {
            return Self::default();
        }

        let mut selected_flags = vec![false; random.Options.len()];
        let mut selected = Vec::new();

        let mut random_state: u64 = (i64::from(player_id) as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ (i64::from(scene_id) as u64).wrapping_mul(0xD1B5_4A32_D192_ED03)
            ^ (i64::from(config_id) as u64).wrapping_mul(0x94D0_49BB_1331_11EB);
        if random_state == 0 {
            random_state = 0xA076_1D64_78BD_642F;
        }

        let draw_count = (random.RandomCount as usize).min(random.Options.len());
        for _ in 0..draw_count {
            let mut total_weight = 0u64;
            for (option, &already_selected) in random.Options.iter().zip(selected_flags.iter()) {
                if already_selected || option.Weight <= 0 || !supported_flow_option(option) {
                    continue;
                }
                total_weight = total_weight.saturating_add(option.Weight as u64);
            }
            if total_weight == 0 {
                break;
            }

            random_state ^= random_state >> 12;
            random_state ^= random_state << 25;
            random_state ^= random_state >> 27;
            let mut remaining_roll = random_state.wrapping_mul(0x2545_F491_4F6C_DD1D) % total_weight;
            let mut selected_index = None;
            for (index, (option, &already_selected)) in
                random.Options.iter().zip(selected_flags.iter()).enumerate()
            {
                if already_selected || option.Weight <= 0 || !supported_flow_option(option) {
                    continue;
                }
                let weight = option.Weight as u64;
                if remaining_roll < weight {
                    selected_index = Some(index);
                    break;
                }
                remaining_roll -= weight;
            }
            let Some(index) = selected_index else {
                break;
            };
            selected_flags[index] = true;
            selected.push(index as i32);
        }

        Self {
            random_option_indices: selected,
        }
    }

    pub fn to_proto(&self) -> InteractComponentPb {
        InteractComponentPb {
            RandomInteractIndex: self.random_option_indices.clone(),
            Interacting: false,
            ..Default::default()
        }
    }
}

fn supported_flow_option(
    entry: &sonoro_gamesv_data::tables::entity_components::interact::RandomInteractOption,
) -> bool {
    let Some(option) = &entry.Option else {
        return false;
    };
    let Some(option_type) = &option.Type else {
        return false;
    };
    if option_type.Type == "Flow" {
        return option_type.Flow.as_ref().is_some_and(|flow| {
            flow.FlowListName.as_deref().is_some_and(|s| !s.is_empty())
                && flow.FlowId.unwrap_or(0) > 0
        });
    }
    if option_type.Type == "Actions" {
        return option_type.Actions.iter().any(|action| {
            action.Name == "PlayFlow"
                && action.Params.as_ref().is_some_and(|p| {
                    p.FlowListName.as_deref().is_some_and(|s| !s.is_empty())
                        && p.FlowId.unwrap_or(0) > 0
                })
        });
    }
    false
}
