use crate::error::*;

use std::path::{Path, PathBuf};

use initial_copy::initial_copy;
use operations::backup_preparations;

mod initial_copy;
mod operations;

pub fn backup(origin_dir: PathBuf, mut target_dir: PathBuf) -> Result<()> {
    let path_target_dir: Box<Path> = target_dir.clone().into();
    // If the target directory is empty is not worth checking the times
    // Just copy it directly

    if path_target_dir.read_dir().unwrap().next().is_none() {
        return initial_copy(origin_dir, target_dir);
    }
    target_dir.push(origin_dir.ancestors().next().unwrap());
    backup_preparations(origin_dir, target_dir)?;
    Ok(())
}
