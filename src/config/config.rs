use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub common: Common,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Common {
    pub listen_port: u16,
    pub database_url: String,
    pub log_level: String,
    pub log_file: Option<String>,
}

impl Config {
    /// Creates a new Config with default values
    pub fn default() -> Self {
        Self {
            common: Common {
                listen_port: 8080,
                database_url: "sqlite://data.db".to_string(),
                log_level: "info".to_string(),
                log_file: None,
            },
        }
    }

    /// Loads configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config =
            toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(config)
    }

    /// Saves configuration to a TOML file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let toml_string =
            toml::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut file = fs::File::create(path)?;
        file.write_all(toml_string.as_bytes())?;

        Ok(())
    }
}
