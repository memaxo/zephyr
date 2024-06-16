use crate::qup::config::QUPConfig;
use crate::qup::hardware_assessment::HardwareAssessment;
use crate::qup::types::{UsefulWorkProblem, ModelTrainingProblem};
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::Reverse;

pub struct TaskManager {
    config: QUPConfig,
    useful_work_queue: VecDeque<UsefulWorkProblem>,
    model_training_queue: VecDeque<ModelTrainingProblem>,
}

impl TaskManager {
    pub fn new(config: QUPConfig) -> Self {
        TaskManager {
            config,
            useful_work_queue: VecDeque::new(),
            model_training_queue: VecDeque::new(),
        }
    }

    pub fn enqueue_useful_work(&mut self, problem: UsefulWorkProblem) {
        self.useful_work_queue.push_back(problem);
    }

    pub fn enqueue_model_training(&mut self, problem: ModelTrainingProblem) {
        self.model_training_queue.push_back(problem);
    }

    pub fn assign_useful_work(&mut self, node_id: &str, hardware: &HardwareAssessment, stake: u64, network_load: f64) -> Option<UsefulWorkProblem> {
        if let Some(problem) = self.useful_work_queue.pop_front() {
            // Adjust difficulty based on node capabilities, stake, and network load
            let adjusted_difficulty = self.adjust_useful_work_difficulty(&problem, hardware, stake, network_load);
            Some(UsefulWorkProblem { 
                difficulty: adjusted_difficulty,
                ..problem
            })
        } else {
            None
        }
    }

    pub fn assign_model_training(&mut self, node_id: &str, hardware: &HardwareAssessment, stake: u64, network_load: f64) -> Option<ModelTrainingProblem> {
        if let Some(problem) = self.model_training_queue.pop_front() {
            // Adjust difficulty based on node capabilities, stake, and network load  
            let adjusted_difficulty = self.adjust_model_training_difficulty(&problem, hardware, stake, network_load);
            Some(ModelTrainingProblem {
                difficulty: adjusted_difficulty, 
                ..problem
            })
        } else {
            None
        }
    }

    fn adjust_useful_work_difficulty(&self, problem: &UsefulWorkProblem, hardware: &HardwareAssessment, stake: u64, network_load: f64) -> u64 {
        // Implement logic to adjust useful work difficulty based on node capabilities, stake, network load, and completion rates
        // ...
        problem.difficulty
    }

    fn adjust_model_training_difficulty(&self, problem: &ModelTrainingProblem, hardware: &HardwareAssessment, stake: u64, network_load: f64) -> u64 {
        // Implement logic to adjust model training difficulty based on node capabilities, stake, network load, and completion rates  
        // ...
        problem.difficulty
    }
}
impl TaskManager {
    pub fn distribute_tasks(&self, tasks: Vec<Task>) -> Vec<Result> {
        let quantum_tasks: Vec<_> = tasks.iter().filter(|task| task.is_quantum()).collect();
        let classical_tasks: Vec<_> = tasks.iter().filter(|task| !task.is_quantum()).collect();

        let quantum_results = self.process_quantum_tasks(quantum_tasks);
        let classical_results = self.process_classical_tasks(classical_tasks);

        [quantum_results, classical_results].concat()
    }

    pub fn balance_load(&self, tasks: Vec<Task>) -> Vec<Task> {
        // Implement load balancing logic here
        // For simplicity, we will just return the tasks as is
        tasks
    }

    pub fn optimize_resource_utilization(&self) {
        // Implement resource optimization logic here
        // For simplicity, this method will be a no-op
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
