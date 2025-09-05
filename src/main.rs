use std::{env, path};

mod domain_services;
mod ip_services;
mod toolkit;

use crate::toolkit::{config, results};

#[tokio::main]
async fn main() -> Result<(), String> {
    let filepath_str = match env::args().nth(1) {
        Some(a) => a,
        None => return Err("argument error:\nconfig file not found.".to_string()),
    };

    let config_path = path::Path::new(&filepath_str);

    let config = match config::from_path(config_path).await {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let prev_results = results::read_results_from_disk(&config).await;

    let ip_service_result = match ip_services::fetch_service_results(&config, &prev_results).await {
        Ok(ip_result) => ip_result,
        Err(e) => return Err(e),
    };

    let domain_service_results =
        domain_services::update_domains(&config, &prev_results, &ip_service_result).await;

    results::write_results_to_disk(
        &config.results_filepath,
        ip_service_result,
        domain_service_results,
    )
    .await
}
