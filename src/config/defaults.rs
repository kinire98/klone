use std::path::PathBuf;

use crate::error::Result;

pub struct Defaults {
    origin: PathBuf,
    target: PathBuf,
}
pub fn get_defaults() -> Result<Defaults> {
    Ok(Defaults {
        origin: PathBuf::new(),
        target: PathBuf::new(),
    })
}
pub fn set_defaults() -> Result<()> {
    Ok(())
}
impl Defaults {
    pub fn origin(&self) -> &PathBuf {
        &self.origin
    }
    pub fn target(&self) -> &PathBuf {
        &self.target
    }
}
