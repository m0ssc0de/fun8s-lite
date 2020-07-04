use crate::error::Error;
use packer::Packer;
use std::fs;

#[derive(Packer)]
#[packer(source = "files/mesh", prefixed = false)]
struct MeshFile;

pub struct ENV {}

impl ENV {
    pub fn setup(&self) -> Result<(), Error> {
        match self.check() {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("setup error {:?}", e);
                self.install()
            }
        }
    }
    fn check(&self) -> Result<(), Error> {
        if let Err(_) = run_cmd!("which nebula") {
            return Err(Error::NeedSetupMesh);
        }
        if let Err(_) = run_cmd!("which yq") {
            return Err(Error::NeedSetupMesh);
        }
        if let Err(_) = run_cmd!("test -f '/etc/systemd/system/nebula.service'") {
            return Err(Error::NeedSetupMesh);
        }
        if let Err(_) = run_cmd!("test -d '/etc/nebula/'") {
            return Err(Error::NeedSetupMesh);
        }
        Ok(())
    }
    fn install(&self) -> Result<(), Error> {
        let _ = run_cmd!("mkdir -p /tmp/tmp-nebula");
        let files = MeshFile::list();
        for f in files {
            let data = MeshFile::get(f).unwrap();
            println!("file: {}", f);
            fs::write(format!("/tmp/tmp-nebula/{}", f), data).expect("Unable to write file");
        }
        if let Err(e) = run_cmd!(
            mkdir -p /etc/nebula/

            cd /tmp/tmp-nebula/
            // wget https://github.com/slackhq/nebula/releases/download/v1.2.0/nebula-linux-amd64.tar.gz
            tar -zxvf nebula-linux-amd64.tar.gz
            pwd
            cp ./nebula ./nebula-cert /usr/local/bin/

            // wget https://raw.githubusercontent.com/slackhq/nebula/master/examples/service_scripts/nebula.service
            cp ./nebula.service /etc/systemd/system/nebula.service
        ) {
            println!("install error {}", e);
            return Err(Error::SetupMeshFail);
        }
        Ok(())
    }
}

pub fn new() -> ENV {
    ENV {}
}
