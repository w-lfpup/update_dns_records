use std::{env, path};

#[tokio::main]
async fn main() -> Result<(), String> {
    let config_path_str = match env::args().nth(1) {
        Some(a) => a,
        None => return Err("argument error:\nconfig file not found.".to_string()),
    };

    let config_path = path::Path::new(&config_path_str);

    let config = match config::from_path(config_path).await {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    // "copy" results from disk
    let prev_results = results::read_results_from_disk(&config.results_filepath).await;

    // update results
    let ip_service_result =
        ip_services::fetch_service_results(&config.ip_services, &prev_results).await;

    let domain_service_results =
        domain_services::update_domains(&config, &prev_results, &ip_service_result).await;

    let results = results::UpdateIpResults::try_from(ip_service_result, domain_service_results);

    results::write_results_to_disk(results, &config.results_filepath).await
}
