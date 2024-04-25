use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use http::Request;
use http_body_util::Empty;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use requests;
use results::{DomainResult, UpdateIpResults};

/*
    Implements a subset of the dyndns2 protocol.
    https://help.dyn.com/remote-access-api/perform-update/
    https://help.dyn.com/remote-access-api/return-codes/
    https://support.google.com/domains/answer/6147083?hl=en

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
    results: &UpdateIpResults,
    domains: &Dyndns2Domains,
) {
    for domain in domains {
        // copy previous results initially
        let prev_domain_result = results.domain_service_results.get(&domain.hostname);
        if let Some(prev_result) = prev_domain_result {
            domain_results.insert(domain.hostname.clone(), prev_result.clone());
        }

        // continue if address did not change and there's no retry
        if !results::address_has_changed(results) && !should_retry(prev_domain_result) {
            continue;
        }

        //  get address or continue
        let address = match (
            &results.ip_service_result.prev_address,
            &results.ip_service_result.address,
        ) {
            (Some(prev_addr), None) => prev_addr,
            (_, Some(addr)) => addr,
            _ => continue,
        };

        // build domain result
        let domain_result = build_domain_result(&domain, &address).await;

        // write over previous entry
        domain_results.insert(domain.hostname.clone(), domain_result);
    }
}

//	only valid retries are
//		- server failed
//		- service returns "911"
fn should_retry(domain_result: Option<&DomainResult>) -> bool {
    if let Some(prev_result) = domain_result {
        if let Some(response) = &prev_result.response {
            return response.body.starts_with("911");
        }
    }

    false
}

async fn build_domain_result(domain: &Dyndns2, address: &str) -> DomainResult {
    let mut domain_result = DomainResult::new(&domain.hostname);

    let request = match get_https_dyndns2_req(&domain, &address) {
        Ok(s) => s,
        Err(e) => {
            domain_result.errors.push(e);
            return domain_result;
        }
    };

    // update domain service
    // create json-able struct from response
    // add to domain result
    match requests::request_http1_tls_response(request).await {
        Ok(r) => domain_result.response = Some(r),
        Err(e) => domain_result.errors.push(e),
    }

    domain_result
}

fn get_https_dyndns2_req(domain: &Dyndns2, ip_addr: &str) -> Result<Request<Empty<Bytes>>, String> {
    let uri_str = domain.service_uri.clone() + "?hostname=" + &domain.hostname + "&myip=" + ip_addr;
    let uri = match uri_str.parse::<http::Uri>() {
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
        .body(Empty::<Bytes>::new())
    {
        Ok(req) => Ok(req),
        Err(e) => Err(e.to_string()),
    }
}
