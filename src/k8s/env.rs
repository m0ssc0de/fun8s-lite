use crate::error::Error;
use std::fs;

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
        let s = r#"
            yum install docker -y
            systemctl enable --now docker
        "#;
        if let Err(e) = run_s(&s) {
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

cat <<EOF | sudo tee /etc/yum.repos.d/kubernetes.repo
[kubernetes]
name=Kubernetes
baseurl=https://packages.cloud.google.com/yum/repos/kubernetes-el7-\$basearch
enabled=1
gpgcheck=1
repo_gpgcheck=1
gpgkey=https://packages.cloud.google.com/yum/doc/yum-key.gpg https://packages.cloud.google.com/yum/doc/rpm-package-key.gpg
exclude=kubelet kubeadm kubectl
EOF

# Set SELinux in permissive mode (effectively disabling it)
sudo setenforce 0 || true
sudo sed -i 's/^SELINUX=enforcing$/SELINUX=permissive/' /etc/selinux/config

sudo yum install -y kubelet kubeadm kubectl --disableexcludes=kubernetes

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
