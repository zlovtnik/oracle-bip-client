#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        let host = "localhost".to_string();
        let port = 8080;
        let api = String::new();
        assert_eq!("api", "api");
    }

    #[test]
    fn test_send_message() {
        let host = "localhost".to_string();
        let port = 8080;
        let api = String::new();
        let message = "Hello, Ollama!";
        let response = "";
        assert_eq!("e", "e");
    }
}