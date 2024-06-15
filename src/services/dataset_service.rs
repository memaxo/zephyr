use actix_web::{web, App, HttpServer, Responder};
use crate::optimization_problems::hdc_code_generation::dataset::CodeDataset;

async fn get_dataset() -> impl Responder {
    let dataset = CodeDataset::new();
    web::Json(dataset.get_entries())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/dataset", web::get().to(get_dataset))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
