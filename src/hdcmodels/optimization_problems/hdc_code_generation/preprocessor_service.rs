use actix_web::{web, App, HttpServer, Responder};
use crate::hdcmodels::optimization_problems::hdc_code_generation::preprocessor::Preprocessor;

async fn preprocess(data: web::Json<Vec<String>>) -> impl Responder {
    let preprocessor = Preprocessor::new(512, 10);
    let preprocessed_data = preprocessor.preprocess(&data);
    web::Json(preprocessed_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/preprocess", web::post().to(preprocess))
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}
