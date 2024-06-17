use crate::error::{Error, ErrorKind};
use fs_extra::dir::create_all;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
#[derive(Debug)]
pub struct UnixFileTime {
    creation_time: i64,
    just_created: bool,
}
impl super::File for UnixFileTime {
    fn get_time(&self) -> i128 {
        self.creation_time as i128
    }
    fn just_created(&self) -> bool {
        self.just_created
    }
}
impl TryFrom<&PathBuf> for UnixFileTime {
    type Error = crate::error::Error;
    // If a folder has an extension will be created as file (No problem with this)
    // If a file doesn't have it it will be created as folder (Half problem with this. Files with
    // no extension are the ones users don't usually work with)
    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        match fs::metadata(value) {
            Ok(file_metadata) => Ok(UnixFileTime {
                creation_time: file_metadata.mtime(),
                just_created: false,
            }),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {
                    match value.extension() {
                        None => {
                            create_all(value, false).unwrap();
                        }
                        Some(_) => {
                            create_file(value)?;
                        }
                    }
                    Ok(UnixFileTime {
                        creation_time: fs::metadata(value).expect("Checked").mtime(),
                        just_created: true,
                    })
                }
                _ => Err(Error {
                    kind: ErrorKind::FSError,
                }),
            },
        }
    }
}
fn create_file(path: &PathBuf) -> Result<(), Error> {
    if let Err(err) = fs::write(path, "") {
        println!("File");
        match err.kind() {
            io::ErrorKind::PermissionDenied => Err(Error {
                kind: ErrorKind::PermissionDenied,
            }),
            io::ErrorKind::AlreadyExists => Ok(()),
            _ => panic!("{:?}", err),
        }
    } else {
        Ok(())
    }
}
