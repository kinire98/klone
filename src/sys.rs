pub fn should_be_backed(file_to_backup: impl File, already_backed_file: impl File) -> bool {
    if already_backed_file.just_created() {
        return true;
    }
    #[cfg(windows)]
    if file_to_backup.is_folder() {
        return true;
    }
    println!(
        "{} > {}",
        file_to_backup.get_time(),
        already_backed_file.get_time()
    );
    file_to_backup.get_time() > already_backed_file.get_time()
}
trait File {
    fn get_time(&self) -> i128;
    fn just_created(&self) -> bool;
    #[cfg(windows)]
    fn is_folder(&self) -> bool;
}
#[cfg(windows)]
pub mod windows {
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
    impl TryFrom<PathBuf> for WindowsFileTime {
        type Error = crate::error::Error;
        fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
            let path: Box<Path> = value.clone().into();
            let metadata = match fs::metadata(value) {
                Ok(time) => time,
                Err(_) => {
                    return Err(Error {
                        kind: ErrorKind::FSError,
                    })
                }
            };
            Ok(WindowsFileTime {
                time: metadata.last_write_time(),
                is_folder: path.is_dir(),
            })
        }
    }
}
#[cfg(unix)]
pub mod unix {
    use crate::error::{Error, ErrorKind};
    use fs_extra::dir::create_all;
    use std::fs;
    use std::io;
    use std::os::unix::fs::MetadataExt;
    use std::path::PathBuf;
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
    impl TryFrom<PathBuf> for UnixFileTime {
        type Error = crate::error::Error;
        fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
            match fs::metadata(&value) {
                Ok(file_metadata) => Ok(UnixFileTime {
                    creation_time: file_metadata.mtime(),
                    just_created: false,
                }),
                Err(err) => match err.kind() {
                    io::ErrorKind::NotFound => {
                        create_all(&value, false).unwrap();
                        Ok(UnixFileTime {
                            creation_time: fs::metadata(&value).unwrap().mtime(),
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
}
