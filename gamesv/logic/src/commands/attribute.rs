use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_fs::formation_info::FormationInfo;
use sonoro_proto::PbEnum;
use sonoro_proto::pb::{
    AttributeChangedNotify, CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof,
    CombatReceiveData, CombatReceiveDataMessageOneof, CombatReceivePackNotify,
    GameplayAttributeData,
};

use crate::commands::respond;
use crate::component::player::PlayerRoleComponent;
use crate::event_queue::EventQueue;
use crate::fsm::types::wake_reason;
use crate::helpers::attributes::{
    self, AttributeChangeType, AttributeValueType, RelatedAttrKind,
};
use crate::helpers::role as role_helper;
use crate::scene::Scene;
use crate::sink::PushSink;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttrMode {
    Max,
    Empty,
    Set,
}

fn heal_and_revive_team<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    scene: &mut Scene,
    fs: &FileSystem,
    role_comp: &mut PlayerRoleComponent,
    sink: &mut S,
) {
    let player_id = scene.player_id;
    role_helper::revive_and_heal_formation(sink, scene, role_comp, events, fs);
    respond(events, player_id, "revived and healed active formation");
}

pub mod heal {
    use super::*;

    pub const ALIAS: &str = "hl";
    pub const DESCRIPTION: &str =
        "heals and revives all roles in the current formation.\nusage: heal";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        scene: &mut Scene,
        fs: &FileSystem,
        role_comp: &mut PlayerRoleComponent,
        sink: &mut S,
    ) {
        heal_and_revive_team(events, scene, fs, role_comp, sink);
    }
}

pub mod revive {
    use super::*;

    pub const ALIAS: &str = "rv";
    pub const DESCRIPTION: &str =
        "revives and heals all roles in the current formation.\nusage: revive";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        scene: &mut Scene,
        fs: &FileSystem,
        role_comp: &mut PlayerRoleComponent,
        sink: &mut S,
    ) {
        heal_and_revive_team(events, scene, fs, role_comp, sink);
    }
}

pub mod max_attribute {
    use super::*;

    pub const ALIAS: &str = "max";
    pub const DESCRIPTION: &str = "set the inputted attribute to its maximum possible value.\nusage: max_attribute [config_id|team] [EAttributeType]\nconfig_id is a number, for example 1108 is hiyuki's config id. use \"team\" to apply to all current team members.\nEAttributeType is a number.";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        scene: &mut Scene,
        fs: &FileSystem,
        sink: &mut S,
        config_id_str: &str,
        attr: i32,
    ) -> io::Result<()> {
        apply_attribute_command(
            events,
            scene,
            fs,
            sink,
            config_id_str,
            attr,
            AttrMode::Max,
            0,
            "maxxed out stat",
        )
    }
}

pub mod empty_attribute {
    use super::*;

    pub const ALIAS: &str = "empty";
    pub const DESCRIPTION: &str = "set the inputted attribute to its lowest possible value (0).\nusage: empty_attribute [config_id|team] [EAttributeType]\nconfig_id is a number, for example 1108 is hiyuki's config id. use \"team\" to apply to all current team members.\nEAttributeType is a number.";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        scene: &mut Scene,
        fs: &FileSystem,
        sink: &mut S,
        config_id_str: &str,
        attr: i32,
    ) -> io::Result<()> {
        apply_attribute_command(
            events,
            scene,
            fs,
            sink,
            config_id_str,
            attr,
            AttrMode::Empty,
            0,
            "emptied out stat",
        )
    }
}

pub mod set_attribute {
    use super::*;

    pub const ALIAS: &str = "set";
    pub const DESCRIPTION: &str = "set the inputted attribute to the inputted value.\nusage: set_attribute [config_id|team] [EAttributeType] [value]\nconfig_id is a number, for example 1108 is hiyuki's config id. use \"team\" to apply to all current team members.\nEAttributeType is a number.\nvalue is a number";

    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        scene: &mut Scene,
        fs: &FileSystem,
        sink: &mut S,
        config_id_str: &str,
        attr: i32,
        value: i32,
    ) -> io::Result<()> {
        apply_attribute_command(
            events,
            scene,
            fs,
            sink,
            config_id_str,
            attr,
            AttrMode::Set,
            value,
            "set stat to inputted value",
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_attribute_command<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    scene: &mut Scene,
    fs: &FileSystem,
    sink: &mut S,
    config_id_str: &str,
    attr: i32,
    mode: AttrMode,
    value: i32,
    response_msg: &'static str,
) -> io::Result<()> {
    let mut data = Vec::new();
    let ids = resolve_config_ids(config_id_str, &scene.formation_info);
    for config_id in ids {
        apply_attr_change(&mut data, scene, fs, config_id, attr, mode, value)?;
    }
    sink.push(
        CombatReceivePackNotify::MSG_ID,
        &CombatReceivePackNotify { Data: data },
    );
    respond(events, scene.player_id, response_msg);
    Ok(())
}

fn resolve_config_ids(config_id_str: &str, formation_info: &FormationInfo) -> Vec<i32> {
    if config_id_str != "team" {
        return config_id_str
            .parse::<i32>()
            .map_or_else(|_| Vec::new(), |id| vec![id]);
    }
    let Ok(idx) = usize::try_from(formation_info.cur_formation) else {
        return Vec::new();
    };
    let Some(formation) = formation_info.formations.get(idx) else {
        return Vec::new();
    };
    formation
        .roles
        .iter()
        .flatten()
        .map(|r| r.role_id)
        .collect()
}

fn apply_attr_change(
    data: &mut Vec<CombatReceiveData>,
    scene: &mut Scene,
    fs: &FileSystem,
    config_id: i32,
    attr: i32,
    mode: AttrMode,
    value: i32,
) -> io::Result<()> {
    let attr_type = attributes::attr_from_id(attr);
    if attr_type.to_i32() != attr {
        return Ok(());
    }
    let Ok(attr_idx) = usize::try_from(attr) else {
        return Ok(());
    };

    let mut dirty_net_ids = Vec::new();
    for idx in 0..scene.entities.len() {
        let entity = &mut scene.entities[idx];
        if entity.config.config_id != config_id {
            continue;
        }
        let Some(attr_comp) = entity.attribute.as_mut() else {
            continue;
        };
        if attr_idx >= attr_comp.attributes.len() {
            continue;
        }

        let effective_value = match mode {
            AttrMode::Max => attributes::get_related_attr(attr_type, RelatedAttrKind::Max)
                .and_then(|max_type| usize::try_from(max_type.to_i32()).ok())
                .and_then(|max_idx| attr_comp.attributes.get(max_idx))
                .map_or(10_000_000, |a| a.current),
            AttrMode::Empty => 0,
            AttrMode::Set => value,
        };

        let _ = attributes::change_attr(
            attr_comp,
            attr_type,
            AttributeChangeType::Override,
            AttributeValueType::Current,
            effective_value,
        );

        let attr_data: GameplayAttributeData =
            attributes::gameplay_attribute_data(attr_type, attr_comp.attributes[attr_idx]);
        let net_id = entity.net_id;
        data.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: net_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::AttributeChangedNotify(
                        AttributeChangedNotify {
                            Attributes: vec![attr_data],
                        },
                    )),
                },
            )),
        });
        entity.save(fs, scene.player_id, scene.instance_id)?;
        dirty_net_ids.push(net_id);
    }

    for net_id in dirty_net_ids {
        scene.mark_fsm_dirty(net_id, wake_reason::ATTRIBUTE);
    }
    Ok(())
}
