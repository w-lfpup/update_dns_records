use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cloudflare {
		pub name: String,
    pub api_token: String,
    pub zone_id: String,
    pub dns_record_id: String,
}

// following types are based on:
// https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-update-dns-record

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CloudflareRequestBody {
	pub content: String,
	pub name: String,
	pub proxied: Option<bool>,
	pub r#type: String,
	pub comment: Option<String>,
	pub tags: Option<Vec<String>>,
	pub ttl: Option<usize>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CloudflareDnsMessage {
	pub integer: usize,
	pub message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CloudflareDnsMeta {
 pub auto_added: bool,
 pub source: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CloudflareDnsARecord {
	pub content: String,
	pub name: String,
	pub proxied: Option<bool>,
	pub r#type: String,
	pub comment: Option<String>,
	pub created_on: String,
	pub id: String,
	pub locked: bool,
	pub meta: Option<CloudflareDnsMeta>,
	pub modified_on: String,
	pub proxiable: bool,
	pub tags: Option<Vec<String>>,
	pub ttl: Option<usize>,
	pub zone_id: Option<String>,
	pub zone_name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CloudflareResponseBody200 {
	pub result: CloudflareDnsARecord,
	pub errors: Vec<CloudflareDnsMessage>,
	pub messages: Vec<CloudflareDnsMessage>,
	pub success: bool, 
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CloudflareErrorResponseBody4xx {
	pub result: Option<CloudflareDnsARecord>,
	pub errors: Vec<CloudflareDnsMessage>,
	pub messages: Vec<CloudflareDnsMessage>,
	pub success: bool,
}
