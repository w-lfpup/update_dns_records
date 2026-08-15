use serde::{Deserialize, Serialize};
// use serde_json::Error as SerdeJsonError;
use std::fmt;
// use std::io::Error as IoError;
// use hyper::Error as HyperError;
// use hyper::http::uri::{InvalidUri};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    Io(String),
    IpAddr(String),
    Hyper(String),
    SerdeJson(String),
    NativeTls(String),
    Uri(String),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::IpAddr(err) => write!(f, "{}", err),
            Error::Hyper(err) => write!(f, "{}", err),
            Error::NativeTls(err) => write!(f, "{}", err),
            Error::SerdeJson(err) => write!(f, "{}", err),
            Error::Uri(err) => write!(f, "{}", err),
            Error::Custom(err) => write!(f, "{}", err),
        }
    }
}
