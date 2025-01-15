mod get_report;
mod get_xml_by_report;
use crate::get_report::get_report;
use dotenv::dotenv;
fn main() {
    dotenv().ok();

    let (url, username, password, report_path) = resolve_parameters();

    let output_format = "xml";
    let params = vec![("P_DEPARTMENT_ID", "80")];
    let result = get_report(&*url, &*username, &*password, &*report_path, output_format, params);
    let test = result.unwrap();
    std::fs::write("test.xml", test).unwrap();
}

fn resolve_parameters() -> (String, String, String, String) {
    let url = dotenv::var("url").expect("erro ao obter url");
    let username = dotenv::var("username").expect("erro ao obter usuario");
    let password = dotenv::var("password").expect("erro ao obter a senha");
    let report_path = dotenv::var("report").expect("erro ao obter o endereco do report");

    (url, username, password, report_path)
}
