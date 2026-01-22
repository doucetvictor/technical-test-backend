use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct ApiConfig {
    pub bind_address: String,
    pub python_api_url: String,
    pub timeout: u64,
    pub logging: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub general: ApiConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
