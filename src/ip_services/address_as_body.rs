use std::net;

use crate::requests::{
    ResponseDetails, create_request_with_empty_body, request_http1_tls_response,
};

pub async fn request_address_as_body(service: &str) -> Result<ResponseDetails, String> {
    let request = match create_request_with_empty_body(service) {
        Ok(req) => req,
        Err(e) => return Err(e),
    };

    match request_http1_tls_response(request).await {
        Ok(res) => Ok(res),
        Err(e) => return Err(e),
    }
}

pub async fn get_ip_address_from_body(response_json: &ResponseDetails) -> Result<String, String> {
    if response_json.status_code != 200 {
        return Err("response_json was not okay".to_string());
    }

    // set address if request is successful
    let ip_address = match response_json.body.trim().parse::<net::IpAddr>() {
        Ok(ip) => ip.to_string(),
        _ => return Err("ip address could not be parsed from response_json".to_string()),
    };

    Ok(ip_address)
}
