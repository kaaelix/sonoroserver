use sonoro_gamesv_fs::file_util::serialize_zon;
use sonoro_gamesv_logic::helpers::{energy, pb_kv};
use sonoro_proto::enums::{ErrorCode, PlayerAttrKey, PlayerAttrType};
use sonoro_proto::pb::{
    BirthdayInfoUpdateNotify, BirthdayInitRequest, BirthdayInitResponse, BirthdayShowSetRequest,
    BirthdayShowSetResponse, ChangeCardRequest, ChangeCardResponse, ChangeHeadPhotoRequest,
    ChangeHeadPhotoResponse, ChangePlayerTitleRequest, ChangePlayerTitleResponse,
    DressPlayerTitleUpdateNotify, ModifyNameRequest, ModifyNameResponse, ModifySignatureRequest,
    ModifySignatureResponse, PlayerAttr, PlayerAttrNotify, PlayerAttrValueOneof,
    PlayerHeadDataRequest, PlayerHeadDataResponse,
    PlayerNameUpdateNotify,
    PlayerTitleInfo, PlayerTitleRequest, PlayerTitleResponse, ReadCardRequest, ReadCardResponse,
    RoleShowListUpdateRequest, RoleShowListUpdateResponse,
};

use super::State;

fn save_basic_info(state: &State) -> Result<(), String> {
    let path = format!("player/{}/basic_info", state.player_id);
    let serialized = serialize_zon(&state.player_components.basic.info).map_err(|e| e.to_string())?;
    state.fs.write_file(&path, &serialized).map_err(|e| e.to_string())
}

fn adjusted_modify_name_time() -> i64 {
    let client_cd = 72 * 60 * 60;
    let testing_cd = 5;
    (energy::now_seconds() - client_cd + testing_cd).max(0)
}

pub fn handle_player_head_data_request(
    state: &mut State,
    _req: PlayerHeadDataRequest,
) -> Result<PlayerHeadDataResponse, String> {
    let ids = state
        .assets
        .tables
        .player_head_re
        .items
        .iter()
        .map(|h| h.Id)
        .collect();
    Ok(PlayerHeadDataResponse {
        PlayerHeadDataIds: ids,
    })
}

pub fn handle_player_title_request(
    state: &mut State,
    _req: PlayerTitleRequest,
) -> Result<PlayerTitleResponse, String> {
    let unlock_time = energy::now_seconds();
    let infos = state
        .assets
        .tables
        .player_title
        .items
        .iter()
        .map(|title| PlayerTitleInfo {
            PlayerTitleId: title.Id,
            IsUnlock: true,
            UnlockTime: unlock_time,
            ..Default::default()
        })
        .collect();
    Ok(PlayerTitleResponse {
        PlayerTitleInfo: infos,
        PlayerTitleLimitInfos: Vec::new(),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_show_list_update_request(
    state: &mut State,
    req: RoleShowListUpdateRequest,
) -> Result<RoleShowListUpdateResponse, String> {
    let mut role_ids = Vec::with_capacity(3);
    for role_id in req.RoleList {
        if role_ids.len() == 3 {
            break;
        }
        if !state.player_components.role.role_map.contains_key(&role_id) {
            continue;
        }
        if !role_ids.contains(&role_id) {
            role_ids.push(role_id);
        }
    }
    state.player_components.basic.info.role_show_list = role_ids;
    save_basic_info(state)?;
    Ok(RoleShowListUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_change_card_request(
    state: &mut State,
    req: ChangeCardRequest,
) -> Result<ChangeCardResponse, String> {
    if state
        .assets
        .tables
        .background_card
        .get_data_by_id(&req.CardId)
        .is_some()
    {
        state.player_components.basic.info.cur_card_id = req.CardId;
        save_basic_info(state)?;
    }
    Ok(ChangeCardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_read_card_request(
    state: &mut State,
    req: ReadCardRequest,
) -> Result<ReadCardResponse, String> {
    let key = format!("profile:read_card:{}", req.CardId);
    pb_kv::persist_claim(&state.fs, &mut state.player_components.progress, &key)
        .map_err(|e| e.to_string())?;
    Ok(ReadCardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_change_player_title_request(
    state: &mut State,
    req: ChangePlayerTitleRequest,
) -> Result<ChangePlayerTitleResponse, String> {
    if req.PlayerTitleId == 0
        || state
            .assets
            .tables
            .player_title
            .get_data_by_id(&req.PlayerTitleId)
            .is_some()
    {
        state.player_components.basic.info.cur_player_title_id = req.PlayerTitleId;
        save_basic_info(state)?;
        let _ = state.conn.push_msg(
            DressPlayerTitleUpdateNotify::MSG_ID,
            &DressPlayerTitleUpdateNotify {
                PlayerTitleId: req.PlayerTitleId,
                PlayerTitleExtraParam: if req.PlayerTitleId == 0 { 0 } else { 1 },
            },
        );
    }
    Ok(ChangePlayerTitleResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_modify_name_request(
    state: &mut State,
    req: ModifyNameRequest,
) -> Result<ModifyNameResponse, String> {
    let raw_name = String::from_utf8_lossy(&req.Name);
    let trimmed = raw_name.trim();
    if trimmed.is_empty() {
        return Ok(ModifyNameResponse {
            Name: state
                .player_components
                .basic
                .info
                .attributes
                .name
                .as_bytes()
                .to_vec(),
            ErrorCode: Some(ErrorCode::ErrRoleNameEmpty),
            ..Default::default()
        });
    }
    if trimmed.len() > 64 {
        return Ok(ModifyNameResponse {
            Name: state
                .player_components
                .basic
                .info
                .attributes
                .name
                .as_bytes()
                .to_vec(),
            ErrorCode: Some(ErrorCode::ErrRoleInvalidNameLength),
            ..Default::default()
        });
    }

    let last_modify = adjusted_modify_name_time();
    state.player_components.basic.info.attributes.name = trimmed.to_string();
    state.player_components.basic.info.last_modify_name_time = last_modify;
    save_basic_info(state)?;

    let name_bytes = state
        .player_components
        .basic
        .info
        .attributes
        .name
        .as_bytes()
        .to_vec();
    let _ = state.conn.push_msg(
        PlayerNameUpdateNotify::MSG_ID,
        &PlayerNameUpdateNotify {
            Name: name_bytes.clone(),
            LastModifyNameTime: last_modify,
        },
    );

    Ok(ModifyNameResponse {
        Name: name_bytes,
        ErrorCode: Some(ErrorCode::Success),
        LastModifyNameTime: last_modify,
        ModifyNameTime: Vec::new(),
    })
}

pub fn handle_change_head_photo_request(
    state: &mut State,
    req: ChangeHeadPhotoRequest,
) -> Result<ChangeHeadPhotoResponse, String> {
    if state
        .assets
        .tables
        .player_head_re
        .get_data_by_id(&req.HeadPhotoId)
        .is_some()
    {
        state.player_components.basic.info.attributes.head_photo = req.HeadPhotoId;
        save_basic_info(state)?;
    }
    Ok(ChangeHeadPhotoResponse {
        HeadPhotoId: state.player_components.basic.info.attributes.head_photo,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_modify_signature_request(
    state: &mut State,
    req: ModifySignatureRequest,
) -> Result<ModifySignatureResponse, String> {
    let raw_sig = String::from_utf8_lossy(&req.Signature);
    let trimmed = raw_sig.trim();
    if trimmed.len() > 128 {
        return Ok(ModifySignatureResponse {
            Signature: state
                .player_components
                .basic
                .info
                .attributes
                .sign
                .as_bytes()
                .to_vec(),
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }

    state.player_components.basic.info.attributes.sign = trimmed.to_string();
    save_basic_info(state)?;
    let sig_bytes = state
        .player_components
        .basic
        .info
        .attributes
        .sign
        .as_bytes()
        .to_vec();

    let _ = state.conn.push_msg(
        PlayerAttrNotify::MSG_ID,
        &PlayerAttrNotify {
            Attributes: vec![PlayerAttr {
                Key: Some(PlayerAttrKey::Sign),
                ValueType: Some(PlayerAttrType::String),
                Value: Some(PlayerAttrValueOneof::StringValue(sig_bytes.clone())),
            }],
        },
    );

    Ok(ModifySignatureResponse {
        Signature: sig_bytes,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_birthday_init_request(
    state: &mut State,
    req: BirthdayInitRequest,
) -> Result<BirthdayInitResponse, String> {
    if req.Birthday > 0 {
        state.player_components.basic.info.birthday = req.Birthday;
        state.player_components.basic.info.display_birthday = true;
        save_basic_info(state)?;
        let reward_year = state
            .player_components
            .progress
            .info
            .get_key_int("birthday:reward_year")
            .max(0) as i32;
        let _ = state.conn.push_msg(
            BirthdayInfoUpdateNotify::MSG_ID,
            &BirthdayInfoUpdateNotify {
                BirthDayReset: true,
                RecentRewardTime: reward_year,
                Roles: Vec::new(),
            },
        );
    }
    Ok(BirthdayInitResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_birthday_show_set_request(
    state: &mut State,
    req: BirthdayShowSetRequest,
) -> Result<BirthdayShowSetResponse, String> {
    state.player_components.basic.info.display_birthday = req.DisPlay;
    save_basic_info(state)?;
    Ok(BirthdayShowSetResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
