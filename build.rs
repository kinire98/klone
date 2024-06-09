use std::fs;
use std::path::PathBuf;

#[cfg(windows)]
fn main() {
    let program_data = PathBuf::from("C:\\ProgramData\\klone");
    if program_data.is_dir() {
        return;
    }
    let _ = fs::create_dir_all(&program_data);
    let _ = fs::write(
        program_data.join("exclusions.json"),
        format!("[{:?},{:?},{:?},{:?}]", "*/.git", "*.o", "*.bin", "*.lock"),
    );
    let _ = fs::write(
        program_data.join("defaults.json"),
        format!(
            "{} {:?}:{:?}, {:?}:{:?} {}\n",
            "{", "origin", "None", "target", "None", "}"
        ),
    );
}
#[cfg(unix)]
fn main() {
    // Not the best for security but, only way I can think of doing this

    use nix::unistd::Uid;
    use std::io::Write;
    use std::process::Command;
    if PathBuf::from("/etc/klone/").is_dir() {
        return;
    }
    let cur_user: String =
        String::from_utf8(Command::new("who | cut -d' ' -f1").output().unwrap().stdout).unwrap();
    if !Uid::effective().is_root() {
        let mut input = String::new();
        print!("Enter sudo password to create the app configuration: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = Command::new("sudo su")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        write!(command.stdin.unwrap(), "{}", input).unwrap();
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
            "{", "origin", "None", "target", "None", "}"
        ),
    );
    let _ = Command::new("sudo")
        .args(["chmod", "-R", "777", "/etc/klone/"])
        .output();
    if !Uid::effective().is_root() {
        Command::new("sudo").args([cur_user]).spawn().unwrap();
    }
}
