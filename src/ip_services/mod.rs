use rand::{thread_rng, Rng};

use crate::toolkit::config::Config;
use crate::toolkit::ip_services::IpServices;
use crate::toolkit::results::{IpServiceResult, UpdateIpResults};

mod address_as_body;

pub async fn fetch_service_results(
    config: &Config,
    prev_results: &Result<UpdateIpResults, String>,
) -> Result<IpServiceResult, String> {
    let service = match prev_results {
        Ok(results) => &results.ip_service_result.service,
        _ => "previous-results-do-not-exist",
    };

    let (ip_service, response_type) = match get_random_ip_service(&config.ip_services, service) {
        Some(r) => r,
        _ => return Err("failed to find ip service".to_string()),
    };

    let response_json_results = match response_type {
        // there could be json responses
        _ => address_as_body::request_address_as_body(&ip_service).await,
    };

    let response_json = match response_json_results {
        Ok(res_json) => res_json,
        Err(e) => return Err(e),
    };

    let ip_address = match response_type {
        // there could be json responses
        _ => address_as_body::get_ip_address_from_body(&response_json).await,
    };

    match ip_address {
        Ok(addr) => {
            let mut ip_struct = IpServiceResult::new(&ip_service);
            ip_struct.response = Some(response_json);
            ip_struct.ip_address = Some(addr);
            Ok(ip_struct)
        }
        Err(e) => Err(e),
    }
}

fn get_random_ip_service(ip_services: &IpServices, prev_service: &str) -> Option<(String, String)> {
    if ip_services.len() == 0 {
        return None;
    }

    if ip_services.len() == 1 {
        return Some(ip_services[0].clone());
    }

    // get previous service index
    let mut prev_index = None;
    for (index, (url, _ip_service_type)) in ip_services.iter().enumerate() {
        if url == &prev_service {
            prev_index = Some(index);
            break;
        };
    }

    // prev service might not exist
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
