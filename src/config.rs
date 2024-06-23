#[derive(Debug, PartialEq)]
pub struct Config {
    pub input_file_path: String,
}

impl Config {
    pub fn build(mut arguments: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        arguments.next();
        let error_message = "Usage: billion_row_challenge <input_file_path>";
        let input_file_path = match arguments.next() {
            Some(argument) => argument,
            None => return Err(error_message),
        };
        Ok(Config { input_file_path })
    }
}
