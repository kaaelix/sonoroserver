use sonoro_gamesv_logic::scene::{CombineDetach, CombineRegisterResult, CombineRelation, Scene};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    AddCombineEntitiesRelationNotify, CharacterAttachInfo, CharacterAttachRequest,
    CharacterAttachResponse, CharacterDetachRequest, CharacterDetachResponse, CombatCommon,
    CombatNotifyDataMessageOneof, CombatReceiveData,
};

use super::combat::push_combat_notify;
use super::State;

fn attach_envelope_error(
    common_entity_id: Option<i64>,
    maybe_info: Option<&CharacterAttachInfo>,
    target_entity_id: i64,
) -> Option<ErrorCode> {
    let info = maybe_info?;
    let common_id = common_entity_id?;
    if info.EntityId == 0 || info.EntityId != common_id {
        return Some(ErrorCode::ErrCombinerEntityNotExists);
    }
    if target_entity_id == 0 || target_entity_id == info.EntityId {
        return Some(ErrorCode::ErrTargetEntityNotExists);
    }
    if info.Pos.is_none() {
        return Some(ErrorCode::ErrLackCombinerOffsetPos);
    }
    if info.Rot.is_none() {
        return Some(ErrorCode::ErrLackCombinerOffsetRotate);
    }
    None
}

pub fn handle_character_attach_request(
    state: &mut State,
    common: Option<CombatCommon>,
    req: CharacterAttachRequest,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) -> CharacterAttachResponse {
    if req.CharacterAttachInfo.is_none() {
        return CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrLackCombinePartInfoParam),
        };
    }
    let common_eid = common.as_ref().map(|c| c.EntityId).filter(|&id| id != 0);
    if common_eid.is_none() {
        return CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrCombinerEntityNotExists),
        };
    }
    if let Some(code) = attach_envelope_error(common_eid, req.CharacterAttachInfo.as_ref(), req.TargetEntity) {
        return CharacterAttachResponse {
            ErrorCode: Some(code),
        };
    }

    let info = req.CharacterAttachInfo.unwrap();
    let Some(scene) = state.scene.as_mut() else {
        return CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrCombinerEntityNotExists),
        };
    };

    let Some(&combine_idx) = scene.net_id_map.get(&info.EntityId) else {
        return CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrCombinerEntityNotExists),
        };
    };
    if scene.entities[combine_idx].character_attach.is_none() {
        return CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrCombineComponentNotExists),
        };
    }

    let Some(&target_idx) = scene.net_id_map.get(&req.TargetEntity) else {
        return CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrTargetEntityNotExists),
        };
    };
    if scene.entities[target_idx].character_attach.is_none() {
        return CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrCombineComponentNotExists),
        };
    }
    let Some(target_parts) = scene.entities[target_idx].part.as_ref() else {
        return CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrTargetPartNotExists),
        };
    };
    if !target_parts.supports_combine_part(info.PartIndex) {
        return CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrTargetPartNotExists),
        };
    }

    let rel = CombineRelation {
        target_entity_id: req.TargetEntity,
        part_index: info.PartIndex,
        position: info.Pos.clone().unwrap_or_default(),
        rotation: info.Rot.clone().unwrap_or_default(),
    };
    match scene.register_combine_relation(info.EntityId, rel) {
        Ok(CombineRegisterResult::Added | CombineRegisterResult::Existing) => {
            if let Some(attach_comp) = scene.entities[combine_idx].character_attach.as_mut() {
                attach_comp.pb_combine_target_server_id = req.TargetEntity;
                if let Some(existing) = attach_comp
                    .pb_combine_part_info_list
                    .iter_mut()
                    .find(|p| p.EntityId == info.EntityId)
                {
                    *existing = info.clone();
                } else {
                    attach_comp.pb_combine_part_info_list.push(info.clone());
                }
            }
            let notify_common = Some(CombatCommon {
                EntityId: info.EntityId,
                ..Default::default()
            });
            push_combat_notify(
                receive_data_pack,
                notify_common,
                CombatNotifyDataMessageOneof::AddCombineEntitiesRelationNotify(
                    AddCombineEntitiesRelationNotify {
                        CharacterAttachInfo: Some(info),
                        TargetEntity: req.TargetEntity,
                    },
                ),
            );
            CharacterAttachResponse {
                ErrorCode: Some(ErrorCode::Success),
            }
        }
        Err(_) => CharacterAttachResponse {
            ErrorCode: Some(ErrorCode::ErrAlreadyCombineToOtherEntity),
        },
    }
}

pub fn handle_character_detach_request(
    state: &mut State,
    common: Option<CombatCommon>,
    req: CharacterDetachRequest,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) -> CharacterDetachResponse {
    let Some(combine_entity_id) = common.as_ref().map(|c| c.EntityId).filter(|&id| id != 0) else {
        return CharacterDetachResponse {
            ErrorCode: Some(ErrorCode::ErrCombineEntityNotFound),
        };
    };
    if req.EntityA == 0 || req.EntityB == 0 || req.EntityA != combine_entity_id {
        return CharacterDetachResponse {
            ErrorCode: Some(ErrorCode::ErrCombineEntityNotFound),
        };
    }

    let Some(scene) = state.scene.as_mut() else {
        return CharacterDetachResponse {
            ErrorCode: Some(ErrorCode::ErrCombineEntityNotFound),
        };
    };
    let Some(&combine_idx) = scene.net_id_map.get(&combine_entity_id) else {
        return CharacterDetachResponse {
            ErrorCode: Some(ErrorCode::ErrCombineEntityNotFound),
        };
    };
    if scene.entities[combine_idx].character_attach.is_none() {
        return CharacterDetachResponse {
            ErrorCode: Some(ErrorCode::ErrCombineComponentNotExists),
        };
    }
    let Some(&target_idx) = scene.net_id_map.get(&req.EntityB) else {
        return CharacterDetachResponse {
            ErrorCode: Some(ErrorCode::ErrTargetEntityNotExists),
        };
    };
    if scene.entities[target_idx].character_attach.is_none() {
        return CharacterDetachResponse {
            ErrorCode: Some(ErrorCode::ErrCombineComponentNotExists),
        };
    }
    if scene
        .detach_combine_relation(combine_entity_id, req.EntityB)
        .is_none()
    {
        return CharacterDetachResponse {
            ErrorCode: Some(ErrorCode::ErrCombineEntityNotFound),
        };
    }

    if let Some(attach_comp) = scene.entities[combine_idx].character_attach.as_mut() {
        attach_comp.pb_combine_target_server_id = 0;
        attach_comp
            .pb_combine_part_info_list
            .retain(|p| p.EntityId != combine_entity_id);
    }
    receive_data_pack.push(Scene::remove_combine_notify(CombineDetach {
        combine_entity_id,
        target_entity_id: req.EntityB,
    }));
    scene.signal_fsm_dissolve_combine(combine_entity_id);

    CharacterDetachResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}
