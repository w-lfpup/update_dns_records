use std::collections::HashMap;

use config::Config;
use results::{DomainResult, IpServiceResult, UpdateIpResults};

#[cfg(feature = "cloudflare")]
use cloudflare;
#[cfg(feature = "dyndns2")]
use dyndns2;

pub async fn update_domains(
    config: &Config,
    prev_results: &Option<UpdateIpResults>,
    ip_service_result: &Option<IpServiceResult>,
) -> Result<HashMap<String, DomainResult>, String> {
    let ip_address = match get_ip_address(prev_results, ip_service_result) {
        Ok(ip) => ip,
        Err(e) => return Err(e),
    };

    let mut domain_results = HashMap::<String, DomainResult>::new();

    // add more services here
    #[cfg(feature = "dyndns2")]
    dyndns2::update_domains(
        &mut domain_results,
        prev_results,
        &ip_address,
        &config.dyndns2,
    )
    .await;

    #[cfg(feature = "dyndns2")]
    cloudflare::update_domains(
        &mut domain_results,
        prev_results,
        &ip_address,
        &config.cloudflare,
    )
    .await;

    Ok(domain_results)
}

// function to get ip address

fn get_ip_address(
    prev_results: &Option<UpdateIpResults>,
    ip_service_result: &Option<IpServiceResult>,
) -> Result<String, String> {
    if let Some(ip_result) = ip_service_result {
        if let Some(ip_addr) = &ip_result.ip_address {
            return Ok(ip_addr.clone());
        }
    }
    if let Some(prev_result) = prev_results {
        if let Some(ip_addr) = &prev_result.ip_service_result.ip_address {
            return Ok(ip_addr.clone());
        }
    }

    Err("there are no ip addresses to update".to_string())
}
