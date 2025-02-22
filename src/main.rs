extern crate dotenv;
extern crate libc;

mod fs;
mod ftp;
mod utils;

use clap::Parser;
use dotenv::dotenv;
use fs::node::{NodeEnum, TraversalType};
use ftp::client::FtpClient;
use log::info;
use serde_json::to_string;
use std::{env, fs::File, io::Write, net::SocketAddr};
use utils::{domain::resolve_domain_to_socket_addr, parser::Args, validator::DomainAllowPort};
use validators::traits::ValidateString;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let debug_level: String = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    env::set_var("RUST_LOG", debug_level.clone());
    env_logger::init();

    info!("Debug level: {}", debug_level);

    let args: Args = Args::parse();
    let domain: DomainAllowPort = DomainAllowPort::parse_str(&args.address).unwrap();
    let socket_addr: SocketAddr = resolve_domain_to_socket_addr(&domain)?;
    let mut client: FtpClient = FtpClient::new(socket_addr, &args.username, &args.password, args.extended)?;
    let root: NodeEnum = client.list_dir(args.depth, args.bfs)?;

    if args.json {
        let json = to_string(&root).unwrap();
        let mut file = File::create("output.json").unwrap();

        file.write_all(json.as_bytes()).unwrap();

        println!("JSON file created successfully.");
    } else {
        if args.bfs {
            println!("{}", root.to_string("", TraversalType::BFS));
        } else {
            println!("{}", root.to_string("", TraversalType::DFS));
        }
    }

    Ok(())
}