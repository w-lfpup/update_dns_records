use std::collections::HashMap;

use crate::requests;
use crate::results;
use crate::type_flyweight::{Config, DomainResult, Cloudflare, UpdateIpResults};

pub async fn update_domains(
    mut domain_results: HashMap<String, DomainResult>,
    results: &UpdateIpResults,
    config: &Config,
) -> HashMap<String, DomainResult> {
	domain_results
}
