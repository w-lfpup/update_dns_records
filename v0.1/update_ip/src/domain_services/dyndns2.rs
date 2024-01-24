use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use http_body_util::Empty;
use http::Request;
use std::collections::HashMap;

use crate::config::Config;
use crate::requests;
use crate::results;
use crate::type_flyweight::{DomainResult, UpdateIpResults};

/*
    Implements a subset of the dyndns2 protocol.
		https://help.dyn.com/remote-access-api/perform-update/
    https://help.dyn.com/remote-access-api/return-codes/
    https://support.google.com/domains/answer/6147083?hl=en
    
    Not all responses are implemented but all responses are recorded.
    Only the 911 warrants a retry
*/

const CLIENT_HEADER_VALUE: &str = "hyper/1.0 rust-client";
// const CLIENT_HEADER_VALUE: &str = "Chrome/41.0 brian.t.vann@gmail.com";

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
        // if jsonable was successful, calculate retry
        //	only valid retries are
        //		- request failed
        //		- service returns "911"
        
        let prev_domain_result = prev_results.domain_service_results.get(&domain.hostname);
        let mut retry = true;
        if let Some(prev_result) = prev_domain_result {
            if let Some(response) = &prev_result.response {
                retry = response.body.starts_with(&"911".to_string());
            }
        }

        // do not update if address has not changed and no retries
        if !prev_results.ip_service_result.address_changed && !retry {
            // add old result to new result
            if let Some(prev_result) = prev_domain_result {
                domain_results.insert(domain.hostname.clone(), prev_result.clone());
            }
            continue;
        }
        // if no address address, add previous results
        let address = match &prev_results.ip_service_result.address {
			    Some(d) => d,
			    _ => {
						if let Some(prev_result) = prev_domain_result {
								domain_results.insert(domain.hostname.clone(), prev_result.clone());
						}
						continue;
					}
				};

        let uri_str = get_https_dyndns2_uri(&domain.domain, &domain.hostname, &address);
        let auth_str = domain.username.to_string() + ":" + &domain.password;
        let auth = general_purpose::STANDARD.encode(&auth_str.as_bytes());
        let auth_value = "Basic ".to_string() + &auth;

        // build request
        let mut domain_result = results::create_domain_result(&domain.hostname);
        let request = match Request::builder()
            .uri(uri_str)
            .header(hyper::header::USER_AGENT, CLIENT_HEADER_VALUE)
            .header(hyper::header::AUTHORIZATION, auth_value)
            .body(Empty::<Bytes>::new())
        {
            Ok(s) => Some(s),
            Err(e) => {
                domain_result.errors.push(e.to_string());
                None
            }
        };

        // if request was successful, get response
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

        // if response was successful, get jsonable struct
        if let Some(res) = response {
            match requests::convert_response_to_json(res).await {
                Ok(r) => domain_result.response = Some(r),
                Err(e) => domain_result.errors.push(e.to_string()),
            }
        };

        // finally push domain_results into
        domain_results.insert(domain.hostname.clone(), domain_result);
    }

    domain_results
}

fn get_https_dyndns2_uri(domain_service: &str, hostname: &str, ip_addr: &str) -> String {
    "https://".to_string()
        + domain_service
        + "/nic/update?hostname="
        + hostname
        + "&myip="
        + ip_addr
}
