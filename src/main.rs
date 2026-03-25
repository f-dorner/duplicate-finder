mod cli;
mod finder;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    match finder::run(args) {
        Ok(()) => println!("Done."),
        Err(err) => eprintln!("Error: {}", err),
    };
}
