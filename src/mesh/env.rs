use crate::error::Error;

pub struct ENV {}

impl ENV {
    pub fn setup(&self) -> Result<(), Error> {
        match self.check() {
            Ok(_) => Ok(()),
            Err(_) => self.install(),
        }
    }
    fn check(&self) -> Result<(), Error> {
        if let Err(_) = run_cmd!("which nebula") {
            return Err(Error::NeedSetupMesh);
        }
        Ok(())
    }
    fn install(&self) -> Result<(), Error> {
        if let Err(_) = run_cmd!(
            NB_TMP=/tmp/tmp-nebula/
            NB_FOLDER=/etc/nebula/
            NB_PATH=/usr/local/bin/

            mkdir -p $NB_TMP
            mkdir -p $NB_FOLDER

            cd $NB_TMP
            wget https://github.com/slackhq/nebula/releases/download/v1.2.0/nebula-linux-amd64.tar.gz
            tar -zxvf nebula-linux-amd64.tar.gz
            cp ./nebula* $NB_PATH/

            wget https://github.com/mikefarah/yq/releases/download/3.3.2/yq_linux_amd64 && mv yq_linux_amd64 $NB_PATH/yq && chmod +x $NB_PATH/yq

            curl https://raw.githubusercontent.com/slackhq/nebula/master/examples/service_scripts/nebula.service > /etc/systemd/system/nebula.service
        ) {
            return Err(Error::SetupMeshFail);
        }
        Ok(())
    }
}

pub fn new() -> ENV {
    ENV {}
}
