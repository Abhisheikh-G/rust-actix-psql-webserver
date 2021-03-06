use::config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
    pub server: ServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}