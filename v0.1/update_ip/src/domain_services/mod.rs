use std::collections::HashMap;

use crate::config::Config;
use crate::type_flyweight::{DomainResult, UpdateIpResults};

mod dyndns2;

pub async fn update_domains(mut results: UpdateIpResults, config: &Config) -> UpdateIpResults {
    // bail early when no address is provided, keep previous results
    if let None = &results.ip_service_result.address {
        return results;
    };

    // add more ifs for more services
    // unfortunately that's the pattern but it's simple
    let mut domain_results = HashMap::<String, DomainResult>::new();
    domain_results = dyndns2::update_domains(domain_results, &results, config).await;

    results.domain_service_results = domain_results;
    results
}
