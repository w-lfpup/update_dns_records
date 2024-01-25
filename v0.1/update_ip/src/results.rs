use tokio::fs;

// can use after mod declaration
use crate::type_flyweight::{Config, UpdateIpResults};

pub async fn load_or_create_results(config: &Config) -> Option<UpdateIpResults> {
    if let Ok(json_as_str) = fs::read_to_string(&config.results_filepath).await {
        if let Ok(r) = serde_json::from_str(&json_as_str) {
            return Some(r);
        }
    };

    Some(UpdateIpResults::new())
}

pub async fn write_to_file(
    results: UpdateIpResults,
    config: &Config,
) -> Result<UpdateIpResults, String> {
    let json_str = match serde_json::to_string_pretty(&results) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    let _ = fs::write(&config.results_filepath, json_str).await;

    Ok(results)
}
