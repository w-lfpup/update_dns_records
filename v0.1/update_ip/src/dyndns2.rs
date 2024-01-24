
pub async fn get_subset_request(
    service_domain: &str,
    ip_addr: &str,
    hostname: &str,
    username: &str,
    password: &str,
) -> String {
    let auth_str = domain.username.to_string() + ":" + &domain.password;

    let mut domain_result = results::create_domain_result(&domain.hostname);
    let auth = general_purpose::STANDARD.encode(&auth_str.as_bytes());
    let auth_value = "Basic ".to_string() + &auth;

    // build request
    let request = match Request::builder()
        .uri(uri_str)
        .header(hyper::header::USER_AGENT, CLIENT_HEADER_VALUE)
        .header(hyper::header::AUTHORIZATION, auth_value)
        .body(Empty::<Bytes>::new())
    {
        Ok(s) => Some(s),
        Err(e) => {
            domain_result.errors.push(e.to_string());
            None
        }
    };

    "https://".to_string()
        + service_domain
        + "/nic/update?hostname="
        + hostname
        + "&myip="
        + ip_addr
}

// get subset request
