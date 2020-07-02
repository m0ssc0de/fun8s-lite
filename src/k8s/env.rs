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
        Ok(())
    }
    fn install(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub fn new() -> ENV {
    ENV {}
}
