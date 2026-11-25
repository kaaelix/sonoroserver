use std::collections::VecDeque;

use crate::events::Event;

#[derive(Debug, Default)]
pub struct EventQueue {
    queue: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn enqueue(&mut self, event: Event) {
        self.queue.push_back(event);
    }

    pub fn enqueue_best_effort(&mut self, event: Event) {
        self.enqueue(event);
    }

    pub fn dequeue(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }

    pub fn drain_all(&mut self) -> impl Iterator<Item = Event> + '_ {
        self.queue.drain(..)
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}
