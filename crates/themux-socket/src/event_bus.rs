// Event bus: ring buffer for cmux-compatible event streaming.

use super::protocol::stream::Event;
use std::collections::VecDeque;

/// Maximum number of events in the ring buffer.
const MAX_EVENTS: usize = 4096;

/// Event bus with ring buffer and subscription support.
#[derive(Debug)]
pub struct EventBus {
    events: VecDeque<Event>,
    next_seq: u64,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            events: VecDeque::with_capacity(MAX_EVENTS),
            next_seq: 0,
        }
    }

    /// Push an event to the ring buffer.
    pub fn push(&mut self, mut event: Event) {
        event.seq = self.next_seq;
        self.next_seq += 1;

        if self.events.len() >= MAX_EVENTS {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    /// Get events after a given sequence number.
    pub fn events_after(&self, after_seq: u64) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.seq > after_seq)
            .collect()
    }

    /// Check if we have events since a given sequence (gap detection).
    pub fn has_gap(&self, after_seq: u64) -> bool {
        if let Some(oldest) = self.events.front() {
            after_seq > 0 && oldest.seq > after_seq + 1
        } else {
            false
        }
    }
}
