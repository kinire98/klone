// /etc/klone
//
// TODO FINISH THE CREATION OF CONFIG FILES
use std::process::Command;
#[cfg(windows)]
fn main() {
    let app_data = PathBuf::from(std::env::var("APP_DATA").expect("No APP_DATA directory"))
        .join("AppData")
        .join("Roaming");
}
#[cfg(unix)]
fn main() {
    // Not the best for security but, only way I can think of doing this
    // TODO Checks if the files exist
    let output = Command::new("sudo")
        .args(&["mkdir", "/etc/klone"])
        .output()
        .expect("Command failed");
    println!("{:?}", output);
    Command::new("sudo")
        .args(&["chmod", "-R", "766", "/etc/klone/"])
        .output()
        .expect("Command for creating config files failed");
    std::fs::write("/etc/klone/exclusions.json", "[]").expect("Write failed");
    std::fs::write(
        "/etc/klone/defaults.json",
        format!(
            "{} {:?}:{:?}, {:?}:{:?} {}\n",
            "{", "origin", "", "target", "", "}"
        ),
    )
    .expect("Write failed");
}
