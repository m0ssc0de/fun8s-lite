use std::fmt;

pub enum Error {
    TokenError,
    NeedSetupMesh,
    SetupMeshFail,
    InitMeshFail,
    JoinMeshFail,
    IPAMPersistenceFail,
    NeedSetupK8s,
    SetupK8sFail,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            TokenError => write!(f, "Parse input token fail"),
            NeedSetupMesh => write!(f, "Need setup mesh env"),
            SetupMeshFail => write!(f, "Setup mesh fail"),
            InitMeshFail => write!(f, "Init mesh fail"),
            JoinMeshFail => write!(f, "Join mesh fail"),
            IPAMPersistenceFail => write!(f, "Can not persistence ip in file"),
            NeedSetupK8s => write!(f, "Need setup k8s env"),
            SetupK8sFail => write!(f, "Setup k8s env fail"),
        }
    }
}
