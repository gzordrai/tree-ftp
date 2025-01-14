extern crate dotenv;

mod fs;
mod ftp;
mod utils;

use dotenv::dotenv;
use ftp::client::FtpClient;
use log::info;
use std::env;
use utils::DomainAllowPort;
use validators::traits::ValidateString;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let debug_level: String = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    env::set_var("RUST_LOG", debug_level.clone());
    env_logger::init();

    info!("Debug level: {}", debug_level);

    let addr: String = env::args().nth(1).expect("Please provide an address");
    let parsed_addr: DomainAllowPort = DomainAllowPort::parse_str(&addr).unwrap();
    let addr_with_port: String = if let Some(port) = parsed_addr.port {
        format!("{}:{}", parsed_addr.domain, port)
    } else {
        format!("{}:21", parsed_addr.domain)
    };

    let mut client: FtpClient = FtpClient::new(&addr_with_port)?;

    client.authenticate("anonymous", "anonymous")?;
    client.retrieve_server_info()?;
    client.passive_mode()?;
    client.list_dir();

    Ok(())
}
