use std::io;
use std::sync::Arc;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::echo_info;
use sonoro_gamesv_fs::special_item_incr;
use sonoro_gamesv_logic::component::player::{
    PlayerBasicComponent, PlayerCosmeticComponent, PlayerEchoComponent, PlayerGachaComponent,
    PlayerGuideComponent, PlayerInventoryComponent, PlayerMailComponent, PlayerMapComponent,
    PlayerMotorComponent, PlayerMusicComponent, PlayerProgressComponent, PlayerQuestComponent,
    PlayerRoleComponent, PlayerSceneComponent, PlayerSettingsComponent, PlayerWeaponComponent,
};
use sonoro_gamesv_logic::coop_service::CoopService;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::handlers::PlayerRuntime;
use sonoro_gamesv_logic::handlers::chat::GmCommandOutput;
use sonoro_gamesv_logic::helpers::condition;
use sonoro_gamesv_logic::helpers::craft::CraftContext;
use sonoro_gamesv_logic::helpers::quest_types::{ActivityNotifyFn, QuestContext};
use sonoro_gamesv_logic::helpers::reward::RewardContext;
use sonoro_gamesv_logic::scene::{Scene, now_millis};
use sonoro_gamesv_logic::schedulers::{BuffTimerScheduler, DirtySaveQueue};

use super::connection::Connection;

pub struct PlayerComponentStorage {
    pub basic: PlayerBasicComponent,
    pub role: PlayerRoleComponent,
    pub weapon: PlayerWeaponComponent,
    pub echo: PlayerEchoComponent,
    pub cosmetic: PlayerCosmeticComponent,
    pub inventory: PlayerInventoryComponent,
    #[allow(dead_code)]
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

        let next_weapon_echo_id = weapon
            .weapon_map
            .keys()
            .copied()
            .chain(echo.echo_map.keys().copied())
            .max()
            .unwrap_or(0)
            + 1;
        let next_special_item_id = next_weapon_echo_id.max(
            echo_info::next_after_preset_groups(&echo.preset_info.groups),
        );
        special_item_incr::ensure_at_least(fs, player_id, next_special_item_id)?;

        Ok(Self {
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

pub struct State {
    pub fs: Arc<FileSystem>,
    pub conn: Connection,
    pub assets: Arc<Assets>,
    pub player_id: i32,
    pub player_components: PlayerComponentStorage,
    pub scene: Option<Scene>,
    pub buff_timers: BuffTimerScheduler,
    pub dirty_saves: DirtySaveQueue,
    pub coop_service: Option<Arc<CoopService>>,
    pub gm_output: GmCommandOutput,
}

impl State {
    pub fn new(
        fs: Arc<FileSystem>,
        conn: Connection,
        assets: Arc<Assets>,
        pcs: PlayerComponentStorage,
        player_id: i32,
    ) -> Self {
        Self {
            fs,
            conn,
            assets,
            player_id,
            player_components: pcs,
            scene: None,
            buff_timers: BuffTimerScheduler::default(),
            dirty_saves: DirtySaveQueue::default(),
            coop_service: None,
            gm_output: GmCommandOutput::default(),
        }
    }

    pub fn cleanup(&mut self) {
        let now_ms = now_millis();
        if let Some(scene) = self.scene.as_ref() {
            if let Err(err) = scene.save_persistent(&self.fs) {
                tracing::error!(
                    "failed to save persistent scene state during session cleanup: {}",
                    err
                );
            }
            if let Err(err) = scene.save_last_scene(&self.player_components.scene, &self.fs) {
                tracing::error!(
                    "failed to save last scene during session cleanup: {}",
                    err
                );
            }
        }
        if let Err(err) = self.dirty_saves.flush_persistent(
            &self.fs,
            &self.player_components.role,
            &self.player_components.weapon,
            now_ms,
        ) {
            tracing::error!(
                "failed to flush persistent dirty saves during session cleanup: {}",
                err
            );
        }
        if let Some(cs) = self.coop_service.as_ref() {
            cs.unregister_online(self.player_id);
        }
        if let Some(scene) = self.scene.as_ref() {
            let _ = scene.clear_entities(&self.fs);
        }
    }

    pub fn as_runtime<'a>(
        &'a mut self,
        events: &'a mut EventQueue,
        activity_notify: Option<ActivityNotifyFn<Connection>>,
    ) -> PlayerRuntime<'a, Connection> {
        PlayerRuntime {
            fs: &self.fs,
            assets: &self.assets,
            sink: &mut self.conn,
            events,
            player_id: self.player_id,
            scene: &mut self.scene,
            buff_timers: &mut self.buff_timers,
            dirty_saves: &mut self.dirty_saves,
            gm_output: &mut self.gm_output,
            basic_comp: &mut self.player_components.basic,
            scene_comp: &mut self.player_components.scene,
            role_comp: &mut self.player_components.role,
            weapon_comp: &mut self.player_components.weapon,
            motor_comp: &mut self.player_components.motor,
            echo_comp: &mut self.player_components.echo,
            cosmetic_comp: &mut self.player_components.cosmetic,
            inventory_comp: &mut self.player_components.inventory,
            map_comp: &mut self.player_components.map,
            quest_comp: &mut self.player_components.quest,
            guide_comp: &mut self.player_components.guide,
            mail_comp: &mut self.player_components.mail,
            progress_comp: &mut self.player_components.progress,
            activity_notify,
        }
    }

    pub fn quest_context<'a>(&'a mut self, events: &'a mut EventQueue) -> QuestContext<'a, Connection> {
        QuestContext {
            assets: &self.assets,
            fs: &self.fs,
            sink: &mut self.conn,
            scene: self.scene.as_mut(),
            scene_comp: Some(&mut self.player_components.scene),
            events,
            basic_comp: &mut self.player_components.basic,
            inventory_comp: &mut self.player_components.inventory,
            weapon_comp: &mut self.player_components.weapon,
            echo_comp: &mut self.player_components.echo,
            map_comp: &mut self.player_components.map,
            quest_comp: &mut self.player_components.quest,
            role_comp: Some(&mut self.player_components.role),
            guide_comp: Some(&mut self.player_components.guide),
            mail_comp: Some(&mut self.player_components.mail),
            progress_comp: Some(&mut self.player_components.progress),
            activity_notify: Some(super::handlers::activity::push_activity_update_notify),
        }
    }

    pub fn reward_context(&mut self) -> RewardContext<'_, Connection> {
        RewardContext {
            fs: &self.fs,
            assets: &self.assets,
            sink: &mut self.conn,
            basic_comp: &mut self.player_components.basic,
            inventory_comp: &mut self.player_components.inventory,
            weapon_comp: &mut self.player_components.weapon,
            echo_comp: &mut self.player_components.echo,
            progress_comp: Some(&mut self.player_components.progress),
        }
    }

    pub fn craft_context(&mut self) -> CraftContext<'_, Connection> {
        CraftContext {
            sink: &mut self.conn,
            fs: &self.fs,
            assets: &self.assets,
            basic_comp: &mut self.player_components.basic,
            inventory_comp: &mut self.player_components.inventory,
            weapon_comp: &mut self.player_components.weapon,
            echo_comp: &mut self.player_components.echo,
        }
    }

    pub fn eval_context(&self) -> condition::EvalContext<'_> {
        condition::EvalContext {
            assets: Some(&self.assets),
            basic_comp: &self.player_components.basic,
            echo_comp: &self.player_components.echo,
            inventory_comp: &self.player_components.inventory,
            map_comp: &self.player_components.map,
            progress_comp: Some(&self.player_components.progress),
            quest_comp: &self.player_components.quest,
            role_comp: Some(&self.player_components.role),
            weapon_comp: Some(&self.player_components.weapon),
            guide_comp: Some(&self.player_components.guide),
            scene: self.scene.as_ref(),
            owner_quest_id: 0,
            owner_is_level_play: false,
            target_role_id: 0,
            self_entity_config_id: 0,
        }
    }
}
