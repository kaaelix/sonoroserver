use crate::commands::{command_specs, respond};
use crate::event_queue::EventQueue;

pub const DESCRIPTION: &str =
    "outputs information regarding commands.\nusage: help [command?]";

pub fn call(events: &mut EventQueue, player_id: i32, command: Option<&str>) {
    let mut found_command = false;

    for spec in command_specs() {
        let matches = command.is_none_or(|cmd| {
            spec.name == cmd || spec.alias.is_some_and(|alias| alias == cmd)
        });
        if !matches {
            continue;
        }
        found_command = true;
        let msg = match spec.alias {
            Some(alias) => format!("{} ({alias}): {}", spec.name, spec.description),
            None => format!("{}: {}", spec.name, spec.description),
        };
        respond(events, player_id, msg);
    }

    if !found_command {
        respond(events, player_id, "no matching commands found");
    }
}
