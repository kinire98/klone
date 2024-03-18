use std::io::{self, Write};
use std::path::PathBuf;

use crate::error::*;

use serde::{Deserialize, Serialize};

#[cfg(unix)]
const DEFAULTS_PATH: &str = "/etc/klone/defaults.json";
#[cfg(windows)]
const DEFAULTS_PATH: &str = r"C:\ProgramData\klone\defaults.json";
#[derive(Serialize, Deserialize, Debug)]
struct Defaults {
    origin: PathBuf,
    target: PathBuf,
}
fn get_defaults() -> Result<Defaults> {
    Ok(
        serde_json::from_str(&std::fs::read_to_string(DEFAULTS_PATH).expect("Temporary"))
            .expect("Temporary"),
    )
}
pub fn get_default_origin() -> Result<PathBuf> {
    Ok(get_defaults().expect("Temporary").origin)
}
pub fn get_default_target() -> Result<PathBuf> {
    Ok(get_defaults().expect("Temporary").target)
}
pub fn set_defaults(path: PathBuf) -> Result<()> {
    let absolute_path = std::fs::canonicalize(&path).map_err(|_| Error {
        kind: ErrorKind::DirectoryDoesNotExist(path.display().to_string()),
    })?;
    if absolute_path.is_file() {
        Err(Error {
            kind: ErrorKind::NotADirectory(absolute_path.display().to_string()),
        })?
    }
    println!("Remember that this configurations are global and affect every user in the computer");
    print!("What is this path for? Target[t]/Origin[o]:");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("An error has happened");
    input.pop();
    #[cfg(windows)]
    input.pop();
    let mut defaults = self::get_defaults()?;
    match input.as_str() {
        "t" => {
            defaults = Defaults {
                target: absolute_path,
                origin: defaults.origin,
            }
        }
        "o" => {
            defaults = Defaults {
                target: defaults.target,
                origin: absolute_path,
            }
        }
        _ => Err(Error {
            kind: ErrorKind::InvalidOption("The only valid options are `t` or `o`".to_string()),
        })?,
    }
    std::fs::write(
        DEFAULTS_PATH,
        serde_json::to_string(&defaults).expect("Temporary"),
    )
    .expect("Temporary");
    Ok(())
}
pub fn print_defaults() -> Result<()> {
    let defaults = self::get_defaults().expect("Temporary");
    println!("The current defaults:");
    println!("Origin directory: {}", defaults.origin.display());
    println!("Target directory: {}", defaults.target.display());
    Ok(())
}
