// this module should be an external lib
// Types should be accessible to other applications
// However, they are tightly coupled to a version of serde

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path;

// beware of hydra
pub type IpServices = Vec<(String, String)>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub results_filepath: path::PathBuf,
    pub ip_services: IpServices,
    pub domain_services: DomainServices,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponseJson {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub timestamp: u128,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IpServiceResult {
    pub prev_address: Option<String>,
    pub address: Option<String>,
    pub service: Option<String>,
    pub address_changed: bool,
    pub errors: Vec<String>,
    pub response: Option<ResponseJson>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainResult {
    pub hostname: String,
    pub errors: Vec<String>,
    pub response: Option<ResponseJson>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateIpResults {
    pub ip_service_result: IpServiceResult,
    pub domain_service_results: HashMap<String, DomainResult>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dyndns2 {
    pub service_uri: String,
    pub hostname: String,
    pub username: String,
    pub password: String,
}

// add domain services here
// beware of hydra
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainServices {
    pub dyndns2: Option<Vec<Dyndns2>>,
}

impl IpServiceResult {
    pub fn new() -> IpServiceResult {
        IpServiceResult {
            prev_address: None,
            address: None,
            service: None,
            address_changed: false,
            errors: Vec::new(),
            response: None,
        }
    }
}

impl DomainResult {
    pub fn new(hostname: &String) -> DomainResult {
        DomainResult {
            hostname: hostname.clone(),
            errors: Vec::<String>::new(),
            response: None,
        }
    }
}

impl UpdateIpResults {
    pub fn new() -> UpdateIpResults {
        UpdateIpResults {
            ip_service_result: IpServiceResult::new(),
            domain_service_results: HashMap::<String, DomainResult>::new(),
        }
    }
}
