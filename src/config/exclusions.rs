use std::io::{self, Write};

use crate::error::*;
use glob::Pattern;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Exclusions(Vec<String>);

pub fn is_excluded(pattern: &str) -> Result<bool> {
    let exclusions_file = std::env::current_dir()
        .unwrap()
        .join("config")
        .join("exclusions.json");
    // Get the file contents
    let file_contents = std::fs::read_to_string(&exclusions_file).map_err(|_| Error {
        kind: ErrorKind::FSError,
    })?;
    // Deserialize the json
    let deserialized: Exclusions =
        serde_json::from_str(&file_contents).expect("Should be valid JSON");
    let mut iter = deserialized.0.iter().filter(|file| {
        Pattern::new(file.as_str())
            .expect("This should't panic") // The pattern was already checked when added
            .matches(pattern)
    });
    // If there is some, that means that the file should be excluded
    Ok(iter.next().is_some())
}
pub fn add_exclusion() -> Result<()> {
    let exclusions_file = std::env::current_dir()
        .unwrap()
        .join("config")
        .join("exclusions.json");
    // Get the already existing exclusions
    let binding = get_pattern("Add the pattern to exclude: ")?;
    let pattern = binding.as_str();
    let mut deserialized: Exclusions = serde_json::from_str(
        &std::fs::read_to_string(&exclusions_file).map_err(|_| Error {
            kind: ErrorKind::FSError,
        })?,
    )
    .map_err(|_| Error {
        kind: ErrorKind::JSONParsingError("exclusions".to_string()),
    })?;
    // Check if valid pattern
    if Pattern::new(pattern).is_err() {
        return Err(Error {
            kind: ErrorKind::InvalidPattern(pattern.to_string()),
        });
    }
    // Check if pattern already exists
    if deserialized.0.contains(&pattern.to_string()) {
        return Err(Error {
            kind: ErrorKind::PatternAlreadyExist,
        });
    }
    // Store pattern
    deserialized.0.push(pattern.to_string());
    // Write it to the file
    let _ = std::fs::write(
        exclusions_file,
        serde_json::to_string(&deserialized).map_err(|_| Error {
            kind: ErrorKind::JSONStringifyingError("exclusions".to_string()),
        })?,
    )
    .map_err(|_| Error {
        kind: ErrorKind::FSError,
    });
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
    // Read file and print it
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
pub fn remove_exclusion() -> Result<()> {
    let exclusions_file = std::env::current_dir()
        .map_err(|_| Error {
            kind: ErrorKind::FSError,
        })?
        .join("config")
        .join("exclusions.json");
    // Binding for getting the info from the file and make the borrow checher happy
    let binding = get_pattern("Introduce the pattern to remove: ")?;
    let pattern = binding.as_str();
    let binding = serde_json::from_str::<Exclusions>(
        &std::fs::read_to_string(&exclusions_file).map_err(|_| Error {
            kind: ErrorKind::FSError,
        })?,
    )
    .map_err(|_| Error {
        kind: ErrorKind::JSONParsingError("exclusions".to_string()),
    })?;
    // Filter out all of the elements that are equal to the exclusion
    // With the filter if it doesn't match returns true and doesn't get filtered
    let rest_of_values: Vec<&String> = binding
        .0
        .iter()
        .filter(|stored_pattern| *stored_pattern != &pattern)
        .collect();
    // Write to disc
    let _ = std::fs::write(
        &exclusions_file,
        serde_json::to_string(&rest_of_values).map_err(|_| Error {
            kind: ErrorKind::JSONStringifyingError("exclsions".to_string()),
        })?,
    );
    // Checks if some exclusion was deleted
    // If rest of values len is less than the binding len something has veen removed
    if rest_of_values.len() == binding.0.len() {
        println!("No exclusion was deleted. Make sure you wrote it right");
    } else if rest_of_values.len() < binding.0.len() {
        println!("The following exclusion has been removed: {}", pattern);
        println!("These are the remaining exclusions:");
        list_exclusions()?;
    } else {
        println!("WTF!? How did you get here?");
    }
    Ok(())
}

fn get_pattern(message: &str) -> Result<String> {
    print!("{}", message);
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| Error {
        kind: ErrorKind::IOError,
    })?;
    input.pop();
    Ok(input)
}
