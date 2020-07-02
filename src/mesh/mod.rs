mod env;

use crate::error::Error;
use crate::token::ARG;

pub fn init() -> Result<ARG, Error> {
    env::new().setup()?;
    generate_ca()?;
    let arg = generate_host()?;
    join(&arg)?;
    Ok(ARG {
        test: "oiwjelijf".to_string(),
    })
}

pub fn create_join() -> Result<ARG, Error> {
    generate_host()?;
    Ok(ARG {
        test: "oiwjelijf".to_string(),
    })
}

pub fn join(arg: &ARG) -> Result<(), Error> {
    println!("{}", arg);
    env::new().setup()?;
    Ok(())
}

fn generate_ca() -> Result<(), Error> {
    Ok(())
}

fn generate_host() -> Result<ARG, Error> {
    Ok(ARG {
        test: "welijfoi".to_string(),
    })
}
