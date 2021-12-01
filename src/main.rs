mod parser;

use crate::parser::ArrisStatus;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    /// IP Address of Arris Cable Modem
    #[structopt(default_value = "192.168.100.1")]
    ip: String,
}

fn send_request(ip: String) -> String {
    let client_builder = reqwest::blocking::ClientBuilder::new();
    let client = client_builder.build().unwrap();

    let url = format!("http://{}/cgi-bin/status_cgi", ip);

    client.get(url).send().unwrap().text().unwrap()
}

fn main() {
    let args = Cli::from_args();

    let response = send_request(args.ip);
    let html = response.as_str();

    let parsed: ArrisStatus = parser::parse_request(html);
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
}
