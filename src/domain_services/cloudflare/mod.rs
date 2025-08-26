use bytes::Bytes;
use http_body_util::Full;
use hyper::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::toolkit::config::Config;
use crate::toolkit::domain_services::cloudflare::Cloudflare;
use crate::toolkit::requests::{request_http1_tls_response, ResponseJson};
use crate::toolkit::results::{DomainResult, UpdateIpResults};

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

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CloudflareMinimalResponseBody {
    pub success: bool,
}

/*
https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-patch-dns-record

PATCH Request
Only update changed parameters
*/

pub async fn update_domains(
    config: &Config,
    prev_results: &Result<UpdateIpResults, String>,
    domain_results: &mut HashMap<String, DomainResult>,
    ip_address: &str,
) {
    let domains = match &config.domain_services.cloudflare {
        Some(domains) => domains,
        _ => return,
    };

    for domain in domains {
        let mut domain_result = match prev_results {
            Ok(results) => match results.domain_service_results.get(&domain.name) {
                Some(domain) => domain.clone(),
                _ => DomainResult::new(&domain.name),
            },
            _ => DomainResult::new(&domain.name),
        };

        let hostname = domain.name.clone();

        if let Some(domain_ip) = &domain_result.ip_address {
            if domain_ip == ip_address {
                domain_results.insert(hostname, domain_result);
                continue;
            }
        }

        // build domain result
        domain_result = build_domain_result(&domain, ip_address).await;
        // write over previous entry
        domain_results.insert(hostname, domain_result);
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

    let response = match request_http1_tls_response(request).await {
        Ok(r) => r,
        Err(e) => {
            domain_result.errors.push(e);
            return domain_result;
        }
    };

    if verify_resposne(&response) {
        domain_result.ip_address = Some(ip_address.to_string());
    }

    domain_result.response = Some(response);
    domain_result
}

fn verify_resposne(res: &ResponseJson) -> bool {
    if res.status_code != 200 {
        return false;
    }

    if let Ok(res_json) = serde_json::from_str::<CloudflareMinimalResponseBody>(&res.body) {
        return res_json.success;
    };

    false
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
