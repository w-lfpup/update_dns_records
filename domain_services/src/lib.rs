use std::collections::HashMap;

use config::Config;
use results::{DomainResult, UpdateIpResults};

use cloudflare;
use dyndns2;

pub async fn update_domains(
    config: &Config,
    results: &UpdateIpResults,
) -> HashMap<String, DomainResult> {
    let mut domain_results = HashMap::<String, DomainResult>::new();

    // add more services here
    
    domain_results = dyndns2::update_domains(domain_results, results, config.domain_services.dyndns2).await;
    domain_results = cloudflare::update_domains(domain_results, results, config.domain_services.cloudflare).await;

    domain_results
}
