use crate::qup::monitoring::{collect_metrics, evaluate_model};
use crate::hdcmodels::hdcmodels::Dataset;
use crate::qup::fault_tolerance::FaultTolerantDistributedTrainingNode;
use log::info;

pub fn report_training_metrics() {
    // Collect and report metrics
    let metrics = collect_metrics();
    info!("Training Metrics: {:?}", metrics);
}

pub fn report_evaluation_metrics(trainer: &FaultTolerantDistributedTrainingNode) {
    // Evaluate the model
    let validation_dataset = Dataset::load("validation");
    let evaluation_metrics = evaluate_model(&trainer.model, &validation_dataset);
    info!("Evaluation Metrics: {:?}", evaluation_metrics);
}
