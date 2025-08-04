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
    pub path: String,

    pub directory: Option<String>,
    pub index: Option<String>,
    pub proxy_pass: Option<String>,
    pub response: Option<ResponseConfig>,

    pub timeout_ms: Option<u64>,
    pub gzip: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseConfig {
    pub status: u16,
    pub body: String,
}

impl Config {
    fn check_route_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (i, route) in self.routes.iter().enumerate() {
            let present_count = [
                route.directory.is_some(),
                route.index.is_some(),
                route.proxy_pass.is_some(),
                route.response.is_some(),
            ]
            .iter()
            .filter(|&&v| v)
            .count();

            if present_count != 1 {
                return Err(format!(
                    "Route #{} must have exactly one of 'directory', 'index', 'proxy_pass' or 'response'",
                    i + 1
                )
                .into());
            }
            
            if route.gzip.is_some() && route.proxy_pass.is_some() {
                return Err(format!(
                    "Route #{}: gzip compression cannot be enabled for proxy_pass routes",
                    i + 1
                )
                .into());
            }
        }

        Ok(())
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path_ref: &Path = path.as_ref();
        let contents = fs::read_to_string(path_ref)?;
        let config: Self = serde_yaml::from_str(&contents)?;
        config.check_route_config()?;
        Ok(config)
    }
}
