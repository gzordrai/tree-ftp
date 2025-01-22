use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(index = 1)]
    pub address: String,

    #[arg(short, long, default_value_t = String::from("anonymous"))]
    pub username: String,

    #[arg(short, long, default_value_t = String::from("anonymous"))]
    pub password: String,

    #[arg(short, long, default_value_t = 1)]
    pub depth: usize,

    #[arg(short, long, default_value_t = false)]
    pub json: bool,

    #[arg(short, long, default_value_t = false)]
    pub bfs: bool,

    #[arg(short, long, default_value_t = false)]
    pub extended: bool,
}
