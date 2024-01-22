use std::net;

use crate::requests;
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
        Ok(req) => req,
        Err(e) => {
            ip_service_result.errors.push(e);
            return ip_service_result;
        }
    };

    let response = match requests::request_http1_tls_response(request).await {
        Ok(r) => r,
        Err(e) => {
            ip_service_result.errors.push(e);
            return ip_service_result;
        }
    };

    // track response
    ip_service_result.response = match requests::convert_response_to_json(response).await {
        Ok(j) => Some(j),
        _ => {
            ip_service_result
                .errors
                .push("failed to create jsonable response".to_string());
            None
        }
    };

    // get address if request is successful
    if let Some(response) = &ip_service_result.response {
        match response.body.parse::<net::IpAddr>() {
            Ok(ip) => ip_service_result.address = Some(ip.to_string()),
            _ => ip_service_result
                .errors
                .push("ip address could not be parsed from response".to_string()),
        }
    }

    ip_service_result
}
