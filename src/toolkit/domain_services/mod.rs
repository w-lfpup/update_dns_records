use serde::{Deserialize, Serialize};

// add domain services here
// beware of hydra

#[cfg(feature = "cloudflare")]
pub mod cloudflare;
#[cfg(feature = "dyndns2")]
pub mod dyndns2;

#[cfg(feature = "cloudflare")]
use cloudflare::Cloudflare;
#[cfg(feature = "dyndns2")]
use dyndns2::Dyndns2;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainServices {
    #[cfg(feature = "cloudflare")]
    pub cloudflare: Option<Vec<Cloudflare>>,
    #[cfg(feature = "dyndns2")]
    pub dyndns2: Option<Vec<Dyndns2>>,
}
