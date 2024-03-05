use chrono::NaiveTime;
use serde::Deserialize;
use std::fs;

fn main() {
    let config = Config::new();
    let registerer = &config.registerer;
    let current_exe = std::env::current_exe().expect("Failed to get current exe path");

    std::process::Command::new("pwsh")
        .arg("-Command")
        .arg("./register.ps1")
        .arg(&registerer.task_name)
        .arg(current_exe)
        .arg(&registerer.task_description)
        .arg(registerer.times_to_string())
        .output()
        .expect("Failed to register command");
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

impl Registerer {
    fn times_to_string(&self) -> String {
        let mut result = String::new();
        for i in &self.notify_times {
            result.push_str(&i.to_string());
            result.push(' ');
        }
        result.pop();
        result
    }
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
