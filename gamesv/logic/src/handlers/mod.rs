pub mod buff;
pub mod chat;
pub mod debugger;
pub mod fsm;
pub mod misc;
pub mod phase;
pub mod player;
pub mod role;
pub mod save;
pub mod scene;
pub mod tags;
pub mod weapon;

use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;

use crate::component::player::{
    PlayerBasicComponent, PlayerCosmeticComponent, PlayerEchoComponent, PlayerGuideComponent,
    PlayerInventoryComponent, PlayerMailComponent, PlayerMapComponent, PlayerMotorComponent,
    PlayerProgressComponent, PlayerQuestComponent, PlayerRoleComponent, PlayerSceneComponent,
    PlayerWeaponComponent,
};
use crate::event_queue::EventQueue;
use crate::events::Event;
use crate::helpers::quest_types::{ActivityNotifyFn, QuestContext};
use crate::scene::{Scene, now_millis};
use crate::schedulers::{BuffTimerScheduler, DirtySaveQueue};
use crate::sink::PushSink;

pub struct PlayerRuntime<'a, S: PushSink + ?Sized> {
    pub fs: &'a FileSystem,
    pub assets: &'a Assets,
    pub sink: &'a mut S,
    pub events: &'a mut EventQueue,
    pub player_id: i32,
    pub scene: &'a mut Option<Scene>,
    pub buff_timers: &'a mut BuffTimerScheduler,
    pub dirty_saves: &'a mut DirtySaveQueue,
    pub gm_output: &'a mut chat::GmCommandOutput,
    pub basic_comp: &'a mut PlayerBasicComponent,
    pub scene_comp: &'a mut PlayerSceneComponent,
    pub role_comp: &'a mut PlayerRoleComponent,
    pub weapon_comp: &'a mut PlayerWeaponComponent,
    pub motor_comp: &'a mut PlayerMotorComponent,
    pub echo_comp: &'a mut PlayerEchoComponent,
    pub cosmetic_comp: &'a mut PlayerCosmeticComponent,
    pub inventory_comp: &'a mut PlayerInventoryComponent,
    pub map_comp: &'a mut PlayerMapComponent,
    pub quest_comp: &'a mut PlayerQuestComponent,
    pub guide_comp: &'a mut PlayerGuideComponent,
    pub mail_comp: &'a mut PlayerMailComponent,
    pub progress_comp: &'a mut PlayerProgressComponent,
    pub activity_notify: Option<ActivityNotifyFn<S>>,
}

impl<'a, S: PushSink + ?Sized> PlayerRuntime<'a, S> {
    pub fn as_quest_ctx(&mut self) -> QuestContext<'_, S> {
        QuestContext {
            assets: self.assets,
            fs: self.fs,
            sink: self.sink,
            scene: self.scene.as_mut(),
            scene_comp: Some(self.scene_comp),
            events: self.events,
            basic_comp: self.basic_comp,
            inventory_comp: self.inventory_comp,
            weapon_comp: self.weapon_comp,
            echo_comp: self.echo_comp,
            map_comp: self.map_comp,
            quest_comp: self.quest_comp,
            role_comp: Some(self.role_comp),
            guide_comp: Some(self.guide_comp),
            mail_comp: Some(self.mail_comp),
            progress_comp: Some(self.progress_comp),
            activity_notify: self.activity_notify,
        }
    }
}

pub fn dispatch_event<S: PushSink + ?Sized>(
    rt: &mut PlayerRuntime<'_, S>,
    event: Event,
) -> io::Result<()> {
    match event {
        Event::EnterGame { .. } => {
            role::ensure_role_attributes(rt.events, rt.assets, rt.role_comp);
            weapon::ensure_role_weapons(
                rt.events,
                rt.fs,
                rt.assets,
                rt.role_comp,
                rt.weapon_comp,
            )?;
            let def_inst = rt.scene_comp.last_scene_info.instance_id;
            phase::run_enter_game_phase(
                rt.events,
                rt.player_id,
                0,
                def_inst,
                0.0,
                0.0,
                0.0,
                0.0,
                true,
                None,
            );
        }
        Event::PushData { .. } => {
            player::push_data(
                rt.sink,
                rt.player_id,
                rt.assets,
                rt.basic_comp,
                rt.role_comp,
            );
            role::push_data(
                rt.sink,
                rt.assets,
                rt.basic_comp,
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                rt.cosmetic_comp,
                rt.inventory_comp,
                rt.quest_comp,
                rt.progress_comp,
            );
            misc::push_data(
                rt.sink,
                rt.fs,
                rt.assets,
                rt.role_comp,
                rt.echo_comp,
                rt.cosmetic_comp,
                rt.map_comp,
                rt.basic_comp,
                rt.inventory_comp,
                rt.quest_comp,
                rt.mail_comp,
                rt.progress_comp,
                rt.activity_notify,
            )?;
        }
        Event::PushDataComplete { .. } => {
            let mut qctx = rt.as_quest_ctx();
            phase::notify_push_data_complete(&mut qctx)?;
        }
        Event::ReconnectResync { .. } => {
            let mut qctx = rt.as_quest_ctx();
            misc::handle_reconnect_resync(&mut qctx)?;
        }
        Event::ConditionStateRefresh { .. } => {
            let mut qctx = rt.as_quest_ctx();
            misc::handle_condition_state_refresh(&mut qctx)?;
        }
        Event::InitialSceneJoin {
            instance_id,
            player_id,
            location,
            rotation,
            pending_flow,
            ..
        } => {
            scene::on_initial_scene_join(
                (instance_id != 0).then_some(instance_id),
                location,
                rotation,
                pending_flow,
                rt.events,
                rt.fs,
                player_id,
                rt.assets,
                rt.scene_comp,
                rt.basic_comp,
                rt.role_comp,
                rt.weapon_comp,
                rt.echo_comp,
                rt.quest_comp,
                rt.scene,
                rt.buff_timers,
                rt.dirty_saves,
            )?;
        }
        Event::SceneSwitch {
            player_id,
            pending_flow,
            ..
        } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                scene::notify_join_scene(
                    pending_flow,
                    rt.events,
                    rt.fs,
                    rt.assets,
                    rt.sink,
                    player_id,
                    rt.scene_comp,
                    rt.basic_comp,
                    rt.role_comp,
                    rt.weapon_comp,
                    rt.motor_comp,
                    rt.echo_comp,
                    rt.inventory_comp,
                    rt.map_comp,
                    rt.quest_comp,
                    rt.progress_comp,
                    active_scene,
                    rt.buff_timers,
                )?;
            }
        }
        Event::AfterSceneJoin {
            player_id,
            pending_flow,
            ..
        } => {
            let fs = rt.fs;
            let events = std::mem::take(rt.events);
            let mut qctx = QuestContext {
                assets: rt.assets,
                fs: rt.fs,
                sink: rt.sink,
                scene: rt.scene.as_mut(),
                scene_comp: Some(rt.scene_comp),
                events: rt.events,
                basic_comp: rt.basic_comp,
                inventory_comp: rt.inventory_comp,
                weapon_comp: rt.weapon_comp,
                echo_comp: rt.echo_comp,
                map_comp: rt.map_comp,
                quest_comp: rt.quest_comp,
                role_comp: Some(rt.role_comp),
                guide_comp: Some(rt.guide_comp),
                mail_comp: Some(rt.mail_comp),
                progress_comp: Some(rt.progress_comp),
                activity_notify: rt.activity_notify,
            };
            *qctx.events = events;
            let mut local_events = EventQueue::new();
            std::mem::swap(qctx.events, &mut local_events);
            scene::after_scene_join(
                pending_flow.as_ref(),
                &mut local_events,
                fs,
                &mut qctx,
                player_id,
            )?;
            while let Some(ev) = rt.events.dequeue() {
                local_events.enqueue(ev);
            }
            *rt.events = local_events;
        }
        Event::UpdateFormations { .. } => {
            if let Some(active_scene) = rt.scene.as_ref() {
                scene::formation_update_notify(rt.sink, active_scene, rt.role_comp, rt.weapon_comp);
            }
        }
        Event::RoleInfoModified { role_id, .. } => {
            save::on_role_info_modified(rt.dirty_saves, role_id);
        }
        Event::WeaponInfoModified { incr_id, .. } => {
            save::on_weapon_info_modified(rt.dirty_saves, incr_id);
        }
        Event::EntityMovement { entity_net_id, .. } => {
            if let Some(active_scene) = rt.scene.as_ref()
                && let Some(handle) = active_scene.entity_handle(entity_net_id)
            {
                debugger::log_entity_movement(active_scene, handle);
                save::on_entity_movement(rt.dirty_saves, handle);
            }
        }
        Event::BuffRemovalByFsmBind { entity, source } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                buff::remove_buff_from_entity_by_fsm_bind(
                    rt.sink,
                    rt.events,
                    active_scene,
                    rt.assets,
                    rt.buff_timers,
                    entity,
                    source,
                );
            }
        }
        Event::BuffRemoval {
            entity,
            handle_ids,
            natural_expiration,
        } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                buff::remove_buff_from_entity(
                    rt.sink,
                    rt.events,
                    active_scene,
                    rt.assets,
                    rt.buff_timers,
                    entity,
                    &handle_ids,
                    natural_expiration.as_ref(),
                );
            }
        }
        Event::BuffRemovalById { entity, buff_id } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                buff::remove_buff_from_entity_by_id(
                    rt.sink,
                    rt.events,
                    active_scene,
                    rt.assets,
                    rt.buff_timers,
                    entity,
                    buff_id,
                );
            }
        }
        Event::BuffAddition {
            target,
            instigator,
            buffs,
        } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                buff::add_buff_to_entity(
                    rt.sink,
                    rt.events,
                    active_scene,
                    rt.buff_timers,
                    rt.assets,
                    target,
                    instigator,
                    &buffs,
                );
            }
        }
        Event::BuffChange { entity, .. } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                fsm::handle_fsm_buff_change(active_scene, entity);
            }
            save::on_buff_change(rt.dirty_saves, entity);
        }
        Event::BuffTimersDue { now_ms } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                buff::handle_buff_timers_due(
                    rt.sink,
                    rt.events,
                    rt.buff_timers,
                    active_scene,
                    rt.assets,
                    now_ms,
                )
                .map_err(io::Error::other)?;
            }
        }
        Event::FsmWake { now_ms, .. } => {
            let assets = rt.assets;
            let fs = rt.fs;
            let buff_timers = &mut *rt.buff_timers;
            let mut tmp_role_comp = std::mem::take(rt.role_comp);
            let mut qctx = QuestContext {
                assets: rt.assets,
                fs: rt.fs,
                sink: rt.sink,
                scene: rt.scene.as_mut(),
                scene_comp: Some(rt.scene_comp),
                events: rt.events,
                basic_comp: rt.basic_comp,
                inventory_comp: rt.inventory_comp,
                weapon_comp: rt.weapon_comp,
                echo_comp: rt.echo_comp,
                map_comp: rt.map_comp,
                quest_comp: rt.quest_comp,
                role_comp: None,
                guide_comp: Some(rt.guide_comp),
                mail_comp: Some(rt.mail_comp),
                progress_comp: Some(rt.progress_comp),
                activity_notify: rt.activity_notify,
            };
            fsm::handle_fsm_wake(now_ms, &mut qctx, assets, &mut tmp_role_comp, fs, buff_timers);
            *rt.role_comp = tmp_role_comp;
        }
        Event::FsmServerAction { entity, kind } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                fsm::handle_fsm_server_action(
                    entity,
                    &kind,
                    active_scene,
                    rt.assets,
                    rt.sink,
                    rt.events,
                    rt.buff_timers,
                );
            }
        }
        Event::FsmLifecycleComplete {
            entity,
            now_ms,
            recheck,
        } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                fsm::handle_fsm_lifecycle_complete(
                    entity,
                    now_ms,
                    recheck,
                    active_scene,
                    rt.assets,
                    rt.sink,
                    rt.events,
                );
            }
        }
        Event::LevelPlayTimersDue { now_ms } => {
            let mut qctx = rt.as_quest_ctx();
            phase::on_level_play_timers_due(&mut qctx, now_ms)?;
        }
        Event::QuestTimersDue { now_ms } => {
            let mut qctx = rt.as_quest_ctx();
            phase::on_quest_timers_due(&mut qctx, now_ms)?;
        }
        Event::ScenePendingUpdates => {
            if let Some(active_scene) = rt.scene.as_mut() {
                scene::handle_scene_pending_updates(now_millis(), active_scene, rt.fs, rt.sink);
            }
        }
        Event::DirtySavesDue { now_ms } => {
            save::on_dirty_saves_due(
                now_ms,
                rt.fs,
                rt.dirty_saves,
                rt.role_comp,
                rt.weapon_comp,
                rt.scene.as_mut(),
                rt.buff_timers,
                rt.assets,
            )?;
        }
        Event::GameplayTagChange {
            entity,
            add_tag_ids,
            remove_tag_ids,
            remove_before_add,
        } => {
            if let Some(active_scene) = rt.scene.as_ref() {
                tags::update_gameplay_tags(
                    rt.sink,
                    active_scene,
                    rt.assets,
                    entity,
                    &add_tag_ids,
                    &remove_tag_ids,
                    remove_before_add,
                );
            }
            let mut qctx = rt.as_quest_ctx();
            fsm::handle_fsm_gameplay_tag_change(&mut qctx, entity, &add_tag_ids, &remove_tag_ids);
        }
        Event::ChatCommandResponse { message, .. } => {
            chat::chat_command_response(rt.gm_output, &message);
        }
        Event::SpatialSpawnsCheck { .. } => {
            if let Some(active_scene) = rt.scene.as_mut() {
                let loc = active_scene
                    .instance
                    .players
                    .first()
                    .map_or(active_scene.last_spatial_check_loc, |p| p.location);
                scene::handle_spatial_spawns_check(
                    loc,
                    rt.events,
                    active_scene,
                    Some(rt.scene_comp),
                    rt.fs,
                    rt.assets,
                    rt.sink,
                    rt.basic_comp,
                    rt.inventory_comp,
                    rt.weapon_comp,
                    rt.echo_comp,
                    rt.map_comp,
                    rt.quest_comp,
                    Some(rt.guide_comp),
                    Some(rt.mail_comp),
                    rt.progress_comp,
                    rt.role_comp,
                    rt.activity_notify,
                )?;
            }
        }
    }
    Ok(())
}
