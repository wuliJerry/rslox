use std::env;

pub enum CommandLineError {
    MissingArgument,
    TooManyArguments,
}

pub fn parse_args() -> Result<String, CommandLineError> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => Err(CommandLineError::MissingArgument),
        2 => Ok(args[1].clone()),
        _ => Err(CommandLineError::TooManyArguments),
    }
}
