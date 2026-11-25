use sonoro_gamesv_data::Assets;
use sonoro_proto::pb::EntityPb;

use crate::scene::Scene;
use crate::schedulers::BuffTimerScheduler;

pub fn build(
    assets: &Assets,
    scene: &mut Scene,
    buff_timers: &mut BuffTimerScheduler,
    entity_id: i64,
    now_ms: i64,
) -> Result<EntityPb, &'static str> {
    build_optional(assets, scene, Some(buff_timers), entity_id, now_ms)
}

pub fn build_optional(
    assets: &Assets,
    scene: &mut Scene,
    buff_timers: Option<&mut BuffTimerScheduler>,
    entity_id: i64,
    now_ms: i64,
) -> Result<EntityPb, &'static str> {
    if let Some(timers) = buff_timers {
        timers.ensure_entity_registered(scene, assets, entity_id, now_ms);
        timers.sync_entity_left_durations(scene, entity_id, now_ms);
    }

    let storage = scene.entity(entity_id).ok_or("EntityNotFound")?;
    Ok(storage.to_proto(assets, now_ms))
}
