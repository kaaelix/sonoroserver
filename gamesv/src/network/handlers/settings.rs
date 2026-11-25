use sonoro_gamesv_logic::helpers::pb_kv;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    InputSettingData, InputSettingRequest, InputSettingResponse, InputSettingUpdateRequest,
    InputSettingUpdateResponse, LanguageSettingUpdateRequest, LanguageSettingUpdateResponse,
};

use super::State;

pub fn handle_language_setting_update_request(
    state: &mut State,
    req: LanguageSettingUpdateRequest,
) -> Result<LanguageSettingUpdateResponse, String> {
    let pc = &mut state.player_components.progress;
    pc.info.set_key_int("setting:lang", i64::from(req.Language));
    pc.save(&state.fs).map_err(|e| e.to_string())?;
    Ok(LanguageSettingUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_input_setting_request(
    state: &mut State,
    _req: InputSettingRequest,
) -> Result<InputSettingResponse, String> {
    let data = pb_kv::load_msg::<InputSettingData>(&state.player_components.progress, "setting:input_pb");
    Ok(InputSettingResponse {
        InputSettingData: Some(data.unwrap_or_default()),
    })
}

pub fn handle_input_setting_update_request(
    state: &mut State,
    req: InputSettingUpdateRequest,
) -> Result<InputSettingUpdateResponse, String> {
    if let Some(ref data) = req.InputSettingData {
        pb_kv::save_msg(
            &state.fs,
            &mut state.player_components.progress,
            "setting:input_pb",
            data,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(InputSettingUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
