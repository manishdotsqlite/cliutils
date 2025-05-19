use clap::Parser;
use utils::{Args, Command};

mod utils;
mod implementation;

fn main() {
    let args = Args::parse();

    let parsed_command = match Command::init(args) {
        Ok(some) => some,
        Err(_) => {
            eprintln!("Error: Couldn't recognize command.");
            return;
        }
    };

    if let Err(e) = parsed_command.execute() {
        eprintln!("Error: {}", e);
    }
}
