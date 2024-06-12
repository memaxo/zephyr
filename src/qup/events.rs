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
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum QUPEvent {
    NewBlockProposal(QUPBlock),
    VoteCast(QUPVote),
    UsefulWorkCompleted(UsefulWorkSolution),
}

pub struct EventSystem {
    sender: broadcast::Sender<QUPEvent>,
}

impl EventSystem {
    pub fn new(buffer_size: usize) -> Self {
        let (sender, _) = broadcast::channel(buffer_size);
        EventSystem { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<QUPEvent> {
        self.sender.subscribe()
    }

    pub fn emit(&self, event: QUPEvent) {
        let _ = self.sender.send(event);
    }
}
