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
use crate::output::cli;
use crate::sys::*;
use std::fs::create_dir_all;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{fs::DirEntry, path::PathBuf};
pub fn backup_preparations(
    origin_dir: PathBuf,
    target_dir: PathBuf,
) -> Result<(), crate::error::Error> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let _wrap = super::wrapper::Wrapper;
    cli(rx)?;
    start_backup(origin_dir, target_dir, tx)
}
fn start_backup(
    origin_dir: PathBuf,
    target_dir: PathBuf,
    tx: Sender<String>,
) -> Result<(), crate::error::Error> {
    if !origin_dir.is_dir() {
        return Ok(());
    }
    for sub_dir in origin_dir.read_dir().unwrap() {
        backup_operations(
            sub_dir.map_err(|_| error::Error {
                kind: error::ErrorKind::IOError,
            })?,
            target_dir.clone(),
            tx.clone(),
        )
        .unwrap();
    }
    Ok(())
}
fn backup_operations(
    dir: DirEntry,
    target_dir: PathBuf,
    tx: Sender<String>,
) -> Result<(), crate::error::Error> {
    let path_to_target_file = target_dir.clone();
    // Add the path to the target directory
    let target_file = path_to_target_file.join(dir.file_name());
    // We check if the directory should be backed and if its a directory
    if is_excluded(dir.path().display().to_string().as_str())? {
        return Ok(());
    }
    if dir.path().is_symlink() {
        return Ok(());
    }
    match (
        should_be_backed(
            <&PathBuf as TryInto<OsType>>::try_into(&dir.path())
                .expect("Shouldn't panic as, file exists. If so, is an external error"),
            <&PathBuf as TryInto<OsType>>::try_into(&target_file)
                .expect("Shouldn't panic as, file exists. If so, is an external error"),
        ),
        dir.path().is_dir() || target_file.is_dir(),
    ) {
        // If shouldn't be backed we finish and return
        (false, _) => {
            println!("doesnt");
            Ok(())}
        ,
        // Should be backed and is a file. We copy the file and return
        (true, false) => {
            println!("Should be here");
            println!("{:?}, \n{}", dir.path().display(), dir.path().is_dir());
            println!("{:?}\n {}", &target_file, target_file.is_dir());
            // Copy contents
            tx.send(dir.path().display().to_string())
                .map_err(|_| error::Error {
                    kind: error::ErrorKind::IOError,
                })?;
            fs_extra::file::copy(
                dir.path(),
                target_file,
                &fs_extra::file::CopyOptions {
                    overwrite: true,
                    ..Default::default()
                },
            )
            .unwrap();
            //           .map_err(|_| error::Error {
            //               kind: error::ErrorKind::FSError,
            //           })?;
            Ok(())
        }
        // If should be backed and is a directory we check inside the directory for files to be
        // backed
        (true, true) => {
            //Check inside directory
            tx.send(dir.path().display().to_string())
                .map_err(|_| error::Error {
                    kind: error::ErrorKind::IOError,
                })?;
            let _ = create_dir_all(&path_to_target_file);
            self::start_backup(dir.path(), target_file, tx)?;
            Ok(())
        }
    }
}
