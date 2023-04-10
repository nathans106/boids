use serde::Deserialize;
use std::fs::File;
use std::io;
use std::io::{Error, Read};
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let mut config_file = File::open("config.toml").unwrap();
    Config::from_file(&mut config_file).unwrap()
});

#[derive(Deserialize)]
pub struct Config {
    pub num_boids: i32,
}

impl Config {
    pub fn from_file(toml_file: &mut File) -> Result<Config, ConfigError> {
        let mut file_contents = String::new();
        toml_file.read_to_string(&mut file_contents)?;
        let config = toml::from_str(&file_contents)?;
        Ok(config)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Read(io::Error),
    Parse(toml::de::Error),
}

impl From<io::Error> for ConfigError {
    fn from(value: Error) -> Self {
        Self::Read(value)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self::Parse(value)
    }
}
