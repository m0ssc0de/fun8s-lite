mod cfg;
mod env;

use crate::error::Error;
use crate::token::ARG;
use base64::decode;
use env::run_s;
use std::fs;
use std::io::Error as ioErr;

enum HostRole {
    Master(ipnetwork::IpNetwork, std::net::IpAddr),
    Node(ipnetwork::IpNetwork),
    API(ipnetwork::IpNetwork),
}

pub fn reset() -> Result<(), Error> {
    let s = r#"
systemctl stop nebula
rm -rf /etc/nebula/*
    "#;
    let _ = env::run_s(s);
    Ok(())
}

pub fn init(pub_addr: std::net::IpAddr) -> Result<ARG, Error> {
    env::new().setup()?;
    generate_ca()?;
    let init_ip = "192.168.100.1/24".parse().unwrap();
    let mut arg = generate_host(HostRole::Master(init_ip, pub_addr))?;
    let varg = generate_host(HostRole::API("192.168.100.100/24".parse().unwrap()))?;
    arg.v_mesh_cfg = varg.v_mesh_cfg;
    join(&arg)?;
    save_ip(("192.168.100.1/24").parse().unwrap()).unwrap();
    Ok(arg)
}

pub fn create_join() -> Result<ARG, Error> {
    match load_ip() {
        Ok(net) => {
            let mut it = net.iter().skip_while(|x| *x <= net.ip());
            match ipnetwork::IpNetwork::new(it.next().unwrap(), net.prefix()) {
                Ok(ip) => {
                    let r = generate_host(HostRole::Node(ip));
                    if let Ok(a) = r {
                        save_ip(ip).unwrap();
                        return Ok(a);
                    }
                    r
                }
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

fn generate_host(host: HostRole) -> Result<ARG, Error> {
    let name = match host {
        HostRole::Master(_, _) => "master",
        HostRole::Node(_) => "node",
        HostRole::API(_) => "api",
    };
    match generate_host_run(host) {
        Ok(arg) => Ok(ARG {
            mesh_cfg: Some(arg),
            v_mesh_cfg: None,
            name: name.to_owned(),
            join: None,
        }),
        Err(e) => {
            println!("{}", e);
            Err(Error::InitMeshFail)
        }
    }
    // if let HostRole::Master(_, _) = host {
    //     match generate_host_run(HostRole::API("192.168.100.100/24")) {
    //         Ok(s) => {
    //             arg.v_mesh_cfg = Some(s);
    //             return Some(arg);
    //         }
    //         Err(e) => {
    //             println!("{}", e);
    //             return Err(Error::InitMeshFail);
    //         }
    //     }
    // }
}

fn generate_host_run(
    // name: &str,
    // ip: ipnetwork::IpNetwork,
    // is_light: Option<std::net::IpAddr>,
    host: HostRole,
) -> Result<String, ioErr> {
    let s = r#"
        cd /etc/nebula/
        SIGN_S
        mkdir -p NODE_N
        mv NODE_N.crt NODE_N/host.crt
        mv NODE_N.key NODE_N/host.key
        cp ca.crt NODE_N/ca.crt
    "#;
    match host {
        HostRole::Master(ip, pub_ip) => {
            let name = "master";
            let sign_s = format!(
                "nebula-cert sign -ca-crt=./ca.crt -ca-key=./ca.key -name '{}' -ip '{}'",
                name, ip
            );
            let s = s.replace("SIGN_S", &sign_s);
            let s = s.replace("NODE_N", name);
            run_s(&s)?;
            let s = cfg::L.replace("21.21.21.21", &pub_ip.to_string());
            let s = s.replace("am_lighthouse: false", "am_lighthouse: true");
            println!("{}", s);
            fs::write(format!("/etc/nebula/{}/config.yml", name), s).expect("Unable to write file");
            get_token("master")
        }
        HostRole::Node(ip) => {
            let name = "node";
            let sign_s = format!(
                "nebula-cert sign -ca-crt=./ca.crt -ca-key=./ca.key -name '{}' -ip '{}'",
                name, ip
            );
            let s = s.replace("SIGN_S", &sign_s);
            let s = s.replace("NODE_N", name);
            run_s(&s)?;
            let s = run_fun!("cat /etc/nebula/config.yml")?.trim().to_string();
            let s = s.replace("am_lighthouse: true", "am_lighthouse: false");
            let s = s.replace("port: 4242", "port: 0");
            println!("{}", s);
            fs::write(format!("/etc/nebula/{}/config.yml", name), s).expect("Unable to write file");
            get_token("node")
        }
        HostRole::API(ip) => {
            let name = "api";
            let sign_s = format!(
                "nebula-cert sign -ca-crt=./ca.crt -ca-key=./ca.key -name '{}' -ip '{}'",
                name, ip
            );
            let s = s.replace("SIGN_S", &sign_s);
            let s = s.replace("NODE_N", name);
            run_s(&s)?;
            let s = run_fun!("cat /etc/nebula/config.yml")?.trim().to_string();
            let s = s.replace("am_lighthouse: true", "am_lighthouse: false");
            let s = s.replace("port: 4242", "port: 0");
            println!("{}", s);
            fs::write(format!("/etc/nebula/{}/config.yml", name), s).expect("Unable to write file");
            get_token("api")
        }
    }
}

fn get_token(name: &str) -> Result<String, ioErr> {
    let s = r#"
        cd /etc/nebula
        tar -zcvf NODE_N.tar.gz NODE_N
        cat NODE_N.tar.gz | base64 -w 0 > /tmp/tmp-nebula/NODE_N.token
    "#;
    let s = s.replace("NODE_N", name);
    run_s(&s)?;
    let r = run_fun!("cat /tmp/tmp-nebula/{}.token", name)?;
    Ok(r.trim().to_string())
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
