use std::{error, fmt::Debug, fmt::Display};

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}
#[derive(Debug)]
pub enum ErrorKind {
    DirectoryDoesNotExist(String),
    InvalidOption(String),
    TargetDirMTimeHigherThanOriginDir,
    FSError,
    OperationAbortedByUser,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::DirectoryDoesNotExist(msg) => {
                write!(f, "The directory you indicated ({}) does not exist.", msg)
            }
            ErrorKind::InvalidOption(msg) => write!(f, "The option you wrote is invalid: {}", msg),
            ErrorKind::FSError => {
                write!(f, "An external error happened")
            }
            ErrorKind::OperationAbortedByUser => write!(f, "The user ended the operation"),
            ErrorKind::TargetDirMTimeHigherThanOriginDir => {
                write!(f, 
                    "The directory where you want to store the backup has a modification time lower than the directory of origin for the backup.\n This means that you modified some data in thetarget directory after the last time you changed some data in the directory of origin."
                    )
            }
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}
