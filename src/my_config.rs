use serde::Deserialize;
use config::ConfigError;
use deadpool_postgres::Config;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}

#[derive(Deserialize)]
pub struct Configuration {
    pub server: ServerConfig,
    pub pg: Config
}

impl Configuration {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut conf = config::Config::new();
        conf.merge(config::Environment::new())?;
        conf.try_into()
    }
}