use std::fs;
use std::io;

use sonoro_proto::pb::{JSPatchNotify, WeatherNotify};

use crate::event_queue::EventQueue;
use crate::events::Event;
use crate::sink::PushSink;

pub mod sequence_patch {
    use super::*;

    pub const ALIAS: &str = "sp";
    pub const DESCRIPTION: &str = "deploys the sequence patch onto your client via JsPatchNotify.\nusage: sequence_patch [mode]\nmodes (mode is a number):\n- 0: disabled, undo any previous effects of any sequence patch from before.\n- 1: partial, only show ui in sequences.\n- 2: full, fully detach camera during a sequence.";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        sink: &mut S,
        player_id: i32,
        mode: u8,
    ) -> io::Result<()> {
        let js_path = match mode {
            0 => "assets/scripts/sequence_patch_profiles/default_restore.js",
            1 => "assets/scripts/sequence_patch_profiles/partial_keep_ui.js",
            2 => "assets/scripts/sequence_patch_profiles/full_detach_camera.js",
            _ => {
                events.enqueue(Event::ChatCommandResponse {
                    player_id,
                    message: "invalid sequence patch mode (expected 0..2).".to_string(),
                });
                return Ok(());
            }
        };
        let js = fs::read_to_string(js_path)?;
        sink.push(JSPatchNotify::MSG_ID, &JSPatchNotify { Content: js.into() });
        events.enqueue(Event::ChatCommandResponse {
            player_id,
            message: "successfully applied sequence patch mode.".to_string(),
        });
        Ok(())
    }
}

pub mod reset_cd {
    use super::*;

    pub const ALIAS: &str = "rcd";
    pub const DESCRIPTION: &str =
        "resets the cooldown of all skills on the team.\nusage: reset_cd";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        sink: &mut S,
        player_id: i32,
    ) -> io::Result<()> {
        let js = fs::read_to_string("assets/scripts/misc_patches/reset_skill_and_switch_cooldowns.js")?;
        sink.push(JSPatchNotify::MSG_ID, &JSPatchNotify { Content: js.into() });
        events.enqueue(Event::ChatCommandResponse {
            player_id,
            message: "successfully reset cooldown.".to_string(),
        });
        Ok(())
    }
}

pub mod weather {
    use super::*;

    pub const ALIAS: &str = "wx";
    pub const DESCRIPTION: &str = "sets the current weather to the inputted id.\nusage: weather [id]";

    /// Pushes a native `WeatherNotify` to update weather cleanly across desktop & mobile clients.
    pub fn push<S: PushSink + ?Sized>(sink: &mut S, weather_id: i32) -> io::Result<()> {
        sink.push(
            WeatherNotify::MSG_ID,
            &WeatherNotify {
                WeatherId: weather_id,
                IsClient: false,
            },
        );
        Ok(())
    }

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        sink: &mut S,
        player_id: i32,
        weather_id: i32,
    ) -> io::Result<()> {
        push(sink, weather_id)?;
        events.enqueue(Event::ChatCommandResponse {
            player_id,
            message: "set weather successfully".to_string(),
        });
        Ok(())
    }
}

pub mod team_cd_controller {
    use super::*;

    pub const ALIAS: &str = "tcc";
    pub const DESCRIPTION: &str = "alters the behavior of the team cooldown controller\nyou can choose between 3 types of controllers\ntype 0 - normal controller, nothing different.\ntype 1 - now the cooldown message is removed.\ntype 2 - you no longer have character cooldowns.\nusage: team_cd_controller [type]";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        sink: &mut S,
        player_id: i32,
        controller_type: i32,
    ) -> io::Result<()> {
        if !(0..=2).contains(&controller_type) {
            events.enqueue(Event::ChatCommandResponse {
                player_id,
                message: "invalid controller type (expected 0..2).".to_string(),
            });
            return Ok(());
        }
        let js_raw =
            fs::read_to_string("assets/scripts/misc_patches/team_switch_cooldown_mode.js")?;
        let js = js_raw.replace("{CONTROLLER_TYPE}", &controller_type.to_string());
        sink.push(JSPatchNotify::MSG_ID, &JSPatchNotify { Content: js.into() });
        events.enqueue(Event::ChatCommandResponse {
            player_id,
            message: "changed controller successfully".to_string(),
        });
        Ok(())
    }
}

pub mod timer_bind {
    use super::*;

    pub const ALIAS: &str = "tbind";
    pub const DESCRIPTION: &str = "adds the timer keybinding to your current binds, Z to stop timer, Y to start/restart timer.\nusage: timer_bind";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        sink: &mut S,
        player_id: i32,
    ) -> io::Result<()> {
        let js = fs::read_to_string("assets/scripts/misc_patches/speedrun_timer_keybind.js")?;
        sink.push(JSPatchNotify::MSG_ID, &JSPatchNotify { Content: js.into() });
        events.enqueue(Event::ChatCommandResponse {
            player_id,
            message: "successfully binded.".to_string(),
        });
        Ok(())
    }
}
