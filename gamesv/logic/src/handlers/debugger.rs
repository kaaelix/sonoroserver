use crate::events::EntityHandle;
use crate::scene::Scene;

pub fn log_entity_movement(scene: &Scene, entity: EntityHandle) {
    let Some(storage) = scene.entity(entity.net_id) else {
        return;
    };
    if let Some(pos) = storage.position {
        tracing::debug!(
            target: "entity_movement",
            "entity with id {} moved to ({:?}, {:?})",
            entity.net_id,
            pos.location,
            pos.rotation
        );
    }
}
