use crate::error::Error;
use packer::Packer;
use std::fs;

#[derive(Packer)]
#[packer(source = "files/img", prefixed = false)]
struct ImgFiles;

#[derive(Packer)]
#[packer(source = "files/docker", prefixed = false)]
struct DockerFiles;

#[derive(Packer)]
#[packer(source = "files/k8s", prefixed = false)]
struct K8sFiles;

pub struct ENV {}

impl ENV {
    pub fn setup(&self) -> Result<(), Error> {
        match self.check() {
            Ok(_) => Ok(()),
            Err(_) => self.install(),
        }
    }
    fn check(&self) -> Result<(), Error> {
        let check_cmd = r#"
            which dockerd
            which kubeadm
            which kubectl
            which kubelet
        "#;
        if let Err(e) = run_s(&check_cmd) {
            println!("check result error {}", e);
            return Err(Error::NeedSetupK8s);
        }
        Ok(())
    }
    fn install(&self) -> Result<(), Error> {
        fs::create_dir_all("/tmp/docker").unwrap();
        let files = DockerFiles::list();
        for f in files {
            let data = DockerFiles::get(f).unwrap();
            println!("file: {}", f);
            fs::write(format!("/tmp/docker/{}", f), data).expect("Unable to write file");
        }

        let s = r#"
            cd /tmp/docker
            yum install -y ./
            systemctl enable --now docker
        "#;
        if let Err(e) = run_s(&s) {
            println!("install docker err {}", e);
            return Err(Error::NeedSetupK8s);
        }

        fs::create_dir_all("/tmp/images").unwrap();
        let files = ImgFiles::list();
        for f in files {
            let data = ImgFiles::get(f).unwrap();
            println!("file: {}", f);
            fs::write(format!("/tmp/images/{}", f), data).expect("Unable to write file");
        }
        if let Err(e) = run_s("docker load -i /tmp/images/img.tar.gz") {
            println!("install docker err {}", e);
            return Err(Error::NeedSetupK8s);
        }

        let s = r#"
modprobe br_netfilter

cat <<EOF | sudo tee /etc/sysctl.d/k8s.conf
net.bridge.bridge-nf-call-ip6tables = 1
net.bridge.bridge-nf-call-iptables = 1
EOF
sudo sysctl --system

# Set SELinux in permissive mode (effectively disabling it)
sudo setenforce 0 || true
sudo sed -i 's/^SELINUX=enforcing$/SELINUX=permissive/' /etc/selinux/config

        "#;
        if let Err(e) = run_s(s) {
            println!("install k8s error {}", e);
            return Err(Error::SetupK8sFail);
        }
        fs::create_dir_all("/tmp/k8s").unwrap();
        let files = K8sFiles::list();
        for f in files {
            let data = K8sFiles::get(f).unwrap();
            println!("file: {}", f);
            fs::write(format!("/tmp/k8s/{}", f), data).expect("Unable to write file");
        }

        let s = r#"
cd /tmp/k8s/
sudo yum install -y ./ --disableexcludes=kubernetes
sudo systemctl enable --now kubelet
        "#;
        if let Err(e) = run_s(s) {
            println!("install k8s error {}", e);
            return Err(Error::SetupK8sFail);
        }
        Ok(())
    }
}

pub fn new() -> ENV {
    ENV {}
}

pub fn run_s(s: &str) -> Result<(), std::io::Error> {
    fs::write("/tmp/s", format!("set -eux\n{}", s))?;
    run_cmd!("bash /tmp/s")?;
    Ok(())
}
