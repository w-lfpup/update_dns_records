use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use http::Request;
use http_body_util::Empty;
use std::collections::HashMap;

use crate::config::Config;
use crate::requests;
use crate::type_flyweight::{DomainResult, Dyndns2, UpdateIpResults};

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
    prev_results: &UpdateIpResults,
    config: &Config,
) -> HashMap<String, DomainResult> {
    // don't fetch results if there are no dyndns2 domains
    let domains = match &config.domain_services.dyndns2 {
        Some(d) => d,
        _ => return domain_results,
    };

    for domain in domains {
        // someone could add or remove a domain from the config file between updates
        let prev_domain_result = prev_results.domain_service_results.get(&domain.hostname);

        // if address did not change and there's no retry, save previous domain_result
        if !prev_results.ip_service_result.address_changed && !should_retry(prev_domain_result) {
            if let Some(prev_result) = prev_domain_result {
                domain_results.insert(domain.hostname.clone(), prev_result.clone());
            }
            continue;
        }

        //  if no address exists move on
        //  the previous domain results could be lost at this junction
        //	(updated: true, retry: false) does not qualify up above
        //  add domain results before continue, looks awkward but *shrugs*
        let address = match &prev_results.ip_service_result.address {
            Some(d) => d,
            _ => {
                if let Some(prev_result) = prev_domain_result {
                    domain_results.insert(domain.hostname.clone(), prev_result.clone());
                }
                continue;
            }
        };

        // build domain result
        let mut domain_result = DomainResult::new(&domain.hostname);
        domain_result = create_build_result(domain_result, domain, &address).await;

        domain_results.insert(domain.hostname.clone(), domain_result);
    }

    domain_results
}

//	only valid retries are
//		- request failed
//		- service returns "911"
fn should_retry(domain_result: Option<&DomainResult>) -> bool {
    if let Some(prev_result) = domain_result {
        if let Some(response) = &prev_result.response {
            return response.body.starts_with("911");
        }
    }

    false
}

async fn create_build_result(
    mut domain_result: DomainResult,
    domain: &Dyndns2,
    address: &str,
) -> DomainResult {
    let request = match get_https_dyndns2_req(&domain, &address) {
        Ok(s) => Some(s),
        Err(e) => {
            domain_result.errors.push(e.to_string());
            None
        }
    };

    // get response
    let mut response = None;
    /*
    if let Some(req) = request {
        response = match requests::request_http1_tls_response(req).await {
            Ok(r) => Some(r),
            Err(e) => {
                domain_result.errors.push(e.to_string());
                None
            }
        };
    }
    */

    // create json-able struct from response
    // add to domain result
    if let Some(res) = response {
        match requests::convert_response_to_json_struct(res).await {
            Ok(r) => domain_result.response = Some(r),
            Err(e) => domain_result.errors.push(e.to_string()),
        }
    };

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
