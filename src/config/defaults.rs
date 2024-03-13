use std::path::PathBuf;

use crate::error::Result;

use serde::{Deserialize, Serialize};

#[cfg(unix)]
const DEFAULTS_PATH: &str = "/etc/klone/defaults.json";
#[cfg(windows)]
const DEFAULTS_PATH: &str = "C:\\ProgramData\\klone\\exclusions.json";
#[derive(Serialize, Deserialize, Debug)]
pub struct Defaults {
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
    Ok(get_defaults().expect("Temporary").origin())
}
pub fn get_default_target() -> Result<PathBuf> {
    Ok(get_defaults().expect("Temporary").target())
}
pub fn set_defaults() -> Result<()> {
    println!("Remember that this configurations are global and affect every user in the computer");
    Ok(())
}
pub fn print_defaults() -> Result<()> {
    let defaults = self::get_defaults().expect("Temporary");
    println!("The current defaults:");
    println!("Origin directory: {}", defaults.origin().display());
    println!("Target directory: {}", defaults.target.display());
    Ok(())
}
impl Defaults {
    pub fn origin(&self) -> PathBuf {
        self.origin.clone()
    }
    pub fn target(&self) -> PathBuf {
        self.target.clone()
    }
}
