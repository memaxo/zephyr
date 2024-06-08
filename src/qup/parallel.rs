use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ParallelProcessor {
    quantum_nodes: Vec<QuantumNode>,
    classical_nodes: Vec<ClassicalNode>,
}

impl ParallelProcessor {
    pub fn new(quantum_nodes: Vec<QuantumNode>, classical_nodes: Vec<ClassicalNode>) -> Self {
        ParallelProcessor {
            quantum_nodes,
            classical_nodes,
        }
    }

    pub fn distribute_tasks(&self, tasks: Vec<Task>) -> Vec<Result> {
        let quantum_tasks: Vec<_> = tasks.iter().filter(|task| task.is_quantum()).collect();
        let classical_tasks: Vec<_> = tasks.iter().filter(|task| !task.is_quantum()).collect();

        let quantum_results = self.process_quantum_tasks(quantum_tasks);
        let classical_results = self.process_classical_tasks(classical_tasks);

        [quantum_results, classical_results].concat()
    }

    fn process_quantum_tasks(&self, tasks: Vec<&Task>) -> Vec<Result> {
        let results = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        for (i, task) in tasks.iter().enumerate() {
            let node = self.quantum_nodes[i % self.quantum_nodes.len()].clone();
            let results = Arc::clone(&results);
            let task = task.clone();

            let handle = thread::spawn(move || {
                let result = node.process(task);
                results.lock().unwrap().push(result);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Arc::try_unwrap(results).unwrap().into_inner().unwrap()
    }

    fn process_classical_tasks(&self, tasks: Vec<&Task>) -> Vec<Result> {
        let results = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        for (i, task) in tasks.iter().enumerate() {
            let node = self.classical_nodes[i % self.classical_nodes.len()].clone();
            let results = Arc::clone(&results);
            let task = task.clone();

            let handle = thread::spawn(move || {
                let result = node.process(task);
                results.lock().unwrap().push(result);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Arc::try_unwrap(results).unwrap().into_inner().unwrap()
    }
}

#[derive(Clone)]
pub struct QuantumNode;

impl QuantumNode {
    pub fn process(&self, task: Task) -> Result {
        // Simulate quantum processing
        thread::sleep(Duration::from_millis(100));
        Result::new("Quantum result")
    }
}

#[derive(Clone)]
pub struct ClassicalNode;

impl ClassicalNode {
    pub fn process(&self, task: Task) -> Result {
        // Simulate classical processing
        thread::sleep(Duration::from_millis(50));
        Result::new("Classical result")
    }
}

#[derive(Clone)]
pub struct Task {
    pub is_quantum: bool,
}

impl Task {
    pub fn is_quantum(&self) -> bool {
        self.is_quantum
    }
}

pub struct Result {
    pub message: String,
}

impl Result {
    pub fn new(message: &str) -> Self {
        Result {
            message: message.to_string(),
        }
    }
}
