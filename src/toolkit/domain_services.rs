use serde::{Deserialize, Serialize};

// following type is based on:
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

#[cfg(feature = "dyndns2")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dyndns2 {
    pub service_uri: String,
    pub hostname: String,
    pub username: String,
    pub password: String,
}

