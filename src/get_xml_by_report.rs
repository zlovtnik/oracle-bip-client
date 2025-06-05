//! Functions for generating SOAP XML for Oracle BI Publisher report requests.

use xml::writer::{EmitterConfig, XmlEvent};
use std::str::from_utf8;
use log::{info, debug};

/// Generates a SOAP XML request for an Oracle BI Publisher report.
///
/// This function creates a properly formatted SOAP envelope containing
/// the necessary parameters to request a report from Oracle BI Publisher.
///
/// # Arguments
///
/// * `report_path` - The absolute path to the report in the Oracle BI Publisher catalog
/// * `output_format` - The desired output format (e.g., "pdf", "xlsx", "html")
/// * `params` - A vector of key-value pairs representing the report parameters
///
/// # Returns
///
/// A String containing the complete XML SOAP request
///
/// # Example
///
/// ```
/// let params = vec![
///     ("DEPARTMENT_ID", "10".to_string()),
///     ("EMPLOYEE_ID", "7566".to_string())
/// ];
/// let xml = get_xml("/path/to/report", "pdf", params);
/// ```
pub fn get_xml(report_path: &str, output_format: &str, params: Vec<(&str, String)>) -> String {
    info!("Generating XML for report: {}", report_path);
    let mut buffer = Vec::new();
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut buffer);

    writer.write(XmlEvent::start_element("soapenv:Envelope")
        .attr("xmlns:soapenv", "http://schemas.xmlsoap.org/soap/envelope/")
        .attr("xmlns:pub", "http://xmlns.oracle.com/oxp/service/PublicReportService")).unwrap();
    writer.write(XmlEvent::start_element("soapenv:Header")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::start_element("soapenv:Body")).unwrap();
    writer.write(XmlEvent::start_element("pub:runReport")).unwrap();
    writer.write(XmlEvent::start_element("pub:reportRequest")).unwrap();
    writer.write(XmlEvent::start_element("pub:attributeFormat")).unwrap();
    writer.write(XmlEvent::characters(output_format)).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::start_element("pub:reportAbsolutePath")).unwrap();
    writer.write(XmlEvent::characters(report_path)).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::start_element("pub:sizeOfDataChunkDownload")).unwrap();
    writer.write(XmlEvent::characters("-1")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::start_element("pub:parameterNameValues")).unwrap();
    for (key, value) in params {
        writer.write(XmlEvent::start_element("pub:item")).unwrap();
        writer.write(XmlEvent::start_element("pub:name")).unwrap();
        writer.write(XmlEvent::characters(key)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();
        writer.write(XmlEvent::start_element("pub:values")).unwrap();
        writer.write(XmlEvent::characters(&value)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();
    }
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    let xml_string = from_utf8(&buffer).unwrap().to_string();
    debug!("Generated XML: {}", xml_string);
    xml_string
}
