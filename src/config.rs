use config::{ConfigError, File};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub app: App,
    pub server: Server,
    pub redis: Redis,
    pub cookie: Cookie,
    pub post_client: Client,
    pub auth_client: Client,
}

#[derive(Clone, Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub url: String,
    pub key: String,
    pub debug: bool,
    pub metadata: AppMetaData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppMetaData {
    pub title: String,
    pub keywords: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub addrs: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Redis {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub secret_key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Client {
    pub addrs: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = config::Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
