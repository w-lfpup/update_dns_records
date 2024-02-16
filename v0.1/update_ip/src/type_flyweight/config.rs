use std::path;

use serde::{Deserialize, Serialize};

use crate::type_flyweight::dyndns2::Dyndns2;
use crate::type_flyweight::cloudflare::Cloudflare;

// add domain services here
// beware of hydra
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainServices {
    pub dyndns2: Option<Vec<Dyndns2>>,
    pub cloudflare: Option<Vec<Cloudflare>>,
}

// ip services are accounted for by response type
// beware of hydra
pub type IpServices = Vec<(String, String)>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub results_filepath: path::PathBuf,
    pub ip_services: IpServices,
    pub domain_services: DomainServices,
}


