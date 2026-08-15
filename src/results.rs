use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

use crate::errors::Error;
use crate::requests::ResponseDetails;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IpServiceResult {
    pub service: String,
    pub ip_address: Option<String>,
    pub response: Option<ResponseDetails>,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainResult {
    pub hostname: String,
    pub ip_address: Option<String>,
    pub response: Option<ResponseDetails>,
    pub error: Option<Error>,
}

impl DomainResult {
    pub fn new(hostname: &str) -> DomainResult {
        DomainResult {
            hostname: hostname.to_string(),
            ip_address: None,
            response: None,
            error: None,
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
        ip_service_result: IpServiceResult,
        domain_service_results: Result<HashMap<String, DomainResult>, Error>,
    ) -> Result<UpdateIpResults, Error> {
        if let Ok(domain_results) = domain_service_results {
            return Ok(UpdateIpResults {
                ip_service_result: ip_service_result,
                domain_service_results: domain_results,
            });
        }

        Err(Error::Custom("couldn't get results".to_string()))
    }
}

pub async fn read_results_from_disk(results_filepath: &PathBuf) -> Result<UpdateIpResults, Error> {
    let json_as_str = match fs::read_to_string(results_filepath).await {
        Ok(json_str) => json_str,
        Err(e) => return Err(Error::Io(e.to_string())),
    };

    match serde_json::from_str(&json_as_str) {
        Ok(results) => Ok(results),
        Err(e) => return Err(Error::Io(e.to_string())),
    }
}

pub async fn write_results_to_disk(
    results_filepath: &PathBuf,
    ip_service_result: IpServiceResult,
    domain_service_results: Result<HashMap<String, DomainResult>, Error>,
) -> Result<(), Error> {
    let ready_results = match UpdateIpResults::try_from(ip_service_result, domain_service_results) {
        Ok(rs) => rs,
        Err(e) => return Err(e),
    };

    let json_str = match serde_json::to_string_pretty(&ready_results) {
        Ok(f) => f,
        Err(e) => return Err(Error::SerdeJson(e.to_string())),
    };

    if let Err(e) = fs::write(results_filepath, json_str).await {
        return Err(Error::Io(e.to_string()));
    };

    Ok(())
}
