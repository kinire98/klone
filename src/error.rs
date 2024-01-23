use std::{error, fmt::Debug, fmt::Display};

pub type Result<T> = std::result::Result<T, Error>;
pub struct Error {
    pub kind: ErrorKind,
}
pub enum ErrorKind {
    DirectoryDoesNotExist(String),
    ErrorReadingFS,
    OperationAbortedByUser,
    InvalidOption,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::DirectoryDoesNotExist(msg) => {
                write!(f, "The directory you indicated ({}) does not exist.", msg)
            }
            ErrorKind::ErrorReadingFS => {
                write!(f, "An external error happened")
            }
            ErrorKind::OperationAbortedByUser => write!(f, "The user ended the operation"),
            ErrorKind::InvalidOption => write!(f, "The option you wrote is invalid. Aborting."),
        }
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::DirectoryDoesNotExist(msg) => {
                write!(f, "The directory you indicated ({}) does not exist.", msg)
            }
            ErrorKind::ErrorReadingFS => {
                write!(f, "An external error happened")
            }
            ErrorKind::OperationAbortedByUser => write!(f, "The user ended the operation"),
            ErrorKind::InvalidOption => write!(f, "The option you wrote is invalid. Aborting."),
        }
    }
}
impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}
