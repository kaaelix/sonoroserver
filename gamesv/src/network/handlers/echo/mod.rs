pub mod calabash;
pub mod presets;
pub mod projector;
pub mod refine;
pub mod shared;
pub mod upgrade;

use std::collections::HashSet;

use sonoro_gamesv_fs::cosmetic_info::CosmeticInfo;
use sonoro_gamesv_logic::component::player::PlayerEchoComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{calabash as calabash_helper, quest_engine};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    PhantomAutoPutRequest, PhantomAutoPutResponse, PhantomItemRequest, PhantomItemResponse,
    PhantomPutOnRequest, PhantomPutOnResponse, PhantomSkinChangeRequest, PhantomSkinChangeResponse,
};

use super::State;

pub fn handle_phantom_item_request(
    state: &mut State,
    _req: PhantomItemRequest,
) -> Result<PhantomItemResponse, String> {
    let phantom_list = shared::phantom_item_list(&state.player_components.echo);
    let equip_list = shared::equip_info_list(
        &state.player_components.role,
        &state.player_components.echo,
    );
    let prop_info = shared::role_prop_info_list(
        &state.assets,
        &mut state.player_components.role,
        &state.player_components.echo,
        &state.player_components.weapon,
    )?;
    let max_cost = calabash_helper::current_cost(
        &state.assets,
        &state.player_components.basic,
        &state.player_components.echo,
    );

    Ok(PhantomItemResponse {
        PhantomItemList: phantom_list,
        EquipInfoList: equip_list,
        PropInfo: prop_info,
        MaxCost: max_cost,
        PhantomSkinList: state.player_components.cosmetic.info.phantom_skins.clone(),
        DirectRefineWeekTimes: state
            .player_components
            .echo
            .calabash_info
            .direct_refine_week_times,
    })
}

pub fn handle_phantom_skin_change_request(
    state: &mut State,
    req: PhantomSkinChangeRequest,
) -> Result<PhantomSkinChangeResponse, String> {
    let Some(echo_id) = state
        .player_components
        .echo
        .echo_map
        .get(&req.IncrId)
        .map(|e| e.id)
    else {
        return Ok(PhantomSkinChangeResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
        });
    };
    let Some(item_config) = state
        .assets
        .tables
        .phantom_item
        .get_data_by_id(&echo_id)
        .map(|r| r.into_owned())
    else {
        return Ok(PhantomSkinChangeResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
        });
    };

    if req.SkinId != 0 {
        let Some(skin_config) = state
            .assets
            .tables
            .phantom_item
            .get_data_by_id(&req.SkinId)
            .map(|r| r.into_owned())
        else {
            return Ok(PhantomSkinChangeResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomSkinNotExist),
            });
        };
        if skin_config.ParentMonsterId != item_config.MonsterId
            || !CosmeticInfo::has(&state.player_components.cosmetic.info.phantom_skins, req.SkinId)
        {
            return Ok(PhantomSkinChangeResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomSkinNotUnlock),
            });
        }
    }

    let mut changed_roles = Vec::new();
    shared::append_roles_with_main_echo(
        &state.player_components.echo,
        req.IncrId,
        &mut changed_roles,
    );

    if let Some(echo) = state.player_components.echo.echo_map.get_mut(&req.IncrId) {
        echo.skin_id = req.SkinId;
    }
    if req.ChangeDefault {
        let mut extra_changed = Vec::new();
        for (&inc_id, other) in &mut state.player_components.echo.echo_map {
            if let Some(other_cfg) = state.assets.tables.phantom_item.get_data_by_id(&other.id)
                && other_cfg.MonsterId == item_config.MonsterId
            {
                other.skin_id = req.SkinId;
                extra_changed.push(inc_id);
            }
        }
        for inc_id in extra_changed {
            shared::append_roles_with_main_echo(
                &state.player_components.echo,
                inc_id,
                &mut changed_roles,
            );
        }
    }

    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;

    if !changed_roles.is_empty() && state.scene.is_some() {
        shared::refresh_role_entities(state, &changed_roles)?;
    }
    Ok(PhantomSkinChangeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_put_on_request(
    state: &mut State,
    events: &mut EventQueue,
    req: PhantomPutOnRequest,
) -> Result<PhantomPutOnResponse, String> {
    if req.Pos < 0 || req.Pos >= 5 {
        return Ok(PhantomPutOnResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            ..Default::default()
        });
    }
    if !shared::role_exists_and_editable(&state.player_components.role, req.RoleId) {
        return Ok(PhantomPutOnResponse {
            ErrorCode: Some(ErrorCode::ErrRoleNotActive),
            ..Default::default()
        });
    }
    if req.IncId != 0
        && !state
            .player_components
            .echo
            .echo_map
            .contains_key(&req.IncId)
    {
        return Ok(PhantomPutOnResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
            ..Default::default()
        });
    }

    let target_pos = req.Pos;
    let target = state
        .player_components
        .echo
        .equipped_echo(req.RoleId, target_pos);
    let source = if req.IncId != 0 {
        state
            .player_components
            .echo
            .equipped_echo_by_incr_id(req.IncId)
    } else {
        None
    };

    if let Some(src_equip) = source
        && !shared::role_exists_and_editable(&state.player_components.role, src_equip.role_id)
    {
        return Ok(PhantomPutOnResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomCannotReplace),
            ..Default::default()
        });
    }

    if req.IncId != 0 {
        let mut next_slots = state.player_components.echo.role_equip(req.RoleId).slots;
        next_slots[target_pos as usize] = req.IncId;
        if let Some(src_equip) = source
            && src_equip.role_id == req.RoleId
        {
            next_slots[src_equip.pos as usize] = target.map_or(0, |t| t.inc_id);
        }
        if let Some(err) = shared::validate_equip_slots(
            &state.assets,
            &state.player_components.basic,
            &state.player_components.echo,
            next_slots,
            ErrorCode::ErrPhantomEquipSourceCost,
        ) {
            return Ok(PhantomPutOnResponse {
                ErrorCode: Some(err),
                ..Default::default()
            });
        }
        if let Some(src_equip) = source
            && src_equip.role_id != req.RoleId
        {
            let mut partner_slots = state
                .player_components
                .echo
                .role_equip(src_equip.role_id)
                .slots;
            partner_slots[src_equip.pos as usize] = target.map_or(0, |t| t.inc_id);
            if let Some(err) = shared::validate_equip_slots(
                &state.assets,
                &state.player_components.basic,
                &state.player_components.echo,
                partner_slots,
                ErrorCode::ErrPhantomEquipTargetCost,
            ) {
                return Ok(PhantomPutOnResponse {
                    ErrorCode: Some(err),
                    ..Default::default()
                });
            }
        }
    }

    let mut changed_roles = vec![req.RoleId];
    if let Some(equip) = target {
        shared::push_unique_role(&mut changed_roles, equip.role_id);
    }
    if let Some(equip) = source {
        shared::push_unique_role(&mut changed_roles, equip.role_id);
    }

    let mut split_infos = Vec::new();
    if req.IncId == 0 {
        if let Some(equip) = target
            && let Some(item) = state.player_components.echo.echo_map.get_mut(&equip.inc_id)
        {
            item.role_id = None;
            item.equipped_pos = -1;
        }
    } else {
        if let Some(target_equip) = target
            && target_equip.inc_id != req.IncId
            && let Some(target_item) = state
                .player_components
                .echo
                .echo_map
                .get_mut(&target_equip.inc_id)
        {
            if let Some(source_equip) = source {
                target_item.role_id = Some(source_equip.role_id);
                target_item.equipped_pos = source_equip.pos;
            } else {
                target_item.role_id = None;
                target_item.equipped_pos = -1;
            }
        }
        if let Some(split) = shared::split_single_from_stack(
            &state.fs,
            &mut state.player_components.echo,
            req.IncId,
        )? {
            split_infos.push(split);
        }
        let Some(item) = state.player_components.echo.echo_map.get_mut(&req.IncId) else {
            return Ok(PhantomPutOnResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
                ..Default::default()
            });
        };
        item.role_id = Some(req.RoleId);
        item.equipped_pos = target_pos;
    }

    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;
    shared::push_role_prop_update(state, &changed_roles)?;
    shared::refresh_role_entities(state, &changed_roles)?;

    {
        let mut qctx = state.quest_context(events);
        if req.IncId != 0
            && let Err(err) =
                quest_engine::on_vision_system_event(&mut qctx, "VisionSetActivated")
        {
            tracing::warn!("on_vision_system_event(VisionSetActivated) error: {err}");
        }
        quest_engine::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    }

    Ok(PhantomPutOnResponse {
        ErrorCode: Some(ErrorCode::Success),
        EquipInfoList: shared::changed_equip_info_list(
            &state.player_components.echo,
            &changed_roles,
        ),
        StackSplitInfos: split_infos,
        StackMergeInfos: Vec::new(),
    })
}

pub fn handle_phantom_auto_put_request(
    state: &mut State,
    events: &mut EventQueue,
    req: PhantomAutoPutRequest,
) -> Result<PhantomAutoPutResponse, String> {
    if !shared::role_exists_and_editable(&state.player_components.role, req.RoleId) {
        return Ok(PhantomAutoPutResponse {
            ErrorCode: Some(ErrorCode::ErrRoleNotActive),
            ..Default::default()
        });
    }

    let mut changed_roles = vec![req.RoleId];
    let mut requested = [0i32; 5];
    let mut seen = HashSet::new();
    let req_len = req.PhantomItemIncrId.len();

    for (index, &inc_id) in req.PhantomItemIncrId.iter().take(5).enumerate() {
        if inc_id != 0 {
            if !state.player_components.echo.echo_map.contains_key(&inc_id) || !seen.insert(inc_id)
            {
                return Ok(PhantomAutoPutResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                    ..Default::default()
                });
            }
            if let Some(prev_equip) = state.player_components.echo.equipped_echo_by_incr_id(inc_id)
                && !shared::role_exists_and_editable(
                    &state.player_components.role,
                    prev_equip.role_id,
                )
            {
                return Ok(PhantomAutoPutResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomAutoEquipFromOther),
                    ..Default::default()
                });
            }
        }
        requested[index] = inc_id;
    }

    if let Some(err) = shared::validate_equip_slots(
        &state.assets,
        &state.player_components.basic,
        &state.player_components.echo,
        requested,
        ErrorCode::ErrPhantomEquipSourceCost,
    ) {
        return Ok(PhantomAutoPutResponse {
            ErrorCode: Some(err),
            ..Default::default()
        });
    }

    for (&inc_id, echo) in &mut state.player_components.echo.echo_map {
        let Some(r_id) = echo.role_id else {
            continue;
        };
        if echo.equipped_pos < 0 || echo.equipped_pos >= 5 {
            continue;
        }
        if r_id == req.RoleId || seen.contains(&inc_id) {
            shared::push_unique_role(&mut changed_roles, r_id);
            echo.role_id = None;
            echo.equipped_pos = -1;
        }
    }

    let mut split_infos = Vec::new();
    for (index, inc_id) in requested.into_iter().enumerate() {
        if inc_id == 0 {
            continue;
        }
        if let Some(split) =
            shared::split_single_from_stack(&state.fs, &mut state.player_components.echo, inc_id)?
        {
            split_infos.push(split);
        }
        if let Some(item) = state.player_components.echo.echo_map.get_mut(&inc_id) {
            item.role_id = Some(req.RoleId);
            item.equipped_pos = index as i32;
        }
    }

    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;
    shared::push_role_prop_update(state, &changed_roles)?;
    shared::refresh_role_entities(state, &changed_roles)?;

    {
        let mut qctx = state.quest_context(events);
        if req_len > 0
            && let Err(err) =
                quest_engine::on_vision_system_event(&mut qctx, "VisionSetActivated")
        {
            tracing::warn!("on_vision_system_event(VisionSetActivated) error: {err}");
        }
        quest_engine::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    }

    Ok(PhantomAutoPutResponse {
        ErrorCode: Some(ErrorCode::Success),
        EquipInfoList: shared::changed_equip_info_list(
            &state.player_components.echo,
            &changed_roles,
        ),
        StackSplitInfos: split_infos,
        StackMergeInfos: Vec::new(),
    })
}
