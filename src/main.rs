use crate::args::parse_config;
use std::env;

pub mod args;

fn process(args: Vec<String>) -> Result<(), clap::Error> {
    let config = parse_config(args)?;
    println!("{} => {}", config.source.display(), config.target.display());
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    process(args).unwrap_or_else(|error| error.exit());
}
