use sonoro_gamesv_logic::component::player::PlayerEchoComponent;
use sonoro_gamesv_logic::helpers::phantom_projector;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    PhantomInteractionEquipRequest, PhantomInteractionEquipResponse,
    PhantomInteractionSkinChangeRequest, PhantomInteractionSkinChangeResponse,
};

use super::super::State;

pub fn handle_phantom_interaction_equip_request(
    state: &mut State,
    req: PhantomInteractionEquipRequest,
) -> Result<PhantomInteractionEquipResponse, String> {
    if req.EquipedMonsterIds.len() != phantom_projector::SLOT_COUNT {
        return Ok(PhantomInteractionEquipResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomInteractionEquipCountNotMatch),
        });
    }

    for &monster_id in &req.EquipedMonsterIds {
        if monster_id != 0 && !phantom_projector::is_unlocked_monster(&state.assets, monster_id) {
            return Ok(PhantomInteractionEquipResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomInteractionNotUnlock),
            });
        }
    }

    phantom_projector::replace_equipped_monsters(
        &mut state.player_components.echo.calabash_info,
        &req.EquipedMonsterIds,
    );
    PlayerEchoComponent::save_calabash(
        &state.fs,
        state.player_id,
        &state.player_components.echo.calabash_info,
    )
    .map_err(|e| e.to_string())?;

    Ok(PhantomInteractionEquipResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_interaction_skin_change_request(
    state: &mut State,
    req: PhantomInteractionSkinChangeRequest,
) -> Result<PhantomInteractionSkinChangeResponse, String> {
    for info in &req.SkinChangeInfos {
        if !phantom_projector::is_unlocked_monster(&state.assets, info.MonsterId) {
            return Ok(PhantomInteractionSkinChangeResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomInteractionNotUnlock),
            });
        }
        if !phantom_projector::is_valid_skin(
            &state.assets,
            &state.player_components.cosmetic.info,
            info.MonsterId,
            info.SkinId,
        ) {
            return Ok(PhantomInteractionSkinChangeResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomSkinNotUnlock),
            });
        }
    }

    for info in &req.SkinChangeInfos {
        phantom_projector::set_equipped_skin(
            &mut state.player_components.echo.calabash_info,
            info.MonsterId,
            info.SkinId,
        );
    }

    PlayerEchoComponent::save_calabash(
        &state.fs,
        state.player_id,
        &state.player_components.echo.calabash_info,
    )
    .map_err(|e| e.to_string())?;

    Ok(PhantomInteractionSkinChangeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
