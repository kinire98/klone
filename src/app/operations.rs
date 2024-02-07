#[cfg(unix)]
use crate::sys::unix::*;
#[cfg(unix)]
type OsType = UnixFileTime;
#[cfg(windows)]
use crate::sys::windows::*;
#[cfg(windows)]
type OsType = WindowsFileTime;
use crate::config::exclusions::is_excluded;
use crate::error;
use crate::sys::*;
use std::fs::create_dir_all;
use std::{fs::DirEntry, io, path::PathBuf};

pub fn backup_operations(
    dir: Result<DirEntry, io::Error>,
    target_dir: PathBuf,
) -> Result<(), crate::error::Error> {
    // Extract the value from the result
    let dir = dir.map_err(|_| error::Error {
        kind: error::ErrorKind::FSError,
    })?;
    let path_to_target_file = target_dir.clone();
    // Add the path to the target directory
    let target_file = path_to_target_file.join(dir.file_name());
    // We check if the directory should be backed and if its a directory
    match (
        should_be_backed(
            <&PathBuf as TryInto<OsType>>::try_into(&dir.path()).unwrap(),
            <&PathBuf as TryInto<OsType>>::try_into(&target_file).unwrap(),
        ),
        dir.path().is_dir(),
    ) {
        // If shouldn't be backed we finish and return
        (false, _) => Ok(()),
        // Should be backed and is a file. We copy the file and return
        (true, false) => {
            if is_excluded(dir.path().display().to_string().as_str())? {
                print!("{}", termion::clear::CurrentLine);
                println!("{} is excluded. Skipping", dir.path().display());
                return Ok(());
            }
            print!("{}", termion::clear::CurrentLine);
            println!("{} is being copied", dir.path().display());
            // Copy contents
            let _ = create_dir_all(&path_to_target_file);
            fs_extra::file::copy(
                dir.path(),
                target_file,
                &fs_extra::file::CopyOptions {
                    overwrite: true,
                    ..Default::default()
                },
            )
            .map_err(|_| error::Error {
                kind: error::ErrorKind::FSError,
            })?;
            Ok(())
        }
        // If should be backed and is a directory we check inside the directory for files to be
        // backed
        (true, true) => {
            //Check inside directory
            super::start_backup(
                dir.path(),
                target_file.join(dir.path().ancestors().last().expect("Should not panic")),
            )?;
            Ok(())
        }
    }
}
