use std::collections::HashMap;

use config::Config;
use results::{DomainResult, UpdateIpResults};

#[cfg(feature = "cloudflare")]
use cloudflare;
#[cfg(feature = "dyndns2")]
use dyndns2;

pub async fn update_domains(
    config: &Config,
    results: &UpdateIpResults,
) -> HashMap<String, DomainResult> {
    let mut domain_results = HashMap::<String, DomainResult>::new();

    // add more services here
    #[cfg(feature = "dyndns2")]
    dyndns2::update_domains(&mut domain_results, results, &config.dyndns2).await;

    #[cfg(feature = "dyndns2")]
    cloudflare::update_domains(&mut domain_results, results, &config.cloudflare).await;

    domain_results
}
