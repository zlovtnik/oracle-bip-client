//! Interface for interacting with Ollama API services.
//!
//! This module provides functionality to communicate with Ollama AI services,
//! including sending messages, selecting models, and creating AI models from
//! JSON configuration files.

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

/// Client for interacting with Ollama API services.
///
/// Provides methods to connect to and communicate with an Ollama API server.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApi {
    /// The hostname or IP address of the Ollama API server
    host: String,
    /// The port number of the Ollama API server
    port: u16,
}

impl OllamaApi {
    /// Creates a new OllamaApi client instance.
    ///
    /// # Arguments
    ///
    /// * `host` - The hostname or IP address of the Ollama API server
    /// * `port` - The port number of the Ollama API server
    ///
    /// # Returns
    ///
    /// A new OllamaApi instance configured with the specified host and port
    pub fn new(host: String, port: u16) -> Self {
        OllamaApi { host, port }
    }

    /// Sends a message to the Ollama API server.
    ///
    /// This method sends a chat message to the API server and returns the response.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send to the API
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A String with the API response text on success
    /// * An error if the API request fails
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The HTTP request cannot be sent
    /// * The response cannot be parsed
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
/// Selects an AI model by name.
///
/// This function is currently a placeholder that returns a formatted string
/// indicating which model was selected.
///
/// # Arguments
///
/// * `model_name` - The name of the model to select
///
/// # Returns
///
/// A string confirming the model selection
#[allow(dead_code)]
pub async fn select_model(model_name: &str) -> String {
    format!("Selected model: {}", model_name)
}

/// Creates AI models from JSON configuration files.
///
/// This function reads all JSON files in the specified directory and creates
/// AI models based on their configurations. Currently, it only logs which files
/// it would process but doesn't actually create models.
///
/// # Arguments
///
/// * `json_directory` - Path to the directory containing JSON model configuration files
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
