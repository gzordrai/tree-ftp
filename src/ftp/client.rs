use log::info;

use crate::ftp::stream::{FtpCommand, FtpStream};
use crate::ftp::error::Result;

pub struct FtpClient {
    stream: FtpStream,
}

impl FtpClient {
    pub fn new(addr: &str) -> Result<Self> {
        let stream: FtpStream = FtpStream::new(addr)?;

        Ok(FtpClient { stream })
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<()> {
        info!("Starting authentication");

        self.stream.send_command(FtpCommand::User(username.to_string()))?;
        self.stream.send_command(FtpCommand::Pass(password.to_string()))?;

        info!("Authentication successful");

        Ok(())
    }

    pub fn retrieve_server_info(&mut self) -> Result<()> {
        info!("Retrieving server information");

        self.stream.send_command(FtpCommand::Syst)?;
        self.stream.send_command(FtpCommand::Feat)?;
        self.stream.send_command(FtpCommand::Pwd)?;
    
        info!("Server information retrieved");

        Ok(())
    }
}