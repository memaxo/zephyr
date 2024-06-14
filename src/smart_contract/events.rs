use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub name: String,
    pub data: HashMap<String, String>,
    pub topics: Vec<String>,
}

pub struct EventManager {
    events: Arc<Mutex<Vec<Event>>>,
    sender: Sender<Event>,
}

impl EventManager {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = Arc::clone(&events);

        thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                events_clone.lock().unwrap().push(event);
            }
        });

        EventManager { events, sender }
    }

    pub fn emit_event(&self, event: Event) {
        self.sender.send(event).expect("Failed to send event");
    }

    pub fn filter_events(&self, topic: &str) -> Vec<Event> {
        let events = self.events.lock().unwrap();
        events.iter().filter(|e| e.topics.contains(&topic.to_string())).cloned().collect()
    }

    pub fn stream_events(&self) -> Receiver<Event> {
        let (stream_sender, stream_receiver) = mpsc::channel();
        let events_clone = Arc::clone(&self.events);

        thread::spawn(move || {
            let mut last_index = 0;
            loop {
                let events = events_clone.lock().unwrap();
                if last_index < events.len() {
                    for event in &events[last_index..] {
                        stream_sender.send(event.clone()).expect("Failed to stream event");
                    }
                    last_index = events.len();
                }
            }
        });

        stream_receiver
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_event() {
        let manager = EventManager::new();
        let event = Event {
            name: "TestEvent".to_string(),
            data: HashMap::new(),
            topics: vec!["test".to_string()],
        };
        manager.emit_event(event.clone());
        let events = manager.filter_events("test");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "TestEvent");
    }

    #[test]
    fn test_filter_events() {
        let manager = EventManager::new();
        let event1 = Event {
            name: "Event1".to_string(),
            data: HashMap::new(),
            topics: vec!["topic1".to_string()],
        };
        let event2 = Event {
            name: "Event2".to_string(),
            data: HashMap::new(),
            topics: vec!["topic2".to_string()],
        };
        manager.emit_event(event1.clone());
        manager.emit_event(event2.clone());
        let events = manager.filter_events("topic1");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "Event1");
    }

    #[test]
    fn test_stream_events() {
        let manager = EventManager::new();
        let event = Event {
            name: "StreamEvent".to_string(),
            data: HashMap::new(),
            topics: vec!["stream".to_string()],
        };
        let receiver = manager.stream_events();
        manager.emit_event(event.clone());
        let received_event = receiver.recv().unwrap();
        assert_eq!(received_event.name, "StreamEvent");
    }
}
