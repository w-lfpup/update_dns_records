use std::env;
use std::path;

use config;
use domain_services;
use ip_services;
use results;

#[tokio::main]
async fn main() {
    let config_path_str = match env::args().nth(1) {
        Some(a) => a,
        None => return println!("argument error:\nconfig file not found."),
    };

    let config_path = path::Path::new(&config_path_str);

    let config = match config::from_path(config_path).await {
        Ok(c) => c,
        Err(e) => return println!("configuration error:\n{}", e),
    };

    // "copy" results from disk
    let prev_results = match results::read_results_from_disk(&config.results_filepath).await {
        Ok(results) => Some(results),
        _ => None,
    };

    // update results
    let ip_service_result =
        ip_services::fetch_service_results(&config.ip_services, &prev_results).await;

    let domain_service_results =
        domain_services::update_domains(&config, &prev_results, &ip_service_result).await;

    let results =
        match results::UpdateIpResults::try_from(ip_service_result, domain_service_results) {
            Ok(c) => c,
            Err(e) => return println!("{}", e),
        };

    // write updated results to disk
    if let Err(e) = results::write_results_to_disk(results, &config.results_filepath).await {
        return println!("file error:\n{}", e);
    };
}
