use crate::error::*;
use glob::Pattern;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Exclusions(Vec<String>);

pub fn is_excluded(path: &PathBuf) -> Result<bool> {
    let exclusions_file = std::env::current_dir()
        .unwrap()
        .join("config")
        .join("exclusions.json");
    let file_contents = std::fs::read_to_string(&exclusions_file).unwrap();
    let deserialized: Exclusions = serde_json::from_str(&file_contents).unwrap();
    let mut iter = deserialized.0.iter().filter(|file| {
        Pattern::new(file.as_str())
            .unwrap()
            .matches(path.display().to_string().as_str())
    });
    Ok(iter.next().is_some())
}
pub fn add_exclusion(path: &PathBuf) -> Result<()> {
    // TODO Dont add exclusion if it already exists
    let exclusions_file = std::env::current_dir()
        .unwrap()
        .join("config")
        .join("exclusions.json");
    let file_contents = std::fs::read_to_string(&exclusions_file).unwrap();
    let mut deserialized: Exclusions = serde_json::from_str(&file_contents).unwrap();
    if Pattern::new(path.display().to_string().as_str()).is_err() {
        return Err(Error {
            kind: ErrorKind::InvalidPattern(path.display().to_string()),
        });
    }
    deserialized.0.push(path.display().to_string());
    std::fs::write(
        exclusions_file,
        serde_json::to_string(&deserialized).unwrap(),
    )
    .unwrap();
    Ok(())
}
pub fn list_exclusions() -> Result<()> {
    let exclusions_file = std::env::current_dir()
        .map_err(|_| Error {
            kind: ErrorKind::FSError,
        })?
        .join("config")
        .join("exclusions.json");
    let mut counter = 1;
    serde_json::from_str::<Exclusions>(&std::fs::read_to_string(exclusions_file).map_err(
        |_| Error {
            kind: ErrorKind::FSError,
        },
    )?)
    .map_err(|_| Error {
        kind: ErrorKind::JSONParsingError("exclusions".to_string()),
    })?
    .0
    .iter()
    .for_each(|exclusion| {
        println!("{} -> {}", counter, exclusion);
        counter += 1;
    });
    Ok(())
}
pub fn remove_exclusion(pattern: &PathBuf) -> Result<String> {
    let exclusions_file = std::env::current_dir()
        .map_err(|_| Error {
            kind: ErrorKind::FSError,
        })?
        .join("config")
        .join("exclusions.json");
    let binding = serde_json::from_str::<Exclusions>(
        &std::fs::read_to_string(exclusions_file).map_err(|_| Error {
            kind: ErrorKind::FSError,
        })?,
    )
    .map_err(|_| Error {
        kind: ErrorKind::JSONParsingError("exclusions".to_string()),
    })?;
    let rest_of_values = binding
        .0
        .iter()
        .filter(|stored_pattern| *stored_pattern != &pattern.display().to_string());
    println!("{:?}", rest_of_values);
    Ok(String::new())
}
