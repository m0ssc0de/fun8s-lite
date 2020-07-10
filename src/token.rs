use crate::error::Error;
#[macro_use]
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct ARG {
    pub mesh_cfg: Option<String>,
    pub v_mesh_cfg: Option<String>,
    pub name: String,
    pub join: Option<String>,
}

impl FromStr for ARG {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(a) => Ok(a),
            Err(e) => {
                println!("{}", e);
                Err(Error::TokenError)
            }
        }
    }
}
impl Display for ARG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
