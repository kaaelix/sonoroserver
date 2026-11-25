use serde::{Deserialize, Serialize};
use sonoro_proto::enums::{PlayerAttrKey, PlayerAttrType};
use sonoro_proto::pb::{PlayerAttr, PlayerAttrValueOneof};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct BasicAttributes {
    pub level: i32,
    pub exp: i32,
    pub coin: i32,
    pub rare_coin: i32,
    pub head_photo: i32,
    pub head_frame: i32,
    pub area_id: i32,
    pub name: String,
    pub sign: String,
    pub sex: i32,
    pub origin_world_level: i32,
    pub cur_world_level: i32,
    pub world_level_time_stamp: i32,
    pub cash_coin: i32,
    pub world_permission: i32,
}

impl Default for BasicAttributes {
    fn default() -> Self {
        Self {
            level: 1,
            exp: 0,
            coin: 0,
            rare_coin: 0,
            head_photo: 82001209,
            head_frame: 0,
            area_id: 1,
            name: "Rover".to_string(),
            sign: "Wuthering Waves".to_string(),
            sex: 0,
            origin_world_level: 1,
            cur_world_level: 1,
            world_level_time_stamp: 0,
            cash_coin: 0,
            world_permission: 0,
        }
    }
}

impl BasicAttributes {
    pub fn to_proto(&self) -> Vec<PlayerAttr> {
        let int_attr = |key: PlayerAttrKey, v: i32| PlayerAttr {
            Key: Some(key),
            ValueType: Some(PlayerAttrType::Int32),
            Value: Some(PlayerAttrValueOneof::Int32Value(v)),
        };
        let str_attr = |key: PlayerAttrKey, s: &str| PlayerAttr {
            Key: Some(key),
            ValueType: Some(PlayerAttrType::String),
            Value: Some(PlayerAttrValueOneof::StringValue(s.as_bytes().to_vec())),
        };
        vec![
            int_attr(PlayerAttrKey::Level, self.level),
            int_attr(PlayerAttrKey::Exp, self.exp),
            int_attr(PlayerAttrKey::Coin, self.coin),
            int_attr(PlayerAttrKey::RareCoin, self.rare_coin),
            int_attr(PlayerAttrKey::HeadPhoto, self.head_photo),
            int_attr(PlayerAttrKey::HeadFrame, self.head_frame),
            int_attr(PlayerAttrKey::AreaId, self.area_id),
            str_attr(PlayerAttrKey::Name, &self.name),
            str_attr(PlayerAttrKey::Sign, &self.sign),
            int_attr(PlayerAttrKey::Sex, self.sex),
            int_attr(PlayerAttrKey::OriginWorldLevel, self.origin_world_level),
            int_attr(PlayerAttrKey::CurWorldLevel, self.cur_world_level),
            int_attr(
                PlayerAttrKey::WorldLevelTimeStamp,
                self.world_level_time_stamp,
            ),
            int_attr(PlayerAttrKey::CashCoin, self.cash_coin),
            int_attr(PlayerAttrKey::WorldPermission, self.world_permission),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct BasicInfo {
    pub attributes: BasicAttributes,
    pub role_show_list: Vec<i32>,
    pub selected_main_role_id: i32,
    pub cur_card_id: i32,
    pub cur_player_title_id: i32,
    pub last_modify_name_time: i64,
    pub birthday: i32,
    pub display_birthday: bool,
    pub energy_count: i32,
    pub last_renew_energy_time: i64,
    pub store_energy_count: i32,
    pub last_store_energy_time: i64,
}

impl Default for BasicInfo {
    fn default() -> Self {
        Self {
            attributes: BasicAttributes::default(),
            role_show_list: Vec::new(),
            selected_main_role_id: 0,
            cur_card_id: 0,
            cur_player_title_id: 0,
            last_modify_name_time: 0,
            birthday: 20260407,
            display_birthday: true,
            energy_count: 240,
            last_renew_energy_time: 0,
            store_energy_count: 0,
            last_store_energy_time: 0,
        }
    }
}
