use bytes::Bytes;
use http_body_util::Full;
use hyper::Request;
use requests::{get_host_and_authority, request_http1_tls_response, ResponseJson};
use std::net;

use requests;

// request with empty body returns response body with IP Address
pub async fn request_address_as_response_body(service: &str) -> Result<String, String> {
    let request = match create_request_with_empty_body(service) {
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

pub fn create_request_with_empty_body(url_string: &str) -> Result<Request<Full<Bytes>>, String> {
    let uri = match hyper::Uri::try_from(url_string) {
        Ok(u) => u,
        Err(e) => return Err(e.to_string()),
    };

    let (_, authority) = match requests::get_host_and_authority(&uri) {
        Some(u) => u.clone(),
        _ => return Err("authority not found in url".to_string()),
    };

    let req = match Request::builder()
        .uri(uri)
        .header(hyper::header::HOST, authority.as_str())
        .body(Full::new(bytes::Bytes::new()))
    {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    Ok(req)
}
