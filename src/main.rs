mod config;
mod handlers;
mod sqlx;
mod tant;

mod utils;

use actix_web::{App, HttpServer};
use log::{info, log};
use tant::tantivy_lib::SEARCH_ENGINE;
//use crate::sqlx::db::establish_connection;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate log;
extern crate log4rs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env_logger::init();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let config = config::application_config::load_config()
        .await
        .expect("Failed to load configuration");
    let search_engine = SEARCH_ENGINE.clone();

    let mut documents = Vec::new();

    let path = Path::new("output.txt");

    // Open the file in read-only mode
    let file = File::open(&path)?;

    // Create a buffered reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Iterate over each line in the file
    for (line_number, line) in reader.lines().enumerate(){
        // Unwrap the result and print the line
        let line = line?;

        let title =  line_number.to_string();
        let body=line;
        documents.push((title, body));
    }
    let duration = SEARCH_ENGINE.add_documents("index1", &documents).unwrap();
    println!("Time taken to add documents: {:?}", duration);



    let address = config.server.address;
    let port = config.server.port;

    info!("Starting server at http://{}:{}", address, port);

    HttpServer::new(|| App::new().service(handlers::submit_sm::http_adapter))
        .workers(8) // Number of worker threads
        .bind((address, port))?
       // .keep_alive(30) // Keep-Alive duration in seconds
        .run()
        .await
}
