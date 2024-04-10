use crate::error::*;

use std::path::{Path, PathBuf};

use initial_copy::initial_copy;
use operations::backup_preparations;

mod initial_copy;
mod operations;
mod wrapper;

/// The function that starts the backup
pub fn backup(origin_dir: PathBuf, target_dir: PathBuf) -> Result<()> {
    let path_target_dir: Box<Path> = target_dir.clone().into();
    // If the target directory is empty is not worth checking the times
    // Just copy it directly
    if !path_target_dir
        .read_dir()
        .expect("Temporary")
        .map(|path| {
            path.expect("Temporary")
                .path()
                .canonicalize()
                .expect("Shouldn't panic")
        })
        .any(|path| path.iter().last() == origin_dir.iter().last())
    {
        return initial_copy(origin_dir, target_dir);
    }
    let target_dir = target_dir.join(origin_dir.iter().last().expect("Temporary"));
    backup_preparations(origin_dir, target_dir)
}
