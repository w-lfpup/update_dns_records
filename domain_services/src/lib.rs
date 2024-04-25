use std::collections::HashMap;

use config::Config;
use results::{DomainResult, UpdateIpResults};

#[cfg(feature = "cloudflare")]
use cloudflare;
#[cfg(feature = "dyndns2")]
use dyndns2;

#[cfg(feature = "dyndns2")]
async fn add_dyndns2_to_domain_results(
    config: &Config,
    doman_results: &mut HashMap<String, DomainResult>,
) -> () {
    dyndns2::update_domains(domain_results, results, config.domain_services.dyndns2).await;
}

#[cfg(feature = "cloudflare")]
async fn add_cloudflare_to_domain_results(
    config: &Config,
    doman_results: &mut HashMap<String, DomainResult>,
) -> () {
    dyndns2::update_domains(domain_results, results, config.domain_services.dyndns2).await;
}

pub async fn update_domains(
    config: &Config,
    results: &UpdateIpResults,
) -> HashMap<String, DomainResult> {
    let mut domain_results = HashMap::<String, DomainResult>::new();

    // add more services here
    #[cfg(feature = "dyndns2")]
    add_dyndns2_to_domain_results(config, results);
    #[cfg(feature = "cloudflare")]
    add_cloudflare_to_domain_results(config, results);

    domain_results
}
