use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
};

use super::stream::{Responses, Stream};
use crate::ftp::error::Result;
use crate::ftp::{command::FtpCommand, error::Error};
use log::{debug, error, info};

pub struct CommandStream {
    addr: SocketAddr,
    stream: TcpStream,
    reconnected: bool,
}

impl CommandStream {
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let stream: TcpStream = TcpStream::connect(addr).map_err(|_| Error::ConnectionError)?;

        info!("Connected to the server");

        Ok(CommandStream {
            addr,
            stream,
            reconnected: false,
        })
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
            FtpCommand::Cdup => "CDUP\r\n".to_string(),
        }
    }

    pub fn send_command(&mut self, cmd: FtpCommand) -> Result<Responses> {
        let command_str: String = CommandStream::format_command(cmd.clone());

        debug!("Sending command: {}", command_str.trim_end());

        match self.stream.write(command_str.as_bytes()) {
            Ok(_) => {
                self.stream.flush().map_err(|_| Error::CommandFlushError)?;

                debug!("Command flushed: {}", command_str.trim_end());

                match cmd {
                    FtpCommand::List => {
                        let mut responses: Responses = self.read_responses()?;

                        if !self.is_reconnected() {
                            let mut additional_responses: Responses = self.read_responses()?;

                            responses.append(&mut additional_responses);
                        }

                        Ok(responses)
                    }
                    _ => Ok(self.read_responses()?),
                }
            }
            Err(e) => {
                if let Some(10053) = e.raw_os_error() {
                    error!("Connection was aborted by the software in your host machine. Attempting to reconnect...");

                    self.reconnect()?;

                    return Ok(Vec::new());
                } else {
                    error!("Error writing command");
                }

                return Err(Error::CommandWriteError);
            }
        }
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

    fn set_reconnected(&mut self, reconnected: bool) {
        self.reconnected = reconnected;
    }

    fn is_reconnected(&mut self) -> bool {
        self.reconnected
    }
}
