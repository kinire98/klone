use crate::error::*;

pub fn should_be_backed(file_to_backup: impl File, already_backed_file: impl File) -> bool {
    #[cfg(windows)]
    if file_to_backup.is_folder() {
        true
    }
    file_to_backup.get_time() > already_backed_file.get_time()
}
trait File {
    fn get_time(&self) -> i64;
    #[cfg(windows)]
    fn is_folder(&self) -> bool;
}
#[cfg(windows)]
mod os_specific {}
#[cfg(unix)]
mod os_specific {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    use std::path::PathBuf;
    struct UnixFileTime(i64);
    impl super::File for UnixFileTime {
        fn get_time(&self) -> i64 {
            self.0
        }
    }
    // Maybe change from PathBuf to another structure
    impl TryFrom<PathBuf> for UnixFileTime {
        type Error = crate::error::Error;
        fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
            match fs::metadata(value) {
                Ok(file_metadata) => Ok(UnixFileTime(file_metadata.mtime())),
                Err(_) => Err(super::Error {
                    kind: super::ErrorKind::ErrorReadingFS,
                }),
            }
        }
    }
}
