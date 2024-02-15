use std::collections::HashMap;

use crate::requests;
use crate::results;

use crate::type_flyweight::config::Config;
use crate::type_flyweight::results::{DomainResult, UpdateIpResults};
use crate::type_flyweight::cloudflare::Cloudflare;
/*
curl --request PUT \
  --url https://api.cloudflare.com/client/v4/zones/zone_id/dns_records/dns_record_id \
  --header 'Content-Type: application/json' \
  --header 'X-Auth-Email: brian.t.vann@gmail.com' \
  --header 'Authorization: Bearer api_key' \
  --data '{
  "content": "0.0.0.0",
  "name": "example.com",
  "proxied": false,
  "type": "A",
  "comment": "Domain verification record",
  "ttl": 60
}'
*/

pub async fn update_domains(
    mut domain_results: HashMap<String, DomainResult>,
    results: &UpdateIpResults,
    config: &Config,
) -> HashMap<String, DomainResult> {
	

	domain_results
}

/*
// bytes as body response
fn get_cloudflare_req(
    domain: &Cloudflare,
    ip_addr: &str,
) -> Result<Request<Empty<Bytes>>, http::Error> {
    let uri_str = "https://api.cloudflare.com/client/v4/zones/" + domain.zone_id + "/dns_records/" + doman.dns_record_id;
    let auth_value = "Bearer ".to_string() + &domain.api_key;
    
    let data = "{
  \"content\": \"0.0.0.0\",
  \"name\": \"example.com\",
  \"type\": \"A\",
  \"ttl\": 60
}";
    
    match Request::builder()
        .uri(uri_str)
        .header(hyper::header::USER_AGENT, CLIENT_HEADER_VALUE)
        .header("X-Auth-Email", &domain.email)
        .header(hyper::header::AUTHORIZATION, auth_value)
        .body(Empty::<Bytes>::new())
    {
        Ok(req) => Ok(req),
        Err(e) => Err(e),
    }
}
*/
