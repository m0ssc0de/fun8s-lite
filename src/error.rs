use std::fmt;

pub enum Error {
    TokenError,
    NeedSetupMesh,
    SetupMeshFail,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TokenError => write!(f, "Parse input token fail"),
            Error::NeedSetupMesh => write!(f, "Need setup mesh env"),
            Error::SetupMeshFail => write!(f, "Setup mesh fail"),
        }
    }
}
