#[macro_use]
extern crate cmd_lib;
#[macro_use]
extern crate clap;
mod error;
mod k8s;
mod mesh;
mod token;

use error::Error;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Error> {
    let matches = clap_app!(myapp =>
        (version: "0.0.1")
        (author: "m0ssc0de <hi.paul.q@gmail.com>")
        (about: "create a k8s cluster on mesh network")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand init =>
            (about: "init a instance for cluster")
            (version: "0.0.1")
            (author: "m0ssc0de <hi.paul.q@gmail.com>")
            (@arg address: -a --address[ADDR] +required "public ip address")
        )
        (@subcommand create =>
            (about: "create a instance for cluster")
            (version: "0.0.1")
            (author: "m0ssc0de <hi.paul.q@gmail.com>")
        )
        (@subcommand join =>
            (about: "join to a cluster")
            (version: "0.0.1")
            (author: "m0ssc0de <hi.paul.q@gmail.com>")
            (@arg token: -t --token[TOKEN] +required "the token created by init instance")
        )
    )
    .get_matches();

    if let Some(m) = matches.subcommand_matches("init") {
        mesh::init(m.value_of("address").unwrap().parse().unwrap())?;
    }

    if let Some(_) = matches.subcommand_matches("create") {
        println!(
            r#"
            Create instance sucessful. Please run the cmd later on the host will run this instance.
            fun8s-lite join -t {}
        "#,
            base64::encode(format!("{}", mesh::create_join().unwrap()))
        );
    }

    if let Some(m) = matches.subcommand_matches("join") {
        mesh::join(
            &String::from_utf8(base64::decode(&m.value_of("token").unwrap()).unwrap())
                .unwrap()
                .parse::<token::ARG>()
                .unwrap(),
        )?;
    }

    Ok(())
}
