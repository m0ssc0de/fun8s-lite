mod error;
mod k8s;
mod mesh;
mod token;

use error::Error;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Error> {
    {
        // cmd init
        let arg = mesh::init()?;
        k8s::init(arg)?;
    }

    {
        // cmd create join
        let arg = mesh::create_join()?;
        k8s::create_join(arg)?;
    }

    {
        let arg = "oiwjocijoiej".parse::<token::ARG>()?;
        mesh::join(&arg)?;
        k8s::join(&arg)?;
    }

    Ok(())
}
