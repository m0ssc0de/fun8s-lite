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
        if let Err(e) = run_cmd!(
            which dockerd
            which kubeadm
            which kubectl
            which kubelet
        ) {
            println!("check result error {}", e);
            return Err(Error::NeedSetupK8s);
        }
        Ok(())
    }
    fn install(&self) -> Result<(), Error> {
        if let Err(e) = run_cmd!(
            yum install docker -y
            systemctl enable --now docker
        ) {
            println!("install docker err {}", e);
            return Err(Error::NeedSetupK8s);
        }
        if let Err(e) = run_cmd!(
            modprobe br_netfilter
            echo -e "net.bridge.bridge-nf-call-ip6tables = 1\n net.bridge.bridge-nf-call-iptables = 1" | sudo tee /etc/sysctl.d/k8s.conf
            sudo sysctl --system
        ) {
            println!("install k8s error {}", e);
            return Err(Error::NeedSetupK8s);
        }
        let k_repo_path = "/etc/yum.repos.d/kubernetes.repo";
        let _ = run_cmd!("touch {}", k_repo_path);
        let k_repo = r#"
        [kubernetes]
        name=Kubernetes
        baseurl=https://packages.cloud.google.com/yum/repos/kubernetes-el7-\$basearch
        enabled=1
        gpgcheck=1
        repo_gpgcheck=1
        gpgkey=https://packages.cloud.google.com/yum/doc/yum-key.gpg https://packages.cloud.google.com/yum/doc/rpm-package-key.gpg
        exclude=kubelet kubeadm kubectl
        "#;
        if let Err(e) = fs::write(k_repo_path, k_repo) {
            println!("write k8s repo error {}", e);
            return Err(Error::NeedSetupK8s);
        }

        if let Err(e) = run_cmd!(
            sudo setenforce 0 || true
            sudo setenforce 0
        ) {
            println!("install k8s error {}", e);
            return Err(Error::NeedSetupK8s);
        }

        if let Err(e) =
            run_cmd!("sudo sed -i 's/^SELINUX=enforcing$/SELINUX=permissive/' /etc/selinux/config")
        {
            println!("isntall k8s error {}", e);
            return Err(Error::NeedSetupK8s);
        }

        if let Err(e) = run_cmd!(
            sudo yum install -y kubelet kubeadm kubectl --disableexcludes=kubernetes
            sudo systemctl enable --now kubelet
        ) {
            println!("isntall k8s error {}", e);
            return Err(Error::NeedSetupK8s);
        }
        Ok(())
    }
}

pub fn new() -> ENV {
    ENV {}
}
