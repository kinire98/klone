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
    // Extract the value from the result
    let dir = match dir {
        Ok(value) => value,
        Err(_) => {
            return Err(error::Error {
                kind: error::ErrorKind::FSError,
            })
        }
    };
    let mut target_subdir = target_dir.clone();
    // Add the path to the target directory
    target_subdir.push(dir.file_name());
    // We check if the directory should be backed and if its a directory
    match (
        should_be_backed(
            <PathBuf as TryInto<OsType>>::try_into(dir.path()).unwrap(),
            <PathBuf as TryInto<OsType>>::try_into(target_subdir.clone()).unwrap(),
        ),
        dir.path().is_dir(),
    ) {
        // If shouldn't be backed we finish and return
        (false, _) => Ok(()),
        // Should be backed and is a file. We copy the file and return
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
        // If should be backed and is a directory we check inside the directory for files to be
        // backed
        (true, true) => {
            //Check inside directory
            super::backup(dir.path(), target_subdir)?;
            Ok(())
        }
    }
}
