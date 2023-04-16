use iced_x86::IcedError;
use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    AsmError(IcedError),
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<IcedError> for Error {
    fn from(err: IcedError) -> Self {
        Error::AsmError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AsmError(err) => {
                write!(f, "{err}")?;
            }
            Error::IoError(err) => {
                write!(f, "{err}")?;
            }
        }
        Ok(())
    }
}
