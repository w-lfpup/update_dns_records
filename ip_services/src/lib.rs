use rand::{thread_rng, Rng};

use results::{IpServiceResult, UpdateIpResults};

mod address_as_body;

// ip services are accounted for by response type
// beware of hydra
pub type IpServices = Vec<(String, String)>;

pub async fn request_ip(ip_services: &IpServices, results: &UpdateIpResults) -> IpServiceResult {
    // preserve the last run's "current" address as this run's previous address
    let mut ip_service_result = IpServiceResult::new();
    ip_service_result.prev_address = match &results.ip_service_result.address {
        Some(address) => Some(address.clone()),
        _ => results.ip_service_result.prev_address.clone(),
    };

    // get service uri and response type or return previous results
    let (ip_service, response_type) = match get_random_ip_service(ip_services, results) {
        Some(r) => r,
        _ => {
            ip_service_result
                .errors
                .push("failed to find ip service".to_string());
            return ip_service_result;
        }
    };

    // preserve service uri and set service results based on response type
    ip_service_result.service = Some(ip_service);
    match response_type {
        _ => address_as_body::request_address_as_response_body(ip_service_result).await,
    }
}

fn get_random_ip_service(
    ip_services: &IpServices,
    results: &UpdateIpResults,
) -> Option<(String, String)> {
    if ip_services.len() == 0 {
        return None;
    }

    if ip_services.len() == 1 {
        return Some(ip_services[0].clone());
    }

    // get previous service index
    let mut prev_index = None;
    if let Some(service) = &results.ip_service_result.service {
        for (index, (url, _ip_service_type)) in ip_services.iter().enumerate() {
            if url == service {
                prev_index = Some(index);
                break;
            };
        }
    }

    // possibility prev service doesn't exist
    let length = match prev_index {
        Some(_index) => ip_services.len() - 1,
        _ => ip_services.len(),
    };

    let mut rng = thread_rng();
    let mut random_index = rng.gen_range(0..length);
    if let Some(index) = prev_index {
        if random_index >= index {
            random_index += 1;
        }
    }

    return Some(ip_services[random_index].clone());
}
