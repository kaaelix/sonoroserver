pub mod attribute;
pub mod battle;
pub mod cheat;
pub mod echo;
pub mod entity;
pub mod flow;
pub mod help;
pub mod js_patches;
pub mod map;
pub mod photo;
pub mod progression;

use std::fmt;
use std::io;
use std::str::FromStr;

use crate::event_queue::EventQueue;
use crate::events::Event;
use crate::handlers::PlayerRuntime;
use crate::sink::PushSink;

pub fn respond(events: &mut EventQueue, player_id: i32, message: impl Into<String>) {
    events.enqueue(Event::ChatCommandResponse {
        player_id,
        message: message.into(),
    });
}

#[derive(Debug, Clone, Copy)]
pub struct CommandSpec {
    pub name: &'static str,
    pub alias: Option<&'static str>,
    pub description: &'static str,
}

#[derive(Debug)]
pub enum CommandError {
    EmptyInput,
    TooManyTokens,
    UnknownFunction,
    ArgCountMismatch,
    InvalidBool,
    InvalidArg(&'static str),
    MissingScene,
    Io(io::Error),
    Logic(&'static str),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "empty input"),
            Self::TooManyTokens => write!(f, "too many command tokens"),
            Self::UnknownFunction => write!(f, "unknown command"),
            Self::ArgCountMismatch => write!(f, "argument count mismatch"),
            Self::InvalidBool => write!(f, "invalid bool argument"),
            Self::InvalidArg(msg) | Self::Logic(msg) => write!(f, "{msg}"),
            Self::MissingScene => write!(f, "player has no active scene"),
            Self::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl std::error::Error for CommandError {}

impl From<io::Error> for CommandError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

const COMMAND_SPECS: &[CommandSpec] = &[
    // entity
    CommandSpec {
        name: "spawn",
        alias: Some(entity::ALIAS),
        description: entity::DESCRIPTION,
    },
    CommandSpec {
        name: "spawn_map",
        alias: Some(entity::spawn_map::ALIAS),
        description: entity::spawn_map::DESCRIPTION,
    },
    CommandSpec {
        name: "reset_formation",
        alias: Some(entity::reset_formation::ALIAS),
        description: entity::reset_formation::DESCRIPTION,
    },
    // echo
    CommandSpec {
        name: "echo",
        alias: Some(echo::echo::ALIAS),
        description: echo::echo::DESCRIPTION,
    },
    // help
    CommandSpec {
        name: "help",
        alias: None,
        description: help::DESCRIPTION,
    },
    // photo
    CommandSpec {
        name: "photo",
        alias: Some(photo::ALIAS),
        description: photo::DESCRIPTION,
    },
    // battle
    CommandSpec {
        name: "battle",
        alias: Some(battle::ALIAS),
        description: battle::DESCRIPTION,
    },
    // attribute
    CommandSpec {
        name: "heal",
        alias: Some(attribute::heal::ALIAS),
        description: attribute::heal::DESCRIPTION,
    },
    CommandSpec {
        name: "revive",
        alias: Some(attribute::revive::ALIAS),
        description: attribute::revive::DESCRIPTION,
    },
    CommandSpec {
        name: "max_attribute",
        alias: Some(attribute::max_attribute::ALIAS),
        description: attribute::max_attribute::DESCRIPTION,
    },
    CommandSpec {
        name: "empty_attribute",
        alias: Some(attribute::empty_attribute::ALIAS),
        description: attribute::empty_attribute::DESCRIPTION,
    },
    CommandSpec {
        name: "set_attribute",
        alias: Some(attribute::set_attribute::ALIAS),
        description: attribute::set_attribute::DESCRIPTION,
    },
    // map
    CommandSpec {
        name: "map",
        alias: Some(map::ALIAS),
        description: map::DESCRIPTION,
    },
    // flow
    CommandSpec {
        name: "flow",
        alias: Some(flow::ALIAS),
        description: flow::DESCRIPTION,
    },
    // js_patches
    CommandSpec {
        name: "sequence_patch",
        alias: Some(js_patches::sequence_patch::ALIAS),
        description: js_patches::sequence_patch::DESCRIPTION,
    },
    CommandSpec {
        name: "reset_cd",
        alias: Some(js_patches::reset_cd::ALIAS),
        description: js_patches::reset_cd::DESCRIPTION,
    },
    CommandSpec {
        name: "weather",
        alias: Some(js_patches::weather::ALIAS),
        description: js_patches::weather::DESCRIPTION,
    },
    CommandSpec {
        name: "team_cd_controller",
        alias: Some(js_patches::team_cd_controller::ALIAS),
        description: js_patches::team_cd_controller::DESCRIPTION,
    },
    CommandSpec {
        name: "timer_bind",
        alias: Some(js_patches::timer_bind::ALIAS),
        description: js_patches::timer_bind::DESCRIPTION,
    },
    // progression
    CommandSpec {
        name: "give",
        alias: Some(progression::give::ALIAS),
        description: progression::give::DESCRIPTION,
    },
    CommandSpec {
        name: "level",
        alias: Some(progression::level::ALIAS),
        description: progression::level::DESCRIPTION,
    },
    CommandSpec {
        name: "resonance_item",
        alias: Some(progression::resonance_item::ALIAS),
        description: progression::resonance_item::DESCRIPTION,
    },
    CommandSpec {
        name: "weapon_item",
        alias: Some(progression::weapon_item::ALIAS),
        description: progression::weapon_item::DESCRIPTION,
    },
    CommandSpec {
        name: "unlockres",
        alias: None,
        description: progression::unlockres::DESCRIPTION,
    },
    // cheat
    CommandSpec {
        name: "world_level",
        alias: Some(cheat::world_level::ALIAS),
        description: cheat::world_level::DESCRIPTION,
    },
    CommandSpec {
        name: "unlock_roles",
        alias: Some(cheat::unlock_roles::ALIAS),
        description: cheat::unlock_roles::DESCRIPTION,
    },
    CommandSpec {
        name: "role_max",
        alias: Some(cheat::role_max::ALIAS),
        description: cheat::role_max::DESCRIPTION,
    },
    CommandSpec {
        name: "role_chain",
        alias: Some(cheat::role_chain::ALIAS),
        description: cheat::role_chain::DESCRIPTION,
    },
    CommandSpec {
        name: "weapon_max",
        alias: Some(cheat::weapon_max::ALIAS),
        description: cheat::weapon_max::DESCRIPTION,
    },
    CommandSpec {
        name: "quest_pick",
        alias: Some(cheat::quest_pick::ALIAS),
        description: cheat::quest_pick::DESCRIPTION,
    },
    CommandSpec {
        name: "quest_step",
        alias: Some(cheat::quest_step::ALIAS),
        description: cheat::quest_step::DESCRIPTION,
    },
    CommandSpec {
        name: "quest_skip",
        alias: Some(cheat::quest_skip::ALIAS),
        description: cheat::quest_skip::DESCRIPTION,
    },
    CommandSpec {
        name: "unlock_tp",
        alias: Some(cheat::unlock_tp::ALIAS),
        description: cheat::unlock_tp::DESCRIPTION,
    },
    CommandSpec {
        name: "unlock_func",
        alias: Some(cheat::unlock_func::ALIAS),
        description: cheat::unlock_func::DESCRIPTION,
    },
    CommandSpec {
        name: "money",
        alias: Some(cheat::money::ALIAS),
        description: cheat::money::DESCRIPTION,
    },
    CommandSpec {
        name: "time_set",
        alias: Some(cheat::time_set::ALIAS),
        description: cheat::time_set::DESCRIPTION,
    },
    CommandSpec {
        name: "repair_account",
        alias: Some(cheat::repair_account::ALIAS),
        description: cheat::repair_account::DESCRIPTION,
    },
    CommandSpec {
        name: "switch_rover",
        alias: Some(cheat::switch_rover::ALIAS),
        description: cheat::switch_rover::DESCRIPTION,
    },
];

pub const fn command_specs() -> &'static [CommandSpec] {
    COMMAND_SPECS
}

fn find_canonical_command(name_or_alias: &str) -> Option<&'static str> {
    COMMAND_SPECS
        .iter()
        .find(|spec| {
            spec.name == name_or_alias || spec.alias.is_some_and(|alias| alias == name_or_alias)
        })
        .map(|spec| spec.name)
}

fn check_arity(args: &[&str], min: usize, max: usize) -> Result<(), CommandError> {
    if args.len() < min || args.len() > max {
        return Err(CommandError::ArgCountMismatch);
    }
    Ok(())
}

fn parse_bool(s: &str) -> Result<bool, CommandError> {
    match s {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err(CommandError::InvalidBool),
    }
}

fn parse_int<T: FromStr>(s: &str) -> Result<T, CommandError> {
    s.parse::<T>()
        .map_err(|_| CommandError::InvalidArg("invalid integer argument"))
}

fn opt_str<'a>(args: &[&'a str], idx: usize) -> Option<&'a str> {
    args.get(idx).copied()
}

fn opt_int<T: FromStr>(args: &[&str], idx: usize) -> Result<Option<T>, CommandError> {
    args.get(idx).map(|s| parse_int(s)).transpose()
}

fn opt_bool(args: &[&str], idx: usize) -> Result<Option<bool>, CommandError> {
    args.get(idx).map(|s| parse_bool(s)).transpose()
}

pub fn dispatch<S: PushSink + ?Sized>(
    rt: &mut PlayerRuntime<'_, S>,
    input: &str,
    now_ms: i64,
) -> Result<(), CommandError> {
    for raw_command in input.split(';') {
        let mut tokens = Vec::new();
        for tok in raw_command.trim_matches(' ').split(' ') {
            if tok.is_empty() {
                continue;
            }
            if tokens.len() >= 64 {
                return Err(CommandError::TooManyTokens);
            }
            tokens.push(tok);
        }
        if tokens.is_empty() {
            return Err(CommandError::EmptyInput);
        }

        let Some(canonical) = find_canonical_command(tokens[0]) else {
            return Err(CommandError::UnknownFunction);
        };
        let args = &tokens[1..];
        dispatch_single(rt, canonical, args, now_ms)?;
    }
    Ok(())
}

fn dispatch_single<S: PushSink + ?Sized>(
    rt: &mut PlayerRuntime<'_, S>,
    canonical: &str,
    args: &[&str],
    now_ms: i64,
) -> Result<(), CommandError> {
    match canonical {
        "spawn" => {
            check_arity(args, 1, 3)?;
            let entity_id = parse_int(args[0])?;
            let is_frozen = opt_bool(args, 1)?;
            let is_tune_broken = opt_bool(args, 2)?;
            let sc = rt.scene.as_mut().ok_or(CommandError::MissingScene)?;
            entity::call(
                rt.events,
                sc,
                rt.fs,
                rt.assets,
                rt.sink,
                rt.basic_comp,
                entity_id,
                is_frozen,
                is_tune_broken,
            )
            .map_err(CommandError::Logic)?;
        }
        "spawn_map" => {
            check_arity(args, 2, 4)?;
            let entity_id = parse_int(args[0])?;
            let source_map_id = parse_int(args[1])?;
            let is_frozen = opt_bool(args, 2)?;
            let is_tune_broken = opt_bool(args, 3)?;
            let sc = rt.scene.as_mut().ok_or(CommandError::MissingScene)?;
            entity::spawn_map::call(
                rt.events,
                sc,
                rt.fs,
                rt.assets,
                rt.sink,
                rt.basic_comp,
                entity_id,
                source_map_id,
                is_frozen,
                is_tune_broken,
            )
            .map_err(CommandError::Logic)?;
        }
        "reset_formation" => {
            check_arity(args, 0, 0)?;
            let sc = rt.scene.as_mut().ok_or(CommandError::MissingScene)?;
            entity::reset_formation::call(
                rt.events,
                sc,
                rt.fs,
                rt.assets,
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                rt.sink,
                rt.buff_timers,
                now_ms,
            )
            .map_err(CommandError::Logic)?;
        }
        "echo" => {
            check_arity(args, 3, 13)?;
            let echo_item_id = parse_int(args[0])?;
            let sonata_id = parse_int(args[1])?;
            let main_letter = args[2];
            echo::echo::call(
                rt.events,
                rt.assets,
                rt.fs,
                rt.sink,
                rt.echo_comp,
                echo_item_id,
                sonata_id,
                main_letter,
                opt_str(args, 3),
                opt_int(args, 4)?,
                opt_str(args, 5),
                opt_int(args, 6)?,
                opt_str(args, 7),
                opt_int(args, 8)?,
                opt_str(args, 9),
                opt_int(args, 10)?,
                opt_str(args, 11),
                opt_int(args, 12)?,
            )?;
        }
        "help" => {
            check_arity(args, 0, 1)?;
            help::call(rt.events, rt.player_id, opt_str(args, 0));
        }
        "photo" => {
            check_arity(args, 0, 1)?;
            let mode = opt_bool(args, 0)?;
            let sc = rt.scene.as_ref().ok_or(CommandError::MissingScene)?;
            photo::call(rt.events, sc, rt.sink, mode);
        }
        "battle" => {
            check_arity(args, 1, 1)?;
            let mode = parse_bool(args[0])?;
            let sc = rt.scene.as_mut().ok_or(CommandError::MissingScene)?;
            battle::call(rt.events, sc, rt.sink, mode);
        }
        "heal" => {
            check_arity(args, 0, 0)?;
            let sc = rt.scene.as_mut().ok_or(CommandError::MissingScene)?;
            attribute::heal::call(rt.events, sc, rt.fs, rt.role_comp, rt.sink);
        }
        "revive" => {
            check_arity(args, 0, 0)?;
            let sc = rt.scene.as_mut().ok_or(CommandError::MissingScene)?;
            attribute::revive::call(rt.events, sc, rt.fs, rt.role_comp, rt.sink);
        }
        "max_attribute" => {
            check_arity(args, 2, 2)?;
            let attr = parse_int(args[1])?;
            let sc = rt.scene.as_mut().ok_or(CommandError::MissingScene)?;
            attribute::max_attribute::call(rt.events, sc, rt.fs, rt.sink, args[0], attr)?;
        }
        "empty_attribute" => {
            check_arity(args, 2, 2)?;
            let attr = parse_int(args[1])?;
            let sc = rt.scene.as_mut().ok_or(CommandError::MissingScene)?;
            attribute::empty_attribute::call(rt.events, sc, rt.fs, rt.sink, args[0], attr)?;
        }
        "set_attribute" => {
            check_arity(args, 3, 3)?;
            let attr = parse_int(args[1])?;
            let value = parse_int(args[2])?;
            let sc = rt.scene.as_mut().ok_or(CommandError::MissingScene)?;
            attribute::set_attribute::call(rt.events, sc, rt.fs, rt.sink, args[0], attr, value)?;
        }
        "map" => {
            check_arity(args, 1, 1)?;
            let map_id = parse_int(args[0])?;
            let sc = rt.scene.as_ref().ok_or(CommandError::MissingScene)?;
            map::call(
                rt.events,
                rt.assets,
                rt.fs,
                sc,
                rt.sink,
                rt.scene_comp,
                map_id,
            )?;
        }
        "flow" => {
            check_arity(args, 3, 3)?;
            let flow_id = parse_int(args[1])?;
            let state_id = parse_int(args[2])?;
            let sc = rt.scene.as_ref().ok_or(CommandError::MissingScene)?;
            flow::call(
                rt.events,
                rt.assets,
                rt.fs,
                sc,
                rt.sink,
                rt.scene_comp,
                args[0],
                flow_id,
                state_id,
            )?;
        }
        "sequence_patch" => {
            check_arity(args, 1, 1)?;
            let mode = parse_int(args[0])?;
            js_patches::sequence_patch::call(rt.events, rt.sink, rt.player_id, mode)?;
        }
        "reset_cd" => {
            check_arity(args, 0, 0)?;
            js_patches::reset_cd::call(rt.events, rt.sink, rt.player_id)?;
        }
        "weather" => {
            check_arity(args, 1, 1)?;
            let weather_id = parse_int(args[0])?;
            js_patches::weather::call(rt.events, rt.sink, rt.player_id, weather_id)?;
        }
        "team_cd_controller" => {
            check_arity(args, 1, 1)?;
            let controller_type = parse_int(args[0])?;
            js_patches::team_cd_controller::call(
                rt.events,
                rt.sink,
                rt.player_id,
                controller_type,
            )?;
        }
        "timer_bind" => {
            check_arity(args, 0, 0)?;
            js_patches::timer_bind::call(rt.events, rt.sink, rt.player_id)?;
        }
        "give" => {
            check_arity(args, 1, 2)?;
            let item_id = parse_int(args[0])?;
            let count_opt = opt_int(args, 1)?;
            progression::give::call(
                rt.events,
                rt.fs,
                rt.assets,
                rt.sink,
                rt.basic_comp,
                rt.inventory_comp,
                rt.weapon_comp,
                rt.echo_comp,
                item_id,
                count_opt,
            )?;
        }
        "level" => {
            check_arity(args, 2, 2)?;
            let target_level = parse_int(args[1])?;
            progression::level::call(
                rt.events,
                rt.scene.as_mut(),
                rt.fs,
                rt.assets,
                rt.sink,
                rt.basic_comp,
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                args[0],
                target_level,
            )?;
        }
        "resonance_item" => {
            check_arity(args, 0, 2)?;
            progression::resonance_item::call(
                rt.events,
                rt.assets,
                rt.fs,
                rt.sink,
                rt.scene.as_ref(),
                rt.inventory_comp,
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                opt_str(args, 0),
                opt_str(args, 1),
            )?;
        }
        "weapon_item" => {
            check_arity(args, 0, 2)?;
            progression::weapon_item::call(
                rt.events,
                rt.assets,
                rt.fs,
                rt.sink,
                rt.weapon_comp,
                rt.role_comp,
                rt.scene.as_ref(),
                opt_str(args, 0),
                opt_str(args, 1),
            )?;
        }
        "unlockres" => {
            check_arity(args, 0, 0)?;
            let mut qctx = rt.as_quest_ctx();
            progression::unlockres::call(&mut qctx)?;
        }
        "world_level" => {
            check_arity(args, 1, 1)?;
            let target_wl = parse_int(args[0])?;
            cheat::world_level::call(rt.events, rt.fs, rt.sink, rt.basic_comp, target_wl)?;
        }
        "unlock_roles" => {
            check_arity(args, 0, 1)?;
            cheat::unlock_roles::call(
                rt.events,
                rt.fs,
                rt.assets,
                rt.sink,
                rt.basic_comp,
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                opt_str(args, 0),
            )?;
        }
        "role_max" => {
            check_arity(args, 0, 2)?;
            let lv_opt = opt_int(args, 1)?;
            cheat::role_max::call(
                rt.events,
                rt.fs,
                rt.assets,
                rt.sink,
                rt.scene.as_mut(),
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                opt_str(args, 0),
                lv_opt,
            )?;
        }
        "role_chain" => {
            check_arity(args, 2, 2)?;
            let rank = parse_int(args[1])?;
            cheat::role_chain::call(
                rt.events,
                rt.fs,
                rt.assets,
                rt.sink,
                rt.scene.as_mut(),
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                args[0],
                rank,
            )?;
        }
        "weapon_max" => {
            check_arity(args, 0, 3)?;
            let lv_opt = opt_int(args, 1)?;
            let rank_opt = opt_int(args, 2)?;
            cheat::weapon_max::call(
                rt.events,
                rt.fs,
                rt.assets,
                rt.sink,
                rt.scene.as_mut(),
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                opt_str(args, 0),
                lv_opt,
                rank_opt,
            )?;
        }
        "quest_pick" => {
            check_arity(args, 1, 2)?;
            let quest_id = parse_int(args[0])?;
            let force_opt = opt_bool(args, 1)?;
            let mut qctx = rt.as_quest_ctx();
            cheat::quest_pick::call(&mut qctx, quest_id, force_opt)?;
        }
        "quest_step" => {
            check_arity(args, 0, 1)?;
            let quest_id_opt = opt_int(args, 0)?;
            let mut qctx = rt.as_quest_ctx();
            cheat::quest_step::call(&mut qctx, quest_id_opt)?;
        }
        "quest_skip" => {
            check_arity(args, 0, 1)?;
            let quest_id_opt = opt_int(args, 0)?;
            let mut qctx = rt.as_quest_ctx();
            cheat::quest_skip::call(&mut qctx, quest_id_opt)?;
        }
        "unlock_tp" => {
            check_arity(args, 0, 0)?;
            cheat::unlock_tp::call(rt.events, rt.fs, rt.assets, rt.sink, rt.map_comp)?;
        }
        "unlock_func" => {
            check_arity(args, 0, 0)?;
            let mut qctx = rt.as_quest_ctx();
            cheat::unlock_func::call(&mut qctx)?;
        }
        "money" => {
            check_arity(args, 0, 0)?;
            cheat::money::call(
                rt.events,
                rt.fs,
                rt.assets,
                rt.sink,
                rt.basic_comp,
                rt.inventory_comp,
                rt.weapon_comp,
                rt.echo_comp,
            )?;
        }
        "time_set" => {
            check_arity(args, 1, 2)?;
            let hour = parse_int(args[0])?;
            let minute_opt = opt_int(args, 1)?;
            let mut qctx = rt.as_quest_ctx();
            cheat::time_set::call(&mut qctx, hour, minute_opt)?;
        }
        "repair_account" => {
            check_arity(args, 0, 0)?;
            let mut qctx = rt.as_quest_ctx();
            cheat::repair_account::call(&mut qctx)?;
        }
        "switch_rover" => {
            check_arity(args, 1, 1)?;
            cheat::switch_rover::call(
                rt.events,
                rt.fs,
                rt.assets,
                rt.sink,
                rt.scene.as_mut(),
                rt.basic_comp,
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                args[0],
            )?;
        }
        _ => return Err(CommandError::UnknownFunction),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_specs_match_aliases_and_preserve_s_collision_order() {
        assert_eq!(find_canonical_command("s"), Some("spawn"));
        assert_eq!(find_canonical_command("role_chain"), Some("role_chain"));
        assert_eq!(find_canonical_command("help"), Some("help"));
        assert_eq!(find_canonical_command("ec"), Some("echo"));
        assert_eq!(find_canonical_command("qpick"), Some("quest_pick"));
        assert_eq!(find_canonical_command("nonexistent"), None);
    }

    #[test]
    fn parse_helpers_validate_bools_and_ints() {
        assert!(parse_bool("true").unwrap());
        assert!(parse_bool("1").unwrap());
        assert!(!parse_bool("false").unwrap());
        assert!(!parse_bool("0").unwrap());
        assert!(parse_bool("maybe").is_err());
        assert_eq!(parse_int::<i32>("42").unwrap(), 42);
        assert!(parse_int::<i32>("abc").is_err());
    }
}
