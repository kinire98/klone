use std::{
    error,
    fmt::{Debug, Display},
};

/// Generic `Result<T>` type for the application
pub type Result<T> = std::result::Result<T, Error>;
/// The Struct containing the ErrorKind enum
pub struct Error {
    pub kind: ErrorKind,
}
/// The ErrorKind enum that states all possible errors that can happen in the application and
/// error messagges that are shown to the user in case those happen
pub enum ErrorKind {
    DirectoryDoesNotExist(String),
    InvalidOption(String),
    InvalidPattern(String),
    JSONParsingError(String),
    JSONStringifyingError(String),
    NotADirectory(String),
    TargetDirMTimeHigherThanOriginDir,
    FSError,
    IOError,
    OperationAbortedByUser,
    PatternAlreadyExist,
    TargetDirInsideOrigin,
    PermissionDenied,
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
            ErrorKind::NotADirectory(path) => write!(f, "The path you provided `{}` is a file when it should be a directory", path),
            ErrorKind::FSError | ErrorKind::IOError => {
                write!(f, "An external error happened")
            }
            ErrorKind::OperationAbortedByUser => write!(f, "The user ended the operation"),
            ErrorKind::TargetDirMTimeHigherThanOriginDir => {
                write!(f, "The directory where you want to store the backup has a modification time lower than the directory of origin for the backup.\n This means that you modified some data in thetarget directory after the last time you changed some data in the directory of origin.")
            },
            ErrorKind::PatternAlreadyExist => write!(f, "The pattern you introduced already exists"),
            ErrorKind::TargetDirInsideOrigin => write!(f, "The directory where you want to store the backup can't be inside or be a child of the directory to back"),
            ErrorKind::PermissionDenied => write!(f, "Make sure you have the correct permissions to read and write in both folders"),
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
            ErrorKind::NotADirectory(path) => write!(f, "The path you provided `{}` is a file when it should be a directory", path),
            ErrorKind::FSError | ErrorKind::IOError => {
                write!(f, "An external error happened")
            }
            ErrorKind::OperationAbortedByUser => write!(f, "The user ended the operation"),
            ErrorKind::TargetDirMTimeHigherThanOriginDir => {
                write!(f,"The directory where you want to store the backup has a modification time lower than the directory of origin for the backup.\n This means that you modified some data in thetarget directory after the last time you changed some data in the directory of origin.")
            },
            ErrorKind::PatternAlreadyExist => write!(f, "The pattern you introduced already exists"),
            ErrorKind::TargetDirInsideOrigin => write!(f, "The directory where you want to store the backup can't be inside or be a child of the directory to back"),
            ErrorKind::PermissionDenied => write!(f, "Make sure you have the correct permissions to read and write in both folders"),
            ErrorKind::UndefinedError => write!(f, "An undefined error has ocurred"),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}
