use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::path;
use tokio::fs;

#[cfg(feature = "cloudflare")]
use cloudflare::Cloudflare;
#[cfg(feature = "dyndns2")]
use dyndns2::Dyndns2;
use ip_services::IpServices;

// add domain services here
// beware of hydra
// prepare for modules by features
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainServices {
    #[cfg(feature = "dyndns2")]
    pub dyndns2: Vec<Dyndns2>,
    #[cfg(feature = "cloudflare")]
    pub cloudflare: Vec<Cloudflare>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub results_filepath: path::PathBuf,
    pub ip_services: IpServices,
    pub domain_services: DomainServices,
}

pub enum ConfigError<'a> {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    GenericError(&'a str),
}

impl fmt::Display for ConfigError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::IoError(io_error) => write!(f, "{}", io_error),
            ConfigError::JsonError(json_error) => write!(f, "{}", json_error),
            ConfigError::GenericError(generic_error) => write!(f, "{}", generic_error,),
        }
    }
}

pub async fn from_path(path: &path::Path) -> Result<Config, ConfigError> {
    // get position relative to working directory
    let config_path = match path.canonicalize() {
        Ok(pb) => pb,
        Err(e) => return Err(ConfigError::IoError(e)),
    };

    let parent_dir = match config_path.parent() {
        Some(p) => p,
        _ => {
            return Err(ConfigError::GenericError(
                "parent directory of config not found",
            ))
        }
    };

    let config_json = match fs::read_to_string(&config_path).await {
        Ok(r) => r,
        Err(e) => return Err(ConfigError::IoError(e)),
    };

    let mut config: Config = match serde_json::from_str(&config_json) {
        Ok(j) => j,
        Err(e) => return Err(ConfigError::JsonError(e)),
    };

    // find a way to verify the parent directory exists
    config.results_filepath = parent_dir.join(&config.results_filepath);

    Ok(config)
}
