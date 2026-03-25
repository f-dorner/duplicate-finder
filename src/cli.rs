use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "duplicate-finder")]
pub struct Args {
    #[arg(long)]
    pub path: PathBuf,

    #[arg(long)]
    pub recursive: bool,
}
