use crate::error::{Error, ErrorKind};
use fs_extra::dir::create_all;
use std::fs;
use std::os::windows::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct WindowsFileTime {
    time: u64,
    is_folder: bool,
    just_created: bool,
}
impl super::File for WindowsFileTime {
    fn get_time(&self) -> i128 {
        self.time as i128
    }
    fn is_folder(&self) -> bool {
        self.is_folder
    }
    fn just_created(&self) -> bool {
        self.just_created
    }
}
impl TryFrom<&PathBuf> for WindowsFileTime {
    type Error = crate::error::Error;
    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let metadata = match fs::metadata(value) {
            Ok(time) => time,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    match value.extension() {
                        None => {
                            create_all(value, false).unwrap();
                        }
                        Some(_) => {
                            create_file(value)?;
                        }
                    }
                    return Ok(WindowsFileTime {
                        time: fs::metadata(value).unwrap().last_write_time(),
                        just_created: true,
                        is_folder: value.is_dir(),
                    });
                }
                _ => {
                    return Err(Error {
                        kind: ErrorKind::FSError,
                    })
                }
            },
        };
        Ok(WindowsFileTime {
            time: metadata.last_write_time(),
            is_folder: value.is_dir(),
            just_created: false,
        })
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
