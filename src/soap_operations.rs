use std::io::Read;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use flate2::bufread::GzDecoder;
use reqwest::blocking::Client;
use roxmltree::Document;

pub fn send_soap_request(url: &str, username: &str, password: &str, body: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client
        .post(url)
        .header("Content-Type", "text/xml")
        .header("SOAPAction", "runReport")
        .header("Authorization", format!("Basic {}", STANDARD.encode(format!("{}:{}", username, password))))
        .body(body)
        .send()?;

    let body = res.bytes()?;

    let mut d = GzDecoder::new(&body[..]);
    let mut decompressed_body = Vec::new();
    d.read_to_end(&mut decompressed_body)?;

    Ok(decompressed_body)
}

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
