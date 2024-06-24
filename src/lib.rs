use config::Config;
use crossbeam_channel::{bounded, unbounded};
use error::RunError;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use weather::WeatherStations;

pub mod config;
pub mod error;
pub mod weather;

/// Configuration to control memory usage by ensuring that at most `CHUNK_COUNT` chunks of `CHUNK_SIZE` lines each are held in memory at any given time, while still allowing for parallel processing of the data.

/// Number of lines to process in one chunk.
const CHUNK_SIZE: usize = 100_000;

/// Number of chunks to process at the same time.
const CHUNK_COUNT: usize = 100;

pub fn run(config: &Config) -> Result<(), RunError> {
    let file = File::open(&config.input_file_path)?;
    let reader = BufReader::new(file);
    let (sender, receiver) = bounded::<Vec<String>>(CHUNK_COUNT);
    let (error_sender, error_receiver) = unbounded();
    std::thread::spawn(move || {
        let mut chunk = Vec::with_capacity(CHUNK_SIZE);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    chunk.push(line);
                    if chunk.len() >= CHUNK_SIZE {
                        if sender.send(chunk).is_err() {
                            return;
                        }
                        chunk = Vec::with_capacity(CHUNK_SIZE);
                    }
                }
                Err(error) => {
                    error_sender.send(error).unwrap();
                    return;
                }
            }
        }
        if !chunk.is_empty() {
            sender.send(chunk).unwrap();
        }
    });
    if let Ok(error) = error_receiver.try_recv() {
        return Err(RunError::InputOutputError(error));
    }
    let weather_stations: WeatherStations = receiver
        .into_iter()
        .par_bridge()
        .map(|chunk| {
            let mut local_weather_stations = WeatherStations::default();
            for line in chunk {
                local_weather_stations.add_measurement(&line);
            }
            local_weather_stations
        })
        .reduce(WeatherStations::default, |mut a, b| {
            a.merge(b);
            a
        });
    println!("{}", weather_stations.output());
    Ok(())
}
