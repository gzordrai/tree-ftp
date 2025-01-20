extern crate dotenv;

mod fs;
mod ftp;
mod utils;

use clap::Parser;
use dotenv::dotenv;
use fs::node::NodeEnum;
use ftp::client::FtpClient;
use log::info;
use serde_json::to_string;
use std::env;
use utils::{parser::Args, validator::DomainAllowPort};
use validators::traits::ValidateString;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let debug_level: String = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    env::set_var("RUST_LOG", debug_level.clone());
    env_logger::init();

    info!("Debug level: {}", debug_level);

    let args: Args = Args::parse();
    let parsed_addr: DomainAllowPort = DomainAllowPort::parse_str(&args.address).unwrap();
    let addr_with_port: String = if let Some(port) = parsed_addr.port {
        format!("{}:{}", parsed_addr.domain, port)
    } else {
        format!("{}:21", parsed_addr.domain)
    };

    let mut client: FtpClient = FtpClient::new(&addr_with_port)?;

    client.authenticate(&args.username, &args.password)?;
    client.retrieve_server_info()?;
    client.passive_mode()?;

    let root: NodeEnum = client.list_dir(args.depth)?;

    if args.json {
        println!("{}", to_string(&root).unwrap());
    } else {
        println!("{}", root.to_string(""));
    }

    Ok(())
}
