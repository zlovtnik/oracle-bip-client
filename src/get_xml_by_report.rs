use xml::writer::{EmitterConfig, XmlEvent};
use std::str::from_utf8;

pub fn get_xml(report_path: &str, output_format: &str, params: Vec<(&str, &str)>) -> String {
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
        writer.write(XmlEvent::characters(value)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();
    }
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    from_utf8(&buffer).unwrap().to_string()

}