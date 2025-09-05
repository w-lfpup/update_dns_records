use std::collections::HashMap;

use crate::toolkit::config::Config;
use crate::toolkit::results::{DomainResult, IpServiceResult, UpdateIpResults};

#[cfg(feature = "cloudflare")]
mod cloudflare;
#[cfg(feature = "dyndns2")]
mod dyndns2;

pub async fn update_domains(
    config: &Config,
    prev_results: &Result<UpdateIpResults, String>,
    ip_service_result: &IpServiceResult,
) -> Result<HashMap<String, DomainResult>, String> {
    let ip_address = match get_ip_address(prev_results, ip_service_result) {
        Ok(ip) => ip,
        Err(e) => return Err(e),
    };

    let mut domain_results = HashMap::<String, DomainResult>::new();

    // add more services here
    #[cfg(feature = "dyndns2")]
    dyndns2::update_domains(config, prev_results, &mut domain_results, &ip_address).await;

    #[cfg(feature = "cloudflare")]
    cloudflare::update_domains(config, prev_results, &mut domain_results, &ip_address).await;

    Ok(domain_results)
}

fn get_ip_address(
    prev_results: &Result<UpdateIpResults, String>,
    ip_service_result: &IpServiceResult,
) -> Result<String, String> {
    if let Some(ip_addr) = &ip_service_result.ip_address {
        return Ok(ip_addr.clone());
    }

    if let Ok(prev_result) = prev_results {
        if let Some(ip_addr) = &prev_result.ip_service_result.ip_address {
            return Ok(ip_addr.clone());
        }
    }

    Err("there are no ip addresses to update".to_string())
}
