use actix_web::{web, App, HttpServer, Responder};
use crate::optimization_problems::hdc_code_generation::model::CodeGenerationModel;
use crate::optimization_problems::hdc_code_generation::trainer::Trainer;
use crate::hdcmodels::Dataset;

async fn train_model() -> impl Responder {
    let model = CodeGenerationModel::new(...); // Initialize with appropriate parameters
    let optimizer = ...; // Initialize optimizer
    let dataset = Dataset::new();
    let mut trainer = Trainer::new(model, optimizer, 32, 10);
    trainer.train(&dataset).unwrap();
    "Model trained successfully"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/train", web::post().to(train_model))
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
