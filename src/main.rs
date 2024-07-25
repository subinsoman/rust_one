mod config;
mod handlers;
mod sqlx;
mod utils;

use actix_web::{HttpServer, App};
use log::{info};
//use crate::sqlx::db::establish_connection;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    //establish_connection().await.expect("Failed to create database pool");
    // Load configuration
    let config = config::application_config::load_config().await.expect("Failed to load configuration");
    let address = config.server.address;
    let port = config.server.port;

    info!("Starting server at http://{}:{}", address, port);

    HttpServer::new(|| {
        App::new()
            .service(handlers::submit_sm::http_adapter)
    })
        .bind((address, port))?
        .run()
        .await
}
