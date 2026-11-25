//! Player/world ZON persistence models and atomic counters for `gamesv`.

pub mod account;
pub mod basic_info;
pub mod camera_settings;
pub mod cosmetic_info;
pub mod echo_info;
pub mod explore_tools_info;
pub mod file_util;
pub mod formation_info;
pub mod gacha_info;
pub mod guide_info;
pub mod incr;
pub mod inventory_info;
pub mod last_scene_info;
pub mod mail_info;
pub mod map_info;
pub mod motor_info;
pub mod music_info;
pub mod progress_info;
pub mod quest_info;
pub mod role_info;
pub mod scene_instance;
pub mod special_item_incr;
pub mod weapon_item;
pub mod zon_codec;

pub use account::Account;
pub use basic_info::BasicInfo;
pub use camera_settings::CameraSettings;
pub use cosmetic_info::CosmeticInfo;
pub use echo_info::EchoInfo;
pub use explore_tools_info::ExploreToolsInfo;
pub use formation_info::FormationInfo;
pub use gacha_info::GachaInfo;
pub use guide_info::GuideInfo;
pub use inventory_info::InventoryInfo;
pub use last_scene_info::LastSceneInfo;
pub use mail_info::MailInfo;
pub use map_info::MapInfo;
pub use motor_info::MotorInfo;
pub use music_info::MusicInfo;
pub use progress_info::ProgressInfo;
pub use quest_info::QuestInfo;
pub use role_info::RoleInfo;
pub use scene_instance::SceneInstance;
pub use weapon_item::WeaponItem;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        BasicInfo, CosmeticInfo, EchoInfo, ExploreToolsInfo, FormationInfo, GachaInfo, GuideInfo,
        InventoryInfo, LastSceneInfo, MailInfo, MapInfo, MotorInfo, MusicInfo, ProgressInfo,
        QuestInfo, RoleInfo, WeaponItem, zon_codec,
    };

    #[test]
    fn parses_real_saved_player_zon_files_if_present() {
        let player_dir = Path::new("/root/wuwa-server/state/player/597938510");
        if !player_dir.exists() {
            return;
        }
        let check_file = |name: &str, f: fn(&str)| {
            let p = player_dir.join(name);
            if p.exists() {
                let text = std::fs::read_to_string(&p).unwrap();
                f(&text);
            }
        };

        check_file("basic_info", |s| {
            let v: BasicInfo = zon_codec::from_zon_str(s).unwrap();
            assert!(!v.attributes.name.is_empty());
        });
        check_file("cosmetic_info", |s| {
            let _: CosmeticInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("explore_tools_info", |s| {
            let _: ExploreToolsInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("formation_info", |s| {
            let _: FormationInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("gacha_info", |s| {
            let _: GachaInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("guide_info", |s| {
            let _: GuideInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("inventory_info", |s| {
            let _: InventoryInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("last_scene", |s| {
            let _: LastSceneInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("mail_info", |s| {
            let _: MailInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("map_info", |s| {
            let _: MapInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("motor_info", |s| {
            let _: MotorInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("music_info", |s| {
            let _: MusicInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("progress_info", |s| {
            let _: ProgressInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("quest_info", |s| {
            let _: QuestInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("role/1102", |s| {
            let _: RoleInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("echo/66", |s| {
            let _: EchoInfo = zon_codec::from_zon_str(s).unwrap();
        });
        check_file("weapon/1", |s| {
            let _: WeaponItem = zon_codec::from_zon_str(s).unwrap();
        });
    }
}
