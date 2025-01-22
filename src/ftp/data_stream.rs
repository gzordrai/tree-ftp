use std::{
    io::{BufRead, BufReader},
    net::{SocketAddr, TcpStream},
};

use log::{debug, info};
use crate::ftp::error::Result;

use super::reconnectable::Reconnectable;

pub struct FtpDataStream {
    addr: SocketAddr,
    stream: TcpStream,
}

impl FtpDataStream {
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let stream: TcpStream = TcpStream::connect(addr)?;

        info!("Connected to the server");

        Ok(FtpDataStream { addr, stream })
    }

    pub fn read_response(&mut self) -> Result<Vec<String>> {
        let mut reader = BufReader::new(&self.stream);
        let mut response_lines = Vec::new();

        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;

            if bytes_read == 0 {
                break;
            }

            debug!("Read line: {}", line.trim_end());

            response_lines.push(line.trim_end().to_string());
        }

        debug!("Full response: {:?}", response_lines);

        Ok(response_lines)
    }
}

impl Reconnectable for FtpDataStream {
    fn reconnect(&mut self) -> Result<()> {
        

        Ok(())
    }
}