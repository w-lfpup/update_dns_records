use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dyndns2 {
    pub hostname: String,
    pub password: String,
    pub service_uri: String,
    pub username: String,
}
