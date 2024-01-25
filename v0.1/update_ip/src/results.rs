use tokio::fs;

use crate::type_flyweight::{Config, UpdateIpResults};

pub async fn load_or_create_results(config: &Config) -> Option<UpdateIpResults> {
    if let Ok(json_as_str) = fs::read_to_string(&config.results_filepath).await {
        if let Ok(r) = serde_json::from_str(&json_as_str) {
            return Some(r);
        }
    };

    Some(UpdateIpResults::new())
}

pub fn address_has_changed(update_ip_results: &UpdateIpResults) -> bool {
    match (
        &update_ip_results.ip_service_result.prev_address,
        &update_ip_results.ip_service_result.address,
    ) {
        (Some(prev_ip), Some(curr_ip)) => prev_ip != curr_ip,
        (None, Some(_curr_ip)) => true,
        _ => false,
    }
}

pub async fn write_to_file(
    results: UpdateIpResults,
    config: &Config,
) -> Result<UpdateIpResults, String> {
    let json_str = match serde_json::to_string_pretty(&results) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    if let Err(e) = fs::write(&config.results_filepath, json_str).await {
        return Err(e.to_string());
    };

    Ok(results)
}
