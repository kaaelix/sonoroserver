pub mod echo;
pub mod role;
pub mod weapon;

use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::zon_config::GameConfig;
use sonoro_gamesv_fs::basic_info::BasicInfo;
use sonoro_gamesv_fs::camera_settings::CameraSettings;
use sonoro_gamesv_fs::cosmetic_info::CosmeticInfo;
use sonoro_gamesv_fs::echo_info;
use sonoro_gamesv_fs::file_util::{load_or_create_zon, load_zon};
use sonoro_gamesv_fs::gacha_info::GachaInfo;
use sonoro_gamesv_fs::guide_info::GuideInfo;
use sonoro_gamesv_fs::inventory_info::InventoryInfo;
use sonoro_gamesv_fs::last_scene_info::LastSceneInfo;
use sonoro_gamesv_fs::mail_info::MailInfo;
use sonoro_gamesv_fs::map_info::MapInfo;
use sonoro_gamesv_fs::motor_info::MotorInfo;
use sonoro_gamesv_fs::music_info::MusicInfo;
use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_gamesv_fs::quest_info::QuestInfo;
use sonoro_gamesv_fs::special_item_incr;

use crate::component::comp_util;
use crate::helpers::inventory as inventory_helper;

pub use echo::PlayerEchoComponent;
pub use role::PlayerRoleComponent;
pub use weapon::PlayerWeaponComponent;

#[derive(Debug, Default)]
pub struct PlayerBasicComponent {
    pub player_id: i32,
    pub info: BasicInfo,
}

impl PlayerBasicComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let info = load_or_create_zon::<BasicInfo>(fs, &format!("player/{player_id}/basic_info"))?;
        Ok(Self { player_id, info })
    }
}

#[derive(Debug, Default)]
pub struct PlayerCosmeticComponent {
    pub player_id: i32,
    pub info: CosmeticInfo,
}

impl PlayerCosmeticComponent {
    pub fn init(fs: &FileSystem, assets: &Assets, player_id: i32) -> io::Result<Self> {
        let mut info =
            load_zon::<CosmeticInfo>(fs, &format!("player/{player_id}/{}", CosmeticInfo::DATA_PATH))?
                .unwrap_or_default();
        if info.is_empty() {
            info.add_defaults(assets);
            Self::save(fs, player_id, &info)?;
        } else if info.ensure_default_phantom_skins(assets) {
            Self::save(fs, player_id, &info)?;
        }
        Ok(Self { player_id, info })
    }

    pub fn save(fs: &FileSystem, player_id: i32, info: &CosmeticInfo) -> io::Result<()> {
        comp_util::save_struct(fs, info, &format!("player/{player_id}/{}", CosmeticInfo::DATA_PATH))
    }
}

#[derive(Debug, Default)]
pub struct PlayerGachaComponent {
    pub player_id: i32,
    pub info: GachaInfo,
}

impl PlayerGachaComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let info =
            load_or_create_zon::<GachaInfo>(fs, &format!("player/{player_id}/{}", GachaInfo::DATA_PATH))?;
        Ok(Self { player_id, info })
    }
}

#[derive(Debug, Default)]
pub struct PlayerGuideComponent {
    pub player_id: i32,
    pub info: GuideInfo,
}

impl PlayerGuideComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let info =
            load_or_create_zon::<GuideInfo>(fs, &format!("player/{player_id}/{}", GuideInfo::DATA_PATH))?;
        Ok(Self { player_id, info })
    }
}

#[derive(Debug, Default)]
pub struct PlayerInventoryComponent {
    pub player_id: i32,
    pub info: InventoryInfo,
}

impl PlayerInventoryComponent {
    pub fn init(fs: &FileSystem, assets: &Assets, player_id: i32) -> io::Result<Self> {
        let path = format!("player/{player_id}/{}", InventoryInfo::DATA_PATH);
        let mut info = load_zon::<InventoryInfo>(fs, &path)?.unwrap_or_default();
        let prev_len = info.normal_items.len();
        let was_empty = info.is_empty();
        if !GameConfig::embedded().normal_progression
            && (was_empty || info.normal_item_count(43010001) == 0)
        {
            inventory_helper::add_default_progression_items(&mut info, assets);
            if prev_len != info.normal_items.len() || was_empty {
                comp_util::save_struct(fs, &info, &path)?;
            }
        }
        Ok(Self { player_id, info })
    }
}

#[derive(Debug, Default)]
pub struct PlayerMailComponent {
    pub player_id: i32,
    pub info: MailInfo,
}

impl PlayerMailComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let info =
            load_or_create_zon::<MailInfo>(fs, &format!("player/{player_id}/{}", MailInfo::DATA_PATH))?;
        Ok(Self { player_id, info })
    }

    pub fn save(&self, fs: &FileSystem) -> io::Result<()> {
        comp_util::save_struct(
            fs,
            &self.info,
            &format!("player/{}/{}", self.player_id, MailInfo::DATA_PATH),
        )
    }
}

#[derive(Debug, Default)]
pub struct PlayerMapComponent {
    pub player_id: i32,
    pub info: MapInfo,
}

impl PlayerMapComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let info =
            load_or_create_zon::<MapInfo>(fs, &format!("player/{player_id}/{}", MapInfo::DATA_PATH))?;
        Ok(Self { player_id, info })
    }

    pub fn save(&self, fs: &FileSystem) -> io::Result<()> {
        comp_util::save_struct(
            fs,
            &self.info,
            &format!("player/{}/{}", self.player_id, MapInfo::DATA_PATH),
        )
    }
}

#[derive(Debug, Default)]
pub struct PlayerMotorComponent {
    pub player_id: i32,
    pub info: MotorInfo,
}

impl PlayerMotorComponent {
    pub fn init(fs: &FileSystem, assets: &Assets, player_id: i32) -> io::Result<Self> {
        let path = format!("player/{player_id}/{}", MotorInfo::DATA_PATH);
        let mut info = load_zon::<MotorInfo>(fs, &path)?.unwrap_or_default();
        let mut should_save = false;
        if info.is_empty() {
            info.add_defaults(assets);
            should_save = true;
        }
        if info.scene_in_use == 0 {
            info.scene_in_use = MotorInfo::default_scene_id(assets);
            should_save |= info.scene_in_use != 0;
        }
        if should_save {
            comp_util::save_struct(fs, &info, &path)?;
        }
        Ok(Self { player_id, info })
    }

    pub fn save(&self, fs: &FileSystem) -> io::Result<()> {
        comp_util::save_struct(
            fs,
            &self.info,
            &format!("player/{}/{}", self.player_id, MotorInfo::DATA_PATH),
        )
    }
}

#[derive(Debug, Default)]
pub struct PlayerMusicComponent {
    pub player_id: i32,
    pub info: MusicInfo,
}

impl PlayerMusicComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let info =
            load_or_create_zon::<MusicInfo>(fs, &format!("player/{player_id}/{}", MusicInfo::DATA_PATH))?;
        Ok(Self { player_id, info })
    }

    pub fn save(&self, fs: &FileSystem) -> io::Result<()> {
        comp_util::save_struct(
            fs,
            &self.info,
            &format!("player/{}/{}", self.player_id, MusicInfo::DATA_PATH),
        )
    }
}

#[derive(Debug, Default)]
pub struct PlayerProgressComponent {
    pub player_id: i32,
    pub info: ProgressInfo,
}

impl PlayerProgressComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let info = load_or_create_zon::<ProgressInfo>(
            fs,
            &format!("player/{player_id}/{}", ProgressInfo::DATA_PATH),
        )?;
        Ok(Self { player_id, info })
    }

    pub fn save(&self, fs: &FileSystem) -> io::Result<()> {
        comp_util::save_struct(
            fs,
            &self.info,
            &format!("player/{}/{}", self.player_id, ProgressInfo::DATA_PATH),
        )
    }
}

/// Ephemeral login restore state for save blocks (never persisted).
#[derive(Debug, Default)]
pub struct LoginRestore {
    pub reenter_quests: Vec<i32>,
    pub pos_rollback: Option<crate::helpers::save_block::PosRollback>,
}

#[derive(Debug, Default)]
pub struct PlayerQuestComponent {
    pub player_id: i32,
    pub info: QuestInfo,
    pub login_restore: LoginRestore,
}

impl PlayerQuestComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let info =
            load_or_create_zon::<QuestInfo>(fs, &format!("player/{player_id}/{}", QuestInfo::DATA_PATH))?;
        Ok(Self {
            player_id,
            info,
            login_restore: LoginRestore::default(),
        })
    }

    pub fn save(&self, fs: &FileSystem) -> io::Result<()> {
        comp_util::save_struct(
            fs,
            &self.info,
            &format!("player/{}/{}", self.player_id, QuestInfo::DATA_PATH),
        )
    }
}

#[derive(Debug, Default)]
pub struct PlayerSceneComponent {
    pub player_id: i32,
    pub last_scene_info: LastSceneInfo,
}

impl PlayerSceneComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let last_scene_info =
            load_or_create_zon::<LastSceneInfo>(fs, &format!("player/{player_id}/last_scene"))?;
        Ok(Self {
            player_id,
            last_scene_info,
        })
    }
}

#[derive(Debug, Default)]
pub struct PlayerSettingsComponent {
    pub player_id: i32,
    pub camera: CameraSettings,
}

impl PlayerSettingsComponent {
    pub fn init(fs: &FileSystem, player_id: i32) -> io::Result<Self> {
        let camera =
            load_or_create_zon::<CameraSettings>(fs, &format!("player/{player_id}/settings/camera"))?;
        Ok(Self { player_id, camera })
    }

    pub fn save(&self, fs: &FileSystem) -> io::Result<()> {
        comp_util::save_struct(
            fs,
            &self.camera,
            &format!("player/{}/settings/camera", self.player_id),
        )
    }
}

#[derive(Debug)]
pub struct PlayerComponentStorage {
    pub player_id: i32,
    pub basic: PlayerBasicComponent,
    pub role: PlayerRoleComponent,
    pub weapon: PlayerWeaponComponent,
    pub echo: PlayerEchoComponent,
    pub cosmetic: PlayerCosmeticComponent,
    pub inventory: PlayerInventoryComponent,
    pub gacha: PlayerGachaComponent,
    pub guide: PlayerGuideComponent,
    pub map: PlayerMapComponent,
    pub music: PlayerMusicComponent,
    pub quest: PlayerQuestComponent,
    pub mail: PlayerMailComponent,
    pub scene: PlayerSceneComponent,
    pub motor: PlayerMotorComponent,
    pub settings: PlayerSettingsComponent,
    pub progress: PlayerProgressComponent,
}

impl PlayerComponentStorage {
    pub fn init(fs: &FileSystem, assets: &Assets, player_id: i32) -> io::Result<Self> {
        let role = PlayerRoleComponent::init(fs, assets, player_id)?;
        let weapon = PlayerWeaponComponent::init(fs, assets, player_id)?;
        let echo = PlayerEchoComponent::init(fs, assets, player_id)?;

        let next_special_item_id =
            special_item_incr::next_after_maps(&weapon.weapon_map, &echo.echo_map)
                .max(echo_info::next_after_preset_groups(&echo.preset_info.groups));
        special_item_incr::ensure_at_least(fs, player_id, next_special_item_id)?;

        Ok(Self {
            player_id,
            basic: PlayerBasicComponent::init(fs, player_id)?,
            role,
            weapon,
            echo,
            cosmetic: PlayerCosmeticComponent::init(fs, assets, player_id)?,
            inventory: PlayerInventoryComponent::init(fs, assets, player_id)?,
            gacha: PlayerGachaComponent::init(fs, player_id)?,
            guide: PlayerGuideComponent::init(fs, player_id)?,
            map: PlayerMapComponent::init(fs, player_id)?,
            music: PlayerMusicComponent::init(fs, player_id)?,
            quest: PlayerQuestComponent::init(fs, player_id)?,
            mail: PlayerMailComponent::init(fs, player_id)?,
            scene: PlayerSceneComponent::init(fs, player_id)?,
            motor: PlayerMotorComponent::init(fs, assets, player_id)?,
            settings: PlayerSettingsComponent::init(fs, player_id)?,
            progress: PlayerProgressComponent::init(fs, player_id)?,
        })
    }
}
