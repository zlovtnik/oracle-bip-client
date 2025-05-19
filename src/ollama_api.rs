use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApi {
    host: String,
    port: u16,
}

impl OllamaApi {
    pub fn new(host: String, port: u16) -> Self {
        OllamaApi { host, port }
    }

    pub async fn send_message(&self, message: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let url = format!("http://{}:{}/api/chat", self.host, self.port);
        let client = reqwest::Client::new();
        let response = client.post(&url)
            .body(message.to_string())
            .send()
            .await?;
        let response_text = response.text().await?;
        Ok(response_text)
    }
}
#[allow(dead_code)]
pub async fn select_model(model_name: &str) -> String {
    format!("Selected model: {}", model_name)
}

#[allow(dead_code)]
pub fn create_ai_models(json_directory: &str) {
    let paths = fs::read_dir(json_directory).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() && path.extension().unwrap() == "json" {
            let _content = fs::read_to_string(&path).unwrap();
            println!("Creating AI model from file: {:?}", path);
        }
    }
}
