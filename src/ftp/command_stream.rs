use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use log::{debug, info};
use crate::ftp::error::Result;
use crate::ftp::command::FtpCommand;

pub struct CommandStream {
    stream: TcpStream,
}

impl CommandStream {
    pub fn new(addr: &str) -> Result<Self> {
        let stream: TcpStream = TcpStream::connect(addr)?;
        info!("Connected to the server");
        Ok(CommandStream { stream })
    }

    pub fn read_response(&mut self) -> Result<String> {
        let mut reader = BufReader::new(&self.stream);
        let mut response = String::new();

        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;

            if bytes_read == 0 {
                break;
            }

            debug!("Read line: {}", line.trim_end());
            response.push_str(&line);

            if line.len() >= 4 && &line[3..4] == " " {
                break;
            }
        }

        debug!("Full response: {}", response.trim_end());
        Ok(response)
    }

    pub fn send_command(&mut self, cmd: FtpCommand) -> Result<String> {
        let command_str = match cmd {
            FtpCommand::User(username) => format!("USER {}\r\n", username),
            FtpCommand::Pass(password) => format!("PASS {}\r\n", password),
            FtpCommand::Syst => "SYST\r\n".to_string(),
            FtpCommand::Feat => "FEAT\r\n".to_string(),
            FtpCommand::Pwd => "PWD\r\n".to_string(),
            FtpCommand::Type(t) => format!("TYPE {}\r\n", t),
            FtpCommand::Pasv => "PASV\r\n".to_string(),
            FtpCommand::List => "LIST\r\n".to_string(),
        };

        debug!("Sending command: {}", command_str.trim_end());
        self.stream.write(command_str.as_bytes())?;
        self.stream.flush()?;
        debug!("Command flushed: {}", command_str.trim_end());

        self.read_response()
    }
}