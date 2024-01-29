use crate::error::*;

use fs_extra::copy_items;

use std::path::{Path, PathBuf};

mod operations;

pub fn backup(origin_dir: PathBuf, target_dir: PathBuf) -> Result<()> {
    let path_target_dir: Box<Path> = target_dir.clone().into();
    // If the target directory is empty is not worth checking the times
    // Just copy it directly
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
    let _ = origin_dir.read_dir().unwrap().into_iter().map(|sub_dir| {
        Ok::<(), Error>(operations::backup_operations(sub_dir, target_dir.clone())?)
    });
    Ok(())
}