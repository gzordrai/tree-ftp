use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use log::{debug, info};

use crate::{
    fs::{directory::Directory, file::File, node::NodeEnum},
    ftp::{
        command::FtpCommand, command_stream::CommandStream, data_stream::FtpDataStream,
        error::Result,
    },
};

pub struct FtpClient {
    data_addr: Option<SocketAddr>,
    ftp_stream: CommandStream,
    ftp_data_stream: Option<FtpDataStream>,
}

impl FtpClient {
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let mut ftp_stream: CommandStream = CommandStream::new(addr)?;
        let response: String = ftp_stream.read_response()?;

        info!("Server response: {}", response);

        Ok(FtpClient {
            data_addr: None,
            ftp_stream: ftp_stream,
            ftp_data_stream: None,
        })
    }

    fn reconnect(&mut self) -> Result<()> {
        Ok(())
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

        let addr: SocketAddr = FtpClient::parse_passive_mode_response(self, response)?;
        self.data_addr = Some(addr.clone());

        debug!("Connecting to data client at {}", addr);

        self.ftp_data_stream = Some(FtpDataStream::new(addr)?);

        Ok(())
    }

    fn parse_passive_mode_response(&mut self, res: String) -> Result<SocketAddr> {
        if let Some(start) = res.find('(') {
            if let Some(end) = res.find(')') {
                let content: &str = &res[start + 1..end];
                let parts: Vec<&str> = content.split(',').collect();

                if parts.len() < 6 {
                    return Err("Invalid data in passive mode response".into());
                }

                let ip = Ipv4Addr::new(
                    parts[0].parse()?,
                    parts[1].parse()?,
                    parts[2].parse()?,
                    parts[3].parse()?,
                );
                let port = parts[4].parse::<u16>()? * 256 + parts[5].parse::<u16>()?;

                return Ok(SocketAddr::new(IpAddr::V4(ip), port));
            } else {
                return Err("Closing parenthesis not found in passive mode response".into());
            }
        }

        if let Some(addr) = self.data_addr {
            return Ok(addr);
        }

        Err("Opening parenthesis not found in passive mode response".into())
    }

    pub fn list_dir(&mut self, depth: usize) -> Result<NodeEnum> {
        if self.ftp_data_stream.is_none() {
            self.passive_mode()?;
        }

        self.ftp_stream.send_command(FtpCommand::List)?;
        self.ftp_stream.read_response()?;

        let response_lines: Vec<String> = self.ftp_data_stream.as_mut().unwrap().read_response()?;
        let mut root: Directory = Directory::new(String::from("."));

        for line in response_lines {
            let node_name: String = Self::get_file_name(&line);

            if line.chars().next() == Some('d') {
                let mut subdir: Directory = Directory::new(node_name.clone());

                Self::populate_dir(self, node_name.clone(), &mut subdir, depth - 1)?;
                root.add(subdir);
            } else {
                root.add(File::new(node_name));
            }
        }

        Ok(NodeEnum::Directory(root))
    }

    fn get_file_name(line: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 9 {
            String::new()
        } else {
            parts[8..].join(" ")
        }
    }

    fn populate_dir(&mut self, dir_name: String, dir: &mut Directory, depth: usize) -> Result<()> {
        if depth == 0 {
            return Ok(());
        }

        self.ftp_stream.send_command(FtpCommand::Cwd(dir_name))?;
        self.passive_mode()?;
        self.ftp_stream.send_command(FtpCommand::List)?;
        self.ftp_stream.read_response()?;
        let response_lines: Vec<String> = self.ftp_data_stream.as_mut().unwrap().read_response()?;

        for line in response_lines {
            let node_name: String = Self::get_file_name(&line);

            if line.chars().next() == Some('d') {
                let mut subdir: Directory = Directory::new(node_name.clone());

                Self::populate_dir(self, node_name.clone(), &mut subdir, depth - 1)?;
                dir.add(subdir);
            } else {
                dir.add(File::new(node_name));
            }
        }

        self.ftp_stream.send_command(FtpCommand::Cdup)?;

        Ok(())
    }
}
