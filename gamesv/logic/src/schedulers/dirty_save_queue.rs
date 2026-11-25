use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::special_item_incr::TRIAL_INCR_MIN;

use crate::component::comp_util;
use crate::component::player::{PlayerRoleComponent, PlayerWeaponComponent};
use crate::event_queue::EventQueue;
use crate::events::{EntityHandle, Event};
use crate::scene::Scene;
use crate::schedulers::BuffTimerScheduler;

const FLUSH_DELAY_MS: i64 = 30_000;

#[derive(Default, Debug)]
pub struct DirtySaveQueue {
    role_ids: Vec<i32>,
    weapon_ids: Vec<i32>,
    position_entity_ids: Vec<i64>,
    buff_entity_ids: Vec<i64>,
    scene_instance_dirty: bool,
    pub flush_due_ms: Option<i64>,
}

impl DirtySaveQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_wake_delay_ms(&self, now_ms: i64) -> Option<i64> {
        let due_ms = self.flush_due_ms?;
        if due_ms <= now_ms {
            return Some(0);
        }
        Some(((due_ms as i128) - (now_ms as i128)).min(i64::MAX as i128) as i64)
    }

    pub fn is_due(&self, now_ms: i64) -> bool {
        self.flush_due_ms.is_some_and(|due| due <= now_ms)
    }

    pub fn enqueue_due(&self, event_queue: &mut EventQueue, now_ms: i64) -> bool {
        if !self.is_due(now_ms) {
            return false;
        }
        event_queue.enqueue(Event::DirtySavesDue { now_ms });
        true
    }

    pub fn mark_role(&mut self, role_id: i32, now_ms: i64) {
        append_unique(&mut self.role_ids, role_id);
        self.arm_if_needed(now_ms);
    }

    pub fn mark_weapon(&mut self, weapon_id: i32, now_ms: i64) {
        append_unique(&mut self.weapon_ids, weapon_id);
        self.arm_if_needed(now_ms);
    }

    pub fn mark_movement(&mut self, entity: EntityHandle, now_ms: i64) {
        self.scene_instance_dirty = true;
        append_unique(&mut self.position_entity_ids, entity.net_id);
        self.arm_if_needed(now_ms);
    }

    pub fn mark_buff_change(&mut self, entity: EntityHandle, now_ms: i64) {
        self.scene_instance_dirty = true;
        append_unique(&mut self.buff_entity_ids, entity.net_id);
        self.arm_if_needed(now_ms);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn flush(
        &mut self,
        fs: &FileSystem,
        role_comp: &PlayerRoleComponent,
        weapon_comp: &PlayerWeaponComponent,
        scene: Option<&mut Scene>,
        buff_timers: &mut BuffTimerScheduler,
        assets: &Assets,
        now_ms: i64,
    ) -> io::Result<()> {
        if !self.has_pending() {
            self.flush_due_ms = None;
            return Ok(());
        }
        let res = self.flush_pending(
            fs,
            role_comp,
            weapon_comp,
            scene,
            buff_timers,
            assets,
            now_ms,
        );
        if let Err(err) = res {
            self.schedule_retry(now_ms);
            return Err(err);
        }
        self.clear_retaining_capacity();
        Ok(())
    }

    pub fn flush_persistent(
        &mut self,
        fs: &FileSystem,
        role_comp: &PlayerRoleComponent,
        weapon_comp: &PlayerWeaponComponent,
        now_ms: i64,
    ) -> io::Result<()> {
        if (!self.role_ids.is_empty() || !self.weapon_ids.is_empty())
            && let Err(err) = self.flush_roles_and_weapons(fs, role_comp, weapon_comp)
        {
            self.schedule_retry(now_ms);
            return Err(err);
        }
        self.clear_retaining_capacity();
        Ok(())
    }

    fn flush_roles_and_weapons(
        &self,
        fs: &FileSystem,
        role_comp: &PlayerRoleComponent,
        weapon_comp: &PlayerWeaponComponent,
    ) -> io::Result<()> {
        for &role_id in &self.role_ids {
            if role_comp.is_trial_role(role_id) {
                continue;
            }
            if let Some(role_info) = role_comp.role_map.get(&role_id) {
                let path = format!("player/{}/role/{role_id}", role_comp.player_id);
                comp_util::save_struct(fs, role_info, &path)?;
            }
        }

        for &weapon_id in &self.weapon_ids {
            if weapon_id >= TRIAL_INCR_MIN {
                continue;
            }
            if let Some(weapon_info) = weapon_comp.weapon_map.get(&weapon_id) {
                let path = format!("player/{}/weapon/{weapon_id}", weapon_comp.player_id);
                comp_util::save_struct(fs, weapon_info, &path)?;
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn flush_pending(
        &self,
        fs: &FileSystem,
        role_comp: &PlayerRoleComponent,
        weapon_comp: &PlayerWeaponComponent,
        scene: Option<&mut Scene>,
        buff_timers: &mut BuffTimerScheduler,
        assets: &Assets,
        now_ms: i64,
    ) -> io::Result<()> {
        self.flush_roles_and_weapons(fs, role_comp, weapon_comp)?;

        if let Some(scene_ref) = scene {
            if self.scene_instance_dirty {
                scene_ref.save_instance(fs)?;
            }
            for &entity_id in &self.position_entity_ids {
                scene_ref.save_entity_by_net_id(fs, entity_id)?;
            }
            for &entity_id in &self.buff_entity_ids {
                buff_timers.ensure_entity_registered(scene_ref, assets, entity_id, now_ms);
            }
            for &entity_id in &self.buff_entity_ids {
                buff_timers.sync_entity_left_durations(scene_ref, entity_id, now_ms);
                scene_ref.save_entity_by_net_id(fs, entity_id)?;
            }
        }
        Ok(())
    }

    pub fn save_all_buffs(
        fs: &FileSystem,
        scene: &mut Scene,
        buff_timers: &mut BuffTimerScheduler,
        assets: &Assets,
        now_ms: i64,
    ) -> io::Result<()> {
        buff_timers.ensure_all_registered(scene, assets, now_ms);
        buff_timers.sync_all_left_durations(scene, now_ms);

        for entity in &scene.entities {
            if entity.buffs.is_some() {
                entity.save(fs, scene.player_id, scene.instance_id)?;
            }
        }
        Ok(())
    }

    pub fn has_pending(&self) -> bool {
        self.scene_instance_dirty
            || !self.role_ids.is_empty()
            || !self.weapon_ids.is_empty()
            || !self.position_entity_ids.is_empty()
            || !self.buff_entity_ids.is_empty()
    }

    pub fn clear_retaining_capacity(&mut self) {
        self.role_ids.clear();
        self.weapon_ids.clear();
        self.position_entity_ids.clear();
        self.buff_entity_ids.clear();
        self.scene_instance_dirty = false;
        self.flush_due_ms = None;
    }

    fn arm_if_needed(&mut self, now_ms: i64) {
        if self.flush_due_ms.is_none() && self.has_pending() {
            self.flush_due_ms = Some(deadline_after(now_ms));
        }
    }

    fn schedule_retry(&mut self, now_ms: i64) {
        self.flush_due_ms = Some(deadline_after(now_ms));
    }
}

fn deadline_after(now_ms: i64) -> i64 {
    ((now_ms as i128) + (FLUSH_DELAY_MS as i128)).min(i64::MAX as i128) as i64
}

fn append_unique<T: PartialEq>(list: &mut Vec<T>, value: T) {
    if !list.contains(&value) {
        list.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dirty_save_deadline_starts_on_first_mutation_and_is_not_postponed() {
        let mut queue = DirtySaveQueue::new();
        assert_eq!(queue.next_wake_delay_ms(100), None);
        assert!(!queue.is_due(100));

        queue.mark_role(1, 100);
        assert_eq!(queue.next_wake_delay_ms(100), Some(30_000));
        assert_eq!(queue.next_wake_delay_ms(30_099), Some(1));
        assert_eq!(queue.next_wake_delay_ms(30_100), Some(0));
        assert!(queue.is_due(30_100));

        queue.mark_weapon(2, 10_000);
        assert_eq!(queue.next_wake_delay_ms(10_000), Some(20_100));
    }

    #[test]
    fn all_dirty_paths_arm_one_deadline_and_successful_completion_clears_it() {
        let mut queue = DirtySaveQueue::new();
        queue.mark_movement(
            EntityHandle {
                index: 0,
                net_id: 10,
            },
            1_000,
        );
        queue.mark_buff_change(
            EntityHandle {
                index: 0,
                net_id: 20,
            },
            2_000,
        );
        queue.mark_role(3, 3_000);
        queue.mark_weapon(4, 4_000);

        assert!(queue.has_pending());
        assert_eq!(queue.flush_due_ms, Some(31_000));
        queue.clear_retaining_capacity();
        assert!(!queue.has_pending());
        assert_eq!(queue.flush_due_ms, None);
    }

    #[test]
    fn failed_completion_retains_dirty_state_and_schedules_stable_retry() {
        let mut queue = DirtySaveQueue::new();
        queue.mark_role(1, 100);
        queue.schedule_retry(40_000);
        assert!(queue.has_pending());
        assert_eq!(queue.flush_due_ms, Some(70_000));

        queue.mark_weapon(2, 50_000);
        assert_eq!(queue.flush_due_ms, Some(70_000));
    }

    #[test]
    fn due_dirty_save_enqueues_one_timestamped_event() {
        let mut queue = DirtySaveQueue::new();
        let mut event_queue = EventQueue::new();

        assert!(!queue.enqueue_due(&mut event_queue, 100));
        queue.mark_role(1, 100);
        assert!(!queue.enqueue_due(&mut event_queue, 30_099));
        assert!(queue.enqueue_due(&mut event_queue, 30_100));

        assert!(matches!(
            event_queue.dequeue(),
            Some(Event::DirtySavesDue { now_ms: 30_100 })
        ));
        assert!(event_queue.dequeue().is_none());
    }

    #[test]
    fn dirty_save_deadlines_clamp_without_wrapping() {
        assert_eq!(deadline_after(i64::MAX), i64::MAX);
        let queue = DirtySaveQueue {
            flush_due_ms: Some(i64::MAX),
            ..Default::default()
        };
        assert_eq!(queue.next_wake_delay_ms(i64::MIN), Some(i64::MAX));
    }
}
