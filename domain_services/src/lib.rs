use std::collections::HashMap;

use config::DomainServices;
use results::{DomainResult, UpdateIpResults};

#[cfg(feature = "cloudflare")]
use cloudflare;
#[cfg(feature = "dyndns2")]
use dyndns2;

pub async fn update_domains(
    domain_services: &DomainServices,
    results: &UpdateIpResults,
) -> HashMap<String, DomainResult> {
    let mut domain_results = HashMap::<String, DomainResult>::new();

    // add more services here
    if cfg!(feature = "dyndns2") {
        dyndns2::update_domains(&mut domain_results, results, &domain_services.dyndns2).await;
    }
    if cfg!(feature = "cloudflare") {
        cloudflare::update_domains(&mut domain_results, results, &domain_services.cloudflare).await;
    }

    domain_results
}
