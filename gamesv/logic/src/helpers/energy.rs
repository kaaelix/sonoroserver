use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_common::fs::FileSystem;
use sonoro_proto::pb::{EnergyInfo, EnergyUpdateNotify};

use crate::component::comp_util;
use crate::component::player::PlayerBasicComponent;
use crate::sink::PushSink;

pub const TYPE_WAVEPLATE: i32 = 5;
pub const TYPE_CRYSTALWAVE_PLATE: i32 = 6;

pub const WAVEPLATE_NATURAL_CAP: i32 = 240;
pub const WAVEPLATE_RENEW_SECONDS: i64 = 360;
pub const WAVEPLATE_HARD_CAP: i32 = 2400;

pub const CRYSTALWAVE_CAP: i32 = 480;
pub const CRYSTALWAVE_RENEW_SECONDS: i64 = 720;

pub fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

fn to_client_time(ts: i64) -> i32 {
    ts.clamp(0, i64::from(i32::MAX)) as i32
}

pub fn sync_energy(basic_comp: &mut PlayerBasicComponent, now_sec: i64) -> bool {
    let info = &mut basic_comp.info;
    let mut changed = false;

    if info.last_renew_energy_time <= 0 {
        info.last_renew_energy_time = now_sec;
        changed = true;
    }
    if info.last_store_energy_time <= 0 {
        info.last_store_energy_time = now_sec;
        changed = true;
    }

    if info.energy_count < WAVEPLATE_NATURAL_CAP && now_sec > info.last_renew_energy_time {
        let elapsed = now_sec - info.last_renew_energy_time;
        let gained = elapsed.div_euclid(WAVEPLATE_RENEW_SECONDS);
        if gained > 0 {
            let space = i64::from(WAVEPLATE_NATURAL_CAP - info.energy_count);
            if gained < space {
                info.energy_count += gained as i32;
                info.last_renew_energy_time += gained * WAVEPLATE_RENEW_SECONDS;
            } else {
                let reach_cap_time = info.last_renew_energy_time + space * WAVEPLATE_RENEW_SECONDS;
                info.energy_count = WAVEPLATE_NATURAL_CAP;
                info.last_renew_energy_time = now_sec;
                info.last_store_energy_time = reach_cap_time;
            }
            changed = true;
        }
    }

    if info.energy_count >= WAVEPLATE_NATURAL_CAP {
        if info.last_renew_energy_time != now_sec {
            info.last_renew_energy_time = now_sec;
            changed = true;
        }
        if info.store_energy_count < CRYSTALWAVE_CAP && now_sec > info.last_store_energy_time {
            let elapsed = now_sec - info.last_store_energy_time;
            let gained = elapsed.div_euclid(CRYSTALWAVE_RENEW_SECONDS);
            if gained > 0 {
                let space = i64::from(CRYSTALWAVE_CAP - info.store_energy_count);
                let add = space.min(gained);
                info.store_energy_count += add as i32;
                if info.store_energy_count >= CRYSTALWAVE_CAP {
                    info.last_store_energy_time = now_sec;
                } else {
                    info.last_store_energy_time += add * CRYSTALWAVE_RENEW_SECONDS;
                }
                changed = true;
            }
        } else if info.store_energy_count >= CRYSTALWAVE_CAP
            && info.last_store_energy_time != now_sec
        {
            info.last_store_energy_time = now_sec;
            changed = true;
        }
    } else if info.last_store_energy_time != now_sec {
        info.last_store_energy_time = now_sec;
        changed = true;
    }

    changed
}

pub fn save_basic_info(fs: &FileSystem, basic_comp: &PlayerBasicComponent) -> io::Result<()> {
    let path = format!("player/{}/basic_info", basic_comp.player_id);
    comp_util::save_struct(fs, &basic_comp.info, &path)
}

pub fn sync_and_save(fs: &FileSystem, basic_comp: &mut PlayerBasicComponent) -> io::Result<()> {
    if sync_energy(basic_comp, now_seconds()) {
        save_basic_info(fs, basic_comp)?;
    }
    Ok(())
}

pub fn can_add_waveplate(basic_comp: &mut PlayerBasicComponent, amount: i32) -> bool {
    if amount <= 0 {
        return false;
    }
    sync_energy(basic_comp, now_seconds());
    basic_comp.info.energy_count < WAVEPLATE_HARD_CAP
        && i64::from(basic_comp.info.energy_count) + i64::from(amount)
            <= i64::from(WAVEPLATE_HARD_CAP)
}

pub fn add_waveplate<S: PushSink + ?Sized>(
    sink: &mut S,
    fs: &FileSystem,
    basic_comp: &mut PlayerBasicComponent,
    amount: i32,
) -> io::Result<bool> {
    if amount <= 0 {
        return Ok(false);
    }
    let now_sec = now_seconds();
    sync_energy(basic_comp, now_sec);
    if basic_comp.info.energy_count >= WAVEPLATE_HARD_CAP
        || i64::from(basic_comp.info.energy_count) + i64::from(amount)
            > i64::from(WAVEPLATE_HARD_CAP)
    {
        return Ok(false);
    }
    let was_below_cap = basic_comp.info.energy_count < WAVEPLATE_NATURAL_CAP;
    basic_comp.info.energy_count += amount;
    if was_below_cap && basic_comp.info.energy_count >= WAVEPLATE_NATURAL_CAP {
        basic_comp.info.last_renew_energy_time = now_sec;
        basic_comp.info.last_store_energy_time = now_sec;
    }
    save_basic_info(fs, basic_comp)?;
    push_energy_notify(sink, basic_comp);
    Ok(true)
}

pub fn consume_waveplate<S: PushSink + ?Sized>(
    sink: &mut S,
    fs: &FileSystem,
    basic_comp: &mut PlayerBasicComponent,
    amount: i32,
) -> io::Result<bool> {
    if amount <= 0 {
        return Ok(true);
    }
    let now_sec = now_seconds();
    sync_energy(basic_comp, now_sec);
    if basic_comp.info.energy_count < amount {
        return Ok(false);
    }
    let was_at_or_above_cap = basic_comp.info.energy_count >= WAVEPLATE_NATURAL_CAP;
    basic_comp.info.energy_count -= amount;
    if was_at_or_above_cap && basic_comp.info.energy_count < WAVEPLATE_NATURAL_CAP {
        basic_comp.info.last_renew_energy_time = now_sec;
        basic_comp.info.last_store_energy_time = now_sec;
    }
    save_basic_info(fs, basic_comp)?;
    push_energy_notify(sink, basic_comp);
    Ok(true)
}

pub fn build_energy_list(
    basic_comp: &PlayerBasicComponent,
    filter_types: &[i32],
) -> Vec<EnergyInfo> {
    let mut list = Vec::new();
    let want_waveplate = filter_types.is_empty() || filter_types.contains(&TYPE_WAVEPLATE);
    let want_store = filter_types.is_empty() || filter_types.contains(&TYPE_CRYSTALWAVE_PLATE);

    if want_waveplate {
        list.push(EnergyInfo {
            EnergyCount: basic_comp.info.energy_count,
            LastRenewEnergyTime: to_client_time(basic_comp.info.last_renew_energy_time),
            EnergyType: TYPE_WAVEPLATE,
        });
    }
    if want_store {
        list.push(EnergyInfo {
            EnergyCount: basic_comp.info.store_energy_count,
            LastRenewEnergyTime: to_client_time(basic_comp.info.last_store_energy_time),
            EnergyType: TYPE_CRYSTALWAVE_PLATE,
        });
    }
    list
}

pub fn push_energy_notify<S: PushSink + ?Sized>(sink: &mut S, basic_comp: &PlayerBasicComponent) {
    sink.push(
        EnergyUpdateNotify::MSG_ID,
        &EnergyUpdateNotify {
            UpdateInfo: build_energy_list(basic_comp, &[]),
        },
    );
}
