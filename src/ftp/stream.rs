use std::{net::{SocketAddr, TcpStream}, thread::sleep, time::{Duration, Instant}};

use log::{error, info};


use crate::ftp::error::Result;
use log::debug;
use std::io::{BufRead, BufReader};

pub type Response = (u16, String);
pub type Responses = Vec<Response>;

pub trait Stream {
    fn get_stream(&self) -> &TcpStream;
    fn get_addr(&self) -> SocketAddr;
    fn set_stream(&mut self, stream: TcpStream);
    fn set_reconnected(&mut self, reconnected: bool);
    fn is_reconnected(&mut self) -> bool;

    fn reconnect(&mut self) -> Result<bool> {
        let addr: SocketAddr = self.get_addr();
        let start_time: Instant = Instant::now();
        let timeout: Duration = Duration::from_secs(300); // 5 minutes
        let retry_interval: Duration = Duration::from_secs(5); // Retry every 5 seconds

        sleep(Duration::from_secs(5)); // For localhost docker ftp

        while start_time.elapsed() < timeout {
            match TcpStream::connect(addr) {
                Ok(new_stream) => {
                    self.set_stream(new_stream);

                    info!("Reconnected to the server at {}", addr);

                    self.set_reconnected(true);
                    self.read_responses()?;

                    return Ok(true);
                },
                Err(_) => {
                    error!("Failed to reconnect to the server at {}. Retrying in 5 seconds...", addr);

                    sleep(retry_interval);
                },
            }
        }

        error!("Failed to reconnect to the server at {} after 5 minutes", addr);
        Err("Failed to reconnect to the server".into())
    }

    fn read_responses(&mut self) -> Result<Vec<Response>> {
        let stream: &TcpStream = self.get_stream();
        let mut reader: BufReader<&TcpStream> = BufReader::new(stream);
        let mut responses: Vec<Response> = Vec::new();

        loop {
            let mut line: String = String::new();
            let bytes_read: usize = match reader.read_line(&mut line) {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    error!("Error reading response: {}. Attempting to reconnect...", e);

                    self.reconnect()?;

                    return Ok(Vec::new())
                }
            };

            if bytes_read == 0 {
                break;
            }

            debug!("Read line: {}", line.trim_end());

            let code = if line.len() >= 3 {
                line[0..3].parse::<u16>().unwrap_or(0)
            } else {
                0
            };

            responses.push((code, line.trim_end().to_string()));

            if line.len() >= 4 && &line[3..4] == " " {
                break;
            }
        }

        debug!("Full response: {:?}", responses);

        Ok(responses)
    }
}
