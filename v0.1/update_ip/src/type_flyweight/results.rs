use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

impl IpServiceResult {
    pub fn new() -> IpServiceResult {
        IpServiceResult {
            prev_address: None,
            address: None,
            service: None,
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
