use std::fmt;

pub trait Service {
    fn name() -> String;
    fn start(&self) -> Result<(), Error>;
    fn stop(&self) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error {
    Start(String, String),
    Stop(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Start(name, err) => {
                write!(f, "The service: {}, failed to start because: {}", name, err)
            }
            Error::Stop(name, err) => {
                write!(f, "The service: {}, fialed to stop because: {}", name, err)
            }
        }
    }
}
