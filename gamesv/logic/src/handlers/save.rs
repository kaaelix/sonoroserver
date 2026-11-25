use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;

use crate::component::player::{PlayerRoleComponent, PlayerWeaponComponent};
use crate::events::EntityHandle;
use crate::scene::{Scene, now_millis};
use crate::schedulers::{BuffTimerScheduler, DirtySaveQueue};

pub fn on_role_info_modified(dirty_saves: &mut DirtySaveQueue, role_id: i32) {
    dirty_saves.mark_role(role_id, now_millis());
}

pub fn on_weapon_info_modified(dirty_saves: &mut DirtySaveQueue, incr_id: i32) {
    dirty_saves.mark_weapon(incr_id, now_millis());
}

pub fn on_entity_movement(dirty_saves: &mut DirtySaveQueue, entity: EntityHandle) {
    dirty_saves.mark_movement(entity, now_millis());
}

pub fn on_buff_change(dirty_saves: &mut DirtySaveQueue, entity: EntityHandle) {
    dirty_saves.mark_buff_change(entity, now_millis());
}

#[allow(clippy::too_many_arguments)]
pub fn on_dirty_saves_due(
    now_ms: i64,
    fs: &FileSystem,
    dirty_saves: &mut DirtySaveQueue,
    role_comp: &PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    scene: Option<&mut Scene>,
    buff_timers: &mut BuffTimerScheduler,
    assets: &Assets,
) -> io::Result<()> {
    dirty_saves.flush(
        fs,
        role_comp,
        weapon_comp,
        scene,
        buff_timers,
        assets,
        now_ms,
    )
}
