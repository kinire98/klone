use crate::config::exclusions::is_excluded;
use crate::error::{Error, ErrorKind};
use std::fs::DirEntry;
use std::path::PathBuf;

pub fn initial_copy(origin_dir: PathBuf, mut target_dir: PathBuf) -> Result<(), Error> {
    target_dir.push(origin_dir.iter().next_back().unwrap());
    start_initial_copy(origin_dir, target_dir)
}
fn start_initial_copy(origin_dir: PathBuf, target_dir: PathBuf) -> Result<(), Error> {
    for child in std::fs::read_dir(origin_dir).unwrap() {
        copy_operations(child, target_dir.clone())?;
    }
    Ok(())
}
fn copy_operations(
    origin_dir: Result<DirEntry, std::io::Error>,
    target_dir: PathBuf,
) -> Result<(), Error> {
    let origin_dir = origin_dir.map_err(|_| Error {
        kind: ErrorKind::FSError,
    })?;
    let path_target_dir = target_dir.clone();
    let target_dir = path_target_dir.join(origin_dir.file_name());
    if is_excluded(origin_dir.path().display().to_string().as_str())? {
        return Ok(());
    }
    if origin_dir.path().is_dir() {
        std::fs::create_dir_all(&target_dir).unwrap();
        return start_initial_copy(origin_dir.path(), target_dir);
    }
    std::fs::create_dir_all(&path_target_dir).unwrap();
    fs_extra::file::copy(
        origin_dir.path(),
        target_dir,
        &fs_extra::file::CopyOptions {
            overwrite: true,
            ..Default::default()
        },
    )
    .map_err(|_| Error {
        kind: ErrorKind::FSError,
    })?;
    Ok(())
}
