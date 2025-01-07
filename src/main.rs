use regex::Regex;
use std::borrow::Cow;
use std::env::args;
use std::io::Read;
use std::net::TcpStream;

fn main() {
    let addr: String = args().nth(1).expect("Please provide an address");

    if !is_valid_ip_or_domain(&addr) {
        panic!("Invalid address format: {}", addr);
    }

    let addr_with_port: String = format!("{}:21", addr);

    if let Ok(mut stream) = TcpStream::connect(addr_with_port) {
        println!("Connected to the server!");

        let mut buffer: [u8; 128] = [0; 128];

        match stream.read(&mut buffer) {
            Ok(size) => {
                let response: Cow<'_, str> = String::from_utf8_lossy(&buffer[..size]);
                println!("Read {} bytes: {}", size, response);
            }
            Err(e) => {
                println!("Failed to read from stream: {}", e);
            }
        }
    } else {
        println!("Couldn't connect to server...");
    }
}

fn is_valid_ip_or_domain(addr: &str) -> bool {
    let ip_regex: Regex = Regex::new(r"^(\d{1,3}\.){3}\d{1,3}$").unwrap();
    let domain_regex: Regex =
        Regex::new(r"^([a-zA-Z0-9]+(-[a-zA-Z0-9]+)*\.)+[a-zA-Z]{2,}$").unwrap();

    return ip_regex.is_match(addr) || domain_regex.is_match(addr);
}
