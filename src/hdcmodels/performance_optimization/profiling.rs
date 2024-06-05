use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Profiler {
    timings: HashMap<String, Duration>,
}

impl Profiler {
    pub fn new() -> Self {
        Profiler {
            timings: HashMap::new(),
        }
    }

    pub fn start_timer(&mut self, label: &str) {
        let start_time = Instant::now();
        self.timings.insert(label.to_string(), start_time.elapsed());
    }

    pub fn stop_timer(&mut self, label: &str) {
        let end_time = Instant::now();
        if let Some(start_time) = self.timings.get_mut(label) {
            *start_time = end_time - *start_time;
        }
    }

    pub fn get_timing(&self, label: &str) -> Option<Duration> {
        self.timings.get(label).cloned()
    }

    pub fn print_timings(&self) {
        println!("Profiling Results:");
        for (label, duration) in &self.timings {
            println!("{}: {:?}", label, duration);
        }
    }
}

#[macro_export]
macro_rules! profile {
    ($profiler:expr, $label:expr, $code:block) => {
        $profiler.start_timer($label);
        let result = $code;
        $profiler.stop_timer($label);
        result
    };
}

pub fn profile_hdcmodels() {
    let mut profiler = Profiler::new();

    // Profile encoding
    profile!(&mut profiler, "encode_data", {
        // Code block for encoding data
        // ...
    });

    // Profile similarity calculation
    profile!(&mut profiler, "calculate_similarity", {
        // Code block for calculating similarity
        // ...
    });

    // Profile model training
    profile!(&mut profiler, "train_model", {
        // Code block for training the model
        // ...
    });

    // Profile model prediction
    profile!(&mut profiler, "predict", {
        // Code block for making predictions
        // ...
    });

    // Print the profiling results
    profiler.print_timings();
}
