use crate::qup::event_handler::{QUPEvent, EventSystem};
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
