use std::env;
use std::path;

use config;
use domain_services;
use ip_services;
use results;

#[tokio::main]
async fn main() {
    let args = match env::args().nth(1) {
        Some(a) => a,
        None => return println!("argument error:\nconfig file not found."),
    };

    let config_path = path::Path::new(&args);
    let config = match config::from_path(config_path).await {
        Ok(c) => c,
        Err(e) => return println!("configuration error:\n{}", e),
    };

    // "copy" results from disk
    let prev_results = match results::load_or_create_results(&config.results_filepath).await {
        Some(r) => r,
        None => return println!("results error:\nresults file not found."),
    };

    // update results
    let ip_service_result = ip_services::request_ip(&config.ip_services, &prev_results).await;
    let domain_service_results = domain_services::update_domains(&config, &prev_results).await;

    let results = results::Results::new(ip_service_result, domain_service_results);

    // write updated results to disk
    if let Err(e) = results::write_to_file(results, &config.results_filepath).await {
        return println!("file error:\n{}", e);
    };
}
