mod cfg;
mod env;

use crate::error::Error;
use crate::token::ARG;
use base64::decode;
use std::fs;
use std::io::Error as ioErr;

pub fn init(pub_addr: std::net::IpAddr) -> Result<(), Error> {
    env::new().setup()?;
    generate_ca()?;
    let init_ip = "192.168.100.1/24".parse().unwrap();
    let arg = generate_host(init_ip, Some(pub_addr))?;
    join(&arg)
}

pub fn create_join() -> Result<ARG, Error> {
    match load_ip() {
        Ok(net) => {
            let mut it = net.iter().skip_while(|x| *x <= net.ip());
            match ipnetwork::IpNetwork::new(it.next().unwrap(), net.prefix()) {
                Ok(ip) => generate_host(ip, None),
                Err(_) => Err(Error::IPAMPersistenceFail),
            }
        }
        Err(e) => {
            println!("{}", e);
            Err(Error::IPAMPersistenceFail)
        }
    }
}

pub fn join(arg: &ARG) -> Result<(), Error> {
    env::new().setup()?;
    if let Err(e) = join_run(arg) {
        println!("{}", e);
        return Err(Error::JoinMeshFail);
    }
    Ok(())
}

fn generate_ca() -> Result<(), Error> {
    if let Err(e) = run_cmd!(
        cd /etc/nebula/
        nebula-cert ca -name haha
    ) {
        println!("generate ca {}", e);
        return Err(Error::InitMeshFail);
    }
    Ok(())
}

fn generate_host(
    private_ip: ipnetwork::IpNetwork,
    pub_addr: Option<std::net::IpAddr>,
) -> Result<ARG, Error> {
    match generate_host_run("node", private_ip, pub_addr) {
        Ok(arg) => {
            if let Err(e) = save_ip(private_ip) {
                println!("{}", e);
                return Err(Error::IPAMPersistenceFail);
            };
            Ok(arg)
        }
        Err(e) => {
            println!("{}", e);
            Err(Error::InitMeshFail)
        }
    }
}

fn generate_host_run(
    name: &str,
    ip: ipnetwork::IpNetwork,
    is_light: Option<std::net::IpAddr>,
) -> Result<ARG, ioErr> {
    run_cmd!(
        cd /etc/nebula/
    )?;
    run_cmd!(
        "nebula-cert sign -ca-crt=./ca.crt -ca-key=./ca.key -name '{}' -ip '{}'",
        name,
        ip
    )?;
    run_cmd!("mkdir -p {}", name)?;
    run_cmd!("mv {}.crt {}/host.crt", name, name)?;
    run_cmd!("mv {}.key {}/host.key", name, name)?;
    run_cmd!("cp ca.crt {}/ca.crt", name)?;
    if let Some(l) = is_light {
        let s = cfg::L.replace("21.21.21.21", &l.to_string());
        println!("{}", s);
        fs::write(format!("/etc/nebula/{}/config.yml", name), s).expect("Unable to write file");
    }

    run_cmd!("tar -zcvf {}.tar.gz {}", name, name)?;
    let r = run_fun!("cat {}.tar.gz | base64 -w 0", name)?;
    Ok(ARG {
        test: "owieojoijf".to_string(),
        mesh_cfg: Some(r.trim().to_string()),
        name: name.to_string(),
    })
}

fn join_run(arg: &ARG) -> Result<(), ioErr> {
    run_cmd!("cd /etc/nebula")?;
    let dec_tar = &decode(arg.mesh_cfg.as_ref().unwrap()).unwrap();
    fs::write("/etc/nebula/tmp.tar.gz", dec_tar)?;
    run_cmd!("tar -zxvf tmp.tar.gz")?;
    run_cmd!("cd /etc/nebula/{}", arg.name)?;
    run_cmd!("pwd")?;
    run_cmd!("cp config.yml ca.crt host.crt host.key /etc/nebula")?;
    run_cmd!("systemctl enable --now nebula")?;
    Ok(())
}

fn save_ip(ip: ipnetwork::IpNetwork) -> Result<(), ioErr> {
    fs::write("/tmp/fun8s-ip", ip.to_string())
}

fn load_ip() -> Result<ipnetwork::IpNetwork, ioErr> {
    Ok(run_fun!("cat /tmp/fun8s-ip")?.trim().parse().unwrap())
}
