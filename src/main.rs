extern crate dotenv;

mod fs;
mod ftp;

use dotenv::dotenv;
use ftp::client::FtpClient;
use log::info;
use regex::Regex;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let debug_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", debug_level.clone());
    env_logger::init();

    info!("Debug level: {}", debug_level);

    let addr: String = env::args().nth(1).expect("Please provide an address");

    if !is_valid_ip_or_domain(&addr) {
        panic!("Invalid address format: {}", addr);
    }

    let addr_with_port: String = format!("{}:21", addr);
    let mut client: FtpClient = FtpClient::new(&addr_with_port)?;

    client.authenticate("anonymous", "anonymous")?;
    client.retrieve_server_info()?;
    client.passive_mode()?;
    client.list_dir();

    Ok(())
}

fn is_valid_ip_or_domain(addr: &str) -> bool {
    let ip_regex: Regex = Regex::new(r"^(\d{1,3}\.){3}\d{1,3}$").unwrap();
    let domain_regex: Regex =
        Regex::new(r"^([a-zA-Z0-9]+(-[a-zA-Z0-9]+)*\.)+[a-zA-Z]{2,}$").unwrap();

    return ip_regex.is_match(addr) || domain_regex.is_match(addr);
}
