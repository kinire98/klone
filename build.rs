use std::fs;
use std::path::PathBuf;
#[cfg(unix)]
use std::process::Command;
#[cfg(windows)]
fn main() {
    let program_data = PathBuf::from("C:\\ProgramData\\klone");
    if program_data.is_dir() {
        return;
    }
    let _ = fs::create_dir_all(&program_data);
    let _ = fs::write(program_data.join("exclusion.json"), "[]");
    let _ = fs::write(
        program_data.join("defaults.json"),
        format!(
            "{} {:?}:{:?}, {:?}:{:?} {}\n",
            "{", "origin", "", "target", "", "}"
        ),
    );
}
#[cfg(unix)]
fn main() {
    // Not the best for security but, only way I can think of doing this
    // TODO Checks if the files exist
    if PathBuf::from("/etc/klone/").is_dir() {
        return;
    }
    let _ = Command::new("sudo").args(["mkdir", "/etc/klone"]).output();
    let _ = Command::new("sudo")
        .args(["chmod", "-R", "777", "/etc/klone/"])
        .output();
    let _ = fs::write("/etc/klone/exclusions.json", "[]");
    let _ = fs::write(
        "/etc/klone/defaults.json",
        format!(
            "{} {:?}:{:?}, {:?}:{:?} {}\n",
            "{", "origin", "", "target", "", "}"
        ),
    );
}
