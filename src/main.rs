mod parser;

use crate::parser::ArrisStatus;

fn send_request() -> String {
    let client_builder = reqwest::blocking::ClientBuilder::new();
    let client = client_builder.build().unwrap();

    client
        .get("http://192.168.100.1/cgi-bin/status_cgi")
        .send()
        .unwrap()
        .text()
        .unwrap()
}

fn main() {
    let response = send_request();
    let html = response.as_str();

    let parsed: ArrisStatus = parser::parse_request(html);
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
}
