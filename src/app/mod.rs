use crate::error::*;

use std::path::{Path, PathBuf};

use self::initial_copy::initial_copy;

mod initial_copy;
mod operations;

pub fn backup(origin_dir: PathBuf, mut target_dir: PathBuf) -> Result<()> {
    let path_target_dir: Box<Path> = target_dir.clone().into();
    // If the target directory is empty is not worth checking the times
    // Just copy it directly

    if path_target_dir.read_dir().unwrap().next().is_none() {
        initial_copy(origin_dir, target_dir);
    }
    target_dir.push(origin_dir.ancestors().next().unwrap());
    start_backup(origin_dir, target_dir)?;
    Ok(())
}
fn start_backup(origin_dir: PathBuf, target_dir: PathBuf) -> Result<()> {
    for sub_dir in origin_dir.read_dir().unwrap() {
        operations::backup_operations(sub_dir, target_dir.clone())?;
    }
    Ok(())
}
