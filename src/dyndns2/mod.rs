use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use http_body_util::Full;
use hyper::Request;
use requests::{request_http1_tls_response, ResponseJson};
use results::{DomainResult, UpdateIpResults};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*
    Implements a subset of the dyndns2 protocol.
    https://help.dyn.com/remote-access-api/perform-update/
    https://help.dyn.com/remote-access-api/return-codes/

    Not all responses are implemented but all responses are recorded.
    Only the 911 response body warrants a retry
*/

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dyndns2 {
    pub service_uri: String,
    pub hostname: String,
    pub username: String,
    pub password: String,
}

pub type Dyndns2Domains = Vec<Dyndns2>;

const CLIENT_HEADER_VALUE: &str = "hyper/1.0 rust-client";

// must return results
pub async fn update_domains(
    domain_results: &mut HashMap<String, DomainResult>,
    prev_results: &Result<UpdateIpResults, String>,
    ip_address: &str,
    optional_domains: &Option<Dyndns2Domains>,
) {
    let domains = match optional_domains {
        Some(domains) => domains,
        _ => return,
    };

    for domain in domains {
        let domain_result = match prev_results {
            Ok(results) => match results.domain_service_results.get(&domain.hostname) {
                Some(domain) => domain.clone(),
                _ => DomainResult::new(&domain.hostname),
            },
            _ => DomainResult::new(&domain.hostname),
        };

        let hostname = domain.hostname.clone();

        if let Some(domain_ip) = &domain_result.ip_address {
            if domain_ip == ip_address {
                domain_results.insert(hostname, domain_result);
                continue;
            }
        }

        // build domain result
        let domain_result = build_domain_result(&domain, ip_address).await;

        // write over previous entry
        domain_results.insert(hostname, domain_result);
    }
}

async fn build_domain_result(domain: &Dyndns2, ip_address: &str) -> DomainResult {
    let mut domain_result = DomainResult::new(&domain.hostname);

    let request = match get_https_dyndns2_req(&domain, &ip_address) {
        Ok(s) => s,
        Err(e) => {
            domain_result.errors.push(e);
            return domain_result;
        }
    };

    // update domain service
    // create json-able struct from response
    // add to domain result
    match request_http1_tls_response(request).await {
        Ok(r) => {
            if verify_resposne(&r) {
                domain_result.ip_address = Some(ip_address.to_string());
            }
        }
        Err(e) => domain_result.errors.push(e),
    }

    domain_result
}

fn verify_resposne(res: &ResponseJson) -> bool {
    res.status_code >= 200 && res.status_code < 300
    // if body starts with nchg | good
}

fn get_https_dyndns2_req(domain: &Dyndns2, ip_addr: &str) -> Result<Request<Full<Bytes>>, String> {
    let uri_str = domain.service_uri.clone() + "?hostname=" + &domain.hostname + "&myip=" + ip_addr;
    let uri = match uri_str.parse::<hyper::Uri>() {
        Ok(u) => u,
        Err(e) => return Err(e.to_string()),
    };
    let host = match uri.host() {
        Some(u) => u.to_string(),
        None => return Err("host not found in uri".to_string()),
    };

    let auth_str = domain.username.to_string() + ":" + &domain.password;
    let auth = general_purpose::STANDARD.encode(&auth_str.as_bytes());
    let auth_value = "Basic ".to_string() + &auth;

    match Request::builder()
        .uri(uri)
        .header(hyper::header::HOST, host)
        .header(hyper::header::USER_AGENT, CLIENT_HEADER_VALUE)
        .header(hyper::header::AUTHORIZATION, auth_value)
        .body(Full::new(bytes::Bytes::new()))
    {
        Ok(req) => Ok(req),
        Err(e) => Err(e.to_string()),
    }
}
