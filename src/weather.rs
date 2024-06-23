use std::collections::HashMap;
use std::str::FromStr;

fn round_towards_positive(value: f64, decimal_places: u32) -> f64 {
    let factor = 10_f64.powi(decimal_places as i32);
    (value * factor).ceil() / factor
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WeatherStationMeasurement {
    pub name: String,
    pub temperature: f64,
}

impl FromStr for WeatherStationMeasurement {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`WeatherStationMeasurement`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use billion_row_challenge::weather::WeatherStationMeasurement;
    ///
    /// let string = "Kunming;19.8";
    /// let expected_result = WeatherStationMeasurement {
    ///     name: String::from("Kunming"),
    ///     temperature: 19.8,
    /// };
    /// let actual_result = WeatherStationMeasurement::from_str(string).unwrap();
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut result = WeatherStationMeasurement::default();
        let mut parts = string.split(';');
        result.name = parts.next().unwrap_or("station").to_string();
        result.temperature = parts.next().unwrap_or("0").trim().parse().unwrap_or(0.0);
        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WeatherStationMeasurements {
    pub name: String,
    pub minimum: f64,
    pub maximum: f64,
    pub count: usize,
    pub sum: f64,
}

impl WeatherStationMeasurements {
    pub fn average(&self) -> f64 {
        if self.count == 0 {
            return 0.0;
        }
        self.sum / (self.count as f64)
    }

    /// Returns a string representation of the [`WeatherStationMeasurements`] instance in the format:
    /// ```text
    /// name=minimum/average/maximum
    /// ```
    /// # Examples
    /// ```
    /// use billion_row_challenge::weather::WeatherStationMeasurements;
    ///
    /// let weather_station = WeatherStationMeasurements {
    ///    name: String::from("Bosaso"),
    ///    minimum: -15.0,
    ///    maximum: 20.3,
    ///    count: 10,
    ///    sum: 100.0,
    /// };
    /// let expected_output = "Bosaso=-15.0/10.0/20.3";
    /// let actual_output = weather_station.output();
    /// assert_eq!(actual_output, expected_output);
    pub fn output(&self) -> String {
        format!(
            "{}={:.1}/{:.1}/{:.1}",
            self.name,
            self.minimum,
            round_towards_positive(self.average(), 1),
            self.maximum
        )
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WeatherStations {
    pub stations: HashMap<String, WeatherStationMeasurements>,
}

impl WeatherStations {
    pub fn add_measurement(&mut self, line: &str) {
        let weather_station_measurement =
            WeatherStationMeasurement::from_str(line).unwrap_or_default();
        match self.stations.get_mut(&weather_station_measurement.name) {
            Some(value) => {
                value.maximum = value.maximum.max(weather_station_measurement.temperature);
                value.minimum = value.minimum.min(weather_station_measurement.temperature);
                value.count += 1;
                value.sum += weather_station_measurement.temperature;
            }
            None => {
                let name = weather_station_measurement.name;
                self.stations.insert(
                    name.clone(),
                    WeatherStationMeasurements {
                        name,
                        minimum: weather_station_measurement.temperature,
                        maximum: weather_station_measurement.temperature,
                        count: 1,
                        sum: weather_station_measurement.temperature,
                    },
                );
            }
        }
    }

    /// Returns a string representation of the [`WeatherStations`] instance in the format:
    /// ```text
    /// {station1, station2, station3}
    /// ```
    /// # Examples
    /// ```
    /// use billion_row_challenge::weather::WeatherStations;
    /// use billion_row_challenge::weather::WeatherStationMeasurements;
    /// use std::collections::HashMap;
    ///
    /// let mut weather_stations = WeatherStations {
    ///    stations: HashMap::new(),
    /// };
    /// weather_stations.stations.insert(
    ///     String::from("Bosaso"),
    ///     WeatherStationMeasurements {
    ///       name: String::from("Bosaso"),
    ///       minimum: -15.0,
    ///       maximum: 20.0,
    ///       count: 10,
    ///       sum: 100.0,
    ///     },
    /// );
    /// weather_stations.stations.insert(
    ///     String::from("Petropavlovsk-Kamchatsky"),
    ///     WeatherStationMeasurements {
    ///       name: String::from("Petropavlovsk-Kamchatsky"),
    ///       minimum: -10.0,
    ///       maximum: 10.0,
    ///       count: 0,
    ///       sum: 0.0,
    ///     },
    /// );
    /// let expected_output = "{Bosaso=-15.0/10.0/20.0, Petropavlovsk-Kamchatsky=-10.0/0.0/10.0}";
    /// let actual_output = weather_stations.output();
    /// assert_eq!(actual_output, expected_output);
    pub fn output(&self) -> String {
        let mut outputs: Vec<String> = vec![];
        let mut station_names: Vec<&String> = self.stations.keys().collect();
        station_names.sort_unstable();
        for station_name in station_names.into_iter() {
            let weather_station = self.stations.get(station_name).unwrap();
            outputs.push(weather_station.output());
        }
        format!("{{{}}}", outputs.join(", "))
    }
}
