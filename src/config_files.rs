use crate::error::*;

pub fn generate_file_structure() -> Result<()> {
    let exclusions_file = std::env::current_dir()
        .map_err(|_| Error {
            kind: ErrorKind::IOError,
        })?
        .parent()
        .unwrap();
    Ok(())
}
