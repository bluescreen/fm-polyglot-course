use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    #[clap(short = 'a', long = "args")]
    pub args: Vec<String>,

    #[clap(short = 'c', long = "config")]
    pub config: Option<PathBuf>,

    #[clap(short = 'p', long = "pwd")]
    pub pwd: Option<PathBuf>,
}
