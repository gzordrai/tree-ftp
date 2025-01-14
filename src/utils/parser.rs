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
}

