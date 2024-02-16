use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cloudflare {
		pub name: String,
		pub email: String,
    pub api_token: String,
    pub zone_id: String,
    pub dns_record_id: String,
}

// following types are based on:
// https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-update-dns-record

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
pub struct CloudflareResponseBody {
	// 4xx status code could give null result
	pub result: Option<CloudflareDnsARecord>,
	pub errors: Vec<CloudflareDnsMessage>,
	pub messages: Vec<CloudflareDnsMessage>,
	pub success: bool,
}
*/
