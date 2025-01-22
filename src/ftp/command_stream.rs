use std::{
    io::{BufRead, BufReader, Write}, net::{SocketAddr, TcpStream}
};

use log::{debug, error, info};
use crate::ftp::error::Result;
use crate::ftp::command::FtpCommand;

use super::reconnectable::Reconnectable;

pub struct CommandStream {
    addr: SocketAddr,
    stream: TcpStream,
}

impl CommandStream {
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let stream: TcpStream = TcpStream::connect(addr)?;

        info!("Connected to the server");

        Ok(CommandStream { addr, stream })
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

    fn format_command(cmd: FtpCommand) -> String {
        match cmd {
            FtpCommand::User(username) => format!("USER {}\r\n", username),
            FtpCommand::Pass(password) => format!("PASS {}\r\n", password),
            FtpCommand::Syst => "SYST\r\n".to_string(),
            FtpCommand::Feat => "FEAT\r\n".to_string(),
            FtpCommand::Pwd => "PWD\r\n".to_string(),
            FtpCommand::Type(t) => format!("TYPE {}\r\n", t),
            FtpCommand::Pasv => "PASV\r\n".to_string(),
            FtpCommand::List => "LIST\r\n".to_string(),
            FtpCommand::Cwd(path) => format!("CWD {}\r\n", path),
            FtpCommand::Cdup => "CDUP\r\n".to_string()
        }
    }

    pub fn send_command(&mut self, cmd: FtpCommand) -> Result<String> {
        let command_str: String = CommandStream::format_command(cmd);

        debug!("Sending command: {}", command_str.trim_end());

        match self.stream.write(command_str.as_bytes()) {
            Ok(_) => self.stream.flush()?,
            Err(e) => {
                error!("{}", e);

                self.reconnect()?;
                self.stream.write(command_str.as_bytes())?;
                self.stream.flush()?;
            }
        }

        debug!("Command flushed: {}", command_str.trim_end());

        self.read_response()
    }
}

impl Reconnectable for CommandStream {
    fn reconnect(&mut self) -> Result<()> {
        let _ = &self.stream;

        self.stream = TcpStream::connect(self.addr)?;

        info!("Reconnected to the server at {}", self.addr);

        Ok(())
    }
}