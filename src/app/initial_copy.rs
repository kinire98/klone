use crate::error::*;
use fs_extra::copy_items;
use std::path::{Path, PathBuf};

pub fn initial_copy(origin_dir: PathBuf, target_dir: PathBuf) -> Result<()> {
    // TODO Fix the initial copy to support exclusions
    let dir: Box<Path> = origin_dir.into();
    match copy_items(&[dir], target_dir, &fs_extra::dir::CopyOptions::default()) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error {
            kind: ErrorKind::FSError,
        }),
    }
}
