use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use http::Request;
use http_body_util::Empty;
use std::collections::HashMap;

use crate::requests;
use crate::results;

use crate::type_flyweight::config::Config;
use crate::type_flyweight::dyndns2::Dyndns2;
use crate::type_flyweight::results::{DomainResult, UpdateIpResults};

/*
    Implements a subset of the dyndns2 protocol.
    https://help.dyn.com/remote-access-api/perform-update/
    https://help.dyn.com/remote-access-api/return-codes/
    https://support.google.com/domains/answer/6147083?hl=en

    Not all responses are implemented but all responses are recorded.
    Only the 911 response body warrants a retry
*/

const CLIENT_HEADER_VALUE: &str = "hyper/1.0 rust-client";

// must return results
pub async fn update_domains(
    mut domain_results: HashMap<String, DomainResult>,
    results: &UpdateIpResults,
    config: &Config,
) -> HashMap<String, DomainResult> {
    // don't fetch results if there are no dyndns2 domains
    let domains = match &config.domain_services.dyndns2 {
        Some(d) => d,
        _ => return domain_results,
    };

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

    domain_results
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
            domain_result.errors.push(e.to_string());
            return domain_result;
        }
    };

    // update domain service here
    // create json-able struct from response
    // add to domain result
    match requests::request_http1_tls_response(request).await {
        Ok(r) => domain_result.response = Some(r),
        Err(e) => domain_result.errors.push(e.to_string()),
    }

    domain_result
}

fn get_https_dyndns2_req(
    domain: &Dyndns2,
    ip_addr: &str,
) -> Result<Request<Empty<Bytes>>, http::Error> {
    let uri_str = domain.service_uri.clone() + "?hostname=" + &domain.hostname + "&myip=" + ip_addr;
    let auth_str = domain.username.to_string() + ":" + &domain.password;
    let auth = general_purpose::STANDARD.encode(&auth_str.as_bytes());
    let auth_value = "Basic ".to_string() + &auth;

    match Request::builder()
        .uri(uri_str)
        .header(hyper::header::USER_AGENT, CLIENT_HEADER_VALUE)
        .header(hyper::header::AUTHORIZATION, auth_value)
        .body(Empty::<Bytes>::new())
    {
        Ok(req) => Ok(req),
        Err(e) => Err(e),
    }
}
