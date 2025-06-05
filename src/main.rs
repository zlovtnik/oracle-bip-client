//! Oracle BI Publisher Client Application
//!
//! A Rust application that interfaces with Oracle BI Publisher services to request
//! and process reports via SOAP API. The application provides a web server interface
//! for making report requests and handling responses.
//!
//! # Modules
//!
//! * `get_report` - Main functionality for retrieving reports
//! * `get_xml_by_report` - XML generation for report requests
//! * `soap_operations` - SOAP communication utilities
//! * `save_to_db` - Database operations for saving report data
//! * `ollama_api` - Interface for Ollama API interactions

mod get_report;
mod get_xml_by_report;
mod save_to_db;
mod soap_operations;
mod ollama_api;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;


/// Application entry point.
///
/// Sets up and runs the web server for the Oracle BI Publisher client application.
/// Loads environment variables from a .env file and initializes the HTTP server
/// on localhost port 8080.
///
/// # Returns
///
/// A Result indicating success or failure of the web server.
///
/// # Errors
///
/// Returns an error if the server fails to bind to the specified address or
/// encounters other IO-related issues.
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
