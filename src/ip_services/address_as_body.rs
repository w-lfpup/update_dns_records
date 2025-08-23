use bytes::Bytes;
use http_body_util::Full;
use hyper::Request;
use std::net;

use crate::toolkit::requests::{get_host_and_authority, request_http1_tls_response, ResponseJson};

pub async fn request_address_as_body(service: &str) -> Result<ResponseJson, String> {
    let request = match create_request_with_empty_body(service) {
        Ok(req) => req,
        Err(e) => return Err(e),
    };

    match request_http1_tls_response(request).await {
        Ok(res) => Ok(res),
        Err(e) => return Err(e),
    }
}

pub async fn get_ip_address_from_body(response_json: &ResponseJson) -> Result<String, String> {
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

fn create_request_with_empty_body(url_string: &str) -> Result<Request<Full<Bytes>>, String> {
    let uri = match hyper::Uri::try_from(url_string) {
        Ok(u) => u,
        Err(e) => return Err(e.to_string()),
    };

    let (_, authority) = match get_host_and_authority(&uri) {
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
