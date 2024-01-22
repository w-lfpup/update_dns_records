use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

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
        retry: false,
        errors: Vec::<String>::new(),
        response: None,
    }
}

fn create_results() -> UpdateIpResults {
    UpdateIpResults {
        ip_service_result: create_ip_service_result(),
        domain_service_results: Vec::<DomainResult>::new(),
    }
}

/*
    part of top-level function series
*/
pub fn load_or_create_results(config: &Config) -> Option<UpdateIpResults> {
    let json_as_str = match File::open(&config.results_filepath) {
        Ok(r) => r,
        Err(e) => return None,
    };

    match serde_json::from_reader(&json_as_str) {
        Ok(j) => Some(j),
        Err(e) => return Some(create_results()),
    }
}

/*
    part of top-level function series
*/
pub fn write_to_file(
    results: UpdateIpResults,
    config: &Config,
) -> Result<UpdateIpResults, std::io::Error> {
    let file = match File::create(&config.results_filepath) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer_pretty(&mut writer, &results);
    // serde_json::to_writer(&mut writer, &results);
    let _ = writer.flush();

    Ok(results)
}

pub fn create_retry_set(results: &UpdateIpResults) -> HashSet<String> {
    let mut retry_set = HashSet::<String>::new();

    for domain_result in &results.domain_service_results {
        if domain_result.retry {
            retry_set.insert(domain_result.hostname.clone());
        }
    }

    retry_set
}
