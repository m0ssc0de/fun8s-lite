mod cfg;
mod env;

use crate::error::Error;
use crate::token::ARG;
use base64::decode;
use env::run_s;
use std::fs;
use std::io::Error as ioErr;

pub fn init(pub_addr: std::net::IpAddr) -> Result<ARG, Error> {
    env::new().setup()?;
    generate_ca()?;
    let init_ip = "192.168.100.1/24".parse().unwrap();
    let arg = generate_host(init_ip, Some(pub_addr))?;
    join(&arg)?;
    Ok(arg)
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
    let s = r#"
        cd /etc/nebula/
        nebula-cert ca -name haha
    "#;
    if let Err(e) = run_s(s) {
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
    let sign_s = format!(
        "nebula-cert sign -ca-crt=./ca.crt -ca-key=./ca.key -name '{}' -ip '{}'",
        name, ip
    );
    let s = r#"
        cd /etc/nebula/
        SIGN_S
        mkdir -p NODE_N
        mv NODE_N.crt NODE_N/host.crt
        mv NODE_N.key NODE_N/host.key
        cp ca.crt NODE_N/ca.crt
    "#;
    let s = s.replace("SIGN_S", &sign_s);
    let s = s.replace("NODE_N", name);
    run_s(&s)?;
    match is_light {
        Some(l) => {
            let s = cfg::L.replace("21.21.21.21", &l.to_string());
            let s = s.replace("am_lighthouse: false", "am_lighthouse: true");
            println!("{}", s);
            fs::write(format!("/etc/nebula/{}/config.yml", name), s).expect("Unable to write file");
        }
        None => {
            let s = run_fun!("cat /etc/nebula/config.yml")?.trim().to_string();
            let s = s.replace("am_lighthouse: true", "am_lighthouse: false");
            println!("{}", s);
            fs::write(format!("/etc/nebula/{}/config.yml", name), s).expect("Unable to write file");
        }
    }

    let s = r#"
        cd /etc/nebula
        tar -zcvf NODE_N.tar.gz NODE_N
        cat NODE_N.tar.gz | base64 -w 0 > /tmp/tmp-nebula/NODE_N.token
    "#;
    let s = s.replace("NODE_N", name);
    run_s(&s)?;
    let r = run_fun!("cat /tmp/tmp-nebula/{}.token", name)?;
    Ok(ARG {
        test: "owieojoijf".to_string(),
        mesh_cfg: Some(r.trim().to_string()),
        name: name.to_string(),
        join: None,
    })
}

fn join_run(arg: &ARG) -> Result<(), ioErr> {
    let dec_tar = &decode(arg.mesh_cfg.as_ref().unwrap()).unwrap();
    fs::write("/etc/nebula/tmp.tar.gz", dec_tar)?;

    let s = r#"
    cd /etc/nebula
    tar -zxvf tmp.tar.gz
    cd /etc/nebula/NODE_N
    cp config.yml ca.crt host.crt host.key /etc/nebula
    systemctl enable --now nebula
    "#;
    let s = s.replace("NODE_N", &arg.name);
    run_s(&s)?;

    Ok(())
}

fn save_ip(ip: ipnetwork::IpNetwork) -> Result<(), ioErr> {
    fs::write("/tmp/fun8s-ip", ip.to_string())
}

fn load_ip() -> Result<ipnetwork::IpNetwork, ioErr> {
    Ok(run_fun!("cat /tmp/fun8s-ip")?.trim().parse().unwrap())
}
