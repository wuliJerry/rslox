#![allow(unused)]

mod utils;
use utils::args::*;

fn main() {
    let path: Option<String> = match parse_args() {
        Ok(p) => {
            Some(p)
        }
        Err(CommandLineError::MissingArgument) => {
            eprintln!("Missing argument");
            std::process::exit(1);
        }
        Err(CommandLineError::TooManyArguments) => {
            eprintln!("Too many arguments");
            std::process::exit(1);
        }
    };
}
