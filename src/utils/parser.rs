use clap::Parser;

/// Command-line arguments for the FTP client.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The address of the FTP server.
    #[arg(index = 1)]
    pub address: String,

    /// The username for FTP authentication. Defaults to "anonymous".
    #[arg(short, long, default_value_t = String::from("anonymous"))]
    pub username: String,

    /// The password for FTP authentication. Defaults to "anonymous".
    #[arg(short, long, default_value_t = String::from("anonymous"))]
    pub password: String,

    /// The depth of directory traversal. Defaults to 1.
    #[arg(short, long, default_value_t = 1)]
    pub depth: usize,

    /// Output the result in JSON format. Defaults to false.
    #[arg(short, long, default_value_t = false)]
    pub json: bool,

    /// Use breadth-first search for directory traversal. Defaults to false.
    #[arg(short, long, default_value_t = false)]
    pub bfs: bool,

    /// Use extended passive mode for data connections. Defaults to false.
    #[arg(short, long, default_value_t = false)]
    pub extended: bool,
}
