use std::{error, fmt::{Debug, Display}};

pub type Result<T> = std::result::Result<T, Error>;
pub struct Error {
    pub kind: ErrorKind,
}
pub enum ErrorKind {
    DirectoryDoesNotExist(String),
    InvalidOption(String),
    InvalidPattern(String),
    JSONParsingError(String),
    JSONStringifyingError(String),
    TargetDirMTimeHigherThanOriginDir,
    FSError,
    IOError,
    OperationAbortedByUser,
    PatternAlreadyExist,
    TargetDirInsideOrigin,
    UndefinedError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::DirectoryDoesNotExist(msg) => {
                write!(f, "The directory you indicated ({}) does not exist.", msg)
            }
            ErrorKind::InvalidOption(msg) => write!(f, "The option you wrote is invalid: {}", msg),
            ErrorKind::JSONParsingError(exclusions_or_configuration) => write!(f, "There was a problem parsing the file with the {}", exclusions_or_configuration),
            ErrorKind::JSONStringifyingError(exclusions_or_configuration) => write!(f, "An error occured storing the {}", exclusions_or_configuration),
            ErrorKind::InvalidPattern(invalid_pattern) => write!(f, "The file pattern you introduced to exclude: \n{}\nis not valid", invalid_pattern),
            ErrorKind::FSError | ErrorKind::IOError => {
                write!(f, "An external error happened")
            }
            ErrorKind::OperationAbortedByUser => write!(f, "The user ended the operation"),
            ErrorKind::TargetDirMTimeHigherThanOriginDir => {
                write!(f, 
                    "The directory where you want to store the backup has a modification time lower than the directory of origin for the backup.\n This means that you modified some data in thetarget directory after the last time you changed some data in the directory of origin."
                    )
            },
            ErrorKind::PatternAlreadyExist => write!(f, "The pattern you introduced already exists"),
            ErrorKind::TargetDirInsideOrigin => write!(f, "The directory where you want to store the backup can't be inside or be a child of the directory to back"),
            ErrorKind::UndefinedError => write!(f, "An undefined error has ocurred"),
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
            ErrorKind::JSONParsingError(exclusions_or_configuration) => write!(f, "There was a problem parsing the file with the {}", exclusions_or_configuration),
            ErrorKind::JSONStringifyingError(exclusions_or_configuration) => write!(f, "An error occured storing the {}", exclusions_or_configuration),
            ErrorKind::InvalidPattern(invalid_pattern) => write!(f, "The file pattern you introduced to exclude: \n{}\nis not valid", invalid_pattern),
            ErrorKind::FSError | ErrorKind::IOError => {
                write!(f, "An external error happened")
            }
            ErrorKind::OperationAbortedByUser => write!(f, "The user ended the operation"),
            ErrorKind::TargetDirMTimeHigherThanOriginDir => {
                write!(f, 
                    "The directory where you want to store the backup has a modification time lower than the directory of origin for the backup.\n This means that you modified some data in thetarget directory after the last time you changed some data in the directory of origin."
                    )
            },
            ErrorKind::PatternAlreadyExist => write!(f, "The pattern you introduced already exists"),
            ErrorKind::TargetDirInsideOrigin => write!(f, "The directory where you want to store the backup can't be inside or be a child of the directory to back"),
            ErrorKind::UndefinedError => write!(f, "An undefined error has ocurred"),
        }
    }
}


impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}
