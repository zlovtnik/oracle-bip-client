mod get_report;
mod get_xml_by_report;
use std::io::Read;
use crate::get_report::get_report;

fn main() {
    let url = "";
    let username = "";
    let password = "";
    let report_path = "";
    let output_format = "xml";
    let params = vec![("P_DEPARTMENT_ID", "80")];
    let result = get_report(url, username, password, report_path, output_format, params);
    let test = result.unwrap();
    std::fs::write("test.xml", test).unwrap();
}