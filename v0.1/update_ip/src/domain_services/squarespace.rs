use bytes::Bytes;
use http::Request;
use http_body_util::Empty;
use std::collections::HashMap;
use std::collections::HashSet;

use base64;
use base64::{engine::general_purpose, Engine as _};

use crate::config::Config;
use crate::requests;
use crate::results;
use crate::type_flyweight::{DomainResult, Squarespace, UpdateIpResults};

/*
https://support.google.com/domains/answer/6147083?hl=en

requests must have a agent user

Response 	Status 	Description
good {user’s IP address} 	Success 	The update was successful. You should not attempt another update until your IP address changes.
nochg {user’s IP address} 	Success 	The supplied IP address is already set for this host. You should not attempt another update until your IP address changes.
nohost 	Error 	The hostname doesn't exist, or doesn't have Dynamic DNS enabled.
badauth 	Error 	The username/password combination isn't valid for the specified host.
notfqdn 	Error 	The supplied hostname isn't a valid fully-qualified domain name.
badagent 	Error 	Your Dynamic DNS client makes bad requests. Ensure the user agent is set in the request.
abuse 	Error 	Dynamic DNS access for the hostname has been blocked due to failure to interpret previous responses correctly.
911 	Error 	An error happened on our end. Wait 5 minutes and retry.
conflict A
conflict AAAA 	Error 	A custom A or AAAA resource record conflicts with the update. Delete the indicated resource record within the DNS settings page and try the update again.
*/

const SERVICE_URI_HOST: &str = "domains.google.com";
const SERVICE_URI_AUTHORITY: &str = "domains.google.com";
const CLIENT_HEADER_VALUE: &str = "Chrome/41.0 brian.t.vann@gmail.com";

// must return results
pub async fn update_domains(
    mut domain_results: HashMap<String, DomainResult>,
    prev_results: &UpdateIpResults,
    config: &Config,
) -> HashMap<String, DomainResult> {
    // don't fetch results if there are no squarespace domains
    let domains = match &config.domain_services.squarespace {
        Some(d) => d,
        _ => return domain_results,
    };

    // don't fetch if there isn't an address
    let address = match &prev_results.ip_service_result.address {
        Some(d) => d,
        _ => return domain_results,
    };

    let address_updated = prev_results.ip_service_result.address_changed;

    for domain in domains {
        // do not update domain if address didn't change
        // and current domain is not in retry set
        let prev_domain_result = prev_results.domain_service_results.get(&domain.hostname);

        // someone could add or remove a domain from the config file between updates
        // if new / not in previous results, "retry"
        // if prev results existed get retry and critical
        let mut retry = true;
        if let Some(prev_result) = prev_domain_result {
            retry = prev_result.retry;
        }

        // do not update if address has not changed and no retries
        if !address_updated && !retry {
            continue;
        }

        let uri_str = get_https_dyndns2_uri(
            SERVICE_URI_HOST,
            &address,
            &domain.hostname,
            &domain.username,
            &domain.password,
        );

        let auth_str = domain.username.to_string() + ":" + &domain.password;

        let mut domain_result = results::create_domain_result(&domain.hostname);
        let auth = general_purpose::STANDARD.encode(&auth_str.as_bytes());
        let auth_value = "Basic ".to_string() + &auth;

        // build request
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
        if let Some(req) = request {
            response = match requests::request_http1_tls_response(req).await {
                Ok(r) => Some(r),
                Err(e) => {
                    domain_result.retry = true;
                    domain_result.errors.push(e.to_string());
                    None
                }
            };
        }

        // if response was successful, get jsonable struct
        if let Some(res) = response {
            match requests::convert_response_to_json(res).await {
                Ok(r) => domain_result.response = Some(r),
                Err(e) => domain_result.errors.push(e.to_string()),
            }
        };

        // if jsonable was successful, calculate retry
        //	only valid retries are
        //		- request failed
        //		- service returns "911"
        if let Some(response) = &domain_result.response {
            domain_result.retry = response.status_code != http::status::StatusCode::OK
                || response.body.starts_with(&"911".to_string());
        };

        // finally push domain_results into
        domain_results.insert(domain.hostname.clone(), domain_result);
    }

    domain_results
}

fn get_https_dyndns2_uri(
    service_domain: &str,
    ip_addr: &str,
    hostname: &str,
    username: &str,
    password: &str,
) -> String {
    "https://domains.google.com/nic/update?hostname=".to_string() + hostname + "&myip=" + ip_addr
}
