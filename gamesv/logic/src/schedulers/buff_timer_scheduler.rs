use std::collections::HashMap;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::buff::{
    Buff, DurationPolicy, PeriodicInhibitionPolicy, StackDurationRefreshPolicy,
    StackPeriodResetPolicy,
};
use sonoro_proto::pb::{
    BuffStackCountNotify, BuffStackCountNotifyGFsOneof, BuffStackCountNotifyTimeOneof,
    CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof, CombatReceiveData,
    CombatReceiveDataMessageOneof, FightBuffInformation,
};

use crate::event_queue::EventQueue;
use crate::events::{BuffAdditionEntry, EntityHandle, Event, NaturalExpiration};
use crate::helpers::buff as buff_helper;
use crate::scene::Scene;

const FAILURE_RETRY_DELAY_MS: i64 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BuffTimerKind {
    Expiry = 0,
    PeriodicPulse = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuffTimerEntry {
    pub kind: BuffTimerKind,
    pub entity_id: i64,
    pub handle_id: i32,
    pub due_ms: i64,
    pub interval_ms: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplicationDisposition {
    pub refresh_duration: bool,
    pub reset_period: bool,
    pub execute_periodic_now: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PeriodicDisposition {
    execute_effect: bool,
    next_due_ms: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpirationDisposition {
    Remove,
    Retain(i32),
}

#[derive(Default, Debug)]
pub struct BuffTimerScheduler {
    heap: Vec<BuffTimerEntry>,
    positions: HashMap<u128, usize>,
    pub initialized: bool,
    pub rebuild_due_ms: Option<i64>,
}

impl BuffTimerScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.clear();
    }

    pub fn upsert(&mut self, entry: BuffTimerEntry) {
        let entry_key = key(entry.entity_id, entry.handle_id, entry.kind);
        if let Some(&index) = self.positions.get(&entry_key) {
            let previous_due = self.heap[index].due_ms;
            self.heap[index] = entry;
            if entry.due_ms < previous_due {
                self.sift_up(index);
            } else if entry.due_ms > previous_due {
                self.sift_down(index);
            }
            return;
        }

        let index = self.heap.len();
        self.heap.push(entry);
        self.positions.insert(entry_key, index);
        self.sift_up(index);
    }

    pub fn cancel(&mut self, entity_id: i64, handle_id: i32, kind: BuffTimerKind) {
        if let Some(&index) = self.positions.get(&key(entity_id, handle_id, kind)) {
            let _ = self.remove_at(index);
        }
    }

    pub fn cancel_handle(&mut self, entity_id: i64, handle_id: i32) {
        self.cancel(entity_id, handle_id, BuffTimerKind::Expiry);
        self.cancel(entity_id, handle_id, BuffTimerKind::PeriodicPulse);
    }

    pub fn cancel_entity(&mut self, entity_id: i64) {
        let mut index = 0;
        while index < self.heap.len() {
            if self.heap[index].entity_id == entity_id {
                let _ = self.remove_at(index);
            } else {
                index += 1;
            }
        }
    }

    pub fn peek_due_ms(&self) -> Option<i64> {
        self.heap.first().map(|e| e.due_ms)
    }

    pub fn next_wake_delay_ms(&self, now_ms: i64) -> Option<i64> {
        if !self.initialized {
            return Some(
                self.rebuild_due_ms
                    .map_or(0, |due| deadline_delay_ms(due, now_ms)),
            );
        }
        self.peek_due_ms().map(|due| deadline_delay_ms(due, now_ms))
    }

    fn peek_due_entry(&self, now_ms: i64) -> Option<BuffTimerEntry> {
        let first = *self.heap.first()?;
        (first.due_ms <= now_ms).then_some(first)
    }

    pub fn pop_due(&mut self, now_ms: i64) -> Option<BuffTimerEntry> {
        let _ = self.peek_due_entry(now_ms)?;
        Some(self.remove_at(0))
    }

    pub fn deadline(&self, entity_id: i64, handle_id: i32, kind: BuffTimerKind) -> Option<i64> {
        self.positions
            .get(&key(entity_id, handle_id, kind))
            .map(|&idx| self.heap[idx].due_ms)
    }

    fn reschedule_existing(
        &mut self,
        entity_id: i64,
        handle_id: i32,
        kind: BuffTimerKind,
        due_ms: i64,
    ) -> bool {
        let Some(&index) = self.positions.get(&key(entity_id, handle_id, kind)) else {
            return false;
        };
        let previous_due_ms = self.heap[index].due_ms;
        self.heap[index].due_ms = due_ms;
        if due_ms < previous_due_ms {
            self.sift_up(index);
        } else if due_ms > previous_due_ms {
            self.sift_down(index);
        }
        true
    }

    pub fn application_disposition(
        buff_data: &Buff,
        is_new: bool,
        was_active: bool,
        is_active: bool,
    ) -> ApplicationDisposition {
        if is_new {
            return ApplicationDisposition {
                refresh_duration: true,
                reset_period: buff_data.Period > 0.0,
                execute_periodic_now: is_active
                    && buff_data.Period > 0.0
                    && buff_data.bExecutePeriodicEffectOnApplication,
            };
        }

        let mut disposition = ApplicationDisposition {
            refresh_duration: buff_data.StackDurationRefreshPolicy
                == StackDurationRefreshPolicy::Refresh,
            reset_period: buff_data.Period > 0.0
                && buff_data.StackPeriodResetPolicy == StackPeriodResetPolicy::Refresh,
            execute_periodic_now: is_active
                && buff_data.Period > 0.0
                && buff_data.bExecutePeriodicEffectOnApplication,
        };
        if was_active == is_active {
            return disposition;
        }

        disposition.reset_period = false;
        if !is_active || buff_data.Period <= 0.0 {
            return disposition;
        }

        match buff_data.PeriodicInhibitionPolicy {
            PeriodicInhibitionPolicy::None => {}
            PeriodicInhibitionPolicy::Reset => disposition.reset_period = true,
            PeriodicInhibitionPolicy::ResetAndExecute => {
                disposition.reset_period = true;
                disposition.execute_periodic_now = true;
            }
        }
        disposition
    }

    fn prepare_periodic(
        &mut self,
        entry: BuffTimerEntry,
        now_ms: i64,
        is_active: bool,
    ) -> Option<PeriodicDisposition> {
        let disposition = periodic_disposition(entry, now_ms, is_active);
        if !self.reschedule_existing(
            entry.entity_id,
            entry.handle_id,
            BuffTimerKind::PeriodicPulse,
            disposition.next_due_ms,
        ) {
            return None;
        }
        Some(disposition)
    }

    fn defer_expiry_retry(&mut self, entry: BuffTimerEntry, now_ms: i64) {
        let retry_due_ms = failure_retry_due_ms(now_ms);
        self.reschedule_existing(
            entry.entity_id,
            entry.handle_id,
            BuffTimerKind::Expiry,
            retry_due_ms,
        );
        if let Some(periodic_due_ms) =
            self.deadline(entry.entity_id, entry.handle_id, BuffTimerKind::PeriodicPulse)
            && periodic_due_ms <= retry_due_ms
        {
            self.reschedule_existing(
                entry.entity_id,
                entry.handle_id,
                BuffTimerKind::PeriodicPulse,
                retry_due_ms,
            );
        }
    }

    pub fn defer_expiration_retry(&mut self, entity_id: i64, handle_id: i32, now_ms: i64) {
        let Some(due_ms) = self.deadline(entity_id, handle_id, BuffTimerKind::Expiry) else {
            return;
        };
        self.defer_expiry_retry(
            BuffTimerEntry {
                kind: BuffTimerKind::Expiry,
                entity_id,
                handle_id,
                due_ms,
                interval_ms: 0,
            },
            now_ms,
        );
    }

    pub fn schedule_new_buff(
        &mut self,
        buff_info: &FightBuffInformation,
        buff_data: &Buff,
        entity_id: i64,
        now_ms: i64,
    ) {
        if !self.initialized {
            return;
        }
        self.register_buff_data(buff_info, buff_data, entity_id, now_ms);
    }

    pub fn refresh_buff(
        &mut self,
        buff_info: &FightBuffInformation,
        buff_data: &Buff,
        entity_id: i64,
        now_ms: i64,
        disposition: ApplicationDisposition,
    ) {
        if !self.initialized {
            return;
        }

        if disposition.refresh_duration {
            if buff_data.DurationPolicy == DurationPolicy::HasDuration
                && buff_info.LeftDuration > 0.0
            {
                self.upsert(BuffTimerEntry {
                    kind: BuffTimerKind::Expiry,
                    entity_id,
                    handle_id: buff_info.HandleId,
                    due_ms: deadline_after_ms(now_ms, seconds_to_ms(buff_info.LeftDuration)),
                    interval_ms: 0,
                });
            } else {
                self.cancel(entity_id, buff_info.HandleId, BuffTimerKind::Expiry);
            }
        } else if buff_data.DurationPolicy == DurationPolicy::HasDuration
            && buff_info.LeftDuration > 0.0
            && self
                .deadline(entity_id, buff_info.HandleId, BuffTimerKind::Expiry)
                .is_none()
        {
            self.upsert(BuffTimerEntry {
                kind: BuffTimerKind::Expiry,
                entity_id,
                handle_id: buff_info.HandleId,
                due_ms: deadline_after_ms(now_ms, seconds_to_ms(buff_info.LeftDuration)),
                interval_ms: 0,
            });
        }

        if buff_data.Period <= 0.0 {
            self.cancel(entity_id, buff_info.HandleId, BuffTimerKind::PeriodicPulse);
            return;
        }
        if disposition.reset_period
            || self
                .deadline(entity_id, buff_info.HandleId, BuffTimerKind::PeriodicPulse)
                .is_none()
        {
            let interval_ms = seconds_to_ms(buff_data.Period);
            self.upsert(BuffTimerEntry {
                kind: BuffTimerKind::PeriodicPulse,
                entity_id,
                handle_id: buff_info.HandleId,
                due_ms: deadline_after_ms(now_ms, interval_ms),
                interval_ms,
            });
        }
    }

    pub fn ensure_initialized(&mut self, scene: &Scene, assets: &Assets, now_ms: i64) {
        if self.initialized {
            return;
        }
        self.clear();

        for entity in &scene.entities {
            let Some(buffs) = entity.buffs.as_ref() else {
                continue;
            };
            for buff_info in &buffs.buffs {
                self.register_buff(assets, buff_info, entity.net_id, now_ms);
            }
        }

        self.initialized = true;
        self.rebuild_due_ms = None;
    }

    pub fn ensure_entity_registered(
        &mut self,
        scene: &Scene,
        assets: &Assets,
        entity_id: i64,
        now_ms: i64,
    ) {
        self.ensure_initialized(scene, assets, now_ms);
        let Some(entity) = scene.entity(entity_id) else {
            return;
        };
        let Some(buffs) = entity.buffs.as_ref() else {
            return;
        };
        for buff_info in &buffs.buffs {
            self.register_missing_buff(assets, buff_info, entity_id, now_ms);
        }
    }

    pub fn ensure_all_registered(&mut self, scene: &Scene, assets: &Assets, now_ms: i64) {
        self.ensure_initialized(scene, assets, now_ms);
        for entity in &scene.entities {
            let Some(buffs) = entity.buffs.as_ref() else {
                continue;
            };
            for buff_info in &buffs.buffs {
                self.register_missing_buff(assets, buff_info, entity.net_id, now_ms);
            }
        }
    }

    pub fn sync_handle_left_duration(
        &self,
        scene: &mut Scene,
        entity_id: i64,
        handle_id: i32,
        now_ms: i64,
    ) {
        let Some(due_ms) = self.deadline(entity_id, handle_id, BuffTimerKind::Expiry) else {
            return;
        };
        let Some(entity) = scene.entity_mut(entity_id) else {
            return;
        };
        let Some(buffs) = entity.buffs.as_mut() else {
            return;
        };
        if let Some(buff_info) = buffs.find_by_handle_mut(handle_id) {
            sync_left_duration(buff_info, due_ms, now_ms);
        }
    }

    pub fn sync_entity_left_durations(&self, scene: &mut Scene, entity_id: i64, now_ms: i64) {
        let Some(&index) = scene.net_id_map.get(&entity_id) else {
            return;
        };
        let Some(buffs) = scene.entities[index].buffs.as_mut() else {
            return;
        };
        for buff_info in &mut buffs.buffs {
            self.sync_buff_left_duration(buff_info, entity_id, now_ms);
        }
    }

    pub fn sync_all_left_durations(&self, scene: &mut Scene, now_ms: i64) {
        for entity in &mut scene.entities {
            let net_id = entity.net_id;
            let Some(buffs) = entity.buffs.as_mut() else {
                continue;
            };
            for buff_info in &mut buffs.buffs {
                self.sync_buff_left_duration(buff_info, net_id, now_ms);
            }
        }
    }

    pub fn drain_one_due(
        &mut self,
        now_ms: i64,
        events: &mut EventQueue,
        scene: &mut Scene,
        assets: &Assets,
        combat_receive_pack: &mut Vec<CombatReceiveData>,
    ) -> Result<(), &'static str> {
        self.ensure_initialized(scene, assets, now_ms);
        let Some(entry) = self.peek_due_entry(now_ms) else {
            return Ok(());
        };

        match entry.kind {
            BuffTimerKind::Expiry => {
                let Some(entity_handle) = scene.entity_handle(entry.entity_id) else {
                    self.cancel_handle(entry.entity_id, entry.handle_id);
                    return Ok(());
                };
                let Some(buff_info) = scene.entities[entity_handle.index]
                    .buffs
                    .as_ref()
                    .and_then(|b| b.find_by_handle(entry.handle_id))
                    .cloned()
                else {
                    self.cancel_handle(entry.entity_id, entry.handle_id);
                    return Ok(());
                };
                let Some(buff_data) = assets.tables.buff.get_data_by_id(&buff_info.BuffId) else {
                    self.cancel_handle(entry.entity_id, entry.handle_id);
                    return Ok(());
                };

                match expiration_disposition(
                    buff_info.StackCount,
                    buff_data.StackExpirationRemoveNumber,
                ) {
                    ExpirationDisposition::Retain(new_stack_count) => {
                        let buff_ref = scene.entities[entity_handle.index]
                            .buffs
                            .as_mut()
                            .and_then(|b| b.find_by_handle_mut(entry.handle_id))
                            .expect("checked above");
                        self.complete_partial_expiration(
                            entry,
                            entity_handle,
                            buff_ref,
                            &buff_data,
                            new_stack_count,
                            now_ms,
                            events,
                            combat_receive_pack,
                        );
                        let updated_info = scene.entities[entity_handle.index]
                            .buffs
                            .as_ref()
                            .and_then(|b| b.find_by_handle(entry.handle_id))
                            .cloned();
                        if let Some(updated) = updated_info {
                            buff_helper::sync_buff_shield(
                                combat_receive_pack,
                                scene,
                                assets,
                                entry.entity_id,
                                &updated,
                                &buff_data,
                            );
                        }
                        buff_helper::sync_continuous_buff_modifiers(
                            combat_receive_pack,
                            scene,
                            entry.entity_id,
                            assets,
                        );
                        return Ok(());
                    }
                    ExpirationDisposition::Remove => {}
                }

                let follow_up_buffs =
                    routine_expiration_entries(assets, &buff_data.RoutineExpirationEffects);
                let instigator = scene
                    .entity_handle(buff_info.InstigatorId)
                    .unwrap_or(entity_handle);
                events.enqueue(Event::BuffRemoval {
                    entity: entity_handle,
                    handle_ids: vec![entry.handle_id],
                    natural_expiration: Some(NaturalExpiration {
                        now_ms,
                        instigator,
                        follow_up_buffs,
                    }),
                });
            }
            BuffTimerKind::PeriodicPulse => {
                let Some(entity_handle) = scene.entity_handle(entry.entity_id) else {
                    self.cancel_handle(entry.entity_id, entry.handle_id);
                    return Ok(());
                };
                let Some(buff_info) = scene.entities[entity_handle.index]
                    .buffs
                    .as_ref()
                    .and_then(|b| b.find_by_handle(entry.handle_id))
                    .cloned()
                else {
                    self.cancel_handle(entry.entity_id, entry.handle_id);
                    return Ok(());
                };
                let Some(buff_data) = assets.tables.buff.get_data_by_id(&buff_info.BuffId) else {
                    self.cancel_handle(entry.entity_id, entry.handle_id);
                    return Ok(());
                };
                let Some(disposition) = self.prepare_periodic(entry, now_ms, buff_info.IsActive)
                else {
                    self.schedule_rebuild(now_ms);
                    return Err("MissingTimerEntry");
                };
                if disposition.execute_effect {
                    buff_helper::execute_periodic_with_assets(
                        combat_receive_pack,
                        entry.entity_id,
                        buff_info.InstigatorId,
                        &buff_data,
                        scene,
                        assets,
                    );
                }
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn complete_partial_expiration(
        &mut self,
        entry: BuffTimerEntry,
        entity: EntityHandle,
        buff_info: &mut FightBuffInformation,
        buff_data: &Buff,
        new_stack_count: i32,
        now_ms: i64,
        events: &mut EventQueue,
        combat_receive_pack: &mut Vec<CombatReceiveData>,
    ) {
        let configured_duration = buff_helper::configured_duration_seconds(buff_data);
        let duration = if configured_duration > 0.0 {
            configured_duration
        } else {
            -1.0
        };
        let stack_notify = BuffStackCountNotify {
            HandleId: entry.handle_id,
            NewStackCount: new_stack_count,
            InstigatorId: buff_info.InstigatorId,
            Time: Some(BuffStackCountNotifyTimeOneof::Duration(duration)),
            gFs: if duration > 0.0 {
                Some(BuffStackCountNotifyGFsOneof::LeftDuration(duration))
            } else {
                None
            },
            ..Default::default()
        };

        combat_receive_pack.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: entry.entity_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::BuffStackCountNotify(
                        stack_notify,
                    )),
                },
            )),
        });
        events.enqueue(Event::BuffChange {
            player_id: 0,
            entity,
            buff_id: buff_info.BuffId,
            stack_count: new_stack_count,
            is_add: false,
            instigator: entity,
        });

        let reset_period = buff_data.Period > 0.0
            && buff_data.StackPeriodResetPolicy == StackPeriodResetPolicy::Refresh;
        let periodic_due_ms = if reset_period {
            deadline_after_ms(now_ms, seconds_to_ms(buff_data.Period))
        } else {
            0
        };
        if reset_period
            && self
                .deadline(
                    entry.entity_id,
                    entry.handle_id,
                    BuffTimerKind::PeriodicPulse,
                )
                .is_none()
        {
            let interval_ms = seconds_to_ms(buff_data.Period);
            self.upsert(BuffTimerEntry {
                kind: BuffTimerKind::PeriodicPulse,
                entity_id: entry.entity_id,
                handle_id: entry.handle_id,
                due_ms: periodic_due_ms,
                interval_ms,
            });
        }

        if duration > 0.0 {
            let expiry_rescheduled = self.reschedule_existing(
                entry.entity_id,
                entry.handle_id,
                BuffTimerKind::Expiry,
                deadline_after_ms(now_ms, seconds_to_ms(duration)),
            );
            debug_assert!(expiry_rescheduled);
        } else {
            self.cancel(entry.entity_id, entry.handle_id, BuffTimerKind::Expiry);
        }
        if reset_period {
            let period_rescheduled = self.reschedule_existing(
                entry.entity_id,
                entry.handle_id,
                BuffTimerKind::PeriodicPulse,
                periodic_due_ms,
            );
            debug_assert!(period_rescheduled);
        }

        buff_info.StackCount = new_stack_count;
        buff_info.Duration = duration;
        buff_info.LeftDuration = duration;
    }

    fn register_buff(
        &mut self,
        assets: &Assets,
        buff_info: &FightBuffInformation,
        entity_id: i64,
        now_ms: i64,
    ) {
        if let Some(buff_data) = assets.tables.buff.get_data_by_id(&buff_info.BuffId) {
            self.register_buff_data(buff_info, &buff_data, entity_id, now_ms);
        }
    }

    fn register_buff_data(
        &mut self,
        buff_info: &FightBuffInformation,
        buff_data: &Buff,
        entity_id: i64,
        now_ms: i64,
    ) {
        if buff_data.DurationPolicy == DurationPolicy::HasDuration && buff_info.LeftDuration > 0.0 {
            self.upsert(BuffTimerEntry {
                kind: BuffTimerKind::Expiry,
                entity_id,
                handle_id: buff_info.HandleId,
                due_ms: deadline_after_ms(now_ms, seconds_to_ms(buff_info.LeftDuration)),
                interval_ms: 0,
            });
        }
        if buff_data.Period > 0.0 {
            let interval_ms = seconds_to_ms(buff_data.Period);
            self.upsert(BuffTimerEntry {
                kind: BuffTimerKind::PeriodicPulse,
                entity_id,
                handle_id: buff_info.HandleId,
                due_ms: deadline_after_ms(now_ms, interval_ms),
                interval_ms,
            });
        }
    }

    fn register_missing_buff(
        &mut self,
        assets: &Assets,
        buff_info: &FightBuffInformation,
        entity_id: i64,
        now_ms: i64,
    ) {
        let Some(buff_data) = assets.tables.buff.get_data_by_id(&buff_info.BuffId) else {
            return;
        };

        if buff_data.DurationPolicy == DurationPolicy::HasDuration
            && buff_info.LeftDuration > 0.0
            && self
                .deadline(entity_id, buff_info.HandleId, BuffTimerKind::Expiry)
                .is_none()
        {
            self.upsert(BuffTimerEntry {
                kind: BuffTimerKind::Expiry,
                entity_id,
                handle_id: buff_info.HandleId,
                due_ms: deadline_after_ms(now_ms, seconds_to_ms(buff_info.LeftDuration)),
                interval_ms: 0,
            });
        }

        if buff_data.Period > 0.0
            && self
                .deadline(entity_id, buff_info.HandleId, BuffTimerKind::PeriodicPulse)
                .is_none()
        {
            let interval_ms = seconds_to_ms(buff_data.Period);
            self.upsert(BuffTimerEntry {
                kind: BuffTimerKind::PeriodicPulse,
                entity_id,
                handle_id: buff_info.HandleId,
                due_ms: deadline_after_ms(now_ms, interval_ms),
                interval_ms,
            });
        }
    }

    fn clear(&mut self) {
        self.heap.clear();
        self.positions.clear();
        self.initialized = false;
        self.rebuild_due_ms = None;
    }

    fn schedule_rebuild(&mut self, now_ms: i64) {
        self.clear();
        self.rebuild_due_ms = Some(failure_retry_due_ms(now_ms));
    }

    fn sync_buff_left_duration(
        &self,
        buff_info: &mut FightBuffInformation,
        entity_id: i64,
        now_ms: i64,
    ) {
        if let Some(due_ms) = self.deadline(entity_id, buff_info.HandleId, BuffTimerKind::Expiry) {
            sync_left_duration(buff_info, due_ms, now_ms);
        }
    }

    fn remove_at(&mut self, index: usize) -> BuffTimerEntry {
        let removed = self.heap[index];
        self.positions
            .remove(&key(removed.entity_id, removed.handle_id, removed.kind));

        let last_index = self.heap.len() - 1;
        if index == last_index {
            self.heap.pop();
            return removed;
        }

        self.heap[index] = self.heap[last_index];
        self.heap.pop();
        let moved = self.heap[index];
        if let Some(pos) =
            self.positions
                .get_mut(&key(moved.entity_id, moved.handle_id, moved.kind))
        {
            *pos = index;
        }

        if index > 0 && less_than(self.heap[index], self.heap[(index - 1) / 2]) {
            self.sift_up(index);
        } else {
            self.sift_down(index);
        }
        removed
    }

    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if !less_than(self.heap[index], self.heap[parent]) {
                break;
            }
            self.swap_entries(index, parent);
            index = parent;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        loop {
            let left = index * 2 + 1;
            if left >= self.heap.len() {
                return;
            }
            let right = left + 1;
            let child = if right < self.heap.len() && less_than(self.heap[right], self.heap[left]) {
                right
            } else {
                left
            };
            if !less_than(self.heap[child], self.heap[index]) {
                return;
            }
            self.swap_entries(index, child);
            index = child;
        }
    }

    fn swap_entries(&mut self, a: usize, b: usize) {
        self.heap.swap(a, b);
        let ea = self.heap[a];
        if let Some(pos) = self
            .positions
            .get_mut(&key(ea.entity_id, ea.handle_id, ea.kind))
        {
            *pos = a;
        }
        let eb = self.heap[b];
        if let Some(pos) = self
            .positions
            .get_mut(&key(eb.entity_id, eb.handle_id, eb.kind))
        {
            *pos = b;
        }
    }
}

fn failure_retry_due_ms(now_ms: i64) -> i64 {
    (now_ms as i128 + FAILURE_RETRY_DELAY_MS as i128).min(i64::MAX as i128) as i64
}

fn next_periodic_due_ms(previous_due_ms: i64, now_ms: i64, stored_interval_ms: i64) -> i64 {
    if previous_due_ms > now_ms {
        return previous_due_ms;
    }
    let interval_ms = stored_interval_ms.max(1) as i128;
    let previous_due = previous_due_ms as i128;
    let now = now_ms as i128;
    let elapsed_periods = (now - previous_due).div_euclid(interval_ms) + 1;
    let next_due = previous_due + elapsed_periods * interval_ms;
    next_due.min(i64::MAX as i128) as i64
}

fn expiration_disposition(stack_count: i32, remove_number: u8) -> ExpirationDisposition {
    let rem = i32::from(remove_number);
    if stack_count <= 0 || rem == 0 || rem >= stack_count {
        ExpirationDisposition::Remove
    } else {
        ExpirationDisposition::Retain(stack_count - rem)
    }
}

fn periodic_disposition(entry: BuffTimerEntry, now_ms: i64, is_active: bool) -> PeriodicDisposition {
    PeriodicDisposition {
        execute_effect: is_active,
        next_due_ms: next_periodic_due_ms(entry.due_ms, now_ms, entry.interval_ms),
    }
}

fn routine_expiration_entries(assets: &Assets, ids: &[i64]) -> Vec<BuffAdditionEntry> {
    ids.iter()
        .copied()
        .filter(|id| assets.tables.buff.get_data_by_id(id).is_some())
        .map(|id| BuffAdditionEntry {
            id,
            is_active: true,
            ..Default::default()
        })
        .collect()
}

fn deadline_delay_ms(due_ms: i64, now_ms: i64) -> i64 {
    if due_ms <= now_ms {
        return 0;
    }
    ((due_ms as i128) - (now_ms as i128)).min(i64::MAX as i128) as i64
}

fn less_than(a: BuffTimerEntry, b: BuffTimerEntry) -> bool {
    if a.due_ms != b.due_ms {
        return a.due_ms < b.due_ms;
    }
    if a.entity_id != b.entity_id {
        return a.entity_id < b.entity_id;
    }
    if a.handle_id != b.handle_id {
        return a.handle_id < b.handle_id;
    }
    (a.kind as u8) < (b.kind as u8)
}

fn key(entity_id: i64, handle_id: i32, kind: BuffTimerKind) -> u128 {
    let entity_bits = entity_id as u64;
    let handle_bits = handle_id as u32;
    ((entity_bits as u128) << 33) | ((handle_bits as u128) << 1) | (kind as u128)
}

fn sync_left_duration(buff_info: &mut FightBuffInformation, due_ms: i64, now_ms: i64) {
    let remaining_ms = ((due_ms as i128) - (now_ms as i128)).max(0);
    let mut remaining_seconds = (remaining_ms as f32) / 1000.0;
    if buff_info.Duration > 0.0 && remaining_seconds > buff_info.Duration {
        remaining_seconds = buff_info.Duration;
    }
    buff_info.LeftDuration = remaining_seconds;
}

fn seconds_to_ms(seconds: f32) -> i64 {
    let milliseconds = f64::from(seconds) * 1000.0;
    if milliseconds.is_nan() || milliseconds <= 1.0 {
        return 1;
    }
    let max_ms = i64::MAX as f64;
    if milliseconds >= max_ms {
        return i64::MAX;
    }
    milliseconds.ceil() as i64
}

fn deadline_after_ms(now_ms: i64, delay_ms: i64) -> i64 {
    ((now_ms as i128) + (delay_ms as i128)).clamp(i64::MIN as i128, i64::MAX as i128) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buff_wake_delays_follow_the_exact_earliest_deadline() {
        let mut scheduler = BuffTimerScheduler::new();

        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(0));
        scheduler.schedule_rebuild(1_000);
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(50));
        scheduler.rebuild_due_ms = Some(1_000);
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(0));

        scheduler.initialized = true;
        scheduler.rebuild_due_ms = None;
        assert_eq!(scheduler.next_wake_delay_ms(1_000), None);

        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 1,
            handle_id: 1,
            due_ms: 2_000,
            interval_ms: 0,
        });
        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 2,
            handle_id: 1,
            due_ms: 3_000,
            interval_ms: 0,
        });
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(1_000));

        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 2,
            handle_id: 1,
            due_ms: 1_500,
            interval_ms: 0,
        });
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(500));
        assert_eq!(scheduler.next_wake_delay_ms(1_500), Some(0));
        assert_eq!(scheduler.next_wake_delay_ms(1_501), Some(0));

        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 2,
            handle_id: 1,
            due_ms: 4_000,
            interval_ms: 0,
        });
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(1_000));
        scheduler.cancel(1, 1, BuffTimerKind::Expiry);
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(3_000));

        scheduler.cancel(2, 1, BuffTimerKind::PeriodicPulse);
        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 3,
            handle_id: 1,
            due_ms: i64::MAX,
            interval_ms: 0,
        });
        assert_eq!(scheduler.next_wake_delay_ms(i64::MIN), Some(i64::MAX));
    }

    #[test]
    fn invalidated_buff_timers_use_only_bounded_rebuild_backoff() {
        let mut scheduler = BuffTimerScheduler {
            initialized: true,
            ..Default::default()
        };

        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 1,
            handle_id: 1,
            due_ms: 2_000,
            interval_ms: 0,
        });
        scheduler.schedule_rebuild(1_000);

        assert!(!scheduler.initialized);
        assert_eq!(scheduler.peek_due_ms(), None);
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(50));
        assert_eq!(scheduler.next_wake_delay_ms(1_049), Some(1));
        assert_eq!(scheduler.next_wake_delay_ms(1_050), Some(0));

        scheduler.schedule_rebuild(i64::MAX);
        assert_eq!(scheduler.next_wake_delay_ms(i64::MAX), Some(0));
    }

    #[test]
    fn periodic_deadlines_preserve_phase_while_skipping_missed_pulses() {
        assert_eq!(next_periodic_due_ms(1_000, 1_000, 100), 1_100);
        assert_eq!(next_periodic_due_ms(1_000, 1_025, 100), 1_100);
        assert_eq!(next_periodic_due_ms(1_000, 1_350, 100), 1_400);
        assert_eq!(next_periodic_due_ms(1_000, 1_100, 100), 1_200);
    }

    #[test]
    fn periodic_deadlines_do_not_accumulate_delayed_wake_drift() {
        let first = next_periodic_due_ms(1_000, 1_025, 100);
        let second = next_periodic_due_ms(first, 1_235, 100);
        let third = next_periodic_due_ms(second, 1_460, 100);

        assert_eq!(first, 1_100);
        assert_eq!(second, 1_300);
        assert_eq!(third, 1_500);
    }

    #[test]
    fn periodic_deadlines_floor_invalid_intervals_and_cannot_wrap() {
        assert_eq!(next_periodic_due_ms(100, 100, 0), 101);
        assert_eq!(next_periodic_due_ms(100, 100, -10), 101);
        assert_eq!(
            next_periodic_due_ms(i64::MAX - 1, i64::MAX, 1),
            i64::MAX
        );
        assert_eq!(next_periodic_due_ms(200, 100, 50), 200);
    }

    #[test]
    fn inactive_periodic_buffs_skip_effects_while_retaining_cadence() {
        let entry = BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 1,
            handle_id: 1,
            due_ms: 1_000,
            interval_ms: 100,
        };

        let inactive = periodic_disposition(entry, 1_025, false);
        assert!(!inactive.execute_effect);
        assert_eq!(inactive.next_due_ms, 1_100);

        let delayed = periodic_disposition(entry, 1_350, false);
        assert!(!delayed.execute_effect);
        assert_eq!(delayed.next_due_ms, 1_400);
    }

    #[test]
    fn periodic_activation_changes_execution_without_changing_cadence() {
        let entry = BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 1,
            handle_id: 1,
            due_ms: 1_000,
            interval_ms: 100,
        };

        let inactive = periodic_disposition(entry, 1_025, false);
        let active = periodic_disposition(entry, 1_025, true);

        assert!(!inactive.execute_effect);
        assert!(active.execute_effect);
        assert_eq!(inactive.next_due_ms, active.next_due_ms);
    }

    #[test]
    fn application_disposition_honors_refresh_and_inhibition_policies() {
        let mut buff_data = Buff {
            Period: 0.5,
            bExecutePeriodicEffectOnApplication: true,
            StackDurationRefreshPolicy: StackDurationRefreshPolicy::Refresh,
            StackPeriodResetPolicy: StackPeriodResetPolicy::Refresh,
            PeriodicInhibitionPolicy: PeriodicInhibitionPolicy::None,
            ..Default::default()
        };

        let new_active = BuffTimerScheduler::application_disposition(&buff_data, true, false, true);
        assert!(new_active.refresh_duration);
        assert!(new_active.reset_period);
        assert!(new_active.execute_periodic_now);

        let new_inactive =
            BuffTimerScheduler::application_disposition(&buff_data, true, false, false);
        assert!(!new_inactive.execute_periodic_now);

        buff_data.StackDurationRefreshPolicy = StackDurationRefreshPolicy::NoRefresh;
        buff_data.StackPeriodResetPolicy = StackPeriodResetPolicy::NoRefresh;
        let unchanged =
            BuffTimerScheduler::application_disposition(&buff_data, false, true, true);
        assert!(!unchanged.refresh_duration);
        assert!(!unchanged.reset_period);
        assert!(unchanged.execute_periodic_now);

        buff_data.StackPeriodResetPolicy = StackPeriodResetPolicy::Refresh;
        let deactivated =
            BuffTimerScheduler::application_disposition(&buff_data, false, true, false);
        assert!(!deactivated.reset_period);
        assert!(!deactivated.execute_periodic_now);

        buff_data.bExecutePeriodicEffectOnApplication = false;
        buff_data.PeriodicInhibitionPolicy = PeriodicInhibitionPolicy::None;
        let activated_preserving =
            BuffTimerScheduler::application_disposition(&buff_data, false, false, true);
        assert!(!activated_preserving.reset_period);
        assert!(!activated_preserving.execute_periodic_now);

        buff_data.PeriodicInhibitionPolicy = PeriodicInhibitionPolicy::Reset;
        let activated_reset =
            BuffTimerScheduler::application_disposition(&buff_data, false, false, true);
        assert!(activated_reset.reset_period);
        assert!(!activated_reset.execute_periodic_now);

        buff_data.PeriodicInhibitionPolicy = PeriodicInhibitionPolicy::ResetAndExecute;
        let activated_executing =
            BuffTimerScheduler::application_disposition(&buff_data, false, false, true);
        assert!(activated_executing.reset_period);
        assert!(activated_executing.execute_periodic_now);

        buff_data.bExecutePeriodicEffectOnApplication = true;
        let overlapping_execution =
            BuffTimerScheduler::application_disposition(&buff_data, false, false, true);
        assert!(overlapping_execution.reset_period);
        assert!(overlapping_execution.execute_periodic_now);
    }

    #[test]
    fn refresh_policies_preserve_or_replace_independent_deadlines() {
        let mut scheduler = BuffTimerScheduler {
            initialized: true,
            ..Default::default()
        };

        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 10,
            handle_id: 20,
            due_ms: 500,
            interval_ms: 0,
        });
        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 10,
            handle_id: 20,
            due_ms: 400,
            interval_ms: 500,
        });

        let buff_data = Buff {
            DurationPolicy: DurationPolicy::HasDuration,
            Period: 0.5,
            ..Default::default()
        };
        let buff_info = FightBuffInformation {
            HandleId: 20,
            LeftDuration: 2.0,
            ..Default::default()
        };
        scheduler.refresh_buff(
            &buff_info,
            &buff_data,
            10,
            100,
            ApplicationDisposition {
                refresh_duration: false,
                reset_period: false,
                execute_periodic_now: false,
            },
        );
        assert_eq!(scheduler.deadline(10, 20, BuffTimerKind::Expiry), Some(500));
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::PeriodicPulse),
            Some(400)
        );

        scheduler.refresh_buff(
            &buff_info,
            &buff_data,
            10,
            100,
            ApplicationDisposition {
                refresh_duration: true,
                reset_period: true,
                execute_periodic_now: false,
            },
        );
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::Expiry),
            Some(2_100)
        );
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::PeriodicPulse),
            Some(600)
        );
    }

    #[test]
    fn preserved_missing_deadlines_are_reconstructed_without_execution() {
        let mut scheduler = BuffTimerScheduler {
            initialized: true,
            ..Default::default()
        };
        let buff_data = Buff {
            DurationPolicy: DurationPolicy::HasDuration,
            Period: 0.5,
            ..Default::default()
        };
        let buff_info = FightBuffInformation {
            HandleId: 20,
            LeftDuration: 2.0,
            ..Default::default()
        };
        scheduler.refresh_buff(
            &buff_info,
            &buff_data,
            10,
            100,
            ApplicationDisposition {
                refresh_duration: false,
                reset_period: false,
                execute_periodic_now: false,
            },
        );
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::Expiry),
            Some(2_100)
        );
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::PeriodicPulse),
            Some(600)
        );
    }

    #[test]
    fn new_buff_scheduling_installs_future_deadlines_only() {
        let mut scheduler = BuffTimerScheduler {
            initialized: true,
            ..Default::default()
        };
        let buff_data = Buff {
            DurationPolicy: DurationPolicy::HasDuration,
            Period: 0.5,
            ..Default::default()
        };
        let buff_info = FightBuffInformation {
            HandleId: 20,
            LeftDuration: 2.0,
            ..Default::default()
        };
        scheduler.schedule_new_buff(&buff_info, &buff_data, 10, 100);
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::Expiry),
            Some(2_100)
        );
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::PeriodicPulse),
            Some(600)
        );
    }

    #[test]
    fn peeking_due_entry_retains_it_until_completion() {
        let mut scheduler = BuffTimerScheduler::new();
        let entry = BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 1,
            handle_id: 1,
            due_ms: 100,
            interval_ms: 0,
        };
        scheduler.upsert(entry);
        assert_eq!(scheduler.peek_due_entry(100), Some(entry));
        assert_eq!(scheduler.peek_due_ms(), Some(100));
        assert_eq!(scheduler.heap.len(), 1);
        assert_eq!(scheduler.positions.len(), 1);
    }

    #[test]
    fn expiration_disposition_removes_or_retains_configured_stacks() {
        assert_eq!(expiration_disposition(0, 1), ExpirationDisposition::Remove);
        assert_eq!(expiration_disposition(-1, 1), ExpirationDisposition::Remove);
        assert_eq!(expiration_disposition(3, 0), ExpirationDisposition::Remove);
        assert_eq!(expiration_disposition(3, 3), ExpirationDisposition::Remove);
        assert_eq!(expiration_disposition(3, 4), ExpirationDisposition::Remove);
        assert_eq!(
            expiration_disposition(3, 1),
            ExpirationDisposition::Retain(2)
        );
    }

    #[test]
    fn partial_expiration_rearms_duration_and_refreshes_periodic_cadence() {
        let mut scheduler = BuffTimerScheduler::new();
        let expiry = BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 10,
            handle_id: 20,
            due_ms: 100,
            interval_ms: 0,
        };
        scheduler.upsert(expiry);
        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 10,
            handle_id: 20,
            due_ms: 100,
            interval_ms: 500,
        });

        let mut buff_info = FightBuffInformation {
            HandleId: 20,
            StackCount: 3,
            InstigatorId: 30,
            Duration: 2.0,
            LeftDuration: 0.0,
            ..Default::default()
        };
        let buff_data = Buff {
            DurationMagnitude: vec![2.0],
            Period: 0.5,
            StackPeriodResetPolicy: StackPeriodResetPolicy::Refresh,
            ..Default::default()
        };
        let mut events = EventQueue::new();
        let mut combat_receive_pack = Vec::new();

        scheduler.complete_partial_expiration(
            expiry,
            EntityHandle {
                index: 0,
                net_id: 10,
            },
            &mut buff_info,
            &buff_data,
            2,
            100,
            &mut events,
            &mut combat_receive_pack,
        );

        assert_eq!(buff_info.StackCount, 2);
        assert_eq!(buff_info.Duration, 2.0);
        assert_eq!(buff_info.LeftDuration, 2.0);
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::Expiry),
            Some(2_100)
        );
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::PeriodicPulse),
            Some(600)
        );
        assert_eq!(combat_receive_pack.len(), 1);
        assert!(matches!(events.dequeue(), Some(Event::BuffChange { .. })));
    }

    #[test]
    fn partial_expiration_preserves_periodic_phase_when_configured() {
        let mut scheduler = BuffTimerScheduler::new();
        let expiry = BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 10,
            handle_id: 20,
            due_ms: 100,
            interval_ms: 0,
        };
        scheduler.upsert(expiry);
        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 10,
            handle_id: 20,
            due_ms: 350,
            interval_ms: 500,
        });

        let mut buff_info = FightBuffInformation {
            HandleId: 20,
            StackCount: 3,
            ..Default::default()
        };
        let buff_data = Buff {
            DurationMagnitude: vec![2.0],
            Period: 0.5,
            StackPeriodResetPolicy: StackPeriodResetPolicy::NoRefresh,
            ..Default::default()
        };
        let mut events = EventQueue::new();
        let mut combat_receive_pack = Vec::new();

        scheduler.complete_partial_expiration(
            expiry,
            EntityHandle {
                index: 0,
                net_id: 10,
            },
            &mut buff_info,
            &buff_data,
            2,
            100,
            &mut events,
            &mut combat_receive_pack,
        );
        assert_eq!(
            scheduler.deadline(10, 20, BuffTimerKind::PeriodicPulse),
            Some(350)
        );
    }

    #[test]
    fn partial_expiration_cancels_non_positive_configured_duration() {
        let mut scheduler = BuffTimerScheduler::new();
        let expiry = BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 10,
            handle_id: 20,
            due_ms: 100,
            interval_ms: 0,
        };
        scheduler.upsert(expiry);

        let mut buff_info = FightBuffInformation {
            HandleId: 20,
            StackCount: 2,
            Duration: 1.0,
            LeftDuration: 0.0,
            ..Default::default()
        };
        let buff_data = Buff {
            DurationMagnitude: vec![-1.0],
            Period: 0.0,
            StackPeriodResetPolicy: StackPeriodResetPolicy::NoRefresh,
            ..Default::default()
        };
        let mut events = EventQueue::new();
        let mut combat_receive_pack = Vec::new();

        scheduler.complete_partial_expiration(
            expiry,
            EntityHandle {
                index: 0,
                net_id: 10,
            },
            &mut buff_info,
            &buff_data,
            1,
            100,
            &mut events,
            &mut combat_receive_pack,
        );

        assert_eq!(buff_info.StackCount, 1);
        assert_eq!(buff_info.Duration, -1.0);
        assert_eq!(buff_info.LeftDuration, -1.0);
        assert_eq!(scheduler.deadline(10, 20, BuffTimerKind::Expiry), None);
    }

    #[test]
    fn failed_expiry_preparation_defers_its_handle_without_blocking_others() {
        let mut scheduler = BuffTimerScheduler::new();
        let expiry = BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 1,
            handle_id: 1,
            due_ms: 100,
            interval_ms: 0,
        };
        scheduler.upsert(expiry);
        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 1,
            handle_id: 1,
            due_ms: 125,
            interval_ms: 100,
        });
        let unrelated = BuffTimerEntry {
            kind: BuffTimerKind::Expiry,
            entity_id: 2,
            handle_id: 1,
            due_ms: 110,
            interval_ms: 0,
        };
        scheduler.upsert(unrelated);

        scheduler.defer_expiry_retry(expiry, 100);

        assert_eq!(scheduler.deadline(1, 1, BuffTimerKind::Expiry), Some(150));
        assert_eq!(
            scheduler.deadline(1, 1, BuffTimerKind::PeriodicPulse),
            Some(150)
        );
        assert_eq!(scheduler.pop_due(110), Some(unrelated));
        assert_eq!(
            scheduler.pop_due(150).map(|e| e.kind),
            Some(BuffTimerKind::Expiry)
        );
        assert_eq!(
            scheduler.pop_due(150).map(|e| e.kind),
            Some(BuffTimerKind::PeriodicPulse)
        );
    }

    #[test]
    fn periodic_failure_retains_the_next_aligned_deadline() {
        let mut scheduler = BuffTimerScheduler::new();
        scheduler.upsert(BuffTimerEntry {
            kind: BuffTimerKind::PeriodicPulse,
            entity_id: 1,
            handle_id: 1,
            due_ms: 100,
            interval_ms: 100,
        });

        let first = scheduler.peek_due_entry(125).unwrap();
        let first_disp = scheduler.prepare_periodic(first, 125, true).unwrap();
        assert!(first_disp.execute_effect);
        assert_eq!(
            scheduler.deadline(1, 1, BuffTimerKind::PeriodicPulse),
            Some(200)
        );

        let second = scheduler.peek_due_entry(250).unwrap();
        let _ = scheduler.prepare_periodic(second, 250, true).unwrap();
        assert_eq!(
            scheduler.deadline(1, 1, BuffTimerKind::PeriodicPulse),
            Some(300)
        );
        assert_eq!(scheduler.heap.len(), 1);
        assert_eq!(scheduler.positions.len(), 1);
    }

    #[test]
    fn failure_retry_deadlines_clamp_without_wrapping() {
        assert_eq!(failure_retry_due_ms(1_000), 1_050);
        assert_eq!(failure_retry_due_ms(i64::MAX), i64::MAX);
    }

    #[test]
    fn buff_seconds_convert_to_milliseconds_with_flooring_and_saturation() {
        assert_eq!(seconds_to_ms(0.0), 1);
        assert_eq!(seconds_to_ms(-1.0), 1);
        assert_eq!(seconds_to_ms(0.0001), 1);
        assert_eq!(seconds_to_ms(1.5), 1_500);
        assert_eq!(seconds_to_ms(f32::MAX), i64::MAX);
        assert_eq!(seconds_to_ms(f32::INFINITY), i64::MAX);
        assert_eq!(seconds_to_ms(f32::NAN), 1);
    }

    #[test]
    fn buff_deadlines_saturate_without_wrapping() {
        assert_eq!(deadline_after_ms(1_000, 500), 1_500);
        assert_eq!(deadline_after_ms(i64::MAX, 1), i64::MAX);
        assert_eq!(deadline_after_ms(i64::MIN, -1), i64::MIN);
    }

    #[test]
    fn remaining_buff_duration_handles_integer_timestamp_limits() {
        let mut buff_info = FightBuffInformation {
            Duration: f32::MAX,
            LeftDuration: f32::MAX,
            ..Default::default()
        };

        sync_left_duration(&mut buff_info, i64::MAX, i64::MIN);
        assert!(buff_info.LeftDuration.is_finite());
        assert!(buff_info.LeftDuration > 0.0);

        sync_left_duration(&mut buff_info, i64::MIN, i64::MAX);
        assert_eq!(buff_info.LeftDuration, 0.0);
    }
}
