use rand::{thread_rng, Rng};

use results::{IpServiceResult, UpdateIpResults};

mod address_as_body;

// ip services are accounted for by response type
// beware of potential hydra
pub type IpServices = Vec<(String, String)>;

pub async fn fetch_service_results(
    ip_services: &IpServices,
    prev_results: &Option<UpdateIpResults>,
) -> Result<IpServiceResult, String> {
    let service = match prev_results {
        Some(results) => &results.ip_service_result.service,
        None => "previous-results-do-not-exist",
    };

    let (ip_service, response_type) = match get_random_ip_service(ip_services, service) {
        Some(r) => r,
        _ => return Err("failed to find ip service".to_string()),
    };

    let address = match response_type {
        // there could be json responses
        _ => address_as_body::request_address_as_response_body(&ip_service).await,
    };

    match address {
        Ok(addr) => {
            let mut ip_struct = IpServiceResult::new(&ip_service);
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
