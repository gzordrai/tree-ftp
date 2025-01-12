use log::{debug, info};


use crate::ftp::{
    command_stream::CommandStream,
    command::FtpCommand,
    data_stream::FtpDataStream,
    error::Result
};

pub struct FtpClient {
    ftp_stream: CommandStream,
    ftp_data_stream: Option<FtpDataStream>,
}

impl FtpClient {
    pub fn new(addr: &str) -> Result<Self> {
        let mut ftp_stream: CommandStream = CommandStream::new(addr)?;
        let response: String = ftp_stream.read_response()?;

        info!("Server response: {}", response);

        Ok(FtpClient {
            ftp_stream: ftp_stream,
            ftp_data_stream: None,
        })
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<()> {
        info!("Starting authentication");

        self.ftp_stream
            .send_command(FtpCommand::User(username.to_string()))?;
        self.ftp_stream
            .send_command(FtpCommand::Pass(password.to_string()))?;

        info!("Authentication successful");

        Ok(())
    }

    pub fn retrieve_server_info(&mut self) -> Result<()> {
        info!("Retrieving server information");

        self.ftp_stream.send_command(FtpCommand::Syst)?;
        self.ftp_stream.send_command(FtpCommand::Feat)?;
        self.ftp_stream.send_command(FtpCommand::Pwd)?;

        info!("Server information retrieved");

        Ok(())
    }

    pub fn passive_mode(&mut self) -> Result<()> {
        info!("Entering passive mode");

        self.ftp_stream
            .send_command(FtpCommand::Type("I".to_string()))?;
        let response: String = self.ftp_stream.send_command(FtpCommand::Pasv)?;

        info!("Passive mode entered");
        debug!("Parsing passive mode response: {}", response);

        let addr: String = FtpClient::parse_passive_mode_response(response)?;

        debug!("Connecting to data client at {}", addr);
        self.ftp_data_stream = Some(FtpDataStream::new(&addr)?);

        Ok(())
    }

    fn parse_passive_mode_response(res: String) -> Result<String> {
        let start: usize = res.find('(').expect("Opening parenthesis not found") + 1;
        let end: usize = res.find(')').expect("Closing parenthesis not found");
        let content: &str = &res[start..end];
        let parts: Vec<&str> = content.split(',').collect();

        if parts.len() < 6 {
            panic!("invalid data");
        }

        let ip: String = format!("{}.{}.{}.{}", parts[0], parts[1], parts[2], parts[3]);
        let port: u16 = parts[4].parse::<u16>()? * 256 + parts[5].parse::<u16>()?;

        Ok(format!("{}:{}", ip, port))
    }

    pub fn list_dir(&mut self) -> Result<Vec<String>> {
        if self.ftp_data_stream.is_none() {
            self.passive_mode()?;
        }
        let data_stream = self.ftp_data_stream.as_mut().ok_or("Data stream not initialized")?;
        let _ = self.ftp_stream.send_command(FtpCommand::List);
        let response_lines = data_stream.read_response()?;
        let mapped_lines: Vec<String> = response_lines.iter().map(|line| {
            if line.chars().next() == Some('d') {
                format!("Directory: {}", line)
            } else {
                format!("File: {}", line)
            }
        }).collect();
    
        for line in &mapped_lines {
            println!("{}", line);
        }
    
        Ok(mapped_lines)
    }
}
