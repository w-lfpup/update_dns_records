use serde::{Deserialize, Serialize};

use bytes::Bytes;
use http::Request;
use http_body_util::Full;
use std::collections::HashMap;

use requests;
use results::{DomainResult, UpdateIpResults};

// following types are based on:
// https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-update-dns-record

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cloudflare {
    pub email: String,
    pub zone_id: String,
    pub dns_record_id: String,
    pub api_token: String,
    pub name: String,
    pub proxied: Option<bool>,
    pub comment: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ttl: Option<usize>,
}

pub type CloudflareDomains = Vec<Cloudflare>;

#[derive(Clone, Serialize, Debug)]
pub struct CloudflareRequestBody {
    pub content: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<usize>,
}

/*
https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-patch-dns-record

PATCH Request
Only update changed parameters
*/

pub async fn update_domains(
    domain_results: &mut HashMap<String, DomainResult>,
    results: &UpdateIpResults,
    cloudflare_domains: &CloudflareDomains,
) {
    for domain in cloudflare_domains {
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

        // build domain result
        let domain_result = build_domain_result(&domain, &address).await;

        // write over previous entry
        domain_results.insert(domain.name.clone(), domain_result);
    }
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
            domain_result.errors.push(e);
            return domain_result;
        }
    };

    match requests::boxed_request_http1_tls_response(request).await {
        Ok(r) => domain_result.response = Some(r),
        Err(e) => domain_result.errors.push(e),
    }

    domain_result
}

fn get_cloudflare_req(domain: &Cloudflare, ip_addr: &str) -> Result<Request<Full<Bytes>>, String> {
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
        Err(e) => return Err(e.to_string()),
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
        Err(e) => Err(e.to_string()),
    }
}
