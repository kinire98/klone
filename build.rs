// Create the configuration files if they don't already exist

fn main() {
    let path = std::env::current_dir().unwrap();
    let path = path.join("config");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
        let path = path.join("defaults.json");
        if !path.exists() {
            std::fs::write(
                &path,
                format!(
                    "{} {:?}:{:?}, {:?}:{:?} {}\n",
                    "{", "origin", "", "target", "", "}"
                ),
            )
            .unwrap();
        }
        let path = path.parent().unwrap().join("exclusions.json");
        if !path.exists() {
            std::fs::write(&path, "[]\n").unwrap();
        }
    }
}
