mod env;

use crate::error::Error;
use crate::token::ARG;
use std::io::Error as ioErr;
use std::io::ErrorKind;

pub fn init(mut a: ARG) -> Result<ARG, Error> {
    env::new().setup()?;
    match init_k8s() {
        Ok(j) => {
            a.join = Some(j);
            Ok(a)
        }
        Err(e) => {
            println!("init k8s error {}", e);
            Err(Error::InitK8sFail)
        }
    }
}

pub fn create_join(mut a: ARG) -> Result<ARG, Error> {
    Ok(a)
}

pub fn join(arg: &ARG) -> Result<(), Error> {
    println!("{}", arg);
    env::new().setup()?;
    Ok(())
}

fn init_k8s() -> Result<String, ioErr> {
    let r = run_fun!(
        "kubeadm init --control-plane-endpoint=192.168.100.1 --pod-network-cidr=192.186.0.0/16"
    )?
    .trim()
    .to_string();
    match find_join(&r) {
        Some(j) => Ok(j.to_string()),
        None => Err(ioErr::from(ErrorKind::NotFound)),
    }
}

fn find_join(s: &str) -> Option<&str> {
    Some(
        s.lines()
            .filter(|l| l.contains("kubeadm join"))
            .collect::<Vec<&str>>()
            .iter()
            .next()?
            .trim(),
    )
}
