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
    serde_json::from_str(&std::fs::read_to_string(DEFAULTS_PATH).map_err(|_| Error {
        kind: ErrorKind::FSError,
    })?)
    .map_err(|_| Error {
        kind: ErrorKind::JSONParsingError("defaults".to_string()),
    })
}
/// Returns the default origin stored in the config json file
pub fn get_default_origin() -> Result<PathBuf> {
    Ok(get_defaults()?.origin)
}
/// Returns the default target stored in the config json file
pub fn get_default_target() -> Result<PathBuf> {
    Ok(get_defaults()?.target)
}
/// Allows to change the default target and origin.   
/// Must be executed two times in order to change both.  
pub fn set_defaults(path: String) -> Result<()> {
    if path == "None" {
        return write_defaults(PathBuf::new());
    }
    let absolute_path = std::fs::canonicalize(&path).map_err(|_| Error {
        kind: ErrorKind::DirectoryDoesNotExist(path),
    })?;
    if absolute_path.is_file() {
        Err(Error {
            kind: ErrorKind::NotADirectory(absolute_path.display().to_string()),
        })?
    }
    write_defaults(absolute_path)
}
fn write_defaults(path: PathBuf) -> Result<()> {
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
                target: path,
                origin: defaults.origin,
            }
        }
        "o" => {
            defaults = Defaults {
                target: defaults.target,
                origin: path,
            }
        }
        _ => Err(Error {
            kind: ErrorKind::InvalidOption("The only valid options are `t` or `o`".to_string()),
        })?,
    }
    std::fs::write(
        DEFAULTS_PATH,
        serde_json::to_string(&defaults).map_err(|_| Error {
            kind: ErrorKind::JSONStringifyingError("defaults".to_string()),
        })?,
    )
    .map_err(|_| Error {
        kind: ErrorKind::FSError,
    })?;
    Ok(())
}
/// Prints to stdout the stored defaults
pub fn print_defaults() -> Result<()> {
    let defaults = self::get_defaults()?;
    println!("The current defaults:");
    println!("Origin directory: {}", defaults.origin.display());
    println!("Target directory: {}", defaults.target.display());
    Ok(())
}
