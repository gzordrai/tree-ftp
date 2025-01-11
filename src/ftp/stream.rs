use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use log::{debug, info};

use crate::ftp::error::Result;

pub enum FtpCommand {
    User(String),
    Pass(String),
    Syst,
    Feat,
    Pwd,
    Type(String),
    Pasv,
    List
}


pub struct FtpStream {
    stream: TcpStream,
}

pub struct  FtpDataStream {
    stream: TcpStream
}

impl FtpStream {
    pub fn new(addr: &str) -> Result<Self> {
        let stream: TcpStream = TcpStream::connect(addr)?;

        info!("Connected to the server");

        let mut ftp_stream: FtpStream = FtpStream { stream };
        let response: String = ftp_stream.read_response()?;

        info!("Server response: {}", response);

        Ok(ftp_stream)
    }

    pub fn read_response(&mut self) -> Result<String> {
        let mut reader: BufReader<&TcpStream> = BufReader::new(&self.stream);
        let mut response: String = String::new();

        loop {
            let mut line: String = String::new();
            let bytes_read: usize = reader.read_line(&mut line)?;

            if bytes_read == 0 {
                break; 
            }

            debug!("Read line: {}", line.trim_end());

            response.push_str(&line);

            // Check if the line is part of a multi-line response
            if line.starts_with(" ") || line.starts_with("\t") {
                continue;
            }

            // Check if the line is the last one of a multi-line response
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
            FtpCommand::List => "LIST\r\n".to_string()
        }
    }

    pub fn send_command(&mut self, cmd: FtpCommand) -> Result<String> {
        let command_str: String = FtpStream::format_command(cmd);

        debug!("Sending command: {}", command_str.trim_end());

        self.stream.write(command_str.as_bytes())?;
        self.stream.flush()?;

        debug!("Command flushed: {}", command_str.trim_end());

        let response: String = self.read_response()?;

        debug!("Server response: {}", response);

        Ok(response)
    }
}

impl FtpDataStream {
    pub fn new(addr: &str) -> Result<Self> {
        let stream: TcpStream = TcpStream::connect(addr)?;

        info!("Connected to the data server");

        Ok(FtpDataStream { stream })
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

            // Check if the line is part of a multi-line response
            if line.starts_with(" ") || line.starts_with("\t") {
                continue;
            }

            // Check if the line is the last one of a multi-line response
            if line.len() >= 4 && &line[3..4] == " " {
                break;
            }
        }

        debug!("Full response: {:?}", response_lines);

        Ok(response_lines)
    }
}