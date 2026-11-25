use sonoro_gamesv_logic::helpers::role as role_helper;
use sonoro_proto::enums::{ErrorCode, SelfPortraitOpState};
use sonoro_proto::pb::{
    CameraSettingInfo, CameraSettingModifyRequest, CameraSettingModifyResponse,
    CameraSettingQueryRequest, CameraSettingQueryResponse, Rotator, SelfPortraitStateRequest,
    SelfPortraitStateResponse, SwitchRoleNotify, SwitchRoleNotifyTransformOneof, Transform,
    Vector,
};

use super::State;

pub fn handle_camera_setting_query_request(
    state: &mut State,
    _req: CameraSettingQueryRequest,
) -> Result<CameraSettingQueryResponse, String> {
    Ok(CameraSettingQueryResponse {
        ErrorCode: Some(ErrorCode::Success),
        CameraSetting: Some(CameraSettingInfo {
            CameraParam: Some(state.player_components.settings.camera.to_proto()),
        }),
    })
}

pub fn handle_camera_setting_modify_request(
    state: &mut State,
    req: CameraSettingModifyRequest,
) -> Result<CameraSettingModifyResponse, String> {
    if let Some(setting) = req.CameraSetting
        && let Some(param) = setting.CameraParam
    {
        state.player_components.settings.camera.from_proto(&param);
        state
            .player_components
            .settings
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    Ok(CameraSettingModifyResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_self_portrait_state_request(
    state: &mut State,
    req: SelfPortraitStateRequest,
) -> Result<SelfPortraitStateResponse, String> {
    let op = req.State.unwrap_or(SelfPortraitOpState::SelfPortraitEnter);
    let pc = &mut state.player_components.progress;
    pc.info.set_key_int("camera:self_portrait", i64::from(op as i32));
    pc.save(&state.fs).map_err(|e| e.to_string())?;

    if op == SelfPortraitOpState::SelfPortraitEnter {
        let gender = state.player_components.basic.info.attributes.sex;
        let player_id = state.player_id;
        let mut notify: Option<SwitchRoleNotify> = None;
        if let Some(scene) = state.scene.as_mut() {
            let cur_idx = scene.formation_info.cur_formation.max(0) as usize;
            if let Some(formation) = scene.formation_info.formations.get_mut(cur_idx) {
                let mut main_slot = None;
                let mut down_entity_id = 0i64;
                for slot in formation.roles.iter().flatten() {
                    if slot.role_id == formation.cur_role {
                        down_entity_id = slot.entity_id;
                    }
                    if main_slot.is_none()
                        && role_helper::is_main_role_for_gender(&state.assets, gender, slot.role_id)
                    {
                        main_slot = Some((slot.role_id, slot.entity_id));
                    }
                }
                if let Some((main_role_id, up_entity_id)) = main_slot
                    && formation.cur_role != main_role_id
                {
                    formation.cur_role = main_role_id;
                    if let Some(&down_idx) = scene.net_id_map.get(&down_entity_id) {
                        scene.entities[down_idx].direct_control = false;
                    }
                    let mut transform = None;
                    if let Some(&up_idx) = scene.net_id_map.get(&up_entity_id) {
                        scene.entities[up_idx].direct_control = true;
                        if let Some(pos) = scene.entities[up_idx].position {
                            transform = Some(SwitchRoleNotifyTransformOneof::Transform(Transform {
                                Pos: Some(Vector {
                                    X: pos.location[0],
                                    Y: pos.location[1],
                                    Z: pos.location[2],
                                }),
                                Rot: Some(Rotator {
                                    Roll: pos.rotation[0],
                                    Pitch: pos.rotation[1],
                                    Yaw: pos.rotation[2],
                                }),
                            }));
                        }
                    }
                    if transform.is_none()
                        && let Some(&down_idx) = scene.net_id_map.get(&down_entity_id)
                        && let Some(pos) = scene.entities[down_idx].position
                    {
                        transform = Some(SwitchRoleNotifyTransformOneof::Transform(Transform {
                            Pos: Some(Vector {
                                X: pos.location[0],
                                Y: pos.location[1],
                                Z: pos.location[2],
                            }),
                            Rot: Some(Rotator {
                                Roll: pos.rotation[0],
                                Pitch: pos.rotation[1],
                                Yaw: pos.rotation[2],
                            }),
                        }));
                    }
                    let _ = scene.save_persistent(&state.fs);
                    notify = Some(SwitchRoleNotify {
                        transform,
                        PlayerId: player_id,
                        DownEntityId: down_entity_id,
                        UpEntityId: up_entity_id,
                        OnStageWithoutControl: false,
                    });
                }
            }
        }
        if let Some(msg) = notify {
            let _ = state.conn.push_msg(SwitchRoleNotify::MSG_ID, &msg);
        }
    }

    Ok(SelfPortraitStateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
