use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

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

pub async fn load_or_create_results(results_filepath: &str) -> Option<UpdateIpResults> {
    if let Ok(json_as_str) = fs::read_to_string(&results_filepath).await {
        if let Ok(r) = serde_json::from_str(&json_as_str) {
            return Some(r);
        }
    };

    Some(UpdateIpResults::new())
}

pub fn address_has_changed(update_ip_results: &UpdateIpResults) -> bool {
    match (
        &update_ip_results.ip_service_result.prev_address,
        &update_ip_results.ip_service_result.address,
    ) {
        (Some(prev_ip), Some(curr_ip)) => prev_ip != curr_ip,
        (None, Some(_curr_ip)) => true,
        _ => false,
    }
}

pub async fn write_to_file(
    results: UpdateIpResults,
    results_filepath: &str,
) -> Result<UpdateIpResults, String> {
    let json_str = match serde_json::to_string_pretty(&results) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    if let Err(e) = fs::write(&results_filepath, json_str).await {
        return Err(e.to_string());
    };

    Ok(results)
}
