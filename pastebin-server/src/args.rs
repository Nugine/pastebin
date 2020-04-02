use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short = "l", long, help = "Log directives")]
    pub log: Option<String>,
    #[structopt(help = "Toml file path")]
    pub config_path: Option<PathBuf>,
}

pub fn get_args() -> Args {
    Args::from_args()
}
