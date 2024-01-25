#[cfg(unix)]
use crate::sys::unix::*;
#[cfg(unix)]
type OsType = UnixFileTime;
#[cfg(windows)]
use crate::sys::windows::*;
#[cfg(windows)]
type OsType = WindowsFileTime;
use crate::error;
use crate::sys::*;
use std::{fs::DirEntry, io, path::PathBuf};
pub fn backup_operations(
    dir: Result<DirEntry, io::Error>,
    target_dir: PathBuf,
) -> Result<(), crate::error::Error> {
    let dir = dir.unwrap();
    let mut target_subdir = target_dir.clone();
    target_subdir.push(dir.file_name());
    match (
        should_be_backed(
            <PathBuf as TryInto<OsType>>::try_into(dir.path()).unwrap(),
            <PathBuf as TryInto<OsType>>::try_into(target_subdir.clone()).unwrap(),
        ),
        dir.path().is_dir(),
    ) {
        (false, _) => Ok(()),
        (true, false) => {
            // Copy contents
            match fs_extra::file::copy(
                dir.path(),
                target_subdir,
                &fs_extra::file::CopyOptions {
                    overwrite: true,
                    ..Default::default()
                },
            ) {
                Ok(_) => Ok(()),
                Err(_) => {
                    return Err(error::Error {
                        kind: error::ErrorKind::FSError,
                    });
                }
            }
        }
        (true, true) => {
            //Check inside directory
            super::start_backup(dir.path(), target_subdir)?;
            Ok(())
        }
    }
}
