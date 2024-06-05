use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

type Constraint = Arc<dyn Fn(&Witness) -> bool + Send + Sync>;
type Witness = Vec<u8>; // Placeholder for the actual Witness type
type EvaluationResult = Result<Vec<bool>, String>;

fn parallel_constraint_evaluation(constraints: &[Constraint], witness: &Witness) -> Result<EvaluationResult, String> {
    if constraints.is_empty() {
        return Err("No constraints provided".to_string());
    }

    let failure = Arc::new(AtomicBool::new(false));
    let results: Vec<_> = constraints
        .par_iter()
        .map_with(failure.clone(), |failure, constraint| {
            if failure.load(Ordering::Relaxed) {
                return Err("Early termination due to constraint failure".to_string());
            }
            let result = constraint(witness);
            if !result {
                failure.store(true, Ordering::Relaxed);
            }
            Ok(result)
        })
        .collect();

    if failure.load(Ordering::Relaxed) {
        return Err("Constraint evaluation failed".to_string());
    }

    Ok(results)
}
