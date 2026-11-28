use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_proto::enums::EAttributeType;
use sonoro_proto::pb::{ArrayIntInt, ArraySkillNode, ResonInfo, RoleInfo as PbRoleInfo};
use sonoro_proto::wire::PbEnum;

use crate::component::entity::attribute::{Attribute, AttributeComponent};
use crate::component::player::{PlayerEchoComponent, PlayerRoleComponent, PlayerWeaponComponent};
use crate::helpers::role_equipment::{DisplayProps, RoleEquipment, Snapshot};

pub fn build_snapshot(
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    role_id: i32,
) -> Result<Snapshot, &'static str> {
    let equipment = role_comp.rebuild_equipment(assets, role_id, weapon_comp, echo_comp)?;
    Ok(equipment.snapshot())
}

pub fn build_preview_snapshot(
    assets: &Assets,
    role_id: i32,
    role_info: &RoleInfo,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
) -> Snapshot {
    RoleEquipment::build(assets, role_id, role_info, weapon_comp, echo_comp).snapshot()
}

pub fn build_phantom_display_props(
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    role_id: i32,
) -> Result<DisplayProps, &'static str> {
    let equipment = role_comp.rebuild_equipment(assets, role_id, weapon_comp, echo_comp)?;
    Ok(equipment.phantom_display_props())
}

pub fn refresh_attribute_component(
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    role_id: i32,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    attr_comp: &mut AttributeComponent,
) -> Result<(), &'static str> {
    let life_idx = EAttributeType::Life.to_i32() as usize;
    let max_idx = EAttributeType::LifeMax.to_i32() as usize;
    let energy_idx = EAttributeType::Energy.to_i32() as usize;
    let energy_max_idx = EAttributeType::EnergyMax.to_i32() as usize;
    let elem_energy_idx = EAttributeType::ElementEnergy.to_i32() as usize;
    let elem_energy_max_idx = EAttributeType::ElementEnergyMax.to_i32() as usize;
    let shield_idx = EAttributeType::Sheild.to_i32() as usize;
    let prev_hp = attr_comp.attributes[life_idx].current;
    let prev_max = attr_comp.attributes[max_idx].current;
    let prev_energy = attr_comp.attributes[energy_idx].current;
    let prev_elem_energy = attr_comp.attributes[elem_energy_idx].current;
    let prev_shield = attr_comp.attributes[shield_idx].current;

    let snapshot = build_snapshot(assets, role_comp, weapon_comp, echo_comp, role_id)?;

    for (index, &base) in snapshot.base_props.iter().enumerate() {
        if index >= attr_comp.attributes.len() {
            break;
        }
        let increment = snapshot.add_props.get(index).copied().unwrap_or(0);
        attr_comp.attributes[index] = Attribute {
            base,
            increment,
            current: base + increment,
        };
    }

    let restore_pool =
        |attrs: &mut AttributeComponent, val_idx: usize, max_idx: usize, prev_val: i32| {
            let max_val = attrs.attributes[max_idx].current;
            let clamped = if max_val > 0 {
                prev_val.clamp(0, max_val)
            } else {
                prev_val.max(0)
            };
            attrs.attributes[val_idx] = Attribute {
                base: clamped,
                increment: 0,
                current: clamped,
            };
        };
    restore_pool(attr_comp, energy_idx, energy_max_idx, prev_energy);
    restore_pool(
        attr_comp,
        elem_energy_idx,
        elem_energy_max_idx,
        prev_elem_energy,
    );
    if prev_shield > 0 {
        attr_comp.attributes[shield_idx] = Attribute {
            base: 0,
            increment: prev_shield,
            current: prev_shield,
        };
    }

    let new_max = attr_comp.attributes[max_idx].current.max(1);
    let role_info = role_comp.role_map.get_mut(&role_id);
    if prev_hp <= 0 && prev_max > 0 {
        attr_comp.attributes[life_idx] = Attribute {
            base: 0,
            increment: 0,
            current: 0,
        };
        if let Some(ri) = role_info {
            ri.current_hp = Some(0);
        }
    } else if prev_max > 0 && prev_hp < prev_max {
        let scaled = (((i64::from(prev_hp) * i64::from(new_max)) / i64::from(prev_max)) as i32)
            .clamp(1, new_max);
        attr_comp.attributes[life_idx] = Attribute {
            base: scaled,
            increment: 0,
            current: scaled,
        };
        if let Some(ri) = role_info {
            ri.current_hp = Some(scaled);
        }
    } else {
        attr_comp.attributes[life_idx] = Attribute {
            base: new_max,
            increment: 0,
            current: new_max,
        };
        if let Some(ri) = role_info {
            ri.current_hp = None;
        }
    }
    Ok(())
}

pub fn to_client_role_info(
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    role_id: i32,
    role_info: &RoleInfo,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
) -> Result<PbRoleInfo, &'static str> {
    let snapshot = build_snapshot(assets, role_comp, weapon_comp, echo_comp, role_id)?;
    Ok(PbRoleInfo {
        RoleId: role_id,
        Name: Vec::new(),
        Level: role_info.level,
        Exp: role_info.exp,
        Breakthrough: role_info.breakthrough,
        Star: role_info.star,
        Favor: role_info.favor,
        CurModel: role_info.cur_model,
        CreateTime: role_info.create_time,
        ResonantChainGroupIndex: role_info.resonant_chain_group_index,
        SkinId: role_info.role_skin_id,
        EnableSelfBgm: role_info.enable_self_bgm,
        Skills: role_info
            .skills
            .iter()
            .map(|&(k, v)| ArrayIntInt { Key: k, Value: v })
            .collect(),
        Phantom: role_info
            .phantom
            .iter()
            .map(|&(k, v)| ArrayIntInt { Key: k, Value: v })
            .collect(),
        BaseProp: snapshot
            .base_props
            .iter()
            .enumerate()
            .map(|(k, &v)| ArrayIntInt {
                Key: k as i32,
                Value: v,
            })
            .collect(),
        AddProp: snapshot
            .add_props
            .iter()
            .enumerate()
            .map(|(k, &v)| ArrayIntInt {
                Key: k as i32,
                Value: v,
            })
            .collect(),
        Reson: role_info
            .reson
            .iter()
            .map(|entry| ResonInfo {
                ResonId: entry.id,
                IsOpen: entry.open,
                Increase: entry.increase,
            })
            .collect(),
        Models: role_info.models.clone(),
        SkillNodeState: role_info
            .skill_node_state
            .iter()
            .map(|node| ArraySkillNode {
                SkillNodeId: node.node_id,
                IsActive: node.active,
                SkillId: node.skill_id,
            })
            .collect(),
    })
}
