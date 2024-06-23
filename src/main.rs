use std::env;
use std::io::ErrorKind;
use std::process;

use billion_row_challenge::config::Config;
use billion_row_challenge::error::RunError;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    match billion_row_challenge::run(&config) {
        Ok(_) => (),
        Err(error) => match error {
            RunError::InputOutputError(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("Error: File `{}` not found.", config.input_file_path)
                    }
                    _ => eprintln!("Error: {error}"),
                }
                process::exit(1);
            }
            RunError::Other(error) => {
                eprintln!("Error: {error}");
                process::exit(1);
            }
        },
    }
}
