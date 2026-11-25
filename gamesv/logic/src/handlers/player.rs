use sonoro_gamesv_data::Assets;
use sonoro_proto::pb::{
    BasicInfoNotify, CardShowEntry, DressPlayerTitleUpdateNotify, RoleShowEntry,
};

use crate::component::player::{PlayerBasicComponent, PlayerRoleComponent};
use crate::sink::PushSink;

pub fn push_data<S: PushSink + ?Sized>(
    sink: &mut S,
    player_id: i32,
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    role_comp: &PlayerRoleComponent,
) {
    let mut notify = BasicInfoNotify {
        Id: player_id,
        Attributes: basic_comp.info.attributes.to_proto().to_vec(),
        RoleShowList: build_role_show_list(&basic_comp.info.role_show_list, role_comp),
        CardUnlockList: build_card_unlock_list(assets),
        CurCardId: basic_comp.info.cur_card_id,
        Birthday: basic_comp.info.birthday,
        DisplayBirthDay: basic_comp.info.display_birthday,
        LastModifyNameTime: basic_comp.info.last_modify_name_time,
        ModifyNameTime: Vec::new(),
        ..Default::default()
    };
    let _ = &mut notify;

    sink.push(BasicInfoNotify::MSG_ID, &notify);
    if basic_comp.info.cur_player_title_id > 0 {
        sink.push(
            DressPlayerTitleUpdateNotify::MSG_ID,
            &DressPlayerTitleUpdateNotify {
                PlayerTitleId: basic_comp.info.cur_player_title_id,
                PlayerTitleExtraParam: 1,
            },
        );
    }
}

fn build_role_show_list(role_ids: &[i32], role_comp: &PlayerRoleComponent) -> Vec<RoleShowEntry> {
    role_ids
        .iter()
        .filter_map(|&role_id| {
            let role = role_comp.role_map.get(&role_id)?;
            Some(RoleShowEntry {
                RoleId: role_id,
                Level: role.level,
            })
        })
        .collect()
}

fn build_card_unlock_list(assets: &Assets) -> Vec<CardShowEntry> {
    assets
        .tables
        .background_card
        .items
        .iter()
        .map(|card| CardShowEntry {
            CardId: card.Id,
            IsRead: true,
        })
        .collect()
}
