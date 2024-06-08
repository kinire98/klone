use crate::error::{Error, ErrorKind};
use fs_extra::dir::create_all;
use std::fs;
use std::fs::OpenOptions;
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
    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        match fs::metadata(value) {
            Ok(file_metadata) => Ok(UnixFileTime {
                creation_time: file_metadata.mtime(),
                just_created: false,
            }),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {
                    if !value.is_file() && !value.is_symlink() {
                        create_all(value, false).unwrap();
                    } else if value.is_file() {
                        let _ = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(value.display().to_string())
                            .expect("Temporary");
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
