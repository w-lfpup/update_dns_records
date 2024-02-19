use bytes::Bytes;
use http::Request;
use http_body_util::Full;
use std::collections::HashMap;

use crate::requests;
use crate::results;

use crate::type_flyweight::cloudflare::{Cloudflare, CloudflareRequestBody};
use crate::type_flyweight::config::Config;
use crate::type_flyweight::results::{DomainResult, UpdateIpResults};

/*
https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-patch-dns-record

PATCH Request
Only update changed parameters
*/

pub async fn update_domains(
    mut domain_results: HashMap<String, DomainResult>,
    results: &UpdateIpResults,
    config: &Config,
) -> HashMap<String, DomainResult> {
    let domains = match &config.domain_services.cloudflare {
        Some(d) => d,
        _ => return domain_results,
    };

    for domain in domains {
        // copy previous results initially
        let prev_domain_result = results.domain_service_results.get(&domain.name);
        if let Some(prev_result) = prev_domain_result {
            domain_results.insert(domain.name.clone(), prev_result.clone());
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

        println!("get results");
        // build domain result
        let domain_result = build_domain_result(&domain, &address).await;

        println!("write over results");
        // write over previous entry
        domain_results.insert(domain.name.clone(), domain_result);
    }

    domain_results
}

// if a response code is 200 no retry, 400 no retry bad info just list it
// if there is no entry, it is an added entry and should be retried
fn should_retry(domain_result: Option<&DomainResult>) -> bool {
    if let Some(prev_result) = domain_result {
        if let Some(response) = &prev_result.response {
            if 500 <= response.status_code && response.status_code < 600 {
                return true;
            }
        }
    }

    false
}

async fn build_domain_result(domain: &Cloudflare, address: &str) -> DomainResult {
    let mut domain_result = DomainResult::new(&domain.name);

    let request = match get_cloudflare_req(&domain, &address) {
        Ok(s) => s,
        Err(e) => {
            domain_result.errors.push(e.to_string());
            return domain_result;
        }
    };

    println!("{:?}", &request);

    // update domain service here
    // create json-able struct from response
    // add to domain result

    match requests::boxed_request_http1_tls_response(request).await {
        Ok(r) => {
            println!("{:?}", &r);
            domain_result.response = Some(r);
        }
        Err(e) => domain_result.errors.push(e.to_string()),
    }

    domain_result
}

// bytes as body response
// make error more noticable
fn get_cloudflare_req(
    domain: &Cloudflare,
    ip_addr: &str,
) -> Result<Request<Full<Bytes>>, http::Error> {
    let uri_str = "https://api.cloudflare.com/client/v4/zones/".to_string()
        + &domain.zone_id
        + "/dns_records/"
        + &domain.dns_record_id;

    let auth_value = "Bearer ".to_string() + &domain.api_token;

    let body = CloudflareRequestBody {
        content: ip_addr.to_string(),
        name: domain.name.clone(),
        proxied: domain.proxied.clone(),
        r#type: "A".to_string(),
        comment: domain.comment.clone(),
        tags: domain.tags.clone(),
        ttl: domain.ttl.clone(),
    };

    let body_str = match serde_json::to_string(&body) {
        Ok(json_str) => json_str,
        Err(_) => "".to_string(),
    };

    match Request::builder()
        .method("PATCH")
        .uri(uri_str)
        .header(hyper::header::HOST, "api.cloudflare.com")
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .header("X-Auth-Email", &domain.email)
        .header(hyper::header::AUTHORIZATION, auth_value)
        .body(Full::new(Bytes::from(body_str)))
    {
        Ok(req) => Ok(req),
        Err(e) => Err(e),
    }
}
