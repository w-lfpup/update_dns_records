use std::net;

use requests;

// request with empty body returns response body with IP Address
pub async fn request_address_as_response_body(service: &str) -> Result<String, String> {
    let request = match requests::create_request_with_empty_body(service) {
        Ok(req) => req,
        Err(e) => return Err(e),
    };

    let response = match requests::request_http1_tls_response(request).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    if response.status_code != 200 {
        return Err("response was not okay".to_string());
    }

    // set address if request is successful
    let ip_address = match response.body.trim().parse::<net::IpAddr>() {
        Ok(ip) => ip.to_string(),
        _ => return Err("ip address could not be parsed from response".to_string()),
    };

    Ok(ip_address)
}
