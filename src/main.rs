use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use log::{info};

mod models;
use models::SubmitSm;

#[post("/esme/http_adapter")]
async fn http_adapter(item: web::Json<SubmitSm>) -> impl Responder {
    let data = item.into_inner();
    info!("Received POST request to /esme/http_adapter with data: {:?}", data);
    HttpResponse::Ok().json(data) // Echoes back the received JSON
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(http_adapter)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
