use std::collections::HashMap;
use tokio::fs;

// can use after mod declaration
use crate::config::Config;
use crate::type_flyweight::DomainResult;
use crate::type_flyweight::IpServiceResult;
use crate::type_flyweight::UpdateIpResults;

// no service initially
pub fn create_ip_service_result() -> IpServiceResult {
    IpServiceResult {
        address: None,
        service: None,
        address_changed: false,
        errors: Vec::new(),
        response: None,
    }
}

pub fn create_domain_result(hostname: &String) -> DomainResult {
    DomainResult {
        hostname: hostname.clone(),
        errors: Vec::<String>::new(),
        response: None,
    }
}

fn create_results() -> UpdateIpResults {
    UpdateIpResults {
        ip_service_result: create_ip_service_result(),
        domain_service_results: HashMap::<String, DomainResult>::new(),
    }
}

/*
    part of top-level function series
*/
pub async fn load_or_create_results(config: &Config) -> Option<UpdateIpResults> {
    let json_as_str = match fs::read_to_string(&config.results_filepath).await {
        Ok(r) => r,
        Err(_e) => return Some(create_results()),
    };

    match serde_json::from_str(&json_as_str) {
        Ok(j) => Some(j),
        Err(_e) => return Some(create_results()),
    }
}

/*
    part of top-level function series
*/
pub async fn write_to_file(
    results: UpdateIpResults,
    config: &Config,
) -> Result<UpdateIpResults, String> {
    let json_str = match serde_json::to_string_pretty(&results) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    let _ = fs::write(&config.results_filepath, json_str).await;

    Ok(results)
}
