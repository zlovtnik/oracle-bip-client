use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

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
