use std::sync::OnceLock;

use sonoro_common::zon::{self, ZonValue};

const DEFAULT_CONFIG_ZON: &str = include_str!("../../src/data/zon/config.zon");
const DEFAULT_CONCERTO_EXCEPTIONS_ZON: &str =
    include_str!("../../src/data/zon/concerto_exceptions.zon");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NormalGachaRates {
    pub five_star_hard_pity_pull: u32,
    pub five_star_soft_pity_start_pull: u32,
    pub five_star_soft_pity_phase_one_end_pull: u32,
    pub five_star_soft_pity_phase_two_end_pull: u32,
    pub five_star_soft_pity_phase_three_end_pull: u32,
    pub five_star_soft_pity_force_pull: u32,
    pub five_star_soft_pity_phase_one_step: u32,
    pub five_star_soft_pity_phase_two_step: u32,
    pub five_star_soft_pity_phase_three_step: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoviceGachaRates {
    pub five_star_hard_pity_pull: u32,
    pub five_star_soft_pity_start_pull: u32,
    pub five_star_soft_pity_step: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GachaRatesConfig {
    pub rate_denominator: u32,
    pub five_star_base_rate: u32,
    pub four_star_base_rate: u32,
    pub featured_five_star_win_numerator: u32,
    pub featured_five_star_win_denominator: u32,
    pub featured_four_star_win_numerator: u32,
    pub featured_four_star_win_denominator: u32,
    pub normal: NormalGachaRates,
    pub novice: NoviceGachaRates,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameConfig {
    pub autocomplete_guides: bool,
    pub normal_progression: bool,
    pub seed_default_echoes: bool,
    pub gacha_rates: GachaRatesConfig,
    pub default_uid: i32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self::embedded()
    }
}

impl GameConfig {
    pub fn embedded() -> Self {
        Self::parse_zon(DEFAULT_CONFIG_ZON)
            .expect("embedded gamesv config.zon must parse cleanly")
    }

    pub fn parse_zon(src: &str) -> Result<Self, String> {
        let root = zon::parse(src).map_err(|e| e.to_string())?;
        let gacha = root
            .get("gacha_rates")
            .ok_or_else(|| "missing gacha_rates in config.zon".to_string())?;
        let normal = gacha
            .get("normal")
            .ok_or_else(|| "missing gacha_rates.normal in config.zon".to_string())?;
        let novice = gacha
            .get("novice")
            .ok_or_else(|| "missing gacha_rates.novice in config.zon".to_string())?;

        Ok(Self {
            autocomplete_guides: get_bool(&root, "autocomplete_guides")?,
            normal_progression: get_bool(&root, "normal_progression")?,
            seed_default_echoes: get_bool(&root, "seed_default_echoes")?,
            gacha_rates: GachaRatesConfig {
                rate_denominator: get_u32(gacha, "rate_denominator")?,
                five_star_base_rate: get_u32(gacha, "five_star_base_rate")?,
                four_star_base_rate: get_u32(gacha, "four_star_base_rate")?,
                featured_five_star_win_numerator: get_u32(
                    gacha,
                    "featured_five_star_win_numerator",
                )?,
                featured_five_star_win_denominator: get_u32(
                    gacha,
                    "featured_five_star_win_denominator",
                )?,
                featured_four_star_win_numerator: get_u32(
                    gacha,
                    "featured_four_star_win_numerator",
                )?,
                featured_four_star_win_denominator: get_u32(
                    gacha,
                    "featured_four_star_win_denominator",
                )?,
                normal: NormalGachaRates {
                    five_star_hard_pity_pull: get_u32(normal, "five_star_hard_pity_pull")?,
                    five_star_soft_pity_start_pull: get_u32(
                        normal,
                        "five_star_soft_pity_start_pull",
                    )?,
                    five_star_soft_pity_phase_one_end_pull: get_u32(
                        normal,
                        "five_star_soft_pity_phase_one_end_pull",
                    )?,
                    five_star_soft_pity_phase_two_end_pull: get_u32(
                        normal,
                        "five_star_soft_pity_phase_two_end_pull",
                    )?,
                    five_star_soft_pity_phase_three_end_pull: get_u32(
                        normal,
                        "five_star_soft_pity_phase_three_end_pull",
                    )?,
                    five_star_soft_pity_force_pull: get_u32(
                        normal,
                        "five_star_soft_pity_force_pull",
                    )?,
                    five_star_soft_pity_phase_one_step: get_u32(
                        normal,
                        "five_star_soft_pity_phase_one_step",
                    )?,
                    five_star_soft_pity_phase_two_step: get_u32(
                        normal,
                        "five_star_soft_pity_phase_two_step",
                    )?,
                    five_star_soft_pity_phase_three_step: get_u32(
                        normal,
                        "five_star_soft_pity_phase_three_step",
                    )?,
                },
                novice: NoviceGachaRates {
                    five_star_hard_pity_pull: get_u32(novice, "five_star_hard_pity_pull")?,
                    five_star_soft_pity_start_pull: get_u32(
                        novice,
                        "five_star_soft_pity_start_pull",
                    )?,
                    five_star_soft_pity_step: get_u32(novice, "five_star_soft_pity_step")?,
                },
            },
            default_uid: get_i32(&root, "default_uid")?,
        })
    }
}

pub fn cached_concerto_exceptions() -> &'static [i32] {
    static CACHE: OnceLock<Vec<i32>> = OnceLock::new();
    CACHE.get_or_init(load_concerto_exceptions).as_slice()
}

pub fn load_concerto_exceptions() -> Vec<i32> {
    let val = zon::parse(DEFAULT_CONCERTO_EXCEPTIONS_ZON)
        .expect("embedded concerto_exceptions.zon must parse cleanly");
    match val {
        ZonValue::Array(items) => items
            .into_iter()
            .filter_map(|v| v.as_i64().and_then(|n| i32::try_from(n).ok()))
            .collect(),
        _ => Vec::new(),
    }
}

fn get_bool(val: &ZonValue, key: &str) -> Result<bool, String> {
    val.get(key)
        .and_then(ZonValue::as_bool)
        .ok_or_else(|| format!("missing or invalid bool field '{key}'"))
}

fn get_i32(val: &ZonValue, key: &str) -> Result<i32, String> {
    let n = val
        .get(key)
        .and_then(ZonValue::as_i64)
        .ok_or_else(|| format!("missing or invalid int field '{key}'"))?;
    i32::try_from(n).map_err(|_| format!("integer overflow for field '{key}'"))
}

fn get_u32(val: &ZonValue, key: &str) -> Result<u32, String> {
    let n = val
        .get(key)
        .and_then(ZonValue::as_i64)
        .ok_or_else(|| format!("missing or invalid int field '{key}'"))?;
    u32::try_from(n).map_err(|_| format!("integer overflow for field '{key}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_embedded_zon_configs() {
        let cfg = GameConfig::embedded();
        assert_eq!(cfg.default_uid, 100000001);
        assert!(cfg.normal_progression);
        assert_eq!(cfg.gacha_rates.rate_denominator, 10000);
        let exceptions = load_concerto_exceptions();
        assert!(exceptions.contains(&1211));
        assert_eq!(exceptions.len(), 11);
    }
}
