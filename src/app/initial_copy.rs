use crate::config::exclusions::is_excluded;
use crate::error::{Error, ErrorKind};
use crate::output::cli;
use core::panic;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};

pub fn initial_copy(origin_dir: PathBuf, mut target_dir: PathBuf) -> Result<(), Error> {
    target_dir.push(origin_dir.iter().next_back().unwrap());
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let _wrap = super::wrapper::Wrapper;
    cli(rx)?;
    start_initial_copy(origin_dir, target_dir, tx).unwrap();
    Ok(())
}
fn start_initial_copy(
    origin_dir: PathBuf,
    target_dir: PathBuf,
    tx: Sender<String>,
) -> Result<(), Error> {
    for child in std::fs::read_dir(origin_dir).map_err(|_| Error {
        kind: ErrorKind::FSError,
    })? {
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
    if origin_dir.path().is_symlink() {
        return Ok(());
    }
    tx.send(origin_dir.path().display().to_string())
        .map_err(|_| Error {
            kind: ErrorKind::IOError,
        })?;
    if origin_dir.path().is_dir() {
        dbg!(origin_dir.path().display());
        match std::fs::create_dir_all(&target_dir) {
            Ok(()) => (),
            Err(err) => match err.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    return Err(Error {
                        kind: ErrorKind::PermissionDenied,
                    })
                }
                _ => (),
            },
        }
        return start_initial_copy(origin_dir.path(), target_dir, tx);
    }
    match fs_extra::file::copy(
        origin_dir.path(),
        target_dir,
        &fs_extra::file::CopyOptions {
            overwrite: true,
            ..Default::default()
        },
    ) {
        Ok(_) => (),
        Err(error) => match error.kind {
            fs_extra::error::ErrorKind::NotFound => (),
            _ => panic!("{:?}", error),
        },
    }
    Ok(())
}
