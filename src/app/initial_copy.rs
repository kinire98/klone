use crate::config::exclusions::is_excluded;
use crate::error::{Error, ErrorKind};
use crate::output::cli;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};

pub fn initial_copy(origin_dir: PathBuf, mut target_dir: PathBuf) -> Result<(), Error> {
    target_dir.push(origin_dir.iter().next_back().unwrap());
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let _wrap = super::wrapper::Wrapper;
    cli(rx)?;
    start_initial_copy(origin_dir, target_dir, tx)
}
fn start_initial_copy(
    origin_dir: PathBuf,
    target_dir: PathBuf,
    tx: Sender<String>,
) -> Result<(), Error> {
    for child in std::fs::read_dir(origin_dir).unwrap() {
        copy_operations(
            child.map_err(|_| Error {
                kind: ErrorKind::IOError,
            })?,
            target_dir.clone(),
            tx.clone(),
        )?;
    }
    Ok(())
}
fn copy_operations(
    origin_dir: DirEntry,
    target_dir: PathBuf,
    tx: Sender<String>,
) -> Result<(), Error> {
    let path_target_dir = target_dir.clone();
    let target_dir = path_target_dir.join(origin_dir.file_name());
    if is_excluded(origin_dir.path().display().to_string().as_str())? {
        return Ok(());
    }
    tx.send(target_dir.display().to_string())
        .map_err(|_| Error {
            kind: ErrorKind::IOError,
        })?;
    if origin_dir.path().is_dir() {
        std::fs::create_dir_all(&target_dir).unwrap();
        return start_initial_copy(origin_dir.path(), target_dir, tx);
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
