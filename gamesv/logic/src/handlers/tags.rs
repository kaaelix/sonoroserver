use sonoro_gamesv_data::Assets;
use sonoro_proto::pb::{
    AnimationGameplayTagNotify, CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof,
    CombatReceiveData, CombatReceiveDataMessageOneof, CombatReceivePackNotify, CommonTagData,
    EntityCommonTagNotify,
};

use crate::component::entity::small::EntityType;
use crate::events::EntityHandle;
use crate::scene::Scene;
use crate::sink::PushSink;

/// Client only attaches `LevelTagComponent` on monsters, scene items, custom/scene entities,
/// and non-vehicle `Common` (`NpcType == 0`) NPCs (`Simple`/`Passerby`/`Vehicle`/`Tuanzi` lack it).
pub fn has_client_level_tag_component(scene: &Scene, assets: Option<&Assets>, net_id: i64) -> bool {
    let Some(&idx) = scene.net_id_map.get(&net_id) else {
        return false;
    };
    let entity = &scene.entities[idx];
    let cfg = entity.config;
    match cfg.entity_type {
        EntityType::Monster
        | EntityType::SceneItem
        | EntityType::Custom
        | EntityType::SceneEntity => true,
        EntityType::Npc => {
            if entity.vehicle.is_some() || entity.fsm.is_none() {
                return false;
            }
            if let Some(a) = assets
                && let Some(le) = a.find_level_entity(scene.instance_id, i64::from(cfg.config_id))
            {
                if let Some(bi) = &le.ComponentsData.BaseInfoComponent
                    && let Some(cat) = &bi.Category
                    && let Some(nt) = cat.NpcType
                    && nt != 0
                {
                    return false;
                }
                if le.BlueprintType == "PasserbyNpc" || le.BlueprintType == "SimpleNpc" {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

pub fn update_gameplay_tags<S: PushSink + ?Sized>(
    sink: &mut S,
    scene: &Scene,
    assets: &Assets,
    entity: EntityHandle,
    add_tag_ids: &[i32],
    remove_tag_ids: &[i32],
    remove_before_add: bool,
) {
    if add_tag_ids.is_empty() && remove_tag_ids.is_empty() {
        return;
    }

    if has_client_level_tag_component(scene, Some(assets), entity.net_id) {
        let mut tags = Vec::new();
        append_common_tag_notifies(&mut tags, add_tag_ids, remove_tag_ids, remove_before_add);
        if !tags.is_empty() {
            sink.push(
                EntityCommonTagNotify::MSG_ID,
                &EntityCommonTagNotify {
                    Id: entity.net_id,
                    Tags: tags,
                },
            );
        }
        return;
    }

    let combat_common = CombatCommon {
        EntityId: entity.net_id,
        ..Default::default()
    };
    let mut data = Vec::new();
    append_gameplay_tag_notifies(
        &mut data,
        &combat_common,
        add_tag_ids,
        remove_tag_ids,
        remove_before_add,
    );
    if !data.is_empty() {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: data },
        );
    }
}

fn append_common_tag_notifies(
    tags: &mut Vec<CommonTagData>,
    add_tag_ids: &[i32],
    remove_tag_ids: &[i32],
    remove_before_add: bool,
) {
    // CommonTagData field 2 is IsAdd (stale generated name RemoveTagIds): false removes, true adds.
    let push_slice = |out: &mut Vec<CommonTagData>, ids: &[i32], is_add: bool| {
        for &tag_id in ids {
            if tag_id != 0 {
                out.push(CommonTagData {
                    TagId: tag_id,
                    RemoveTagIds: is_add,
                });
            }
        }
    };
    if remove_before_add {
        push_slice(tags, remove_tag_ids, false);
        push_slice(tags, add_tag_ids, true);
    } else {
        push_slice(tags, add_tag_ids, true);
        push_slice(tags, remove_tag_ids, false);
    }
}

fn append_gameplay_tag_notifies(
    data: &mut Vec<CombatReceiveData>,
    combat_common: &CombatCommon,
    add_tag_ids: &[i32],
    remove_tag_ids: &[i32],
    remove_before_add: bool,
) {
    let push_slice = |out: &mut Vec<CombatReceiveData>, ids: &[i32], add: bool| {
        for &tag_id in ids {
            out.push(CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(combat_common.clone()),
                        Message: Some(CombatNotifyDataMessageOneof::AnimationGameplayTagNotify(
                            AnimationGameplayTagNotify {
                                AddTagIds: tag_id,
                                RemoveTagIds: add,
                            },
                        )),
                    },
                )),
            });
        }
    };
    if remove_before_add {
        push_slice(data, remove_tag_ids, false);
        push_slice(data, add_tag_ids, true);
    } else {
        push_slice(data, add_tag_ids, true);
        push_slice(data, remove_tag_ids, false);
    }
}
