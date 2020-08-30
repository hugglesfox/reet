use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Record {
    name: String,
    #[serde(alias = "type")]
    content: String,
}

#[derive(Deserialize)]
struct Cloudflare {
    zone_id: String,
    api_key: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub frequency: usize,
    pub log_level: String,
    pub cloudflare: Cloudflare,
    pub record: Record,
}

impl Settings {
    pub fn new(path: PathBuf, prefix: &str) -> Result<Self, ConfigError> {
        Config::new()
            .merge(File::from(path))?
            .merge(Environment::with_prefix("reet"))?
            .try_into()
    }
}
