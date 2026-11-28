use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_proto::PbEnum;
use sonoro_proto::pb::{
    AttributeChangedNotify, CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof,
    CombatReceiveData, CombatReceiveDataMessageOneof, CombatReceivePackNotify,
    GameplayAttributeData, PbGetRoleListNotify,
};

use crate::component::entity::attribute::AttributeComponent;
use crate::component::player::{PlayerEchoComponent, PlayerRoleComponent, PlayerWeaponComponent};
use crate::fsm::types::wake_reason;
use crate::helpers::{attributes, buff, role_stats};
use crate::scene::Scene;
use crate::sink::PushSink;

#[allow(clippy::too_many_arguments)]
pub fn refresh_role<S: PushSink + ?Sized>(
    sink: &mut S,
    fs: &FileSystem,
    scene: &mut Scene,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    role_id: i32,
) -> io::Result<()> {
    let mut target_idx = None;
    for idx in 0..scene.entities.len() {
        if scene.entities[idx].config.config_id != role_id {
            continue;
        }
        if scene.entities[idx].attribute.is_none() {
            continue;
        }
        let net_id = scene.entities[idx].net_id;
        {
            let attr_comp = scene.entities[idx]
                .attribute
                .as_mut()
                .expect("checked is_some above");
            role_stats::refresh_attribute_component(
                assets,
                role_comp,
                role_id,
                weapon_comp,
                echo_comp,
                attr_comp,
            )
            .map_err(io::Error::other)?;
        }
        // Snapshot reset cleared buff deltas from attributes; clear stale tracking and re-sync.
        if let Some(buffs) = scene.entities[idx].buffs.as_mut() {
            buffs.set_applied_buff_mods(Vec::new());
        }
        let mut buff_pack = Vec::new();
        buff::sync_continuous_buff_modifiers(&mut buff_pack, scene, net_id, assets);
        let ent = &mut scene.entities[idx];
        if let (Some(buffs), Some(attr_comp)) = (ent.buffs.as_ref(), ent.attribute.as_mut()) {
            buff::sync_shield_attribute(&mut buff_pack, net_id, buffs, attr_comp);
        }
        scene.mark_fsm_dirty(net_id, wake_reason::ATTRIBUTE);
        scene.save_entity_by_net_id(fs, net_id)?;
        target_idx = Some(idx);
        break;
    }

    if let Some(idx) = target_idx
        && let Some(attr_comp) = scene.entities[idx].attribute.as_ref()
    {
        let net_id = scene.entities[idx].net_id;
        push_attribute_change(sink, net_id, attr_comp);
    }

    push_role_info_notify(sink, assets, role_comp, weapon_comp, echo_comp, role_id);
    Ok(())
}

pub fn push_role_info_notify<S: PushSink + ?Sized>(
    sink: &mut S,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    role_id: i32,
) {
    push_role_info_notify_for_roles(sink, assets, role_comp, weapon_comp, echo_comp, &[role_id]);
}

pub fn push_role_info_notify_for_roles<S: PushSink + ?Sized>(
    sink: &mut S,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    role_ids: &[i32],
) {
    let mut role_list = Vec::with_capacity(role_ids.len());
    for &role_id in role_ids {
        let Some(role_info) = role_comp.role_map.get(&role_id).cloned() else {
            continue;
        };
        if let Ok(pb_role) = role_stats::to_client_role_info(
            assets,
            role_comp,
            role_id,
            &role_info,
            weapon_comp,
            echo_comp,
        ) {
            role_list.push(pb_role);
        }
    }

    if !role_list.is_empty() {
        sink.push(
            PbGetRoleListNotify::MSG_ID,
            &PbGetRoleListNotify {
                RoleList: role_list,
            },
        );
    }
}

fn push_attribute_change<S: PushSink + ?Sized>(
    sink: &mut S,
    entity_id: i64,
    attr_comp: &AttributeComponent,
) {
    let mut attrs: Vec<GameplayAttributeData> = Vec::with_capacity(attr_comp.attributes.len());
    let max_enum = sonoro_proto::enums::EAttributeType::MAX.to_i32();
    for (index, attr) in attr_comp.attributes.iter().enumerate() {
        let idx_i32 = index as i32;
        if idx_i32 >= max_enum {
            break;
        }
        let attr_type = attributes::attr_from_id(idx_i32);
        attrs.push(attributes::gameplay_attribute_data(attr_type, *attr));
    }

    sink.push(
        CombatReceivePackNotify::MSG_ID,
        &CombatReceivePackNotify {
            Data: vec![CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(CombatCommon {
                            EntityId: entity_id,
                            ..Default::default()
                        }),
                        Message: Some(CombatNotifyDataMessageOneof::AttributeChangedNotify(
                            AttributeChangedNotify { Attributes: attrs },
                        )),
                    },
                )),
            }],
        },
    );
}
