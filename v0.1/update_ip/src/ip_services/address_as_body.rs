use std::net;

use crate::requests;
use crate::type_flyweight::IpServiceResult;

// request with empty body returns response body with IP Address
pub async fn request_address_as_response_body(
    mut ip_service_result: IpServiceResult,
    ip_service: String,
) -> IpServiceResult {
    ip_service_result.service = Some(ip_service);
    let request = match requests::create_request_with_empty_body(&ip_service) {
        Ok(req) => req,
        Err(e) => {
            ip_service_result.errors.push(e);
            return ip_service_result;
        }
    };

    ip_service_result.response = match requests::request_http1_tls_response(request).await {
        Ok(r) => Some(r),
        Err(e) => {
            ip_service_result.errors.push(e);
            None
        }
    };

    // set address if request is successful
    if let Some(response) = &ip_service_result.response {
        if response.status_code != 200 {
            ip_service_result
                .errors
                .push("response was not okay".to_string());
            return ip_service_result;
        }

        match response.body.trim().parse::<net::IpAddr>() {
            Ok(ip) => ip_service_result.address = Some(ip.to_string()),
            _ => ip_service_result
                .errors
                .push("ip address could not be parsed from response".to_string()),
        }
    }

    ip_service_result
}
