use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Exclusions(Vec<String>);

pub fn is_excluded(path: &PathBuf) -> Result<bool> {
    let exclusions_file = std::env::current_exe()
        .unwrap()
        .join("config")
        .join("exclusions.json");
    let file_contents = std::fs::read_to_string(&exclusions_file).unwrap();
    let deserialized: Exclusions = serde_json::from_str(&file_contents).unwrap();
    let mut iter = deserialized
        .0
        .iter()
        .filter(|file| **file == path.display().to_string());
    Ok(iter.next().is_some())
}
pub fn add_exclusion(path: &PathBuf) -> Result<()> {
    let exclusions_file = std::env::current_exe()
        .unwrap()
        .join("config")
        .join("exclusions.json");
    let file_contents = std::fs::read_to_string(&exclusions_file).unwrap();
    let mut deserialized: Exclusions = serde_json::from_str(&file_contents).unwrap();
    deserialized.0.push(path.display().to_string());
    std::fs::write(
        exclusions_file,
        serde_json::to_string(&deserialized).unwrap(),
    )
    .unwrap();
    Ok(())
}
