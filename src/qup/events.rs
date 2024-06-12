use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub enum QUPEvent {
    NewBlockProposed,
    VoteCast,
    FinalizedBlock,
}

pub struct EventManager {
    subscribers: HashMap<QUPEvent, Vec<Arc<dyn Fn() + Send + Sync>>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event: QUPEvent, callback: Arc<dyn Fn() + Send + Sync>) {
        self.subscribers.entry(event).or_default().push(callback);
    }

    pub fn emit(&self, event: QUPEvent) {
        if let Some(callbacks) = self.subscribers.get(&event) {
            for callback in callbacks {
                callback();
            }
        }
    }
}
