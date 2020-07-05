##  What is fun8s-lite?

`fun8s-lite` is an ultimate simplicity tool that aims to establish a crossing cloud native Kubernetes cluster.

## Details of implement

The project was implemented with `rust`. This language has many modern programming 
features. It has nice performance and robustness. Although this project doesn't need 
such high performance, it's a great learning opportunity to complete a project by `rust`.

With the help of [nebula](https://github.com/slackhq/nebula), we could set up a crossing 
cloud `mesh network`. `mesh network` could let nodes communicating with each other directly, not only through the central server. Based on the work of the `nebula`, `fun8s-lite` make the whole 
process easier. All operations only have `3` commands, `init`, `create`, `join`. Even 
more, with the same `3` commands, it will help us to establish a native Kubernetes 
cluster over `internet`. No mater cluster lay on the `internet` or `intranet`, the 
operations both are effortless and rapid.

At the beginning of project, lots of implements based on 
executing shell script. They will be gradually replaced on 
demand. Most file operation and script execution benefit from the
`cmd_lib` crate.

The last feature is offline version. All packages and files need 
to be download will be embedded in the binary. So the offline 
version binary is the offline package. Maybe it will bring a 
little convenience.

## Quick Start

0. Download binary cli from the [release page](https://github.com/m0ssc0de/fun8s-lite/releases). 
You could choose `fun8s-lite-offline`, if you are in an intranet.
> At the beginning of this project, it will only support "centos 7"/amd64. Other's os may be added later.

1. Choose a host that is published to other hosts. Keep 
the public address and open the firewall or security 
group on the `4242` port. Assuming the host address is 
`123.123.123.123`. We just name it to host A. We could run the command below.

- `fun8s-lite init --address 123.123.123.123`

With the successful execution of the command, a cluster 
was successfully created.

2. If we need adding a new node to this cluster. Just execute this command on host A.

- `fun8s-lite create`

With the successful execution, we can get a command that looks like this.

`fun8s-lite join --token *********`

The token may be extensive. I will improve it later.
Different from the token from kubeadm, it will only be use once.

3. Login the node will be added to the cluster, we created before. Execute the join command, we got in step 2.

- `fun8s-lite join --token *********`

## Building fun8s-lite from source

With the help of `cargo`, just run `cargo build` will build the online version of this project.

If you want to build the offline version, make sure the `files` folder is filled by content defined in the text file in it.

I'm working on the high available version. If you're interested in it, welcome to leave an issue or an email. English is not my first language, welcome to submit grammatical errors.