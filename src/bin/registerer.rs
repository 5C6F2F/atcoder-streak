use chrono::NaiveTime;
use serde::Deserialize;
use std::fs;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Deserialize)]
struct Config {
    registerer: Registerer,
}

impl Config {
    fn new() -> Self {
        let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
        let config: Config = toml::from_str(&config_str).expect("Failed to parse config.toml");

        config
    }
}

#[derive(Debug, Deserialize)]
struct Registerer {
    task_name: String,
    task_description: String,
    #[serde(deserialize_with = "deserialize_times")]
    notify_times: Vec<NaiveTime>,
}

fn deserialize_times<'de, D>(deserializer: D) -> Result<Vec<NaiveTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let times_str: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
    times_str
        .into_iter()
        .map(|time_str| {
            time_str
                .parse::<NaiveTime>()
                .map_err(serde::de::Error::custom)
        })
        .collect()
}
