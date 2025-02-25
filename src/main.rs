mod get_report;
mod get_xml_by_report;
mod save_to_db;
mod soap_operations;
mod ollama_api;

use crate::get_report::get_report;
use crate::save_to_db::save_to_db;
use crate::ollama_api::{OllamaApi, select_model, create_ai_models};
use dotenv::dotenv;
use std::env;
use log::info;
use env_logger;
use actix_web::{web, App, HttpServer, Responder};

async fn llama_route() -> impl Responder {
    let ollama_host = dotenv::var("ollama_host").expect("erro ao obter ollama_host");
    let ollama_port = dotenv::var("ollama_port").expect("erro ao obter ollama_port").parse().expect("erro ao converter ollama_port para u16");
    let ollama_api = OllamaApi::new(ollama_host, ollama_port);
    let message = "Hello, Ollama!";
    match ollama_api.send_message(message) {
        Ok(response) => format!("Received response from Ollama API: {}", response),
        Err(e) => format!("Failed to send message to Ollama API: {}", e),
    }
}

#[tokio::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting main function");

    dotenv().ok();

    let (url, username, password, report_path, params) = resolve_parameters();

    let output_format = "xml";
    let result = get_report(&*url, &*username, &*password, &*report_path, output_format, params);
    let test = result.unwrap();
    std::fs::write("test.xml", test).unwrap();

    let db_url = dotenv::var("db_url").expect("erro ao obter db_url");
    let report_data = std::fs::read_to_string("test.xml").expect("erro ao ler o arquivo de relatório");
    save_to_db(&db_url, &report_data).expect("erro ao salvar no banco de dados");

    let ollama_host = dotenv::var("ollama_host").expect("erro ao obter ollama_host");
    let ollama_port = dotenv::var("ollama_port").expect("erro ao obter ollama_port").parse().expect("erro ao converter ollama_port para u16");
    let ollama_api = OllamaApi::new(ollama_host, ollama_port);
    let message = "Hello, Ollama!";
    match ollama_api.send_message(message) {
        Ok(response) => println!("Received response from Ollama API: {}", response),
        Err(e) => eprintln!("Failed to send message to Ollama API: {}", e),
    }

    // Call the new model selection functionality
    let model = select_model("model_name");
    println!("Selected model: {}", model);

    // Create new AI models based on a directory with JSON files
    create_ai_models("path/to/json/directory");

    info!("End of main function");

    HttpServer::new(|| {
        App::new()
            .route("/llama", web::get().to(llama_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn resolve_parameters() -> (String, String, String, String, Vec<(&'static str, String)>) {
    let url = dotenv::var("url").expect("erro ao obter url");
    let username = dotenv::var("username").expect("erro ao obter usuario");
    let password = dotenv::var("password").expect("erro ao obter a senha");
    let report_path = dotenv::var("report").expect("erro ao obter o endereco do report");

    let mut params = Vec::new();
    for (key, value) in env::vars() {
        if key.starts_with("P_") {
            let key: &'static str = Box::leak(key.into_boxed_str());
            params.push((key, value));
        }
    }

    (url, username, password, report_path, params)
}
