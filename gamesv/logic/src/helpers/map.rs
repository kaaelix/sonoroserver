use sonoro_gamesv_data::Assets;
use sonoro_proto::pb::MapUnlockFieldNotify;

use crate::component::player::PlayerMapComponent;
use crate::sink::PushSink;

pub fn unlock_fog_and_siblings<S: PushSink + ?Sized>(
    sink: &mut S,
    assets: &Assets,
    map_comp: &mut PlayerMapComponent,
    fog_id: i32,
) -> bool {
    if fog_id <= 0 {
        return false;
    }
    let mut changed = false;
    if map_comp.info.unlock_fog(fog_id) {
        changed = true;
        sink.push(
            MapUnlockFieldNotify::MSG_ID,
            &MapUnlockFieldNotify { FieldId: fog_id },
        );
    }
    if let Some(fog_cfg) = assets.tables.map_fog.get_data_by_id(&fog_id)
        && fog_cfg.AreaId > 0
    {
        for sibling in &assets.tables.map_fog.items {
            if sibling.AreaId == fog_cfg.AreaId
                && sibling.Fog > 0
                && map_comp.info.unlock_fog(sibling.Fog)
            {
                changed = true;
                sink.push(
                    MapUnlockFieldNotify::MSG_ID,
                    &MapUnlockFieldNotify {
                        FieldId: sibling.Fog,
                    },
                );
            }
        }
    }
    changed
}
