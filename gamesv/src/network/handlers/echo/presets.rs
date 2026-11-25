use std::collections::HashSet;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::echo_info::VisionGroup;
use sonoro_gamesv_fs::special_item_incr;
use sonoro_gamesv_logic::component::player::PlayerEchoComponent;
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::enums::{ErrorCode, PhantomBatchOper, PhantomSettingType};
use sonoro_proto::pb::{
    AddVisionEquipGroupRequest, AddVisionEquipGroupResponse, ApplyVisionGroupRequest,
    ApplyVisionGroupResponse, ChangeVisionGroupNameRequest, ChangeVisionGroupNameResponse,
    CostCombinationInfo, CostPhantomAttrRecommendInfo, DeleteVisionEquipGroupRequest,
    DeleteVisionEquipGroupResponse, ItemFuncValueUpdateNotify, MainPhantomRecommendInfo,
    OnePhantomSetting,
    PhantomAttrRecommendInfo, PhantomAttrRecommendRequest, PhantomAttrRecommendResponse,
    PhantomBatchOperRequest, PhantomBatchOperResponse, PhantomFetterGroupInfo,
    PhantomFetterRecommendInfo, PhantomFetterRecommendRequest, PhantomFetterRecommendResponse,
    PhantomSettingRequest, PhantomSettingResponse, PhantomSettingUpdateRequest,
    PhantomSettingUpdateResponse, PutVisionGroupToTopRequest, PutVisionGroupToTopResponse,
    VisionEquipGroupInfoRequest, VisionEquipGroupInfoResponse,
};

use super::super::State;
use super::shared;

pub fn handle_vision_equip_group_info_request(
    state: &mut State,
    _req: VisionEquipGroupInfoRequest,
) -> Result<VisionEquipGroupInfoResponse, String> {
    Ok(VisionEquipGroupInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        VisionEquipList: shared::vision_equip_group_list(&state.player_components.echo),
        IsOldData: false,
    })
}

pub fn handle_add_vision_equip_group_request(
    state: &mut State,
    req: AddVisionEquipGroupRequest,
) -> Result<AddVisionEquipGroupResponse, String> {
    if !state.player_components.role.role_map.contains_key(&req.RoleId) {
        return Ok(AddVisionEquipGroupResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            ..Default::default()
        });
    }
    let equip = state.player_components.echo.role_equip(req.RoleId);
    if state.player_components.echo.preset_info.groups.len() >= 20 {
        return Ok(AddVisionEquipGroupResponse {
            ErrorCode: Some(ErrorCode::ErrVisionSkillSlotNotFound),
            ..Default::default()
        });
    }

    let incr_id = special_item_incr::next(&state.fs, state.player_id).map_err(|e| e.to_string())?;
    let name = String::from_utf8_lossy(&req.Name).into_owned();
    state
        .player_components
        .echo
        .preset_info
        .groups
        .push(VisionGroup {
            incr_id,
            name,
            slots: equip.slots,
        });
    PlayerEchoComponent::save_presets(
        &state.fs,
        state.player_id,
        &state.player_components.echo.preset_info,
    )
    .map_err(|e| e.to_string())?;

    Ok(AddVisionEquipGroupResponse {
        ErrorCode: Some(ErrorCode::Success),
        VisionEquipList: shared::vision_equip_group_list(&state.player_components.echo),
        IsOldData: false,
    })
}

pub fn handle_apply_vision_group_request(
    state: &mut State,
    req: ApplyVisionGroupRequest,
) -> Result<ApplyVisionGroupResponse, String> {
    let (equip_info_list, split_infos, err_opt) = apply_group_impl(state, req.RoleId, req.Index)?;
    if let Some(err) = err_opt {
        return Ok(ApplyVisionGroupResponse {
            ErrorCode: Some(err),
            ..Default::default()
        });
    }
    Ok(ApplyVisionGroupResponse {
        ErrorCode: Some(ErrorCode::Success),
        EquipInfoList: equip_info_list,
        StackSplitInfos: split_infos,
        StackMergeInfos: Vec::new(),
    })
}

pub fn handle_put_vision_group_to_top_request(
    state: &mut State,
    req: PutVisionGroupToTopRequest,
) -> Result<PutVisionGroupToTopResponse, String> {
    if req.Index < 0
        || (req.Index as usize) >= state.player_components.echo.preset_info.groups.len()
    {
        return Ok(PutVisionGroupToTopResponse {
            ErrorCode: Some(ErrorCode::ErrVisionSkillSlotNotFound),
            ..Default::default()
        });
    }
    let idx = req.Index as usize;
    if idx != 0 {
        let group = state
            .player_components
            .echo
            .preset_info
            .groups
            .remove(idx);
        state
            .player_components
            .echo
            .preset_info
            .groups
            .insert(0, group);
        PlayerEchoComponent::save_presets(
            &state.fs,
            state.player_id,
            &state.player_components.echo.preset_info,
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(PutVisionGroupToTopResponse {
        ErrorCode: Some(ErrorCode::Success),
        VisionEquipList: shared::vision_equip_group_list(&state.player_components.echo),
    })
}

pub fn handle_change_vision_group_name_request(
    state: &mut State,
    req: ChangeVisionGroupNameRequest,
) -> Result<ChangeVisionGroupNameResponse, String> {
    if req.Index < 0
        || (req.Index as usize) >= state.player_components.echo.preset_info.groups.len()
    {
        return Ok(ChangeVisionGroupNameResponse {
            ErrorCode: Some(ErrorCode::ErrVisionSkillSlotNotFound),
            ..Default::default()
        });
    }
    let idx = req.Index as usize;
    state.player_components.echo.preset_info.groups[idx].name =
        String::from_utf8_lossy(&req.Name).into_owned();
    PlayerEchoComponent::save_presets(
        &state.fs,
        state.player_id,
        &state.player_components.echo.preset_info,
    )
    .map_err(|e| e.to_string())?;

    Ok(ChangeVisionGroupNameResponse {
        ErrorCode: Some(ErrorCode::Success),
        VisionEquipList: shared::vision_equip_group_list(&state.player_components.echo),
    })
}

pub fn handle_delete_vision_equip_group_request(
    state: &mut State,
    req: DeleteVisionEquipGroupRequest,
) -> Result<DeleteVisionEquipGroupResponse, String> {
    if req.Index < 0
        || (req.Index as usize) >= state.player_components.echo.preset_info.groups.len()
    {
        return Ok(DeleteVisionEquipGroupResponse {
            ErrorCode: Some(ErrorCode::ErrVisionSkillSlotNotFound),
            ..Default::default()
        });
    }
    state
        .player_components
        .echo
        .preset_info
        .groups
        .remove(req.Index as usize);
    PlayerEchoComponent::save_presets(
        &state.fs,
        state.player_id,
        &state.player_components.echo.preset_info,
    )
    .map_err(|e| e.to_string())?;

    Ok(DeleteVisionEquipGroupResponse {
        ErrorCode: Some(ErrorCode::Success),
        VisionEquipList: shared::vision_equip_group_list(&state.player_components.echo),
    })
}

#[allow(clippy::type_complexity)]
fn apply_group_impl(
    state: &mut State,
    role_id: i32,
    group_index: i32,
) -> Result<
    (
        Vec<sonoro_proto::pb::RolePhantomEquipInfo>,
        Vec<sonoro_proto::pb::PhantomStackSplitInfo>,
        Option<ErrorCode>,
    ),
    String,
> {
    if !shared::role_exists_and_editable(&state.player_components.role, role_id) {
        return Ok((Vec::new(), Vec::new(), Some(ErrorCode::ErrRoleNotActive)));
    }
    let Some(group) = (group_index >= 0)
        .then(|| {
            state
                .player_components
                .echo
                .preset_info
                .groups
                .get(group_index as usize)
                .cloned()
        })
        .flatten()
    else {
        return Ok((
            Vec::new(),
            Vec::new(),
            Some(ErrorCode::ErrVisionSkillSlotNotFound),
        ));
    };

    let mut seen = HashSet::new();
    for inc_id in group.slots {
        if inc_id == 0 {
            continue;
        }
        if !state.player_components.echo.echo_map.contains_key(&inc_id) || !seen.insert(inc_id) {
            return Ok((
                Vec::new(),
                Vec::new(),
                Some(ErrorCode::ErrPhantomItemNotExist),
            ));
        }
    }

    if let Some(err) = shared::validate_equip_slots(
        &state.assets,
        &state.player_components.basic,
        &state.player_components.echo,
        group.slots,
        ErrorCode::ErrPhantomEquipSourceCost,
    ) {
        return Ok((Vec::new(), Vec::new(), Some(err)));
    }

    let mut changed_roles = vec![role_id];
    for (&inc_id, echo) in &mut state.player_components.echo.echo_map {
        let Some(r_id) = echo.role_id else {
            continue;
        };
        if echo.equipped_pos < 0 || echo.equipped_pos >= 5 {
            continue;
        }
        if r_id == role_id || seen.contains(&inc_id) {
            shared::push_unique_role(&mut changed_roles, r_id);
            echo.role_id = None;
            echo.equipped_pos = -1;
        }
    }

    let mut split_infos = Vec::new();
    for (index, inc_id) in group.slots.into_iter().enumerate() {
        if inc_id == 0 {
            continue;
        }
        if let Some(split) =
            shared::split_single_from_stack(&state.fs, &mut state.player_components.echo, inc_id)?
        {
            split_infos.push(split);
        }
        if let Some(item) = state.player_components.echo.echo_map.get_mut(&inc_id) {
            item.role_id = Some(role_id);
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

    let equip_list =
        shared::changed_equip_info_list(&state.player_components.echo, &changed_roles);
    Ok((equip_list, split_infos, None))
}

pub fn handle_phantom_batch_oper_request(
    state: &mut State,
    req: PhantomBatchOperRequest,
) -> Result<PhantomBatchOperResponse, String> {
    let new_func = match req.Oper.unwrap_or(PhantomBatchOper::Reset) {
        PhantomBatchOper::Lock => 1,
        PhantomBatchOper::Disuse => 2,
        PhantomBatchOper::Reset => 0,
    };
    let mut changed = false;
    for incr_id in req.IncrId {
        if let Some(item) = state.player_components.echo.echo_map.get_mut(&incr_id) {
            item.func_value = new_func;
            changed = true;
            state.conn.push(
                ItemFuncValueUpdateNotify::MSG_ID,
                &ItemFuncValueUpdateNotify {
                    IncrId: incr_id,
                    FuncValue: new_func,
                },
            );
        }
    }
    if changed {
        PlayerEchoComponent::save_all(
            &state.fs,
            state.player_id,
            &state.player_components.echo.echo_map,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(PhantomBatchOperResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

#[allow(dead_code)]
pub fn handle_phantom_setting_request(
    state: &mut State,
    _req: PhantomSettingRequest,
) -> Result<PhantomSettingResponse, String> {
    let progress = &state.player_components.progress.info;
    let mut auto_lock = Vec::new();
    let mut auto_disuse = Vec::new();
    for i in 0..5 {
        let lval = progress.get_key_int(&format!("phantom_set:0:{i}"));
        if lval != 0 {
            let name = progress.get_key_str(&format!("phantom_set:0:name:{i}"));
            auto_lock.push(OnePhantomSetting {
                Index: i,
                On: lval > 0,
                Name: name.as_bytes().to_vec(),
                RuleList: Vec::new(),
            });
        }
        let dval = progress.get_key_int(&format!("phantom_set:1:{i}"));
        if dval != 0 {
            let name = progress.get_key_str(&format!("phantom_set:1:name:{i}"));
            auto_disuse.push(OnePhantomSetting {
                Index: i,
                On: dval > 0,
                Name: name.as_bytes().to_vec(),
                RuleList: Vec::new(),
            });
        }
    }
    Ok(PhantomSettingResponse {
        AutoLockSettings: auto_lock,
        AutoDisuseSettings: auto_disuse,
    })
}

#[allow(dead_code)]
pub fn handle_phantom_setting_update_request(
    state: &mut State,
    req: PhantomSettingUpdateRequest,
) -> Result<PhantomSettingUpdateResponse, String> {
    if let Some(ref setting) = req.Setting {
        let stype = match req.SettingType.unwrap_or(PhantomSettingType::AutoLock) {
            PhantomSettingType::AutoLock => 0,
            PhantomSettingType::AutoDisuse => 1,
        };
        let key = format!("phantom_set:{stype}:{}", setting.Index);
        state
            .player_components
            .progress
            .info
            .set_key_int(&key, if setting.On { 1 } else { -1 });
        let nkey = format!("phantom_set:{stype}:name:{}", setting.Index);
        let name_str = String::from_utf8_lossy(&setting.Name);
        state
            .player_components
            .progress
            .info
            .set_key_str(&nkey, &name_str);
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(PhantomSettingUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
        Setting: req.Setting,
    })
}

pub fn build_cost_attr_recommends() -> Vec<CostPhantomAttrRecommendInfo> {
    [(4, 20001), (3, 7), (1, 1)]
        .into_iter()
        .map(|(cost, main)| CostPhantomAttrRecommendInfo {
            Cost: cost,
            MainAttrRecommendInfos: vec![PhantomAttrRecommendInfo {
                AttrType: main,
                AddType: 2,
                Usage: 1,
            }],
            ViceAttrRecommendInfos: [20001, 20002]
                .into_iter()
                .map(|va| PhantomAttrRecommendInfo {
                    AttrType: va,
                    AddType: 2,
                    Usage: 1,
                })
                .collect(),
        })
        .collect()
}

pub fn build_fetter_recommends(
    assets: &Assets,
    role_id: i32,
) -> Vec<PhantomFetterRecommendInfo> {
    let mut selected_group_ids: Vec<i32> = assets
        .tables
        .phantom_recommend
        .get_data_by_id(&role_id)
        .map(|cfg| {
            cfg.RecommendFetterGroups
                .iter()
                .copied()
                .filter(|&id| id > 0)
                .collect()
        })
        .unwrap_or_default();

    if selected_group_ids.is_empty() {
        selected_group_ids = assets
            .tables
            .phantom_fetter_group
            .items
            .iter()
            .take(3)
            .map(|fg| fg.Id)
            .collect();
    }

    let mut groups: Vec<PhantomFetterGroupInfo> = Vec::new();
    for &group_id in &selected_group_ids {
        let mut counts: Vec<i32> = assets
            .tables
            .phantom_fetter_group
            .get_data_by_id(&group_id)
            .map(|fg| fg.FetterMap.keys().copied().filter(|&k| k > 0).collect())
            .unwrap_or_default();
        counts.sort_unstable();
        if counts.is_empty() {
            continue;
        }
        for count in counts {
            groups.push(PhantomFetterGroupInfo {
                PhantomFetterGroupId: group_id,
                CountNeed: count,
            });
            if groups.len() >= 3 {
                break;
            }
        }
        if groups.len() >= 3 {
            break;
        }
    }
    if groups.len() < 2 {
        for fg in &assets.tables.phantom_fetter_group.items {
            let Some(&first_count) = fg.FetterMap.keys().filter(|&&k| k > 0).min() else {
                continue;
            };
            if groups
                .iter()
                .any(|g| g.PhantomFetterGroupId == fg.Id && g.CountNeed == first_count)
            {
                continue;
            }
            groups.push(PhantomFetterGroupInfo {
                PhantomFetterGroupId: fg.Id,
                CountNeed: first_count,
            });
            if groups.len() >= 2 {
                break;
            }
        }
    }

    let mut seen_monsters = HashSet::new();
    let mut main_phantoms = Vec::new();
    for pi in &assets.tables.phantom_item.items {
        if pi.MonsterId <= 0 {
            continue;
        }
        let Some(&matched_group) = pi
            .FetterGroup
            .iter()
            .find(|g| selected_group_ids.contains(g))
        else {
            continue;
        };
        if !seen_monsters.insert(pi.MonsterId) {
            continue;
        }
        main_phantoms.push(MainPhantomRecommendInfo {
            Usage: 1,
            MonsterId: pi.MonsterId,
            FetterGroupId: matched_group,
        });
        if main_phantoms.len() >= 3 {
            break;
        }
    }

    vec![PhantomFetterRecommendInfo {
        Usage: 1,
        PhantomFetterGroupInfos: groups,
        CostCombinationInfos: vec![CostCombinationInfo {
            CostId: 43311,
            Usage: 1,
        }],
        MainPhantomRecommendInfos: main_phantoms,
    }]
}

pub fn handle_phantom_attr_recommend_request(
    _state: &mut State,
    _req: PhantomAttrRecommendRequest,
) -> Result<PhantomAttrRecommendResponse, String> {
    Ok(PhantomAttrRecommendResponse {
        ErrorCode: Some(ErrorCode::Success),
        CostPhantomAttrRecommendInfos: build_cost_attr_recommends(),
    })
}

pub fn handle_phantom_fetter_recommend_request(
    state: &mut State,
    req: PhantomFetterRecommendRequest,
) -> Result<PhantomFetterRecommendResponse, String> {
    Ok(PhantomFetterRecommendResponse {
        ErrorCode: Some(ErrorCode::Success),
        PhantomFetterRecommendInfos: build_fetter_recommends(&state.assets, req.RoleId),
    })
}
