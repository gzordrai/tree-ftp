use std::{net::{SocketAddr, TcpStream}, thread::sleep, time::{Duration, Instant}};

use log::{error, info};


use crate::ftp::error::Result;
use log::debug;
use std::io::{BufRead, BufReader};

pub trait Stream {
    fn get_stream(&self) -> &TcpStream;
    fn get_addr(&self) -> SocketAddr;
    fn set_stream(&mut self, stream: TcpStream);

    fn reconnect(&mut self) -> Result<()> {
        let addr: SocketAddr = self.get_addr();
        let start_time: Instant = Instant::now();
        let timeout: Duration = Duration::from_secs(300); // 5 minutes
        let retry_interval: Duration = Duration::from_secs(5); // Retry every 5 seconds

        while start_time.elapsed() < timeout {
            match TcpStream::connect(addr) {
                Ok(new_stream) => {
                    self.set_stream(new_stream);

                    info!("Reconnected to the server at {}", addr);

                    return Ok(());
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

    fn read_responses(&mut self) -> Result<Vec<String>> {
        let stream: &TcpStream = self.get_stream();
        let mut reader: BufReader<&TcpStream> = BufReader::new(stream);
        let mut response_lines: Vec<String> = Vec::new();

        loop {
            let mut line: String = String::new();
            let bytes_read: usize = reader.read_line(&mut line)?;

            if bytes_read == 0 {
                break;
            }

            debug!("Read line: {}", line.trim_end());

            response_lines.push(line.trim_end().to_string());

            if line.len() >= 4 && &line[3..4] == " " {
                break;
            }
        }

        debug!("Full response: {:?}", response_lines);

        Ok(response_lines)
    }
}
