use serde::{Deserialize, Serialize};

// following type is based on:
// https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-update-dns-record
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cloudflare {
    pub api_token: String,
    pub comment: Option<String>,
    pub dns_record_id: String,
    pub email: String,
    pub name: String,
    pub proxied: Option<bool>,
    pub r#type: String,
    pub tags: Option<Vec<String>>,
    pub ttl: Option<usize>,
    pub zone_id: String,
}
