use config::{ConfigError, FileFormat};
use deadpool_postgres;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config
}

impl Config {
    pub fn from_json(file: &str) -> Result<Self, ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::File::new(file, FileFormat::Json))?;
        cfg.try_into()
    }
}