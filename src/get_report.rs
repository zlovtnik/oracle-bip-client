use std::io::Read;
use flate2::bufread::GzDecoder;
use crate::get_xml_by_report::get_xml;

pub fn get_report(url: &str, username: &str, password: &str, report_path: &str, output_format: &str, params: Vec<(&str, &str)>)
                  -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let body = get_xml(report_path, output_format, params);

    let res = client
        .post(url)
        .header("Content-Type", "text/xml")
        .header("SOAPAction", "runReport")
        .header("Authorization", format!("Basic {}", base64::encode(format!("{}:{}", username, password))))
        .body(body)
        .send()?;

    let body = res.bytes()?;

    let mut d = GzDecoder::new(&body[..]);
    let mut decompressed_body = Vec::new();
    d.read_to_end(&mut decompressed_body)?;

    let decompressed_body = std::str::from_utf8(&decompressed_body)?;
    println!("{:?}", decompressed_body);
    let doc = roxmltree::Document::parse(&decompressed_body)?;
    let node = doc.root_element();
    let ns = node.lookup_namespace_uri(None);
    let mut result = Vec::new();
    for elem in node.descendants() {
        if elem.has_tag_name("reportBytes") {
            let text = elem.text().unwrap();
            result = base64::decode(text)?;
        }
    }
    Ok(result)
}