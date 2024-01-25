use std::collections::HashMap;

use crate::type_flyweight::{Config, DomainResult, UpdateIpResults};

mod dyndns2;

pub async fn update_domains(
    results: &UpdateIpResults,
    config: &Config,
) -> HashMap<String, DomainResult> {
    // add more services here
    let mut domain_results = HashMap::<String, DomainResult>::new();
    domain_results = dyndns2::update_domains(domain_results, results, config).await;

    domain_results
}
