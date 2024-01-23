use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::path;
use tokio::fs;

use crate::type_flyweight::DomainServices;
use crate::type_flyweight::IpServices;

const PARENT_NOT_FOUND_ERR: &str = "parent directory of config not found";
const FILE_IS_NOT_FILE_ERR: &str = "config.results_filepath is not a file";

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
            ConfigError::GenericError(generic_error) => write!(f, "{}", generic_error),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub results_filepath: path::PathBuf,
    pub ip_services: IpServices,
    pub domain_services: DomainServices,
}

pub async fn from_filepath(filepath: &path::PathBuf) -> Result<Config, ConfigError> {
    // get position relative to working directory
    let config_pathbuff = match filepath.canonicalize() {
        Ok(pb) => pb,
        Err(e) => return Err(ConfigError::IoError(e)),
    };

    let parent_dir = match config_pathbuff.parent() {
        Some(p) => p,
        _ => return Err(ConfigError::GenericError(PARENT_NOT_FOUND_ERR)),
    };

    let config_json = match fs::read_to_string(&config_pathbuff).await {
        Ok(r) => r,
        Err(e) => return Err(ConfigError::IoError(e)),
    };

    let mut config: Config = match serde_json::from_str(&config_json) {
        Ok(j) => j,
        Err(e) => return Err(ConfigError::JsonError(e)),
    };

    // config make sure parent directory of resuilts file exists

    // check configutaion
    config.results_filepath = parent_dir.join(&config.results_filepath);

    Ok(config)
}
