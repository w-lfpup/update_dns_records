use serde::{Deserialize, Serialize};

use bytes::Bytes;
use http::Request;
use http_body_util::Full;
use std::collections::HashMap;

use requests;
use results::{DomainResult, ResponseJson, UpdateIpResults};

// following types are based on:
// https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-update-dns-record

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cloudflare {
    pub email: String,
    pub zone_id: String,
    pub dns_record_id: String,
    pub api_token: String,
    pub name: String,
    pub r#type: String,
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
    prev_results: &Option<UpdateIpResults>,
    ip_address: &str,
    cloudflare_domains: &CloudflareDomains,
) {
    for domain in cloudflare_domains {
        let domain_result = match prev_results {
            Some(results) => match results.domain_service_results.get(&domain.name) {
                Some(domain) => domain,
                _ => &DomainResult::new(&domain.name),
            },
            _ => &DomainResult::new(&domain.name),
        };

        if let Some(domain_ip) = &domain_result.ip_address {
            if domain_ip == ip_address {
                continue;
            }
        }

        // build domain result
        let domain_result = build_domain_result(&domain, ip_address).await;
        // write over previous entry
        domain_results.insert(domain.name.clone(), domain_result);
    }
}

async fn build_domain_result(domain: &Cloudflare, ip_address: &str) -> DomainResult {
    let mut domain_result = DomainResult::new(&domain.name);

    let request = match get_cloudflare_req(&domain, &ip_address) {
        Ok(s) => s,
        Err(e) => {
            domain_result.errors.push(e);
            return domain_result;
        }
    };

    // update domain service
    // create json-able struct from response
    // add to domain result
    match requests::boxed_request_http1_tls_response(request).await {
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
        r#type: domain.r#type.clone(),
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
