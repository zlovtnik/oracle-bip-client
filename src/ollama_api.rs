use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApi {
    host: String,
    port: u16,
}

impl OllamaApi {
    pub fn new(host: String, port: u16) -> Self {
        OllamaApi { host, port }
    }

    pub fn send_message(&self, message: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("http://{}:{}/api/chat", self.host, self.port);
        let client = Client::new();
        let response = client.post(&url).body(message.to_string()).send()?;
        let response_text = response.text()?;
        Ok(response_text)
    }
}

pub fn select_model(model_name: &str) -> String {
    // Placeholder for model selection logic
    format!("Selected model: {}", model_name)
}

pub fn create_ai_models(json_directory: &str) {
    let paths = fs::read_dir(json_directory).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() && path.extension().unwrap() == "json" {
            let file_content = fs::read_to_string(&path).unwrap();
            // Placeholder for AI model creation logic using file_content
            println!("Creating AI model from file: {:?}", path);
        }
    }
}
