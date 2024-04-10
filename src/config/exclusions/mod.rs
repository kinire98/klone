use std::io::{self, Write};

use crate::error::*;
use glob::Pattern;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
#[cfg(unix)]
const EXCLUSIONS_PATH: &str = "/etc/klone/exclusions.json";
#[cfg(windows)]
const EXCLUSIONS_PATH: &str = r"C:\ProgramData\klone\exclusions.json";

#[cfg(windows)]
const SYS_EXCLUSIONS: &[&str] = &["*/.git", "*.o", "*.bin", "*.lock"];

#[derive(Serialize, Deserialize, Debug)]
struct Exclusions(Vec<String>);

mod cache;
/// Tells if a file or folder is excluded based on the patterns stored in the JSON file
pub fn is_excluded(pattern: &str) -> Result<bool> {
    let binding = cache::get_exclusions()?;
    let mut iter = binding.iter().filter(|file| {
        Pattern::new(file)
            .expect("This should't panic") // The pattern was already checked when added
            .matches(pattern)
    });
    // If there is some, that means that the file should be excluded
    Ok(iter.next().is_some())
}
/// Adds the exclusion to the JSON file.  
/// To see more info about these patterns check the [project's wiki](https://github.com/kinire98/klone/wiki/Exclusions)
/// or the [glob Pattern Struct documentation](https://docs.rs/glob/latest/glob/struct.Pattern.html)
pub fn add_exclusion() -> Result<()> {
    // Get the already existing exclusions
    let binding = get_pattern("Add the pattern to exclude: ")?;
    let pattern = binding.as_str();
    let mut deserialized: Exclusions = serde_json::from_str(
        &std::fs::read_to_string(EXCLUSIONS_PATH).map_err(|_| Error {
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
    std::fs::write(
        EXCLUSIONS_PATH,
        serde_json::to_string(&deserialized).map_err(|_| Error {
            kind: ErrorKind::JSONStringifyingError("exclusions".to_string()),
        })?,
    )
    .map_err(|_| Error {
        kind: ErrorKind::FSError,
    })?;
    Ok(())
}
/// Prints all the exclusions to stdout
pub fn list_exclusions() -> Result<()> {
    let mut counter = 1;
    // Read file and print it
    serde_json::from_str::<Exclusions>(&std::fs::read_to_string(EXCLUSIONS_PATH).map_err(
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
/// Removes exclusions from the JSON file.  
/// In Windows you won't be able to remote the .git, *.o, *.bin and *.lock files.  
/// This is because an issue with the Windows file permission system that tells that files that
/// match these patterns are always blocked and you don't haver permission to work with them.
pub fn remove_exclusion() -> Result<()> {
    // Binding for getting the info from the file and make the borrow checher happy
    let binding = get_pattern("Introduce the pattern to remove: ")?;
    #[cfg(windows)]
    if SYS_EXCLUSIONS.contains(&binding.as_str()) {
        println!(
            "You can't remove the .git, *.o, *.bin and *.lock exclusion in Windows, because an issue with the permissions. Windows tells they are alwaysblockes"
        );
        return Ok(());
    }
    let pattern = binding.as_str();
    let binding = serde_json::from_str::<Exclusions>(
        &std::fs::read_to_string(EXCLUSIONS_PATH).map_err(|_| Error {
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
        .filter(|stored_pattern| *stored_pattern != pattern)
        .collect();
    // Write to disc
    let _ = std::fs::write(
        EXCLUSIONS_PATH,
        serde_json::to_string(&rest_of_values).map_err(|_| Error {
            kind: ErrorKind::JSONStringifyingError("exclsions".to_string()),
        })?,
    );
    // Checks if some exclusion was deleted
    // If rest of values len is less than the binding len something has veen removed
    match rest_of_values.len().cmp(&binding.0.len()) {
        Ordering::Equal => println!("No exclusion was deleted. Make sure you wrote it right"),
        Ordering::Less => {
            println!("The following exclusion has been removed: {}", pattern);
            println!("These are the remaining exclusions:");
            list_exclusions()?;
        }
        Ordering::Greater => println!("WTF!? How did you get here?"),
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
    #[cfg(windows)]
    input.pop();
    Ok(input)
}
