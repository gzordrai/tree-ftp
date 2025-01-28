use std::{
    cell::RefCell,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    rc::Rc,
};

use log::{debug, info};

use crate::{
    fs::{directory::Directory, file::File, node::NodeEnum},
    ftp::{
        command::FtpCommand,
        command_stream::CommandStream,
        data_stream::DataStream,
        error::{Error, Result},
    },
};

use super::stream::{Responses, Stream};

/// Represents an FTP client for communicating with an FTP server.
pub struct FtpClient {
    extended: bool,
    data_addr: Option<SocketAddr>,
    ftp_stream: CommandStream,
    ftp_data_stream: Option<DataStream>,
    username: String,
    password: String,
}

impl FtpClient {
    /// Creates a new `FtpClient` and connects to the given address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address of the FTP server.
    /// * `username` - The username for authentication.
    /// * `password` - The password for authentication.
    /// * `extended` - A boolean indicating whether to use extended passive mode.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `FtpClient` or an `Error`.
    pub fn new(
        addr: SocketAddr,
        username: &String,
        password: &String,
        extended: bool,
    ) -> Result<Self> {
        let mut ftp_stream: CommandStream = CommandStream::new(addr)?;
        let response: Responses = ftp_stream.read_responses()?;

        info!("Server response: {:?}", response);

        Ok(FtpClient {
            username: username.to_string(),
            password: password.to_string(),
            extended,
            data_addr: None,
            ftp_stream: ftp_stream,
            ftp_data_stream: None,
        })
    }

    /// Authenticates the user with the provided username and password.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for authentication.
    /// * `password` - The password for authentication.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn authenticate(&mut self, username: &String, password: &String) -> Result<()> {
        info!("Starting authentication");

        self.ftp_stream
            .send_command(FtpCommand::User(username.to_string()))?;
        self.ftp_stream
            .send_command(FtpCommand::Pass(password.to_string()))?;

        info!("Authentication successful");

        Ok(())
    }

    /// Retrieves server information by sending various FTP commands.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
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

    /// Enters passive mode or extended passive mode based on the `extended` flag.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn passive_mode(&mut self) -> Result<()> {
        let command: FtpCommand = if self.extended {
            debug!("Entering in extended passive mode");

            FtpCommand::Epsv
        } else {
            debug!("Entering in passive mode");

            FtpCommand::Pasv
        };
        let mut responses: Responses = self.ftp_stream.send_command(command.clone())?;

        debug!("Passive mode entered");

        let response: String = loop {
            let response = responses.pop().ok_or_else(|| {
                self.ftp_stream
                    .send_command(command.clone())?
                    .pop()
                    .ok_or(Error::NoResponseReceived)
            });

            match response {
                Ok((_, response)) => {
                    if !response.is_empty() {
                        break response;
                    } else {
                        info!("Empty response received, retrying...");
                        responses = self.ftp_stream.send_command(command.clone())?;
                    }
                }
                Err(e) => {
                    if let Err(Error::NoResponseReceived) = e {
                        info!("No response received, attempting to reconnect...");
                        self.ftp_stream.reconnect()?;
                        responses = self.ftp_stream.send_command(command.clone())?;
                    } else {
                        return Err(e.unwrap_err());
                    }
                }
            }
        };

        let addr: SocketAddr = FtpClient::parse_passive_mode_response(self, response)?;

        self.data_addr = Some(addr.clone());

        debug!("Connecting to data client at {}", addr);

        self.ftp_data_stream = Some(DataStream::new(addr)?);

        Ok(())
    }

    /// Parses the response from the server to determine the data connection address.
    ///
    /// # Arguments
    ///
    /// * `res` - The response string from the server.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SocketAddr` or an `Error`.
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
                        return Err(Error::InvalidParsedData);
                    }

                    let ip: IpAddr = self.ftp_stream.get_addr().ip();
                    let port: u16 = parts[3]
                        .parse::<u16>()
                        .map_err(|_| Error::InvalidParsedPort)?;

                    return Ok(SocketAddr::new(ip, port));
                } else {
                    debug!("Parsing passive mode");

                    parts = content.split(',').collect();

                    if parts.len() < 6 {
                        return Err(Error::InvalidParsedData);
                    }

                    let ip: Ipv4Addr = Ipv4Addr::new(
                        parts[0].parse().map_err(|_| Error::InvalidParsedIp)?,
                        parts[1].parse().map_err(|_| Error::InvalidParsedIp)?,
                        parts[2].parse().map_err(|_| Error::InvalidParsedIp)?,
                        parts[3].parse().map_err(|_| Error::InvalidParsedIp)?,
                    );
                    let port: u16 = parts[4]
                        .parse::<u16>()
                        .map_err(|_| Error::InvalidParsedPort)?
                        * 256
                        + parts[5]
                            .parse::<u16>()
                            .map_err(|_| Error::InvalidParsedPort)?;

                    return Ok(SocketAddr::new(IpAddr::V4(ip), port));
                }
            } else {
                return Err(Error::InvalidParsedData);
            }
        }

        if let Some(addr) = self.data_addr {
            return Ok(addr);
        }

        Err(Error::InvalidParsedData)
    }

    /// Lists the directory contents up to a specified depth using either BFS or DFS.
    ///
    /// # Arguments
    ///
    /// * `depth` - The depth to which the directory contents should be listed.
    /// * `bfs` - A boolean indicating whether to use BFS (true) or DFS (false).
    ///
    /// # Returns
    ///
    /// A `Result` containing the root `NodeEnum` or an `Error`.
    pub fn list_dir(&mut self, depth: usize, bfs: bool) -> Result<NodeEnum> {
        let username = self.username.clone();
        let password = self.password.clone();

        self.authenticate(&username, &password)?;
        self.retrieve_server_info()?;
        self.passive_mode()?;

        self.ftp_stream.send_command(FtpCommand::List)?;

        let responses = self.ftp_data_stream.as_mut().unwrap().read_responses()?;
        let mut root = Directory::new(String::from("."));

        if bfs {
            debug!("BFS enabled");

            self.process_responses_bfs(responses, &mut root, depth)?;
        } else {
            self.process_responses_dfs(responses, &mut root, depth)?;
        }

        if self.ftp_stream.is_reconnected() {
            self.ftp_stream.set_reconnected(false);
            self.list_dir(depth, bfs)
        } else {
            Ok(NodeEnum::Directory(root))
        }
    }

    /// Processes the responses using DFS and populates the directory up to a specified depth.
    ///
    /// # Arguments
    ///
    /// * `responses` - The responses from the server.
    /// * `dir` - The directory to populate.
    /// * `depth` - The depth to which the directory should be populated.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn process_responses_dfs(
        &mut self,
        responses: Responses,
        dir: &mut Directory,
        depth: usize,
    ) -> Result<()> {
        for response in responses {
            if self.ftp_stream.is_reconnected() {
                return Ok(());
            }

            let (code, line) = response;
            let node_name = Self::parse_filename(&line);

            if line.chars().next() == Some('d') {
                let mut subdir = Directory::new(node_name.clone());

                if code < 500 && depth > 0 {
                    self.populate_dir_dfs(node_name.clone(), &mut subdir, depth - 1)?;
                }

                dir.add(subdir);
            } else {
                dir.add(File::new(node_name));
            }
        }

        Ok(())
    }

    /// Processes the responses using BFS and populates the directory up to a specified depth.
    ///
    /// # Arguments
    ///
    /// * `responses` - The responses from the server.
    /// * `dir` - The directory to populate.
    /// * `depth` - The depth to which the directory should be populated.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn process_responses_bfs(
        &mut self,
        responses: Responses,
        dir: &mut Directory,
        depth: usize,
    ) -> Result<()> {
        let root: Rc<RefCell<Directory>> = Rc::new(RefCell::new(std::mem::take(dir)));
        let mut queue: Vec<(Rc<RefCell<Directory>>, Vec<(u16, String)>, usize)> =
            vec![(root.clone(), responses, depth)];

        while let Some((current_dir, current_responses, current_depth)) = queue.pop() {
            for response in current_responses {
                if self.ftp_stream.is_reconnected() {
                    return Ok(());
                }

                let (code, line) = response;
                let node_name: String = Self::parse_filename(&line);

                if line.chars().next() == Some('d') {
                    let subdir: Rc<RefCell<Directory>> =
                        Rc::new(RefCell::new(Directory::new(node_name.clone())));

                    if code < 500 && current_depth > 0 {
                        let subdir_responses = self.populate_dir_bfs(
                            node_name.clone(),
                            &mut subdir.borrow_mut(),
                            current_depth - 1,
                        )?;

                        queue.push((subdir.clone(), subdir_responses, current_depth - 1));
                    }

                    current_dir
                        .borrow_mut()
                        .add(NodeEnum::Directory((*subdir.borrow()).clone()));
                } else {
                    current_dir.borrow_mut().add(File::new(node_name));
                }
            }
        }

        *dir = Rc::try_unwrap(root).unwrap().into_inner();

        Ok(())
    }

    /// Populates the directory using DFS up to a specified depth.
    ///
    /// # Arguments
    ///
    /// * `dir_name` - The name of the directory to populate.
    /// * `dir` - The directory to populate.
    /// * `depth` - The depth to which the directory should be populated.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn populate_dir_dfs(
        &mut self,
        dir_name: String,
        dir: &mut Directory,
        depth: usize,
    ) -> Result<()> {
        if depth == 0 || self.ftp_stream.is_reconnected() {
            return Ok(());
        }

        self.ftp_stream.send_command(FtpCommand::Cwd(dir_name))?;
        self.passive_mode()?;
        self.ftp_stream.send_command(FtpCommand::List)?;

        if self.ftp_stream.is_reconnected() {
            return Ok(());
        }

        let responses: Responses = self.ftp_data_stream.as_mut().unwrap().read_responses()?;
        self.process_responses_dfs(responses, dir, depth)?;

        self.ftp_stream.send_command(FtpCommand::Cdup)?;

        Ok(())
    }

    /// Populates the directory using BFS up to a specified depth.
    ///
    /// # Arguments
    ///
    /// * `dir_name` - The name of the directory to populate.
    /// * `dir` - The directory to populate.
    /// * `depth` - The depth to which the directory should be populated.
    ///
    /// # Returns
    ///
    /// A `Result` containing the server's responses or an `Error`.
    fn populate_dir_bfs(
        &mut self,
        dir_name: String,
        dir: &mut Directory,
        depth: usize,
    ) -> Result<Responses> {
        if depth == 0 || self.ftp_stream.is_reconnected() {
            return Ok(vec![]);
        }

        self.ftp_stream.send_command(FtpCommand::Cwd(dir_name))?;
        self.passive_mode()?;
        self.ftp_stream.send_command(FtpCommand::List)?;

        if self.ftp_stream.is_reconnected() {
            return Ok(vec![]);
        }

        let responses = self.ftp_data_stream.as_mut().unwrap().read_responses()?;

        self.process_responses_bfs(responses.clone(), dir, depth)?;
        self.ftp_stream.send_command(FtpCommand::Cdup)?;

        Ok(responses)
    }

    /// Parses the filename from a response line.
    ///
    /// # Arguments
    ///
    /// * `line` - The response line containing the filename.
    ///
    /// # Returns
    ///
    /// A `String` containing the parsed filename.
    fn parse_filename(line: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 9 {
            String::new()
        } else {
            parts[8..].join(" ")
        }
    }
}
