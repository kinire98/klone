use crate::error::{Error, ErrorKind};
use std::fs;
use std::os::windows::prelude::MetadataExt;
use std::path::{Path, PathBuf};

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
        let path: Box<Path> = *value.clone().into();
        let metadata = match fs::metadata(value) {
            Ok(time) => time,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => todo!(),
                _ => {
                    return Err(Error {
                        kind: ErrorKind::FSError,
                    })
                }
            },
        };
        Ok(WindowsFileTime {
            time: metadata.last_write_time(),
            is_folder: path.is_dir(),
            just_created: false,
        })
    }
}
