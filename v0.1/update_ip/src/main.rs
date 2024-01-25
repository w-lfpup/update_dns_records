use std::env;
use std::path;

mod config;
mod domain_services;
mod ip_services;
mod requests;
mod results;
mod type_flyweight;

/*
    Load configuration file.
    Load previous results (or create new results).

    The results struct is passed through a series of top-level functions.
    Each function updates the results struct.

    Finally the results struct is written to disk.
*/

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

    // this is essentially a "copy" of the results on disk
    let mut results = match results::load_or_create_results(&config).await {
        Some(r) => r,
        None => return println!("results error:\nresults file not found."),
    };

    // update in-memory results
    results.ip_service_result = ip_services::request_ip(&results, &config).await;
    results.domain_service_results = domain_services::update_domains(&results, &config).await;

    // then write updated results to disk
    if let Err(e) = results::write_to_file(results, &config).await {
        return println!("file error:\n{}", e);
    };
}
