use crate::get_xml_by_report::get_xml;
use crate::soap_operations::{send_soap_request, process_soap_response};
use log::{info, debug};

pub fn get_report(url: &str, username: &str, password: &str, report_path: &str, output_format: &str, params: Vec<(&str, String)>)
                  -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    info!("Starting get_report function");
    let body = get_xml(report_path, output_format, params);
    debug!("SOAP request body: {}", body);
    let decompressed_body = send_soap_request(url, username, password, body)?;
    debug!("SOAP response body: {:?}", decompressed_body);
    let result = process_soap_response(decompressed_body)?;
    info!("End of get_report function");
    Ok(result)
}
