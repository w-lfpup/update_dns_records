use std::collections::HashMap;

use crate::type_flyweight::config::Config;
use crate::type_flyweight::results::{DomainResult, UpdateIpResults};

mod cloudflare;
mod dyndns2;

pub async fn update_domains(
    config: &Config,
    results: &UpdateIpResults,
) -> HashMap<String, DomainResult> {
    let mut domain_results = HashMap::<String, DomainResult>::new();

    // add more services here
    domain_results = dyndns2::update_domains(domain_results, results, config).await;
    domain_results = cloudflare::update_domains(domain_results, results, config).await;

    domain_results
}
