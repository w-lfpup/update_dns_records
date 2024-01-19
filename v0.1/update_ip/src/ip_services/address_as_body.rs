use std::net;

use crate::requests;
use crate::results;
use crate::type_flyweight::IpServiceResult;

// base use case
// request without a body returns response with IP Address as body
pub async fn request_address_as_response_body(
    mut ip_service_result: IpServiceResult,
) -> IpServiceResult {
    // bail early if ip_service is None
    let ip_service = match &ip_service_result.service {
        Some(service) => service,
        _ => return ip_service_result,
    };

    let request = match requests::create_request_with_empty_body(&ip_service) {
        Ok(req) => Some(req),
        _ => {
            ip_service_result
                .errors
                .push("could not create request".to_string());
            None
        }
    };

    let mut response = None;
    if let Some(req) = request {
        match requests::request_http1_tls_response(req).await {
            Ok(r) => response = Some(r),
            _ => {
                ip_service_result
                    .errors
                    .push("ip service request failed".to_string());
            }
        };
    }

    if let Some(res) = response {
        match requests::convert_response_to_json(res).await {
            Ok(j) => ip_service_result.response = Some(j),
            _ => {
                ip_service_result
                    .errors
                    .push("failed to create jsonable response".to_string());
            }
        };
    };

    if let Some(response) = &ip_service_result.response {
        match response.body.parse::<net::IpAddr>() {
            Ok(_ip) => ip_service_result.address = Some(response.body.clone()),
            _ => ip_service_result
                .errors
                .push("ip address could not be parsed from response".to_string()),
        }
    }

    ip_service_result
}
