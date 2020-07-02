mod env;

use crate::error::Error;
use crate::token::ARG;

pub fn init(mut a: ARG) -> Result<ARG, Error> {
    a.test = "owijeofij".to_string();
    // a.test_update()?;
    Ok(a)
}

pub fn create_join(mut a: ARG) -> Result<ARG, Error> {
    a.test_update()?;
    Ok(a)
}

pub fn join(arg: &ARG) -> Result<(), Error> {
    println!("{}", arg);
    env::new().setup()?;
    Ok(())
}
