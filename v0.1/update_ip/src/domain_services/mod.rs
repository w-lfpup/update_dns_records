use crate::config::Config;
use crate::results;
use crate::type_flyweight::UpdateIpResults;
use crate::type_flyweight::DomainResult;
mod squarespace;

// must update and return results
pub async fn update_domains(mut results: UpdateIpResults, config: &Config) -> UpdateIpResults {
    // bail early when no address is provided
    // or if there is no update
		println!("{:?}", &results.ip_service_result.address);
    let ip_address = match &results.ip_service_result.address {
        Some(ip) => ip.clone(),
        _ => {
            return results;
        }
    };
    
    let retry_set = results::create_retry_set(&results);
    
    
    // add more ifs for more services
    // unfortunately that's the pattern but it's simple
    let mut domain_results = Vec::<DomainResult>::new();
    domain_results = squarespace::update_domains(domain_results, &results, config, &retry_set).await;

		results.domain_service_results = domain_results;
		
    results
}
