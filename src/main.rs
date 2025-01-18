mod get_report;
mod get_xml_by_report;
mod save_to_db;
mod soap_operations;

use crate::get_report::get_report;
use crate::save_to_db::save_to_db;
use dotenv::dotenv;
use std::env;
use log::{info, error};
use env_logger;

fn main() {
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

    info!("End of main function");
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
