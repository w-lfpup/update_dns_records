use std::fmt;

pub enum Error {
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(err) => write!(f, "{}", err),
        }
    }
}
