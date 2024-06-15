use actix_web::{web, App, HttpServer, Responder};
use crate::optimization_problems::hdc_code_generation::evaluator::Evaluator;
use crate::optimization_problems::hdc_code_generation::model::CodeGenerationModel;
use crate::optimization_problems::hdc_code_generation::preprocessor::Preprocessor;
use crate::optimization_problems::hdc_code_generation::dataset::CodeDataset;

async fn evaluate_model() -> impl Responder {
    let dataset = CodeDataset::new();
    let preprocessor = Preprocessor::new(512, 10);
    let model = CodeGenerationModel::new(...); // Initialize with appropriate parameters
    let evaluator = Evaluator::new(&dataset, preprocessor);
    let result = evaluator.evaluate(&model).unwrap();
    web::Json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/evaluate", web::get().to(evaluate_model))
    })
    .bind("127.0.0.1:8084")?
    .run()
    .await
}
