use bytes::Bytes;
use http::Request;
use http_body_util::Empty;
use std::collections::HashSet;

use crate::config::Config;
use crate::type_flyweight::{DomainResult, Squarespace, UpdateIpResults};
use crate::requests;

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

// must return results
pub async fn update_domains(
    mut domain_results: Vec<DomainResult>,
    prev_results: &UpdateIpResults,
    config: &Config,
    retry_set: &HashSet<String>,
) -> Vec<DomainResult> {
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

    println!("iterating through domains");

    for domain in domains {
        // do not update domain if address didn't change
        // and
        println!("{:?}", domain);
        if !address_updated && !retry_set.contains(&domain.hostname) {
            continue;
        }
        println!("made it past the addition sieve");
        let mut domain_result = DomainResult {
            domain: domain.hostname.clone(),
            retry: false,
            errors: Vec::<String>::new(),
            response: None,
        };

        let uri_str = get_uri(domain, address);

				// build request
				let mut request = None;
       	match Request::builder()
            .uri(uri_str)
            .header(hyper::header::HOST, "domains.google.com:443")
            .header(hyper::header::USER_AGENT, "hyper/1.0 rust-client")
            .body(Empty::<Bytes>::new())
        {
            Ok(s) => request = Some(s),
            _ => domain_result
                    .errors
                    .push("could not build squarespace dns request".to_string()),
        };

				// if request was successful, get response
        let mut res = None;
        if let Some(req) = request {
            match requests::request_http1_tls_response(req).await {
                Ok(r) => res = Some(r),
                _ => {
                    domain_result
                        .errors
                        .push("squarespace request failed".to_string());
                }
            };
        }

				// if response was successful, get jsonable struct
        if let Some(res) = res {
            match requests::convert_response_to_json(res).await {
              Ok(r) => domain_result.response = Some(r),
              _ => domain_result
                    .errors
                    .push("could not create jsonable response".to_string()),
            }
        };

				// if jsonable was successful, calculate retry
        //	only valid retries are
        //		- request failed
        //		- service returns "911"
        if let Some(response) = &domain_result.response {
            domain_result.retry = response.status_code != http::status::StatusCode::OK
                || response.body.starts_with(&"911".to_string());
        }

				// finally push domain_results into
        domain_results.push(domain_result);
    }

    domain_results
}

fn get_uri(domain: &Squarespace, ip_addr: &str) -> String {
    "https://".to_string()
        + &domain.username
        + ":"
        + &domain.password
        + "@domains.google.com/nic/update?hostname="
        + &domain.hostname
        + "&myip="
        + &ip_addr
}
