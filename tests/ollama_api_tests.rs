use super::ollama_api::OllamaApi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let host = "localhost".to_string();
        let port = 8080;
        let api = OllamaApi::new(host.clone(), port);
        assert_eq!(api.host, host);
        assert_eq!(api.port, port);
    }

    #[test]
    fn test_send_message() {
        let host = "localhost".to_string();
        let port = 8080;
        let api = OllamaApi::new(host, port);
        let message = "Hello, Ollama!";
        let response = api.send_message(message);
        assert!(response.is_ok());
        assert_eq!(response.unwrap(), "Expected response from Ollama API");
    }
}
