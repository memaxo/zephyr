use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct TransactionQueue {
    queue: Arc<Mutex<VecDeque<Transaction>>>,
}

impl TransactionQueue {
    pub fn new() -> Self {
        TransactionQueue {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn enqueue(&self, transaction: Transaction) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(transaction);
    }

    pub fn dequeue(&self) -> Option<Transaction> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }

    pub fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }
}

pub struct Buffer<T> {
    buffer: Arc<Mutex<Vec<T>>>,
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Buffer {
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, item: T) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(item);
    }

    pub fn get_all(&self) -> Vec<T> 
    where
        T: Clone,
    {
        let buffer = self.buffer.lock().unwrap();
        buffer.clone()
    }

    pub fn clear(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }

    pub fn len(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.len()
    }
}
