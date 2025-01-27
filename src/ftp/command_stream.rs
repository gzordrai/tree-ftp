use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
};

use log::{debug, error, info};
use crate::ftp::error::Result;
use crate::ftp::command::FtpCommand;
use super::stream::{Responses, Stream};

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

    fn format_command(cmd: FtpCommand) -> String {
        match cmd {
            FtpCommand::User(username) => format!("USER {}\r\n", username),
            FtpCommand::Pass(password) => format!("PASS {}\r\n", password),
            FtpCommand::Syst => "SYST\r\n".to_string(),
            FtpCommand::Feat => "FEAT\r\n".to_string(),
            FtpCommand::Pwd => "PWD\r\n".to_string(),
            FtpCommand::Type(t) => format!("TYPE {}\r\n", t),
            FtpCommand::Pasv => "PASV\r\n".to_string(),
            FtpCommand::Epsv => "EPSV\r\n".to_string(),
            FtpCommand::List => "LIST\r\n".to_string(),
            FtpCommand::Cwd(path) => format!("CWD {}\r\n", path),
            FtpCommand::Cdup => "CDUP\r\n".to_string()
        }
    }

    pub fn send_command(&mut self, cmd: FtpCommand) -> Result<Responses> {
        let command_str: String = CommandStream::format_command(cmd);

        debug!("Sending command: {}", command_str.trim_end());

        match self.stream.write(command_str.as_bytes()) {
            Ok(_) => self.stream.flush()?,
            Err(e) => {
                error!("Error writing command: {}. Attempting to reconnect...", e);

                self.reconnect()?;

                self.stream.write(command_str.as_bytes())?;
                self.stream.flush()?;
            }
        }

        debug!("Command flushed: {}", command_str.trim_end());

        Ok(self.read_responses()?)
    }
}

impl Stream for CommandStream {
    fn get_stream(&self) -> &TcpStream {
        &self.stream
    }

    fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    fn set_stream(&mut self, stream: TcpStream) {
        self.stream = stream;
    }
}