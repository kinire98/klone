use crate::error::*;
use crate::sys::should_be_backed;
#[cfg(unix)]
use crate::sys::unix::*;
use fs_extra::copy_items;
#[cfg(unix)]
type OsType = UnixFileTime;
#[cfg(windows)]
use crate::sys::windows::*;
#[cfg(windows)]
type OsType = WindowsFileTime;

use std::path::{Path, PathBuf};

pub fn backup(origin_dir: PathBuf, target_dir: PathBuf) -> Result<()> {
    let path_target_dir: Box<Path> = target_dir.clone().into();
    if path_target_dir.read_dir().unwrap().next().is_none() {
        let dir: Box<Path> = origin_dir.into();
        match copy_items(&[dir], target_dir, &fs_extra::dir::CopyOptions::default()) {
            Ok(_) => return Ok(()),
            Err(_) => {
                return Err(Error {
                    kind: ErrorKind::FSError,
                })
            }
        }
    }

    // Some testing :)
    println!("origin_dir: {} ", origin_dir.display());
    println!("target_dir: {} ", target_dir.display());
    println!(
        "Should be backed: {}",
        should_be_backed(
            <PathBuf as TryInto<OsType>>::try_into(origin_dir).unwrap(),
            <PathBuf as TryInto<OsType>>::try_into(target_dir).unwrap()
        )
    );
    Ok(())
}
