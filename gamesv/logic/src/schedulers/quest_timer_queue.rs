use std::collections::HashMap;

/// Scheduled completion deadline for a `Timer` or `WaitTime` child quest node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestTimerEntry {
    pub quest_id: i32,
    pub node_id: i32,
    pub due_ms: i64,
}

#[derive(Default, Debug)]
pub struct QuestTimerQueue {
    heap: Vec<QuestTimerEntry>,
    positions: HashMap<u64, usize>,
}

impl QuestTimerQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.heap.clear();
        self.positions.clear();
    }

    /// Inserts or updates a quest node timer deadline.
    pub fn upsert(&mut self, entry: QuestTimerEntry) {
        let entry_key = key(entry.quest_id, entry.node_id);
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

    pub fn contains(&self, quest_id: i32, node_id: i32) -> bool {
        self.positions
            .contains_key(&key(quest_id, node_id))
    }

    pub fn get_due_ms(&self, quest_id: i32, node_id: i32) -> Option<i64> {
        self.positions
            .get(&key(quest_id, node_id))
            .map(|&idx| self.heap[idx].due_ms)
    }

    pub fn entries(&self) -> &[QuestTimerEntry] {
        &self.heap
    }

    pub fn cancel(&mut self, quest_id: i32, node_id: i32) {
        if let Some(&index) = self.positions.get(&key(quest_id, node_id)) {
            let _ = self.remove_at(index);
        }
    }

    pub fn cancel_quest(&mut self, quest_id: i32) {
        let mut index = 0;
        while index < self.heap.len() {
            if self.heap[index].quest_id == quest_id {
                let _ = self.remove_at(index);
            } else {
                index += 1;
            }
        }
    }

    pub fn pop_due(&mut self, now_ms: i64) -> Option<QuestTimerEntry> {
        if self.heap.first().is_none_or(|e| e.due_ms > now_ms) {
            return None;
        }
        Some(self.remove_at(0))
    }

    pub fn next_wake_delay_ms(&self, now_ms: i64) -> Option<i64> {
        let first = self.heap.first()?;
        if first.due_ms <= now_ms {
            return Some(0);
        }
        let delay = (first.due_ms as i128) - (now_ms as i128);
        Some(delay.min(i64::MAX as i128) as i64)
    }

    fn remove_at(&mut self, index: usize) -> QuestTimerEntry {
        let removed = self.heap[index];
        self.positions.remove(&key(removed.quest_id, removed.node_id));

        let last_index = self.heap.len() - 1;
        if index == last_index {
            self.heap.pop();
            return removed;
        }

        self.heap[index] = self.heap[last_index];
        self.heap.pop();
        let moved = self.heap[index];
        if let Some(pos) = self.positions.get_mut(&key(moved.quest_id, moved.node_id)) {
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
        if let Some(pos) = self.positions.get_mut(&key(ea.quest_id, ea.node_id)) {
            *pos = a;
        }
        let eb = self.heap[b];
        if let Some(pos) = self.positions.get_mut(&key(eb.quest_id, eb.node_id)) {
            *pos = b;
        }
    }
}

fn less_than(a: QuestTimerEntry, b: QuestTimerEntry) -> bool {
    if a.due_ms != b.due_ms {
        return a.due_ms < b.due_ms;
    }
    if a.quest_id != b.quest_id {
        return a.quest_id < b.quest_id;
    }
    a.node_id < b.node_id
}

fn key(quest_id: i32, node_id: i32) -> u64 {
    (((quest_id as u32) as u64) << 32) | ((node_id as u32) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_timer_queue_orders_deadlines_updates_in_place_and_cancels() {
        let mut queue = QuestTimerQueue::new();

        queue.upsert(QuestTimerEntry {
            quest_id: 139000025,
            node_id: 107,
            due_ms: 2_000,
        });
        queue.upsert(QuestTimerEntry {
            quest_id: 139000025,
            node_id: 116,
            due_ms: 1_000,
        });
        assert_eq!(queue.next_wake_delay_ms(500), Some(500));
        assert_eq!(queue.pop_due(999), None);

        // Reschedule node 116 later than node 107.
        queue.upsert(QuestTimerEntry {
            quest_id: 139000025,
            node_id: 116,
            due_ms: 3_000,
        });
        assert_eq!(queue.get_due_ms(139000025, 107), Some(2_000));

        let first = queue.pop_due(2_000).expect("node 107 should be due");
        assert_eq!(first.node_id, 107);
        assert_eq!(queue.pop_due(2_500), None);

        queue.cancel_quest(139000025);
        assert_eq!(queue.next_wake_delay_ms(5_000), None);
    }
}
