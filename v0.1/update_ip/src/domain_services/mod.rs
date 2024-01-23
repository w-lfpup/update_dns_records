use std::collections::HashMap;

use crate::config::Config;
use crate::results;
use crate::type_flyweight::{DomainResult, UpdateIpResults};

mod squarespace;

// must update and return results
pub async fn update_domains(mut results: UpdateIpResults, config: &Config) -> UpdateIpResults {
    // bail early when no address is provided
    // keep previous results
    if let None = &results.ip_service_result.address {
        return results;
    };

    // add more ifs for more services
    // unfortunately that's the pattern but it's simple
    let mut domain_results = results.domain_service_results.clone();
    domain_results = squarespace::update_domains(domain_results, &results, config).await;

    results.domain_service_results = domain_results;

    results
}
