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
        Some(a) => path::PathBuf::from(a),
        None => return println!("argument error:\nconfig file not found."),
    };

    let config = match config::from_filepath(&args).await {
        Ok(c) => c,
        Err(e) => return println!("configuration error:\n{}", e),
    };

    let mut results = match results::load_or_create_results(&config) {
        Some(r) => r,
        None => return println!("results error:\nresults file not found."),
    };

    results = ip_services::request_ip(results, &config).await;
    // results = domain_services::update_domains(results, &config).await;

    if let Err(e) = results::write_to_file(results, &config) {
        return println!("file error:\n{}", e);
    };
}
