use std::{
    cell::RefCell,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    rc::Rc,
};

use log::{debug, info};

use crate::{
    fs::{directory::Directory, file::File, node::NodeEnum},
    ftp::{
        command::FtpCommand, command_stream::CommandStream, data_stream::DataStream, error::Result,
    },
};

use super::stream::{Responses, Stream};

pub struct FtpClient {
    extended: bool,
    data_addr: Option<SocketAddr>,
    ftp_stream: CommandStream,
    ftp_data_stream: Option<DataStream>,
    username: String,
    password: String,
}

impl FtpClient {
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

    pub fn authenticate(&mut self, username: &String, password: &String) -> Result<()> {
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
        let mut responses = self.ftp_stream.send_command(command)?;

        debug!("Passive mode entered");

        let (_, response) = responses.pop().ok_or("No response received")?;
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

<<<<<<< HEAD
    fn process_responses_bfs(
        &mut self,
        responses: Responses,
        dir: &mut Directory,
        depth: usize,
    ) -> Result<()> {
        let root: Rc<RefCell<Directory>> = Rc::new(RefCell::new(std::mem::take(dir)));
        let mut queue: Vec<(Rc<RefCell<Directory>>, Vec<(u16, String)>, usize)> = vec![(root.clone(), responses, depth)];

=======
    fn process_responses_bfs(&mut self, responses: Responses, dir: &mut Directory, depth: usize) -> Result<()> {
        let root = Rc::new(RefCell::new(std::mem::take(dir)));
        let mut queue = vec![(root.clone(), responses, depth)];
    
>>>>>>> d40e8779e2430965e5302507c06a54ad64faa131
        while let Some((current_dir, current_responses, current_depth)) = queue.pop() {
            for response in current_responses {
                if self.ftp_stream.is_reconnected() {
                    return Ok(());
                }
<<<<<<< HEAD

                let (code, line) = response;
                let node_name: String = Self::parse_filename(&line);

                if line.chars().next() == Some('d') {
                    let subdir: Rc<RefCell<Directory>> = Rc::new(RefCell::new(Directory::new(node_name.clone())));

=======
    
                let (code, line) = response;
                let node_name = Self::parse_filename(&line);
    
                if line.chars().next() == Some('d') {
                    let subdir = Rc::new(RefCell::new(Directory::new(node_name.clone())));
    
>>>>>>> d40e8779e2430965e5302507c06a54ad64faa131
                    if code < 500 && current_depth > 0 {
                        let subdir_responses = self.populate_dir_bfs(
                            node_name.clone(),
                            &mut subdir.borrow_mut(),
                            current_depth - 1,
                        )?;
<<<<<<< HEAD

                        queue.push((subdir.clone(), subdir_responses, current_depth - 1));
                    }

                    current_dir 
                        .borrow_mut()
                        .add(NodeEnum::Directory((*subdir.borrow()).clone()));
=======
    
                        queue.push((subdir.clone(), subdir_responses, current_depth - 1));
                    }
    
                    current_dir.borrow_mut().add(NodeEnum::Directory((*subdir.borrow()).clone()));
>>>>>>> d40e8779e2430965e5302507c06a54ad64faa131
                } else {
                    current_dir.borrow_mut().add(File::new(node_name));
                }
            }
        }
<<<<<<< HEAD

        *dir = Rc::try_unwrap(root).unwrap().into_inner();

=======
    
        *dir = Rc::try_unwrap(root).unwrap().into_inner();
    
>>>>>>> d40e8779e2430965e5302507c06a54ad64faa131
        Ok(())
    }

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

<<<<<<< HEAD
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

=======

    fn populate_dir_bfs(&mut self, dir_name: String, dir: &mut Directory, depth: usize) -> Result<Responses> {
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
    
>>>>>>> d40e8779e2430965e5302507c06a54ad64faa131
        Ok(responses)
    }

    fn parse_filename(line: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 9 {
            String::new()
        } else {
            parts[8..].join(" ")
        }
    }
}
