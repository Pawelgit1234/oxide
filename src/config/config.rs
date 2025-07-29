use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub routes: Vec<RouteConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub tls: Option<TlsConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, Deserialize)]
pub struct RouteConfig{
    pub directory: Option<String>,
    pub index: Option<String>,
    pub proxy_pass: Option<String>,

    pub timeout_ms: Option<u64>,
    pub gzip: Option<bool>,

    pub response: Option<ResponseConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseConfig {
    pub status: u16,
    pub body: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path_ref: &Path = path.as_ref();
        let contents = fs::read_to_string(path_ref)?;
        let config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
