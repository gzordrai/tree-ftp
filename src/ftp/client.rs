use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use log::{debug, info};

use crate::{
    fs::{directory::Directory, file::File, node::NodeEnum},
    ftp::{
        command::FtpCommand, command_stream::CommandStream, data_stream::DataStream, error::Result,
    },
};

use super::stream::Stream;

pub struct FtpClient {
    extended: bool,
    data_addr: Option<SocketAddr>,
    ftp_stream: CommandStream,
    ftp_data_stream: Option<DataStream>,
}

impl FtpClient {
    pub fn new(addr: SocketAddr, extended: bool) -> Result<Self> {
        let mut ftp_stream: CommandStream = CommandStream::new(addr)?;
        let response: String = ftp_stream
            .read_responses()?
            .get(0)
            .ok_or("No response received")?
            .to_string();

        info!("Server response: {}", response);

        Ok(FtpClient {
            extended,
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
        self.ftp_stream
            .send_command(FtpCommand::Type("I".to_string()))?;

        info!("Server information retrieved");

        Ok(())
    }

    pub fn passive_mode(&mut self) -> Result<()> {
        let command: FtpCommand = if self.extended {
            debug!("Entering in extended passive mode");

            FtpCommand::Epsv
        } else {
            debug!("Entering in passive mode");

            FtpCommand::Pasv
        };
        let mut responses: Vec<String> = self.ftp_stream.send_command(command)?;

        debug!("Passive mode entered");

        let response: String = responses.pop().ok_or("No response received")?;
        let addr: SocketAddr = FtpClient::parse_passive_mode_response(self, response)?;

        self.data_addr = Some(addr.clone());

        debug!("Connecting to data client at {}", addr);

        self.ftp_data_stream = Some(DataStream::new(addr)?);

        Ok(())
    }

    fn parse_passive_mode_response(&mut self, res: String) -> Result<SocketAddr> {
        debug!("Parsing passive mode response: {}", res);

        if let Some(start) = res.find('(') {
            if let Some(end) = res.find(')') {
                let content: &str = &res[start + 1..end];
                let parts: Vec<&str>;

                if self.extended {
                    debug!("Parsing extended passive mode");

                    parts = content.split('|').collect();

                    if parts.len() != 5 {
                        return Err("Invalid data in extended passive mode response".into());
                    }

                    let ip: IpAddr = self.ftp_stream.get_addr().ip();
                    let port: u16 = parts[3].parse::<u16>()?;

                    return Ok(SocketAddr::new(ip, port));
                } else {
                    debug!("Parsing passive mode");

                    parts = content.split(',').collect();

                    if parts.len() < 6 {
                        return Err("Invalid data in passive mode response".into());
                    }

                    let ip: Ipv4Addr = Ipv4Addr::new(
                        parts[0].parse()?,
                        parts[1].parse()?,
                        parts[2].parse()?,
                        parts[3].parse()?,
                    );
                    let port: u16 = parts[4].parse::<u16>()? * 256 + parts[5].parse::<u16>()?;

                    return Ok(SocketAddr::new(IpAddr::V4(ip), port));
                }
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
        self.passive_mode()?;

        self.ftp_stream.send_command(FtpCommand::List)?;
        self.ftp_stream.read_responses()?;

        let response_lines: Vec<String> =
            self.ftp_data_stream.as_mut().unwrap().read_responses()?;
        let mut root: Directory = Directory::new(String::from("."));

        for line in response_lines {
            let node_name: String = Self::parse_filename(&line);

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

    fn parse_filename(line: &str) -> String {
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
        self.ftp_stream.read_responses()?;

        let response_lines: Vec<String> =
            self.ftp_data_stream.as_mut().unwrap().read_responses()?;

        for line in response_lines {
            let node_name: String = Self::parse_filename(&line);

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
