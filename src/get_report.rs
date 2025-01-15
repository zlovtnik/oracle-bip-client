use crate::get_xml_by_report::get_xml;
use crate::soap_operations::{send_soap_request, process_soap_response};

pub fn get_report(url: &str, username: &str, password: &str, report_path: &str, output_format: &str, params: Vec<(&str, String)>)
                  -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let body = get_xml(report_path, output_format, params);
    let decompressed_body = send_soap_request(url, username, password, body)?;
    let result = process_soap_response(decompressed_body)?;
    Ok(result)
}
