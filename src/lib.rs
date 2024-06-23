use config::Config;
use error::RunError;
use std::fs;
use weather::WeatherStations;

pub mod config;
pub mod error;
pub mod weather;

pub fn run(config: &Config) -> Result<(), RunError> {
    let file_content = fs::read_to_string(&config.input_file_path)?;
    let mut lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
    let mut weather_stations = WeatherStations::default();
    for line in lines.iter_mut() {
        weather_stations.add_measurement(line);
    }
    println!("{}", weather_stations.output());
    Ok(())
}
