mod get_report;
mod get_xml_by_report;
mod save_to_db;
mod soap_operations;
mod ollama_api;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize the server
    HttpServer::new(|| {
        App::new()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
