use std::collections::HashMap;
pub struct ResponseJson {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub timestamp: u128,
}
