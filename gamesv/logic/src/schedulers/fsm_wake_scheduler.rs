use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum WakeKind {
    RootTimer = 0,
    PendingTimeout = 1,
    Paralysis = 2,
    DelayedSuicide = 3,
    DelayedDestroy = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WakeEntry {
    pub entity_id: i64,
    pub fsm_id: i32,
    pub kind: WakeKind,
    pub due_ms: i64,
}

#[derive(Default, Debug)]
pub struct FsmWakeScheduler {
    heap: Vec<WakeEntry>,
    positions: HashMap<u128, usize>,
    dirty_entities: VecDeque<i64>,
    dirty_set: HashSet<i64>,
}

impl FsmWakeScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.heap.clear();
        self.positions.clear();
        self.dirty_entities.clear();
        self.dirty_set.clear();
    }

    pub fn mark_dirty(&mut self, entity_id: i64) {
        if self.dirty_set.insert(entity_id) {
            self.dirty_entities.push_back(entity_id);
        }
    }

    pub fn pop_dirty(&mut self) -> Option<i64> {
        while let Some(entity_id) = self.dirty_entities.pop_front() {
            if self.dirty_set.remove(&entity_id) {
                return Some(entity_id);
            }
        }
        None
    }

    pub fn upsert(&mut self, entry: WakeEntry) {
        let entry_key = key(entry.entity_id, entry.fsm_id, entry.kind);
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

    pub fn cancel(&mut self, entity_id: i64, fsm_id: i32, kind: WakeKind) {
        let entry_key = key(entity_id, fsm_id, kind);
        if let Some(&index) = self.positions.get(&entry_key) {
            let _ = self.remove_at(index);
        }
    }

    pub fn cancel_entity(&mut self, entity_id: i64) {
        self.dirty_set.remove(&entity_id);
        let mut index = 0;
        while index < self.heap.len() {
            if self.heap[index].entity_id == entity_id {
                let _ = self.remove_at(index);
            } else {
                index += 1;
            }
        }
    }

    pub fn pop_due(&mut self, now_ms: i64) -> Option<WakeEntry> {
        if self.heap.first().is_none_or(|e| e.due_ms > now_ms) {
            return None;
        }
        Some(self.remove_at(0))
    }

    pub fn next_wake_delay_ms(&self, now_ms: i64) -> Option<i64> {
        if !self.dirty_set.is_empty() {
            return Some(0);
        }
        self.heap
            .first()
            .map(|e| deadline_delay_ms(e.due_ms, now_ms))
    }

    fn remove_at(&mut self, index: usize) -> WakeEntry {
        let removed = self.heap[index];
        self.positions
            .remove(&key(removed.entity_id, removed.fsm_id, removed.kind));

        let last_index = self.heap.len() - 1;
        if index == last_index {
            self.heap.pop();
            return removed;
        }

        self.heap[index] = self.heap[last_index];
        self.heap.pop();
        let moved = self.heap[index];
        if let Some(pos) = self
            .positions
            .get_mut(&key(moved.entity_id, moved.fsm_id, moved.kind))
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
        if let Some(pos) = self.positions.get_mut(&key(ea.entity_id, ea.fsm_id, ea.kind)) {
            *pos = a;
        }
        let eb = self.heap[b];
        if let Some(pos) = self.positions.get_mut(&key(eb.entity_id, eb.fsm_id, eb.kind)) {
            *pos = b;
        }
    }
}

fn deadline_delay_ms(due_ms: i64, now_ms: i64) -> i64 {
    if due_ms <= now_ms {
        return 0;
    }
    let delay = (due_ms as i128) - (now_ms as i128);
    delay.min(i64::MAX as i128) as i64
}

fn less_than(a: WakeEntry, b: WakeEntry) -> bool {
    if a.due_ms != b.due_ms {
        return a.due_ms < b.due_ms;
    }
    if a.entity_id != b.entity_id {
        return a.entity_id < b.entity_id;
    }
    if a.kind == WakeKind::DelayedSuicide && b.kind == WakeKind::DelayedDestroy {
        return true;
    }
    if a.kind == WakeKind::DelayedDestroy && b.kind == WakeKind::DelayedSuicide {
        return false;
    }
    if a.fsm_id != b.fsm_id {
        return a.fsm_id < b.fsm_id;
    }
    (a.kind as u8) < (b.kind as u8)
}

fn key(entity_id: i64, fsm_id: i32, kind: WakeKind) -> u128 {
    let entity_bits = entity_id as u64;
    let fsm_bits = fsm_id as u32;
    ((entity_bits as u128) << 40) | ((fsm_bits as u128) << 8) | (kind as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wake_delay_follows_dirty_work_and_earliest_deadline() {
        let mut scheduler = FsmWakeScheduler::new();
        assert_eq!(scheduler.next_wake_delay_ms(1_000), None);

        scheduler.upsert(WakeEntry {
            entity_id: 1,
            fsm_id: 1,
            kind: WakeKind::RootTimer,
            due_ms: 2_000,
        });
        scheduler.upsert(WakeEntry {
            entity_id: 2,
            fsm_id: 1,
            kind: WakeKind::PendingTimeout,
            due_ms: 1_500,
        });
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(500));
        assert_eq!(scheduler.next_wake_delay_ms(1_500), Some(0));

        scheduler.mark_dirty(3);
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(0));
        assert_eq!(scheduler.pop_dirty(), Some(3));
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(500));
    }

    #[test]
    fn wake_delay_follows_rescheduling_cancellation_and_entity_removal() {
        let mut scheduler = FsmWakeScheduler::new();

        scheduler.upsert(WakeEntry {
            entity_id: 1,
            fsm_id: 1,
            kind: WakeKind::RootTimer,
            due_ms: 3_000,
        });
        scheduler.upsert(WakeEntry {
            entity_id: 1,
            fsm_id: 1,
            kind: WakeKind::RootTimer,
            due_ms: 2_000,
        });
        assert_eq!(scheduler.next_wake_delay_ms(1_000), Some(1_000));
        scheduler.cancel(1, 1, WakeKind::RootTimer);
        assert_eq!(scheduler.next_wake_delay_ms(1_000), None);

        scheduler.mark_dirty(2);
        scheduler.upsert(WakeEntry {
            entity_id: 2,
            fsm_id: 1,
            kind: WakeKind::Paralysis,
            due_ms: 2_000,
        });
        scheduler.cancel_entity(2);
        assert_eq!(scheduler.next_wake_delay_ms(1_000), None);
    }

    #[test]
    fn wake_delay_clamps_widened_subtraction() {
        let mut scheduler = FsmWakeScheduler::new();
        scheduler.upsert(WakeEntry {
            entity_id: 1,
            fsm_id: 1,
            kind: WakeKind::DelayedDestroy,
            due_ms: i64::MAX,
        });
        assert_eq!(scheduler.next_wake_delay_ms(i64::MIN), Some(i64::MAX));
    }
}
