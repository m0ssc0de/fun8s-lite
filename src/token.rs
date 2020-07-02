use crate::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

pub struct ARG {
    pub test: String,
}

impl FromStr for ARG {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 10 {
            return Err(Error::TokenError);
        }
        Ok(ARG {
            test: s.clone().to_string(),
        })
    }
}
impl Display for ARG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(ARG test:{}", self.test)
    }
}

impl ARG {
    pub fn test_update(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
