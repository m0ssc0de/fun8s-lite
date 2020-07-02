use std::fmt;

pub enum Error {
    TokenError,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TokenError => write!(f, "Parse input token fail"),
        }
    }
}
