use crate::error::*;

#[cfg(unix)]
const EXCLUSIONS_PATH: &str = "/etc/klone/exclusions.json";
#[cfg(windows)]
const EXCLUSIONS_PATH: &str = r"C:\ProgramData\klone\exclusions.json";

static mut CACHED_EXCLUSIONS: Vec<String> = Vec::new();

pub fn get_exclusions() -> Result<Vec<String>> {
    if unsafe { CACHED_EXCLUSIONS.is_empty() } {
        return unsafe { Ok(CACHED_EXCLUSIONS.clone()) };
    }
    // Get the file contents
    let file_contents = std::fs::read_to_string(EXCLUSIONS_PATH).map_err(|_| Error {
        kind: ErrorKind::FSError,
    })?;
    // Deserialize the json
    let deserialized: Vec<String> =
        serde_json::from_str(&file_contents).expect("Should be valid JSON");
    unsafe {
        CACHED_EXCLUSIONS = deserialized.clone();
    }
    Ok(deserialized)
}
