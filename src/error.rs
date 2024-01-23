use std::{error, fmt::Debug, fmt::Display};

pub type Result<T> = std::result::Result<T, Error>;
pub struct Error {
    pub kind: ErrorKind,
}
pub enum ErrorKind {
    DirectoryDoesNotExist(String),
    InvalidOption(String),
    ErrorReadingFS,
    OperationAbortedByUser,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::DirectoryDoesNotExist(msg) => {
                write!(f, "The directory you indicated ({}) does not exist.", msg)
            }
            ErrorKind::InvalidOption(msg) => write!(f, "The option you wrote is invalid: {}", msg),
            ErrorKind::ErrorReadingFS => {
                write!(f, "An external error happened")
            }
            ErrorKind::OperationAbortedByUser => write!(f, "The user ended the operation"),
        }
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::DirectoryDoesNotExist(msg) => {
                write!(f, "The directory you indicated ({}) does not exist.", msg)
            }
            ErrorKind::InvalidOption(msg) => write!(f, "The option you wrote is invalid: {}", msg),
            ErrorKind::ErrorReadingFS => {
                write!(f, "An external error happened")
            }
            ErrorKind::OperationAbortedByUser => write!(f, "The user ended the operation"),
        }
    }
}
impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}
