//! SOAP operations for communicating with Oracle BI Publisher services.
//!
//! This module handles SOAP request transmission and response processing
//! for the Oracle BI Publisher API, including HTTP operations and XML parsing.

use std::io::Read;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use flate2::bufread::GzDecoder;
use reqwest::blocking::Client;
use roxmltree::Document;
use log::{info, debug};

/// Sends a SOAP request to Oracle BI Publisher and returns the decompressed response.
///
/// This function handles the HTTP communication with the BI Publisher SOAP API,
/// including authentication and content type headers. It also handles GZIP
/// decompression of the response data.
///
/// # Arguments
///
/// * `url` - The URL of the Oracle BI Publisher SOAP API endpoint
/// * `username` - The username for authentication
/// * `password` - The password for authentication
/// * `body` - The XML SOAP request body as a String
///
/// # Returns
///
/// A Result containing either:
/// * `Vec<u8>` - The decompressed response bytes on success
/// * `Box<dyn std::error::Error>` - An error if the request or decompression fails
///
/// # Errors
///
/// This function will return an error if:
/// * The HTTP request fails
/// * The response cannot be read
/// * The response cannot be decompressed
pub fn send_soap_request(url: &str, username: &str, password: &str, body: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    info!("Sending SOAP request to URL: {}", url);
    let client = Client::new();

    let res = client
        .post(url)
        .header("Content-Type", "text/xml")
        .header("SOAPAction", "runReport")
        .header("Authorization", format!("Basic {}", STANDARD.encode(format!("{}:{}", username, password))))
        .body(body)
        .send()?;

    let body = res.bytes()?;
    debug!("SOAP response body: {:?}", body);

    let mut d = GzDecoder::new(&body[..]);
    let mut decompressed_body = Vec::new();
    d.read_to_end(&mut decompressed_body)?;
    debug!("Decompressed SOAP response body: {:?}", decompressed_body);

    Ok(decompressed_body)
}

/// Processes a decompressed SOAP response from Oracle BI Publisher.
///
/// This function parses the XML response, extracts the report content from the
/// `reportBytes` element, and decodes it from Base64.
///
/// # Arguments
///
/// * `decompressed_body` - The decompressed SOAP response as bytes
///
/// # Returns
///
/// A Result containing either:
/// * `Vec<u8>` - The decoded report content as bytes on success
/// * `Box<dyn std::error::Error>` - An error if parsing or decoding fails
///
/// # Errors
///
/// This function will return an error if:
/// * The byte data cannot be converted to a UTF-8 string
/// * The XML cannot be parsed
/// * The `reportBytes` element is not found
/// * The Base64 decoding fails
pub fn process_soap_response(decompressed_body: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let decompressed_body = std::str::from_utf8(&decompressed_body)?;
    println!("{:?}", decompressed_body);
    let doc = Document::parse(&decompressed_body)?;
    let node = doc.root_element();
    let _ns = node.lookup_namespace_uri(None);
    let mut result = Vec::new();
    for elem in node.descendants() {
        if elem.has_tag_name("reportBytes") {
            let text = elem.text().unwrap();
            result = STANDARD.decode(text)?;
        }
    }
    Ok(result)
}
