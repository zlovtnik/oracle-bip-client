//! Main module for retrieving reports from Oracle BI Publisher.
//!
//! This module provides the primary interface for requesting and receiving
//! reports from an Oracle BI Publisher server via SOAP API.

use crate::get_xml_by_report::get_xml;
use crate::soap_operations::{send_soap_request, process_soap_response};
use log::{info, debug};

/// Retrieves a report from Oracle BI Publisher.
///
/// This function orchestrates the complete process of generating a report from Oracle BI Publisher:
/// 1. Generates the SOAP XML request
/// 2. Sends the request to the BI Publisher server
/// 3. Processes the SOAP response
/// 4. Returns the report content as bytes
///
/// # Arguments
///
/// * `url` - The URL of the Oracle BI Publisher SOAP API endpoint
/// * `username` - The username for authentication
/// * `password` - The password for authentication
/// * `report_path` - The absolute path to the report in the BI Publisher catalog
/// * `output_format` - The desired output format (e.g., "pdf", "xlsx", "html")
/// * `params` - A vector of key-value pairs representing report parameters
///
/// # Returns
///
/// A Result containing either:
/// * `Vec<u8>` - The report content as bytes on success
/// * `Box<dyn std::error::Error>` - An error if any step in the process fails
///
/// # Example
///
/// ```
/// let params = vec![
///     ("DEPARTMENT_ID", "10".to_string())
/// ];
/// match get_report(
///     "https://bipserver.example.com/xmlpserver/services/PublicReportService",
///     "username", 
///     "password", 
///     "/path/to/report", 
///     "pdf", 
///     params
/// ) {
///     Ok(report_data) => { /* handle report data */ },
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
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
