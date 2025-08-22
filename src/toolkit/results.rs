use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

use crate::toolkit::requests::ResponseJson;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IpServiceResult {
    pub service: String,
    pub ip_address: Option<String>,
    pub response: Option<ResponseJson>,
}

impl IpServiceResult {
    pub fn new(service: &str) -> IpServiceResult {
        IpServiceResult {
            service: service.to_string(),
            ip_address: None,
            response: None,
        }
    }
}

// DONT only SAVE errors. Save response.
// If validated save save ip_address update and results

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainResult {
    pub hostname: String,
    pub ip_address: Option<String>,
    pub errors: Vec<String>,
}

impl DomainResult {
    pub fn new(hostname: &str) -> DomainResult {
        DomainResult {
            hostname: hostname.to_string(),
            ip_address: None,
            errors: Vec::<String>::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateIpResults {
    pub ip_service_result: IpServiceResult,
    pub domain_service_results: HashMap<String, DomainResult>,
}

impl UpdateIpResults {
    pub fn try_from(
        ip_service_result: Result<IpServiceResult, String>,
        domain_service_results: Result<HashMap<String, DomainResult>, String>,
    ) -> Result<UpdateIpResults, String> {
        if let (Ok(ip_result), Ok(domain_results)) = (ip_service_result, domain_service_results) {
            return Ok(UpdateIpResults {
                ip_service_result: ip_result,
                domain_service_results: domain_results,
            });
        }

        Err("couldn't get results".to_string())
    }
}

pub async fn read_results_from_disk(results_filepath: &PathBuf) -> Result<UpdateIpResults, String> {
    let json_as_str = match fs::read_to_string(&results_filepath).await {
        Ok(json_str) => json_str,
        Err(e) => return Err(e.to_string()),
    };

    match serde_json::from_str(&json_as_str) {
        Ok(results) => Ok(results),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn write_results_to_disk(
    results: Result<UpdateIpResults, String>,
    results_filepath: &PathBuf,
) -> Result<(), String> {
    let ready_results = match results {
        Ok(rs) => rs,
        Err(e) => return Err(e),
    };

    let json_str = match serde_json::to_string_pretty(&ready_results) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    if let Err(e) = fs::write(&results_filepath, json_str).await {
        return Err(e.to_string());
    };

    Ok(())
}
