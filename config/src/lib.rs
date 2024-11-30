use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::path;
use tokio::fs;

use ip_services::IpServices;

// ddns services
#[cfg(feature = "cloudflare")]
use cloudflare::Cloudflare;
#[cfg(feature = "dyndns2")]
use dyndns2::Dyndns2;

// add domain services here
// beware of hydra
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub results_filepath: path::PathBuf,
    pub ip_services: IpServices,
    #[cfg(feature = "dyndns2")]
    pub dyndns2: Vec<Dyndns2>,
    #[cfg(feature = "cloudflare")]
    pub cloudflare: Vec<Cloudflare>,
}

pub async fn from_path(file_path: &path::Path) -> Result<Config, String> {
    // get position relative to working directory
    let config_path = match path::absolute(file_path) {
        Ok(pb) => pb,
        Err(e) => return Err(e.to_string()),
    };

    let parent_dir = match config_path.parent() {
        Some(p) => p,
        _ => {
            return Err("parent directory of config not found".to_string());
        }
    };

    let config_json = match fs::read_to_string(&config_path).await {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    let mut config: Config = match serde_json::from_str(&config_json) {
        Ok(j) => j,
        Err(e) => return Err(e.to_string()),
    };

    // find a way to verify the parent directory exists
    config.results_filepath = parent_dir.join(&config.results_filepath);

    Ok(config)
}
